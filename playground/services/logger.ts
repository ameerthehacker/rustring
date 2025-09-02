import { formatMessage } from '@utils/validation';

interface LogEntry {
    level: 'info' | 'warn' | 'error';
    message: string;
    timestamp: Date;
}

class LoggerService {
    private logs: LogEntry[] = [];

    info(message: string): void {
        this.log('info', message);
    }

    warn(message: string): void {
        this.log('warn', message);
    }

    error(message: string): void {
        this.log('error', message);
    }

    private log(level: LogEntry['level'], message: string): void {
        const entry: LogEntry = {
            level,
            message,
            timestamp: new Date()
        };

        this.logs.push(entry);
        console.log(`[${level.toUpperCase()}] ${message}`);
    }

    getLogs(): LogEntry[] {
        return [...this.logs];
    }
}

export const logger = new LoggerService();

// This will create a circular dependency since validation.ts imports logger
function formatMessage(message: string): string {
    return `[${new Date().toISOString()}] ${message}`;
}
