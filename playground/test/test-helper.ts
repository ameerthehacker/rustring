import { validateEmail } from '@utils/validation';
import { Button } from '@components/button';

export function createTestButton(email: string) {
    if (!validateEmail(email)) {
        throw new Error('Invalid email for test');
    }
    
    return Button;
}

export const TEST_CONFIG = {
    timeout: 5000,
    retries: 3
};
