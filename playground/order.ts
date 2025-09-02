import { User, UserService } from './user';
import { Product } from './product';

export interface Order {
    id: string;
    userId: string;
    products: Product[];
    total: number;
    status: 'pending' | 'completed' | 'cancelled';
}

export class OrderService {
    private orders: Map<string, Order> = new Map();
    private userService = new UserService();

    createOrder(userId: string, products: Product[]): Order {
        const user = this.userService.getUserById(userId);
        if (!user) {
            throw new Error('User not found');
        }

        const total = products.reduce((sum, product) => sum + product.price, 0);
        
        const order: Order = {
            id: Math.random().toString(36),
            userId,
            products,
            total,
            status: 'pending'
        };

        this.orders.set(order.id, order);
        return order;
    }
}
