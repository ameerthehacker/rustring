import { User } from './user';
import { logger } from './logger';

export interface AuthToken {
    token: string;
    expiresAt: Date;
    userId: string;
}

export class AuthService {
    private tokens: Map<string, AuthToken> = new Map();

    login(email: string, password: string): AuthToken | null {
        logger.info(`Login attempt for email: ${email}`);
        
        // Simulate authentication logic
        if (email && password.length >= 6) {
            const token: AuthToken = {
                token: Math.random().toString(36),
                expiresAt: new Date(Date.now() + 3600000), // 1 hour
                userId: Math.random().toString(36)
            };

            this.tokens.set(token.token, token);
            logger.info(`User authenticated: ${token.userId}`);
            return token;
        }

        logger.warn(`Authentication failed for email: ${email}`);
        return null;
    }

    validateToken(token: string): User | null {
        const authToken = this.tokens.get(token);
        
        if (!authToken || authToken.expiresAt < new Date()) {
            logger.warn(`Invalid or expired token: ${token}`);
            return null;
        }

        logger.info(`Token validated for user: ${authToken.userId}`);
        return {
            id: authToken.userId,
            name: 'Test User',
            email: 'test@example.com',
            orders: []
        };
    }
}
