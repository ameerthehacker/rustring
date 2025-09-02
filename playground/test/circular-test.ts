import { createTestButton } from './test-helper';

export function runCircularTest() {
    const button = createTestButton('test@example.com');
    return button;
}

// This will create a circular dependency
import { runCircularTest as circularRef } from './circular-test';
