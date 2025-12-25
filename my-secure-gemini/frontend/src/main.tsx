import React from 'react'
import ReactDOM from 'react-dom/client'
// import App from './App.tsx'
// 修正前
// import App from './App.tsx'

// 修正後
import App from './App'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)