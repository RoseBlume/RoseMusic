import { Show } from "solid-js";
import { stringTracks, debugMode } from "../lib/signals";
export function DebugSettings() {
    return (
        <Show when={debugMode()}>
          <div class="debug-section">
            <h2>Debug Info</h2>
            <p>{stringTracks()}</p>
          </div>
        </Show>
    );
}