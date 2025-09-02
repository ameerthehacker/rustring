import { DeepB } from './deep-b';

export interface DeepAInterface {
    name: string;
    value: number;
}

export class DeepA implements DeepAInterface {
    name = 'DeepA';
    value = 1;

    private deepB = new DeepB();

    process(): string {
        return `${this.name}: ${this.value} -> ${this.deepB.process()}`;
    }

    getValue(): number {
        return this.value + this.deepB.getValue();
    }
}
