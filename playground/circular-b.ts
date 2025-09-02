import { CircularA } from './circular-a';

export class CircularB {
    private a: CircularA | null = null;

    getName(): string {
        return 'B' + (this.a ? this.a.getName() : '');
    }

    getValue(): number {
        return 2 + (this.a ? this.a.getValue() : 0);
    }

    setA(a: CircularA): void {
        this.a = a;
    }
}
