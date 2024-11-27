import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';

console.log('__chris main 1');
ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>
);
console.log('__chris main 2');
