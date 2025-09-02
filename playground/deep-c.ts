import { DeepD } from './deep-d';

export class DeepC {
    private deepD = new DeepD();

    process(): string {
        return `DeepC -> ${this.deepD.process()}`;
    }

    getValue(): number {
        return 3 + this.deepD.getValue();
    }
}
