import { Order } from './order';
import { validateEmail } from './utils';

export interface User {
    id: string;
    name: string;
    email: string;
    orders: Order[];
}

export class UserService {
    private users: Map<string, User> = new Map();

    createUser(name: string, email: string): User {
        if (!validateEmail(email)) {
            throw new Error('Invalid email');
        }

        const user: User = {
            id: Math.random().toString(36),
            name,
            email,
            orders: []
        };

        this.users.set(user.id, user);
        return user;
    }

    getUserById(id: string): User | undefined {
        return this.users.get(id);
    }
}
