export interface ButtonProps {
    title: string;
    onClick?: () => void;
    variant?: 'primary' | 'secondary' | 'danger';
}

export interface ModalProps {
    title: string;
    children?: React.ReactNode;
    onClose: () => void;
}

export interface User {
    id: string;
    name: string;
    email: string;
}

export interface ValidationResult {
    isValid: boolean;
    errors: string[];
}
