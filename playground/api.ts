import { AuthService } from './auth';
import { UserService } from './user';
import { OrderService } from './order';
import { ProductService } from './product';

export class ApiService {
    private authService = new AuthService();
    private userService = new UserService();
    private orderService = new OrderService();
    private productService = new ProductService();

    // User endpoints
    async createUser(name: string, email: string) {
        return this.userService.createUser(name, email);
    }

    async getUser(id: string) {
        return this.userService.getUserById(id);
    }

    // Product endpoints
    async createProduct(name: string, price: number, category: string) {
        return this.productService.createProduct(name, price, category);
    }

    async getProduct(id: string) {
        return this.productService.getProductById(id);
    }

    // Order endpoints
    async createOrder(userId: string, productIds: string[]) {
        const products = productIds
            .map(id => this.productService.getProductById(id))
            .filter(Boolean) as any[];
            
        return this.orderService.createOrder(userId, products);
    }

    // Auth endpoints
    async login(email: string, password: string) {
        return this.authService.login(email, password);
    }

    async validateToken(token: string) {
        return this.authService.validateToken(token);
    }
}
