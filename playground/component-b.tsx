import React from 'react';
import { ComponentA } from './component-a';

interface ComponentBProps {
    message: string;
    showA?: boolean;
}

export const ComponentB: React.FC<ComponentBProps> = ({ message, showA = false }) => {
    return (
        <div className="component-b">
            <p>{message}</p>
            {showA && <ComponentA title="Nested A" />}
        </div>
    );
};
