<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from '@tauri-apps/plugin-dialog';

  let folderPath = "";
  let hashFilePath = "";
  let results: string[] = [];

  async function selectFolder() {
    const selected = await open({ directory: true });
    if (selected && typeof selected === "string") folderPath = selected;
  }

  async function selectHashFile() {
    const selected = await open({ filters: [{ name: "Text", extensions: ["txt"] }] });
    if (selected && typeof selected === "string") hashFilePath = selected;
  }

  async function runScan() {
    results = ["🕒 Scanning..."];
    try {
      const res = await invoke<string[]>("scan_folder", {
        folderPath,
        hashFile: hashFilePath,
      });
      results = res.map(r => r.startsWith("🚨") ? `🚨 ${r.slice(2)}` : r.startsWith("✅") ? `✅ ${r.slice(2)}` : r);
    } catch (err) {
      results = [`❌ Error: ${err}`];
    }
  }
</script>

<main class="container">
  <h1>🔒 SHA-256 Hash Verifier</h1>
  <p class="subtitle">
    Verify <strong>SHA-256</strong> hashes of files to check for malware by using exported <strong>.txt</strong> hash lists from 
    <a href="https://bazaar.abuse.ch/export/" target="_blank" rel="noopener noreferrer" class="link-text">Malware Bazaar</a>.
  </p>
    
  <div class="actions">
    <button on:click={selectFolder}>📂 Select Folder</button>
    <span>{folderPath}</span>
  </div>

  <div class="actions">
    <button on:click={selectHashFile}>📄 Select Hash File</button>
    <span>{hashFilePath}</span>
  </div>

  <div class="actions">
    <button class="run" on:click={runScan} disabled={!folderPath || !hashFilePath}>🚀 Run Scan</button>
  </div>

  <div class="results">
    {#each results as r}
      <p class={r.includes("🚨") ? "bad" : r.includes("✅") ? "good" : ""}>{r}</p>
    {/each}
  </div>

  <footer class="footer">
    <span>Made by Farhan Khondakar • </span>
    <a href="https://github.com/FarhanKhondakar" target="_blank" rel="noopener noreferrer" class="footer-link" aria-label="Visit Farhan Khondakar's GitHub Profile">
      <i class="fab fa-github git-logo"></i> 
    </a>
  </footer>
</main>

<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css" />

<style>
/* Style for the text link */
.link-text {
  color: #646cff;
  text-decoration: underline;
  cursor: pointer;
  font-size: inherit;
}

.link-text:hover {
  text-decoration: none;
  color: #4e56f3;
}

/* Footer styles */
.footer-link {
  display: inline-flex;
  align-items: center;
  text-decoration: none;
}

.git-logo {
  font-size: 20px;
  color: gray;
}

.footer-link:hover {
  text-decoration: underline;
  color: #4e56f3;
}

/* Other styles remain unchanged */
.container {
  max-width: 800px;
  margin: 5vh auto;
  padding: 1rem;
  text-align: center;
  font-family: 'Inter', sans-serif;
}

.subtitle {
  margin-top: -0.5rem;
  color: #666;
  font-size: 0.9rem;
}

.actions {
  margin: 1rem 0;
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 0.75rem;
  flex-wrap: wrap;
}

button {
  border-radius: 8px;
  border: none;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  background-color: #646cff;
  color: white;
  cursor: pointer;
  transition: 0.2s;
  box-shadow: 0 2px 6px rgba(0,0,0,0.2);
}

button:hover {
  background-color: #4e56f3;
}

button.run {
  background-color: #24c8db;
}

button.run:hover {
  background-color: #1db0c1;
}

.results {
  margin-top: 2rem;
  max-height: 300px;
  overflow-y: auto;
  text-align: left;
  padding: 1rem;
  border: 1px solid #ccc;
  border-radius: 8px;
  background-color: #f8f8f8;
  font-family: monospace;
}

.results p {
  margin: 0.3rem 0;
}

.results .bad {
  color: red;
}

.results .good {
  color: green;
}

.footer {
  margin-top: 2rem;
  font-size: 0.85rem;
  color: #666;
  text-align: right;
}

.footer a {
  color: #646cff;
  text-decoration: none;
}

.footer a:hover {
  text-decoration: underline;
}

@media (prefers-color-scheme: dark) {
  .container {
    color: #f0f0f0;
  }

  .results {
    background-color: #1e1e1e;
    border-color: #333;
  }

  .footer {
    color: #aaa;
  }
}
</style>