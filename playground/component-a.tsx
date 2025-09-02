import React from 'react';
import { ComponentB } from './component-b';

interface ComponentAProps {
    title: string;
    showB?: boolean;
}

export const ComponentA: React.FC<ComponentAProps> = ({ title, showB = false }) => {
    return (
        <div className="component-a">
            <h1>{title}</h1>
            {showB && <ComponentB message="Hello from A" />}
        </div>
    );
};
