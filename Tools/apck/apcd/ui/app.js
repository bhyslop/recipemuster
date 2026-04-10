// Ann's PHI Clipbuddy — frontend
// On window focus: read clipboard via Tauri command, display raw content

let lastContent = null;

window.addEventListener("focus", async () => {
  try {
    const content = await window.__TAURI__.core.invoke("read_clipboard");
    if (content === lastContent) return;
    lastContent = content;
    document.getElementById("clipboard-content").textContent = content;
    document.getElementById("status").textContent = "Clipboard content:";
  } catch (e) {
    document.getElementById("status").textContent = "Error reading clipboard: " + e;
  }
});
