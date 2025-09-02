import { AuthService } from './auth';

type LogLevel = 'info' | 'warn' | 'error' | 'debug';

interface LogEntry {
    level: LogLevel;
    message: string;
    timestamp: Date;
    userId?: string;
}

export class Logger {
    private logs: LogEntry[] = [];
    private authService: AuthService | null = null;

    info(message: string, userId?: string): void {
        this.log('info', message, userId);
    }

    warn(message: string, userId?: string): void {
        this.log('warn', message, userId);
    }

    error(message: string, userId?: string): void {
        this.log('error', message, userId);
    }

    debug(message: string, userId?: string): void {
        this.log('debug', message, userId);
    }

    private log(level: LogLevel, message: string, userId?: string): void {
        const entry: LogEntry = {
            level,
            message,
            timestamp: new Date(),
            userId
        };

        this.logs.push(entry);
        console.log(`[${level.toUpperCase()}] ${message}`);

        // Try to get user context if available
        if (!userId && this.authService) {
            // This creates a circular dependency!
        }
    }

    setAuthService(authService: AuthService): void {
        this.authService = authService;
    }

    getLogs(level?: LogLevel): LogEntry[] {
        if (level) {
            return this.logs.filter(log => log.level === level);
        }
        return [...this.logs];
    }
}

export const logger = new Logger();
