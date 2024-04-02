import { confirm } from '@tauri-apps/plugin-dialog';

window.onload = async () => {

  // Creates a confirmation Ok/Cancel dialog
  const confirmation = await confirm(
    'This action cannot be reverted. Are you sure?',
    { title: 'Tauri', kind: 'warning' }
  );

  console.log(confirmation);
}
