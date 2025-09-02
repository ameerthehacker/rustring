export interface Product {
    id: string;
    name: string;
    price: number;
    category: string;
}

export class ProductService {
    private products: Map<string, Product> = new Map();

    createProduct(name: string, price: number, category: string): Product {
        const product: Product = {
            id: Math.random().toString(36),
            name,
            price,
            category
        };

        this.products.set(product.id, product);
        return product;
    }

    getProductById(id: string): Product | undefined {
        return this.products.get(id);
    }

    getProductsByCategory(category: string): Product[] {
        return Array.from(this.products.values())
            .filter(product => product.category === category);
    }
}
