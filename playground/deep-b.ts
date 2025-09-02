import { DeepC } from './deep-c';

export class DeepB {
    private deepC = new DeepC();

    process(): string {
        return `DeepB -> ${this.deepC.process()}`;
    }

    getValue(): number {
        return 2 + this.deepC.getValue();
    }
}
