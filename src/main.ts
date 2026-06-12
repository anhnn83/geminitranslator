// src/main.ts --v2.2
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { readText } from "@tauri-apps/plugin-clipboard-manager";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { enable, disable, isEnabled } from "@tauri-apps/plugin-autostart";

const appWindow = getCurrentWindow();

if (appWindow.label === "popup") {
  document.body.classList.add("popup-mode");
  const settingsView = document.getElementById("settingsView");
  const popupView = document.getElementById("popupView");
  const resultContent = document.getElementById("resultContent");
  const closeBtn = document.getElementById("closeBtn");
  const pinBtn = document.getElementById("pinBtn") as HTMLButtonElement;

  if (settingsView) settingsView.style.display = "none";
  if (popupView) popupView.style.display = "block";

  let isPinned = true;
  if (pinBtn) {
    pinBtn.classList.add("pinned");
    pinBtn.innerText = "📌 Pinned";
    pinBtn.addEventListener("click", () => {
      isPinned = !isPinned;
      if (isPinned) {
        pinBtn.classList.add("pinned");
        pinBtn.innerText = "📌 Pinned";
      } else {
        pinBtn.classList.remove("pinned");
        pinBtn.innerText = "📌 Unpinned";
      }
    });
  }

  if (closeBtn) closeBtn.addEventListener("click", async () => { await invoke("hide_popup"); });
  window.addEventListener("blur", async () => { if (!isPinned) await invoke("hide_popup"); });

  // 1. Text Logic
  listen("trigger-translate-text", async () => {
    if (!resultContent) return;
    try {
      const isAnalyzerMode = localStorage.getItem("analyzerMode") === "true"; // 🌟 LẤY TRẠNG THÁI
      resultContent.innerText = "⏳ Reading highlighted text...";
      const text = await readText();
      
      if (!text || text.trim() === "") {
        resultContent.innerText = "❌ No text found in clipboard! Please highlight text and try again.";
        return;
      }
      if (text.length > 3500000) {
        resultContent.innerText = "❌ Text is too long (Max 3.5M chars). Please select a shorter segment.";
        return;
      }

      const apiKey = localStorage.getItem("apiKey");
      const targetLang = localStorage.getItem("targetLang") || "Vietnamese";

      if (!apiKey) {
        resultContent.innerText = "❌ Missing Gemini API Key. Please set it in Settings!";
        return;
      }

      // 🌟 THÔNG BÁO THEO CHẾ ĐỘ
      resultContent.innerText = isAnalyzerMode 
        ? "⏳ Analyzing code/terminal output as a Tech Expert..." 
        : "⏳ Connecting to Gemini 2.5 Flash-Lite...";

      // 🌟 TRUYỀN BIẾN XUỐNG RUST
      const response = await invoke("translate_text", { apiKey, text, targetLang, isAnalyzerMode });
      resultContent.innerText = response as string;

    } catch (error) {
      resultContent.innerText = `❌ Connection Error: ${error}`;
    }
  });

  // 2. Image Logic
  listen("trigger-translate-image", async () => {
    if (!resultContent) return;
    try {
      const isAnalyzerMode = localStorage.getItem("analyzerMode") === "true"; // 🌟 LẤY TRẠNG THÁI
      resultContent.innerText = "⏳ Extracting image from Clipboard...";
      const base64Image = await invoke("get_clipboard_image_base64");
      const apiKey = localStorage.getItem("apiKey");
      const targetLang = localStorage.getItem("targetLang") || "Vietnamese";

      if (!apiKey) {
        resultContent.innerText = "❌ Missing Gemini API Key. Please set it in Settings!";
        return;
      }

      // 🌟 THÔNG BÁO THEO CHẾ ĐỘ
      resultContent.innerText = isAnalyzerMode 
        ? "⏳ Analyzing terminal/code screenshot with Gemini Vision..." 
        : "⏳ Analyzing image using Gemini Vision OCR...";
      
      // 🌟 TRUYỀN BIẾN XUỐNG RUST
      const response = await invoke("translate_image", { apiKey, base64Image, targetLang, isAnalyzerMode });
      resultContent.innerText = response as string;

    } catch (error) {
      resultContent.innerText = `❌ Error: ${error}\n\n💡 Tip: Please use your OS screenshot tool (e.g., Win+Shift+S) to capture an area FIRST, then press the Image Translation shortcut.`;
    }
  });

} else {
  // --- CHẾ ĐỘ SETTINGS ---
  const apiKeyInput = document.getElementById("apiKey") as HTMLInputElement;
  const targetLangSelect = document.getElementById("targetLang") as HTMLSelectElement;
  const autoStartCheckbox = document.getElementById("autoStart") as HTMLInputElement;
  const analyzerModeCheckbox = document.getElementById("analyzerMode") as HTMLInputElement; // 🌟 BẮT ELEMENT MỚI
  const saveBtn = document.getElementById("saveBtn") as HTMLButtonElement;
  const testApiBtn = document.getElementById("testApiBtn") as HTMLButtonElement;
  const statusMsg = document.getElementById("statusMsg") as HTMLParagraphElement;
  const textShortcutInput = document.getElementById("textShortcut") as HTMLInputElement;
  const imageShortcutInput = document.getElementById("imageShortcut") as HTMLInputElement;
  
  const helpBtn = document.getElementById("helpBtn") as HTMLButtonElement;
  const closeHelpBtn = document.getElementById("closeHelpBtn") as HTMLButtonElement;
  const helpModal = document.getElementById("helpModal") as HTMLDivElement;

  if (helpBtn) helpBtn.addEventListener("click", () => helpModal.style.display = "flex");
  if (closeHelpBtn) closeHelpBtn.addEventListener("click", () => helpModal.style.display = "none");

  function handleFocus(e: FocusEvent) {
    const target = e.target as HTMLInputElement;
    target.classList.add("recording");
    target.dataset.oldValue = target.value;
    target.value = "Listening...";
  }

  function handleBlur(e: FocusEvent) {
    const target = e.target as HTMLInputElement;
    target.classList.remove("recording");
    if (target.value === "Listening...") {
      target.value = target.dataset.oldValue || "";
    }
  }

  function handleShortcutInput(e: KeyboardEvent) {
    e.preventDefault(); 
    let keys = [];
    if (e.metaKey) keys.push("Super");
    if (e.ctrlKey) keys.push("Control");
    if (e.altKey) keys.push("Alt");
    if (e.shiftKey) keys.push("Shift");

    let key = e.key.toUpperCase();
    if (["CONTROL", "SHIFT", "ALT", "META"].includes(key)) return; 
    if (key === " ") key = "Space";

    keys.push(key);
    const target = e.target as HTMLInputElement;
    target.value = keys.join("+");
    target.blur(); 
  }

  if (textShortcutInput) {
    textShortcutInput.addEventListener("keydown", handleShortcutInput);
    textShortcutInput.addEventListener("focus", handleFocus);
    textShortcutInput.addEventListener("blur", handleBlur);
  }
  if (imageShortcutInput) {
    imageShortcutInput.addEventListener("keydown", handleShortcutInput);
    imageShortcutInput.addEventListener("focus", handleFocus);
    imageShortcutInput.addEventListener("blur", handleBlur);
  }

  window.addEventListener("DOMContentLoaded", async () => {
    if (apiKeyInput) apiKeyInput.value = localStorage.getItem("apiKey") || "";
    if (targetLangSelect) targetLangSelect.value = localStorage.getItem("targetLang") || "Vietnamese";
    // 🌟 LOAD TRẠNG THÁI CHECKBOX MỚI
    if (analyzerModeCheckbox) analyzerModeCheckbox.checked = localStorage.getItem("analyzerMode") === "true";
    
    try {
      const autostartStatus = await isEnabled();
      if (autoStartCheckbox) autoStartCheckbox.checked = autostartStatus;
    } catch (e) {
      console.error("Autostart check failed:", e);
    }
    
    const tShortcut = localStorage.getItem("textShortcut") || "Super+Alt+X";
    const iShortcut = localStorage.getItem("imageShortcut") || "Super+Alt+C";
    if (textShortcutInput) textShortcutInput.value = tShortcut;
    if (imageShortcutInput) imageShortcutInput.value = iShortcut;

    try { await invoke("register_shortcuts", { textShortcut: tShortcut, imageShortcut: iShortcut }); } 
    catch (e) { console.error("Shortcut init failed:", e); }
  });

  if (saveBtn) {
    saveBtn.addEventListener("click", async () => {
      localStorage.setItem("apiKey", apiKeyInput.value);
      localStorage.setItem("targetLang", targetLangSelect.value);
      localStorage.setItem("textShortcut", textShortcutInput.value);
      localStorage.setItem("imageShortcut", imageShortcutInput.value);
      // 🌟 LƯU TRẠNG THÁI CHECKBOX MỚI
      localStorage.setItem("analyzerMode", analyzerModeCheckbox.checked.toString());
      
      try {
        if (autoStartCheckbox.checked) {
          await enable();
        } else {
          await disable();
        }
      } catch (e) {
        console.error("Failed to toggle autostart:", e);
      }
      
      try {
        await invoke("register_shortcuts", { textShortcut: textShortcutInput.value, imageShortcut: imageShortcutInput.value });
        statusMsg.style.color = "#4cd137";
        statusMsg.innerText = "✅ Settings & Shortcuts saved! Autostart updated.";
      } catch (error) {
        statusMsg.style.color = "#e84118";
        statusMsg.innerText = `❌ Error: ${error}`;
      }
      setTimeout(() => (statusMsg.innerText = ""), 4000);
    });
  }

  if (testApiBtn) {
    testApiBtn.addEventListener("click", async () => {
      const apiKey = apiKeyInput.value.trim();
      if (!apiKey) {
        statusMsg.style.color = "#e84118";
        statusMsg.innerText = "❌ Please enter API Key first!";
        return;
      }
      statusMsg.style.color = "#00a8ff";
      statusMsg.innerText = "⏳ Testing API connection...";
      try {
        // Test API chạy ngầm không cần cờ analyzer mode
        const response = await invoke("translate_text", { apiKey: apiKey, text: "Hello, test!", targetLang: targetLangSelect.value, isAnalyzerMode: false });
        statusMsg.style.color = "#4cd137";
        statusMsg.innerText = `✅ Success: ${response}`;
      } catch (error) {
        statusMsg.style.color = "#e84118";
        statusMsg.innerText = `❌ Error: ${error}`;
      }
    });
  }
}