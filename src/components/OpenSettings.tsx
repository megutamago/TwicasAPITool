import { invoke } from '@tauri-apps/api/tauri';

let isSettingsDialogOpen = false;

export function setupOpenSettings() {
  const settingsButton = document.getElementById('settings-button')!;
  const settingsDialog = document.getElementById('settings-dialog')!;
  const accessTokenInput = document.getElementById('accessTokenInput') as HTMLInputElement;
  const saveAccessTokenBtn = document.getElementById('saveAccessTokenBtn')!;

  settingsButton.addEventListener('click', () => {
    if (!isSettingsDialogOpen) {
      settingsDialog.style.display = 'block';
      isSettingsDialogOpen = true;
    } else {
      settingsDialog.style.display = 'none';
      isSettingsDialogOpen = false;
    }
  });

  saveAccessTokenBtn.addEventListener('click', async () => {
    const accessToken = accessTokenInput.value;
    try {
      const response = await invoke('save_access_token', { token: accessToken });
      console.log('Access token saved. Response:', response);
    } catch (error) {
      console.error('Error saving access token:', error);
    }
    // ダイアログを非表示にする
    settingsDialog.style.display = 'none';
    isSettingsDialogOpen = false;
    accessTokenInput.value = ''; // 入力フィールドをクリアする
  });
}
