import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

function App() {
  const [message, setMessage] = useState('Ready');

  async function checkBackend() {
    try {
      const response = await invoke('backend_status');
      setMessage(response);
    } catch (error) {
      setMessage(`Backend error: ${error}`);
    }
  }

  return (
    <main className="app-shell">
      <section className="hero-card">
        <p className="eyebrow">Tauri + React + Rust</p>
        <h1>GUI Gen Firmware</h1>
        <p className="description">
          Firmware image generation UI scaffold for header editing, signing,
          encryption, and final image export workflows.
        </p>
        <button type="button" onClick={checkBackend}>
          Check Rust Backend
        </button>
        <p className="status">{message}</p>
      </section>
    </main>
  );
}

export default App;
