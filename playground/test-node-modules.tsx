import React from 'react';
import { useState } from 'react';
import axios from 'axios';
import { Button } from '@components/button';
import { logger } from '@services/logger';

export const TestComponent = () => {
    const [data, setData] = useState(null);

    const fetchData = async () => {
        try {
            const response = await axios.get('/api/data');
            setData(response.data);
            logger.info('Data fetched successfully');
        } catch (error) {
            logger.error('Failed to fetch data');
        }
    };

    return (
        <div>
            <h1>Test Component</h1>
            <Button title="Fetch Data" onClick={fetchData} />
            {data && <pre>{JSON.stringify(data, null, 2)}</pre>}
        </div>
    );
};
