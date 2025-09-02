import React from 'react';
import { Button } from '@components/button';
import { Modal } from '@components/modal';
import { validateEmail } from '@utils/validation';
import { logger } from '@services/logger';
import type { User } from '#types';

interface AppProps {
    user?: User;
}

export const App: React.FC<AppProps> = ({ user }) => {
    const [showUserModal, setShowUserModal] = React.useState(false);

    const handleUserAction = () => {
        if (user && validateEmail(user.email)) {
            logger.info(`User action performed: ${user.name}`);
            setShowUserModal(true);
        } else {
            logger.warn('Invalid user or email');
        }
    };

    return (
        <div className="app">
            <h1>App with TypeScript Aliases</h1>
            
            <Button 
                title={user ? `Hello, ${user.name}!` : 'Login'}
                onClick={handleUserAction}
                variant="primary"
            />

            {showUserModal && (
                <Modal 
                    title="User Profile"
                    onClose={() => setShowUserModal(false)}
                >
                    <p>Email: {user?.email}</p>
                    <Button 
                        title="Update Profile"
                        onClick={() => logger.info('Profile update requested')}
                        variant="secondary"
                    />
                </Modal>
            )}
        </div>
    );
};
