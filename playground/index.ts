// Main entry point - this file has no circular dependencies
export { UserService, type User } from './user';
export { ProductService, type Product } from './product';
export { OrderService, type Order } from './order';
export { AuthService, type AuthToken } from './auth';
export { Logger, logger } from './logger';
export { ApiService } from './api';
export { validateEmail, formatCurrency, generateId } from './utils';

// These have circular dependencies - be careful!
export { CircularA } from './circular-a';
export { CircularB } from './circular-b';
export { ComponentA } from './component-a';
export { ComponentB } from './component-b';
export { DeepA } from './deep-a';
