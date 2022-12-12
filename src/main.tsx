import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './index.css';

import Initialize from './native/pkg';

Initialize().then(() => {
  ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>,
  );
});
