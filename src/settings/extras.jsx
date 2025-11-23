import { Show } from "solid-js";
import { handleMenuClick, scanMusic } from "../lib/funcs";
import { settings } from "../lib/signals";
export function ExtraSettings() {
  return (
    <Show when={settings()}>
        <ul>
            <li onClick={handleMenuClick}>
                <h2>Home</h2>
            </li>
            <li onClick={() => {
                scanMusic();
                handleMenuClick();
                }}>
                <h2>Scan Music Folder</h2>
            </li>
        </ul>
    </Show>
  );
}