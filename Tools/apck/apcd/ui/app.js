// Ann's PHI Clipbuddy — frontend
// On window focus: consume clipboard via Tauri IPC, render triage view

let triageData = null;   // { findings, plain_text } from last clinical result
let toggleStates = [];   // parallel to findings: "elide" or "pass"

// ---------------------------------------------------------------------------
// Focus handler — invoke backend on every window focus
// ---------------------------------------------------------------------------

window.addEventListener("focus", async () => {
  try {
    const result = await window.__TAURI__.core.invoke("consume_clipboard");
    zapcapHandleResult(result);
  } catch (e) {
    document.getElementById("diagnostic").textContent = "Error: " + e;
  }
});

// ---------------------------------------------------------------------------
// Result dispatch
// ---------------------------------------------------------------------------

function zapcapHandleResult(result) {
  switch (result.type) {
    case "unchanged":
      break;
    case "clinical":
      zapcapShowTriage(result);
      break;
    case "non_clinical":
      zapcapShowNonClinical(result);
      break;
  }
}

// ---------------------------------------------------------------------------
// Non-clinical state — show diagnostic, return to instruction view
// ---------------------------------------------------------------------------

function zapcapShowNonClinical(result) {
  triageData = null;
  toggleStates = [];
  document.getElementById("instruction-state").style.display = "";
  document.getElementById("triage-state").style.display = "none";
  document.getElementById("diagnostic").textContent =
    "Not clinical (" + result.content_length + " bytes, " +
    result.content_type + "): " + result.preview;
}

// ---------------------------------------------------------------------------
// Triage state — document preview + findings panel
// ---------------------------------------------------------------------------

function zapcapShowTriage(result) {
  triageData = result;
  toggleStates = result.findings.map(function() { return "elide"; });

  document.getElementById("instruction-state").style.display = "none";
  document.getElementById("triage-state").style.display = "";

  zapcapRenderPreview();
  zapcapRenderFindings();
}

// ---------------------------------------------------------------------------
// Document preview — plain text with inline PHI highlights
// ---------------------------------------------------------------------------

function zapcapRenderPreview() {
  var plain = triageData.plain_text;
  var findings = triageData.findings;

  // Build index-annotated list sorted by offset
  var sorted = [];
  for (var i = 0; i < findings.length; i++) {
    sorted.push({ f: findings[i], idx: i });
  }
  sorted.sort(function(a, b) {
    return a.f.offset - b.f.offset || b.f.length - a.f.length;
  });

  var parts = [];
  var pos = 0;

  for (var s = 0; s < sorted.length; s++) {
    var entry = sorted[s];
    var f = entry.f;
    if (f.offset < pos) continue; // skip overlap

    // Text before this finding
    if (f.offset > pos) {
      parts.push(zapcapEscape(plain.substring(pos, f.offset)));
    }

    // Finding span
    var state = toggleStates[entry.idx];
    var cls;
    if (state === "pass") {
      cls = "phi-pass";
    } else if (f.severity === "red") {
      cls = "phi-red";
    } else {
      cls = "phi-yellow";
    }
    parts.push(
      '<span class="' + cls + '" data-index="' + entry.idx + '">' +
      zapcapEscape(plain.substring(f.offset, f.offset + f.length)) +
      "</span>"
    );
    pos = f.offset + f.length;
  }

  // Remaining text
  if (pos < plain.length) {
    parts.push(zapcapEscape(plain.substring(pos)));
  }

  document.getElementById("document-preview").innerHTML = parts.join("");
}

// ---------------------------------------------------------------------------
// Findings panel — two sections: questionable (yellow) and definite (red)
// ---------------------------------------------------------------------------

function zapcapRenderFindings() {
  var findings = triageData.findings;
  var yellowHtml = [];
  var redHtml = [];

  for (var i = 0; i < findings.length; i++) {
    var f = findings[i];
    var html = zapcapFindingEntry(f, i);
    if (f.severity === "yellow") {
      yellowHtml.push(html);
    } else {
      redHtml.push(html);
    }
  }

  document.getElementById("yellow-list").innerHTML =
    yellowHtml.length > 0 ? yellowHtml.join("") :
    '<div class="empty-section">None detected</div>';

  document.getElementById("red-list").innerHTML =
    redHtml.length > 0 ? redHtml.join("") :
    '<div class="empty-section">None detected</div>';
}

function zapcapFindingEntry(finding, index) {
  var state = toggleStates[index];
  var label = zapcapFormatCategory(finding.category);
  var btnClass = "finding-toggle " + state;
  var btnText = state === "elide" ? "ELIDE \u25BC" : "PASS \u25BC";

  return '<div class="finding-entry">' +
    '<span class="finding-text">' + zapcapEscape(finding.text) + '</span>' +
    '<span class="finding-category">' + label + "</span>" +
    '<button class="' + btnClass + '" onclick="zapcapToggle(' + index + ')">' +
    btnText + "</button>" +
    "</div>";
}

// ---------------------------------------------------------------------------
// Toggle — flip elide/pass, re-render preview and findings
// ---------------------------------------------------------------------------

function zapcapToggle(index) {
  toggleStates[index] = toggleStates[index] === "elide" ? "pass" : "elide";
  zapcapRenderPreview();
  zapcapRenderFindings();
}

// ---------------------------------------------------------------------------
// Utilities
// ---------------------------------------------------------------------------

function zapcapFormatCategory(cat) {
  return cat.replace(/_/g, " ").toUpperCase();
}

function zapcapEscape(text) {
  var el = document.createElement("span");
  el.textContent = text;
  return el.innerHTML;
}

// ---------------------------------------------------------------------------
// Copy anonymized — send toggle states to backend, write clean text to clipboard
// ---------------------------------------------------------------------------

async function zapcapCopyAnonymized() {
  if (!triageData) return;
  var btn = document.getElementById("copy-btn");
  try {
    await window.__TAURI__.core.invoke("copy_anonymized", { toggle_states: toggleStates });
    btn.textContent = "Copied!";
    setTimeout(function() {
      btn.textContent = "Copy Anonymized to Clipboard";
    }, 2000);
  } catch (e) {
    btn.textContent = "Error: " + e;
    setTimeout(function() {
      btn.textContent = "Copy Anonymized to Clipboard";
    }, 4000);
  }
}
