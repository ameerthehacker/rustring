import React from 'react';
import { Button } from '@components/button';
import { ModalProps } from '#types';

export const Modal: React.FC<ModalProps> = ({ title, children, onClose }) => {
    return (
        <div className="modal-overlay">
            <div className="modal-content">
                <div className="modal-header">
                    <h2>{title}</h2>
                    <Button 
                        title="Close" 
                        onClick={onClose}
                        variant="secondary"
                    />
                </div>
                {children && (
                    <div className="modal-body">
                        {children}
                    </div>
                )}
            </div>
        </div>
    );
};
