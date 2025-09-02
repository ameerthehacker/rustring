import { CircularB } from './circular-b';

export class CircularA {
    private b: CircularB;

    constructor() {
        this.b = new CircularB();
    }

    getName(): string {
        return 'A' + this.b.getName();
    }

    getValue(): number {
        return 1 + this.b.getValue();
    }
}
