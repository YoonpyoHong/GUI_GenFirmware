import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

const initialForm = {
  keyRootDir: '',
  firmwarePath: '',
  outputDir: '',
  versionMajor: 0,
  versionMinor: 1,
  versionPatch: 0,
  versionBuild: 0,
};

function App() {
  const [form, setForm] = useState(initialForm);
  const [plan, setPlan] = useState(null);
  const [message, setMessage] = useState('Ready');

  function updateField(name, value) {
    setForm((current) => ({
      ...current,
      [name]: value,
    }));
  }

  async function validateInputs() {
    setMessage('Validating firmware generation inputs...');
    setPlan(null);

    try {
      const response = await invoke('validate_generation_inputs', {
        request: {
          ...form,
          versionMajor: Number(form.versionMajor),
          versionMinor: Number(form.versionMinor),
          versionPatch: Number(form.versionPatch),
          versionBuild: Number(form.versionBuild),
        },
      });

      setPlan(response);
      setMessage('Validation finished');
    } catch (error) {
      setMessage(`Validation error: ${error}`);
    }
  }

  return (
    <main className="app-shell">
      <section className="workspace-card">
        <header className="page-header">
          <p className="eyebrow">Firmware Generator Test Structure</p>
          <h1>GUI Gen Firmware</h1>
          <p className="description">
            Input key slot folders, a built firmware file, and version information
            to validate the firmware generation plan before implementing binary
            generation.
          </p>
        </header>

        <section className="form-grid">
          <label className="field wide">
            <span>Key root directory</span>
            <input
              value={form.keyRootDir}
              placeholder="C:\\keys"
              onChange={(event) => updateField('keyRootDir', event.target.value)}
            />
            <small>Expected folders: 0, 1, 2, ... 9</small>
          </label>

          <label className="field wide">
            <span>Built firmware file</span>
            <input
              value={form.firmwarePath}
              placeholder="C:\\firmware\\app.bin"
              onChange={(event) => updateField('firmwarePath', event.target.value)}
            />
          </label>

          <label className="field wide">
            <span>Output directory</span>
            <input
              value={form.outputDir}
              placeholder="C:\\firmware\\output"
              onChange={(event) => updateField('outputDir', event.target.value)}
            />
          </label>

          <label className="field">
            <span>Major</span>
            <input
              type="number"
              min="0"
              max="255"
              value={form.versionMajor}
              onChange={(event) => updateField('versionMajor', event.target.value)}
            />
          </label>

          <label className="field">
            <span>Minor</span>
            <input
              type="number"
              min="0"
              max="255"
              value={form.versionMinor}
              onChange={(event) => updateField('versionMinor', event.target.value)}
            />
          </label>

          <label className="field">
            <span>Patch</span>
            <input
              type="number"
              min="0"
              max="255"
              value={form.versionPatch}
              onChange={(event) => updateField('versionPatch', event.target.value)}
            />
          </label>

          <label className="field">
            <span>Build</span>
            <input
              type="number"
              min="0"
              max="255"
              value={form.versionBuild}
              onChange={(event) => updateField('versionBuild', event.target.value)}
            />
          </label>
        </section>

        <div className="actions">
          <button type="button" onClick={validateInputs}>
            Validate Test Structure
          </button>
          <p className="status">{message}</p>
        </div>

        {plan && (
          <section className="result-panel">
            <h2>Generation Plan</h2>
            <dl className="summary-list">
              <div>
                <dt>Version</dt>
                <dd>{plan.versionText}</dd>
              </div>
              <div>
                <dt>Version U32</dt>
                <dd>0x{plan.versionU32.toString(16).padStart(8, '0').toUpperCase()}</dd>
              </div>
              <div>
                <dt>Firmware</dt>
                <dd>{plan.firmwarePath}</dd>
              </div>
              <div>
                <dt>Output</dt>
                <dd>{plan.outputDir}</dd>
              </div>
            </dl>

            <h3>Key Slot Check</h3>
            <div className="slot-grid">
              {plan.keySlots.map((slot) => (
                <article className={`slot-card ${slot.exists ? 'ok' : 'warn'}`} key={slot.index}>
                  <strong>Slot {slot.index}</strong>
                  <span>{slot.exists ? 'Ready' : 'Missing files'}</span>
                  <small>{slot.folder}</small>
                </article>
              ))}
            </div>

            {plan.warnings.length > 0 && (
              <div className="warning-box">
                <h3>Warnings</h3>
                <ul>
                  {plan.warnings.map((warning) => (
                    <li key={warning}>{warning}</li>
                  ))}
                </ul>
              </div>
            )}
          </section>
        )}
      </section>
    </main>
  );
}

export default App;
