import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

const initialForm = {
  symmetricKeyDir: '',
  privateKeyDir: '',
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
  const [result, setResult] = useState(null);
  const [message, setMessage] = useState('Ready');

  function updateField(name, value) {
    setForm((current) => ({
      ...current,
      [name]: value,
    }));
  }

  function requestPayload() {
    return {
      ...form,
      versionMajor: Number(form.versionMajor),
      versionMinor: Number(form.versionMinor),
      versionPatch: Number(form.versionPatch),
      versionBuild: Number(form.versionBuild),
    };
  }

  async function selectDirectory(fieldName) {
    const selected = await open({
      directory: true,
      multiple: false,
    });

    if (typeof selected === 'string') {
      updateField(fieldName, selected);
    }
  }

  async function selectFirmwareFile() {
    const selected = await open({
      directory: false,
      multiple: false,
      filters: [
        { name: 'Firmware', extensions: ['bin', 'hex', 'srec', 'mot'] },
        { name: 'All Files', extensions: ['*'] },
      ],
    });

    if (typeof selected === 'string') {
      updateField('firmwarePath', selected);
    }
  }

  async function validateInputs() {
    setMessage('Validating firmware generation inputs...');
    setPlan(null);
    setResult(null);

    try {
      const response = await invoke('validate_generation_inputs', {
        request: requestPayload(),
      });

      setPlan(response);
      setMessage('Validation finished');
    } catch (error) {
      setMessage(`Validation error: ${error}`);
    }
  }

  async function generateFirmwareImage() {
    setMessage('Generating header, encrypted firmware, and final image...');
    setResult(null);

    try {
      const response = await invoke('generate_firmware_image', {
        request: requestPayload(),
      });

      setResult(response);
      setMessage('Firmware image generated');
    } catch (error) {
      setMessage(`Generation error: ${error}`);
    }
  }

  return (
    <main className="app-shell">
      <section className="workspace-card">
        <header className="page-header">
          <p className="eyebrow">Firmware Generator Test Structure</p>
          <h1>GUI Gen Firmware</h1>
          <p className="description">
            Generate a 1KB FWAC header, AES-256-CBC encrypted firmware, and a
            final 2KB-aligned image with a 64-byte raw ECDSA signature at the end.
          </p>
        </header>

        <section className="form-grid">
          <label className="field wide path-field">
            <span>Symmetric key directory</span>
            <div className="path-row">
              <input
                value={form.symmetricKeyDir}
                placeholder="C:\\keys\\symmetric"
                onChange={(event) => updateField('symmetricKeyDir', event.target.value)}
              />
              <button type="button" className="secondary" onClick={() => selectDirectory('symmetricKeyDir')}>
                Browse
              </button>
            </div>
            <small>Expected files: 1.bin, 2.bin, ... 10.bin</small>
          </label>

          <label className="field wide path-field">
            <span>Private key directory</span>
            <div className="path-row">
              <input
                value={form.privateKeyDir}
                placeholder="C:\\keys\\private"
                onChange={(event) => updateField('privateKeyDir', event.target.value)}
              />
              <button type="button" className="secondary" onClick={() => selectDirectory('privateKeyDir')}>
                Browse
              </button>
            </div>
            <small>Expected files: 1.txt, 2.txt, ... 10.txt</small>
          </label>

          <label className="field wide path-field">
            <span>Built firmware file</span>
            <div className="path-row">
              <input
                value={form.firmwarePath}
                placeholder="C:\\firmware\\app.bin"
                onChange={(event) => updateField('firmwarePath', event.target.value)}
              />
              <button type="button" className="secondary" onClick={selectFirmwareFile}>
                Browse
              </button>
            </div>
          </label>

          <label className="field wide path-field">
            <span>Output directory</span>
            <div className="path-row">
              <input
                value={form.outputDir}
                placeholder="C:\\firmware\\output"
                onChange={(event) => updateField('outputDir', event.target.value)}
              />
              <button type="button" className="secondary" onClick={() => selectDirectory('outputDir')}>
                Browse
              </button>
            </div>
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
          <button type="button" onClick={generateFirmwareImage}>
            Generate Firmware Image
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

            <h3>Key File Check</h3>
            <div className="slot-grid">
              {plan.keySlots.map((slot) => (
                <article className={`slot-card ${slot.exists ? 'ok' : 'warn'}`} key={slot.index}>
                  <strong>Index {slot.index}</strong>
                  <span>{slot.exists ? 'Ready' : 'Missing files'}</span>
                  <small>SYM: {slot.symmetricKeyPath}</small>
                  <small>PRI: {slot.privateKeyPath}</small>
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

        {result && (
          <section className="result-panel">
            <h2>Generated Output</h2>
            <dl className="summary-list">
              <div>
                <dt>Derived Key Index</dt>
                <dd>{result.keyIndex}</dd>
              </div>
              <div>
                <dt>Version U32</dt>
                <dd>0x{result.versionU32.toString(16).padStart(8, '0').toUpperCase()}</dd>
              </div>
              <div>
                <dt>Firmware Length</dt>
                <dd>{result.firmwareLength} bytes</dd>
              </div>
              <div>
                <dt>Final Image Length</dt>
                <dd>{result.finalImageLength} bytes</dd>
              </div>
              <div>
                <dt>Header</dt>
                <dd>{result.headerPath}</dd>
              </div>
              <div>
                <dt>Encrypted Firmware</dt>
                <dd>{result.encryptedFirmwarePath}</dd>
              </div>
              <div>
                <dt>Final Image</dt>
                <dd>{result.finalImagePath}</dd>
              </div>
              <div>
                <dt>FW SHA-256</dt>
                <dd>{result.firmwareHashHex}</dd>
              </div>
              <div className="wide-summary">
                <dt>Header SHA-256</dt>
                <dd>{result.headerHashHex}</dd>
              </div>
            </dl>

            {result.warnings.length > 0 && (
              <div className="warning-box">
                <h3>Warnings</h3>
                <ul>
                  {result.warnings.map((warning) => (
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
