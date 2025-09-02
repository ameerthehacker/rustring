import React from 'react';
import { Modal } from '@components/modal';
import { validateProps } from '@utils/validation';
import { ButtonProps } from '#types';

export const Button: React.FC<ButtonProps> = ({ title, onClick, variant = 'primary' }) => {
    const [showModal, setShowModal] = React.useState(false);

    const handleClick = () => {
        if (validateProps({ title, onClick })) {
            onClick?.();
            setShowModal(true);
        }
    };

    return (
        <div>
            <button 
                className={`btn btn-${variant}`}
                onClick={handleClick}
            >
                {title}
            </button>
            {showModal && (
                <Modal 
                    title="Action Complete" 
                    onClose={() => setShowModal(false)}
                />
            )}
        </div>
    );
};
