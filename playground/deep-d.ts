import { DeepA } from './deep-a';

export class DeepD {
    private deepA: DeepA | null = null;

    process(): string {
        return this.deepA ? `DeepD -> ${this.deepA.process()}` : 'DeepD (end)';
    }

    getValue(): number {
        return 4 + (this.deepA ? this.deepA.getValue() : 0);
    }

    setDeepA(deepA: DeepA): void {
        this.deepA = deepA;
    }
}
