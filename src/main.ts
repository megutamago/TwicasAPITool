import { invoke } from '@tauri-apps/api/tauri';
import { diplayTable } from './components/SupportingList';
import { setupOpenSettings } from './components/OpenSettings';
import { SupportingList } from './types/SupportingList';

let displayInputEl: HTMLInputElement | null;
let displayMsgEl: HTMLElement | null;

window.addEventListener("DOMContentLoaded", () => {
  // SupportingList
  displayInputEl = document.querySelector("#display-input");
  displayMsgEl = document.querySelector("#display-msg");
  document.querySelector("#display-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    display();
  });

  // openSettings
  setupOpenSettings();
});

async function display() {
  if (displayMsgEl && displayInputEl) {
    const contents: SupportingList = await invoke("ladder", {
      input: displayInputEl.value,
    });
    if (contents[0] && contents[1].length > 1) {
      diplayTable(displayMsgEl, contents)
      displayMsgEl.style.color = "white";
    } else {
      displayMsgEl.textContent = "データ取得に失敗しました。"
      displayMsgEl.style.color = "red";
    }
  }
}