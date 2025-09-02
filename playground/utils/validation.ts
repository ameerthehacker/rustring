import { logger } from '@services/logger';

export function validateProps(props: Record<string, any>): boolean {
    const missingProps = Object.entries(props)
        .filter(([key, value]) => value === undefined || value === null)
        .map(([key]) => key);

    if (missingProps.length > 0) {
        logger.warn(`Missing required props: ${missingProps.join(', ')}`);
        return false;
    }

    logger.info('Props validation passed');
    return true;
}

export function validateEmail(email: string): boolean {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    const isValid = emailRegex.test(email);
    
    if (!isValid) {
        logger.warn(`Invalid email format: ${email}`);
    }
    
    return isValid;
}
