# RustyRing 🦀

A fast and accurate circular dependency detector for JavaScript and TypeScript projects, built in Rust.

## Features

- 🔍 **Accurate Detection**: Uses regex patterns to extract imports from JS/TS files
- 🗺️ **Path Resolution**: Integrates with `oxc-resolver` to resolve aliased and relative imports
- 📊 **Dependency Graph**: Builds a complete dependency graph using strongly connected components
- 🎯 **Multiple Import Styles**: Supports ES6 imports, CommonJS require, dynamic imports, and re-exports
- ⚡ **Fast**: Written in Rust for optimal performance
- 📝 **Detailed Output**: Shows exact circular paths and line numbers with verbose mode

## Supported File Types

- `.js` - JavaScript files
- `.jsx` - JavaScript React components
- `.ts` - TypeScript files
- `.tsx` - TypeScript React components
- `.mjs` - ES Module JavaScript files
- `.cjs` - CommonJS files

## Supported Import Patterns

The tool detects the following import patterns:

```javascript
// ES6 imports
import { func } from './module.js';
import * as mod from './module.js';
import defaultExport from './module.js';

// CommonJS require
const { func } = require('./module.js');

// Dynamic imports
import('./module.js');

// Re-exports
export { func } from './module.js';
export * from './module.js';
```

## Installation

Clone this repository and build:

```bash
git clone <repository-url>
cd rustyring
cargo build --release
```

## Usage

### Basic Usage

Analyze a single entry file:

```bash
cargo run -- src/main.js
```

### Multiple Entry Files

Analyze multiple entry files:

```bash
cargo run -- src/main.js src/app.js src/utils.js
```

### Verbose Output

Get detailed information about dependencies and line numbers:

```bash
cargo run -- src/main.js --verbose
```

### Specify Project Root

Set a custom project root directory:

```bash
cargo run -- src/main.js --root /path/to/project
```

## Example Output

### No Circular Dependencies

```
🔍 Analyzing dependencies...
📊 Processed 10 files
🔗 Found 25 imports
✅ No circular dependencies found!
```

### Circular Dependencies Found

```
🔍 Analyzing dependencies...
📊 Processed 4 files
🔗 Found 5 imports
🔴 Found 2 circular dependencies:

Circular Dependency #1:
  ├─ src/utils.js →
  └─ src/helpers.js → src/utils.js (circular)

Circular Dependency #2:
  ├─ src/components/Button.jsx →
  └─ src/components/Modal.jsx → src/components/Button.jsx (circular)
```

### Verbose Output

```bash
cargo run -- src/main.js --verbose
```

```
🔍 Analyzing dependencies...
📊 Processed 4 files
🔗 Found 5 imports
🔴 Found 1 circular dependencies:

Circular Dependency #1:
  ├─ src/a.js →
  └─ src/b.js → src/a.js (circular)
  Dependencies involved:
    From src/a.js:
      - Line 1: ./b.js → src/b.js
    From src/b.js:
      - Line 1: ./a.js → src/a.js
```

## Exit Codes

- `0`: No circular dependencies found
- `1`: Circular dependencies detected or error occurred

## Algorithm

The tool uses the following approach:

1. **Import Extraction**: Uses regex patterns to extract import statements from source files
2. **Path Resolution**: Leverages `oxc-resolver` to resolve relative and aliased import paths
3. **Graph Building**: Constructs a directed graph where nodes are files and edges are dependencies
4. **Cycle Detection**: Uses Tarjan's strongly connected components algorithm to find circular dependencies
5. **Result Formatting**: Presents circular dependencies in a clear, actionable format

## Configuration

The tool uses `oxc-resolver` which automatically handles:

- TypeScript path mapping (via `tsconfig.json`)
- Package.json main field resolution
- Node.js module resolution
- File extensions resolution (.js, .ts, .jsx, .tsx, etc.)

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

This project is licensed under the MIT License.
