import { invoke } from '@tauri-apps/api/tauri';
import { diplayTable } from './components/SupportersList';
import { setupOpenSettings } from './components/OpenSettings';
import { SupportersList } from './types/SupportersList';

let displayInputEls: (HTMLInputElement | null)[] = [];
let displayMsgEl: HTMLElement | null;

window.addEventListener("DOMContentLoaded", () => {
  // SupportersList
  displayInputEls[0] = document.querySelector("#display-input1");
  displayInputEls[1] = document.querySelector("#display-input2");
  displayMsgEl = document.querySelector("#display-msg");
  document.querySelector("#display-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    display();
  });

  // openSettings
  setupOpenSettings();
});

async function display() {
  if (displayMsgEl && displayInputEls[0] && displayInputEls[1]) {
    const contents: SupportersList = await invoke("ladder", {
      input: {
        user_id: displayInputEls[0].value,
        offset: displayInputEls[1].value
      }
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