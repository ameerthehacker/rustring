use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use oxc_resolver::{ResolveOptions, Resolver, EnforceExtension, TsconfigOptions, TsconfigReferences};
use petgraph::{algo, Graph};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    path::{Path, PathBuf},
    time::Instant,
};

#[derive(Clone, Debug, ValueEnum)]
enum OutputFormat {
    /// Human-readable text output (default)
    Text,
    /// JSON format for programmatic use
    Json,
    /// GraphViz DOT format for visualization
    Dot,
}

#[derive(Parser, Debug)]
#[command(name = "rustyring")]
#[command(about = "A tool for detecting circular dependencies in JavaScript/TypeScript projects")]
struct Args {
    /// Entry files to analyze
    #[arg(value_name = "ENTRY_FILES")]
    entry_files: Vec<PathBuf>,

    /// Project root directory (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    root: PathBuf,

    /// Show verbose output (only for text format)
    #[arg(short, long)]
    verbose: bool,

    /// Output format
    #[arg(short, long, value_enum, default_value = "text")]
    output: OutputFormat,

    /// Output file (defaults to stdout)
    #[arg(long)]
    output_file: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ImportInfo {
    from: PathBuf,
    to: String,
    resolved_to: Option<PathBuf>,
    line_number: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CircularDependency {
    cycle: Vec<PathBuf>,
}

#[derive(Serialize, Deserialize)]
struct AnalysisResult {
    files_processed: usize,
    imports_found: usize,
    circular_dependencies: Vec<CircularDependency>,
    imports: Vec<ImportInfo>,
}

struct DependencyAnalyzer {
    import_patterns: Vec<Regex>,
    visited_files: HashSet<PathBuf>,
    imports: Vec<ImportInfo>,
    graph: Graph<PathBuf, ()>,
    node_indices: HashMap<PathBuf, petgraph::graph::NodeIndex>,
    resolver_cache: HashMap<PathBuf, Resolver>,
    project_root: PathBuf,
}

impl DependencyAnalyzer {
    fn new(root: &Path) -> Result<Self> {
        // Regex patterns for different import styles
        let import_patterns = vec![
            // ES6 imports: import ... from '...'
            Regex::new(r#"import\s+(?:(?:\{[^}]*\}|\*\s+as\s+\w+|\w+)(?:\s*,\s*(?:\{[^}]*\}|\*\s+as\s+\w+|\w+))*\s+from\s+)?['""]([^'""\s]+)['""]"#)?,
            // CommonJS require: require('...')
            Regex::new(r#"require\s*\(\s*['""]([^'""\s]+)['""]"#)?,
            // Dynamic imports: import('...')
            Regex::new(r#"import\s*\(\s*['""]([^'""\s]+)['""]"#)?,
            // Re-exports: export ... from '...'
            Regex::new(r#"export\s+(?:\*|\{[^}]*\})\s+from\s+['""]([^'""\s]+)['""]"#)?,
        ];

        Ok(Self {
            import_patterns,
            visited_files: HashSet::new(),
            imports: Vec::new(),
            graph: Graph::new(),
            node_indices: HashMap::new(),
            resolver_cache: HashMap::new(),
            project_root: root.to_path_buf(),
        })
    }

    fn find_tsconfig_for_file(&self, file_path: &Path) -> Option<PathBuf> {
        let mut current_dir = file_path.parent()?;
        
        loop {
            let tsconfig_path = current_dir.join("tsconfig.json");
            if tsconfig_path.exists() {
                return Some(tsconfig_path);
            }
            
            // Stop if we've reached the project root or filesystem root
            if current_dir == self.project_root || current_dir.parent().is_none() {
                break;
            }
            
            current_dir = current_dir.parent()?;
        }
        
        None
    }

    fn get_resolver_for_file(&mut self, file_path: &Path) -> Result<&Resolver> {
        let tsconfig_path = self.find_tsconfig_for_file(file_path);
        
        // Use the tsconfig path (or None) as the cache key
        let cache_key = tsconfig_path.clone().unwrap_or_else(|| PathBuf::from("no_tsconfig"));
        
        if !self.resolver_cache.contains_key(&cache_key) {
            let tsconfig = if let Some(ref tsconfig_path) = tsconfig_path {
                Some(TsconfigOptions {
                    config_file: tsconfig_path.clone(),
                    references: TsconfigReferences::Auto,
                })
            } else {
                None
            };

            let resolver = Resolver::new(ResolveOptions {
                alias: vec![],
                alias_fields: vec![],
                builtin_modules: false,
                condition_names: vec!["import".to_string(), "require".to_string(), "node".to_string()],
                description_files: vec!["package.json".to_string()],
                enforce_extension: EnforceExtension::Auto,
                exports_fields: vec![vec!["exports".to_string()]],
                extension_alias: vec![],
                extensions: vec![
                    ".ts".to_string(),
                    ".tsx".to_string(),
                    ".js".to_string(),
                    ".jsx".to_string(),
                    ".mjs".to_string(),
                    ".cjs".to_string(),
                ],
                fallback: vec![],
                fully_specified: false,
                imports_fields: vec![vec!["imports".to_string()]],
                main_fields: vec!["main".to_string(), "module".to_string(), "browser".to_string()],
                main_files: vec!["index".to_string()],
                modules: vec!["node_modules".to_string()],
                resolve_to_context: false,
                prefer_relative: false,
                prefer_absolute: false,
                restrictions: vec![],
                roots: vec![self.project_root.clone()],
                symlinks: true,
                tsconfig,
            });
            
            self.resolver_cache.insert(cache_key.clone(), resolver);
        }
        
        Ok(self.resolver_cache.get(&cache_key).unwrap())
    }

    fn is_supported_file(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            matches!(extension, "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs")
        } else {
            false
        }
    }

    fn extract_imports_from_file(&mut self, file_path: &Path) -> Result<Vec<ImportInfo>> {
        let content = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

        let mut imports = Vec::new();
        let import_patterns = self.import_patterns.clone(); // Clone to avoid borrow conflicts

        for (line_num, line) in content.lines().enumerate() {
            for pattern in &import_patterns {
                for captures in pattern.captures_iter(line) {
                    if let Some(import_path) = captures.get(1) {
                        let import_str = import_path.as_str();

                        let mut import_info = ImportInfo {
                            from: file_path.to_path_buf(),
                            to: import_str.to_string(),
                            resolved_to: None,
                            line_number: line_num + 1,
                        };

                        // Get the appropriate resolver for this file and resolve the import
                        let resolver = self.get_resolver_for_file(file_path)?;
                        let base_dir = file_path.parent().unwrap_or(Path::new("."));
                        let resolved_path_option = match resolver.resolve(base_dir, import_str) {
                            Ok(resolution) => Some(resolution.path().to_path_buf()),
                            Err(_) => None,
                        };

                        // Skip if resolved path contains node_modules
                        if let Some(ref resolved_path) = resolved_path_option {
                            if resolved_path.to_string_lossy().contains("node_modules") {
                                continue;
                            }
                        }

                        import_info.resolved_to = resolved_path_option;

                        imports.push(import_info);
                    }
                }
            }
        }

        Ok(imports)
    }

    fn build_dependency_graph(&mut self, entry_files: &[PathBuf]) -> Result<()> {
        let mut queue = VecDeque::new();

        // Add entry files to the queue
        for entry_file in entry_files {
            if !self.is_supported_file(entry_file) {
                eprintln!("Warning: Skipping unsupported file: {}", entry_file.display());
                continue;
            }
            queue.push_back(entry_file.clone());
        }

        while let Some(current_file) = queue.pop_front() {
            if self.visited_files.contains(&current_file) {
                continue;
            }

            if !current_file.exists() {
                eprintln!("Warning: File does not exist: {}", current_file.display());
                continue;
            }

            self.visited_files.insert(current_file.clone());

            // Add current file to graph
            let current_node = *self.node_indices.entry(current_file.clone()).or_insert_with(|| {
                self.graph.add_node(current_file.clone())
            });

            // Extract imports from current file
            match self.extract_imports_from_file(&current_file) {
                Ok(file_imports) => {
                    for import in file_imports {
                        if let Some(resolved_path) = &import.resolved_to {
                            if self.is_supported_file(resolved_path) {
                                // Add target file to graph
                                let target_node = *self.node_indices.entry(resolved_path.clone()).or_insert_with(|| {
                                    self.graph.add_node(resolved_path.clone())
                                });

                                // Add edge from current file to imported file
                                self.graph.add_edge(current_node, target_node, ());

                                // Add to queue for processing
                                queue.push_back(resolved_path.clone());
                            }
                        }

                        self.imports.push(import);
                    }
                }
                Err(e) => {
                    eprintln!("Error processing file {}: {}", current_file.display(), e);
                }
            }
        }

        Ok(())
    }

    fn find_circular_dependencies(&self) -> Vec<CircularDependency> {
        let mut circular_deps = Vec::new();

        // Find all strongly connected components with more than one node
        let sccs = algo::tarjan_scc(&self.graph);
        
        for scc in sccs {
            if scc.len() > 1 {
                let cycle: Vec<PathBuf> = scc.iter()
                    .map(|&node_idx| self.graph[node_idx].clone())
                    .collect();
                
                circular_deps.push(CircularDependency { cycle });
            }
        }

        circular_deps
    }

    fn get_analysis_result(&self, circular_deps: &[CircularDependency]) -> AnalysisResult {
        AnalysisResult {
            files_processed: self.visited_files.len(),
            imports_found: self.imports.len(),
            circular_dependencies: circular_deps.to_vec(),
            imports: self.imports.clone(),
        }
    }
}

fn print_text_results(result: &AnalysisResult, verbose: bool, project_root: &Path) {
    println!("📊 Processed {} files", result.files_processed);
    println!("🔗 Found {} imports", result.imports_found);

    if result.circular_dependencies.is_empty() {
        println!("✅ No circular dependencies found!");
        return;
    }

    println!("🔴 Found {} circular dependencies:", result.circular_dependencies.len());
    println!();

    for (i, circular_dep) in result.circular_dependencies.iter().enumerate() {
        println!("Circular Dependency #{}:", i + 1);
        
        for (j, file) in circular_dep.cycle.iter().enumerate() {
            let next_file = if j == circular_dep.cycle.len() - 1 {
                &circular_dep.cycle[0] // Point back to first file to complete the circle
            } else {
                &circular_dep.cycle[j + 1]
            };

            let current_relative = file.strip_prefix(project_root).unwrap_or(file).to_string_lossy();
            let next_relative = next_file.strip_prefix(project_root).unwrap_or(next_file).to_string_lossy();

            if j == circular_dep.cycle.len() - 1 {
                println!("  └─ {} → {} (completes circle)", current_relative, next_relative);
            } else {
                println!("  ├─ {} → {}", current_relative, next_relative);
            }
        }
        
        if verbose {
            println!("  Dependencies involved:");
            for file in &circular_dep.cycle {
                let file_imports: Vec<&ImportInfo> = result.imports.iter()
                    .filter(|import| import.from == *file && 
                            circular_dep.cycle.iter().any(|f| Some(f) == import.resolved_to.as_ref()))
                    .collect();
                
                if !file_imports.is_empty() {
                    let file_relative = file.strip_prefix(project_root).unwrap_or(file).to_string_lossy();
                    println!("    From {}:", file_relative);
                    for import in file_imports {
                        if let Some(resolved) = &import.resolved_to {
                            let resolved_relative = resolved.strip_prefix(project_root).unwrap_or(resolved).to_string_lossy();
                            println!("      - Line {}: {} → {}", 
                                import.line_number, import.to, resolved_relative);
                        }
                    }
                }
            }
        }
        
        println!();
    }
}

fn generate_dot_output(result: &AnalysisResult, project_root: &Path) -> String {
    let mut dot = String::new();
    dot.push_str("digraph circular_dependencies {\n");
    dot.push_str("  rankdir=LR;\n");
    dot.push_str("  node [shape=box, style=rounded, color=red, fontcolor=red];\n");
    dot.push_str("  edge [fontsize=10, color=red, penwidth=2];\n\n");

    if result.circular_dependencies.is_empty() {
        dot.push_str("  no_cycles [label=\"No Circular Dependencies Found\", shape=ellipse, color=green, fontcolor=green];\n");
        dot.push_str("}\n");
        return dot;
    }

    // Collect all files involved in circular dependencies
    let mut circular_files = HashSet::new();
    for cycle in &result.circular_dependencies {
        for file in &cycle.cycle {
            circular_files.insert(file);
        }
    }

    // Create a mapping of file paths to node IDs (only for circular files)
    let mut file_to_id = HashMap::new();
    let mut node_counter = 0;

    // Create nodes only for files involved in circular dependencies
    for file in &circular_files {
        let id = format!("n{}", node_counter);
        file_to_id.insert((*file).clone(), id.clone());
        node_counter += 1;

        // Use relative path from project root
        let relative_path = file.strip_prefix(project_root).unwrap_or(file);
        let label = relative_path.to_string_lossy().to_string();

        dot.push_str(&format!("  {} [label=\"{}\"];\n", id, label));
    }

    dot.push_str("\n");

    // Create edges only between circular files
    for import in &result.imports {
        if let Some(ref resolved) = import.resolved_to {
            // Only include edges where both files are in circular dependencies
            if circular_files.contains(&import.from) && circular_files.contains(resolved) {
                if let (Some(from_id), Some(to_id)) = (file_to_id.get(&import.from), file_to_id.get(resolved)) {
                    dot.push_str(&format!("  {} -> {};\n", from_id, to_id));
                }
            }
        }
    }

    // Group circular dependencies visually
    for (cycle_idx, cycle) in result.circular_dependencies.iter().enumerate() {
        dot.push_str(&format!("\n  subgraph cluster_cycle_{} {{\n", cycle_idx));
        dot.push_str(&format!("    label=\"Circular Dependency #{}\";\n", cycle_idx + 1));
        dot.push_str("    style=dashed;\n");
        dot.push_str("    color=red;\n");
        
        for file in &cycle.cycle {
            if let Some(node_id) = file_to_id.get(file) {
                dot.push_str(&format!("    {};\n", node_id));
            }
        }
        
        dot.push_str("  }\n");
    }

    dot.push_str("}\n");
    dot
}

fn write_output(content: &str, output_file: Option<&PathBuf>) -> Result<()> {
    match output_file {
        Some(path) => {
            fs::write(path, content)
                .with_context(|| format!("Failed to write to file: {}", path.display()))?;
            eprintln!("Output written to: {}", path.display());
        }
        None => {
            print!("{}", content);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.entry_files.is_empty() {
        eprintln!("Error: At least one entry file must be provided");
        std::process::exit(1);
    }

    let start_time = Instant::now();

    if !matches!(args.output, OutputFormat::Json | OutputFormat::Dot) {
        println!("🔍 Analyzing dependencies...");
    }

    let mut analyzer = DependencyAnalyzer::new(&args.root)?;
    
    // Build dependency graph
    analyzer.build_dependency_graph(&args.entry_files)?;
    
    // Find circular dependencies
    let circular_deps = analyzer.find_circular_dependencies();
    let result = analyzer.get_analysis_result(&circular_deps);
    
    let elapsed = start_time.elapsed();
    
    // Generate output based on format
    match args.output {
        OutputFormat::Text => {
            print_text_results(&result, args.verbose, &args.root);
            print_timing_info(elapsed);
        }
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&result)
                .context("Failed to serialize result to JSON")?;
            write_output(&json, args.output_file.as_ref())?;
            eprintln!("⏱️  Analysis completed in {:.2?}", elapsed);
        }
        OutputFormat::Dot => {
            let dot = generate_dot_output(&result, &args.root);
            write_output(&dot, args.output_file.as_ref())?;
            eprintln!("⏱️  Analysis completed in {:.2?}", elapsed);
        }
    }

    // Exit with error code if circular dependencies found (except for output formats)
    if !circular_deps.is_empty() && matches!(args.output, OutputFormat::Text) {
        std::process::exit(1);
    }

    Ok(())
}

fn print_timing_info(elapsed: std::time::Duration) {
    let total_ms = elapsed.as_millis();
    
    if total_ms < 1000 {
        println!("⏱️  Analysis completed in {}ms", total_ms);
    } else {
        let seconds = elapsed.as_secs_f64();
        println!("⏱️  Analysis completed in {:.2}s", seconds);
    }
}
