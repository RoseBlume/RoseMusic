
import { listen } from "@tauri-apps/api/event";

import { LoadingAnimation, GhostBar } from "../ui";

import { setPercent } from "../lib/signals";
import { finishSearch, scanMusic, updateWindow } from "../lib/funcs";

export function Splashscreen() {
    scanMusic();
    listen('finished-searching', (event) => {
        finishSearch(event.payload.obj);
    });

    listen('scan-progress', (event) => {
        setPercent(event.payload.percentage);
        if (event.payload.percentage == 100) {
            updateWindow();
        }
    });

    return(
        <div>
            <LoadingAnimation />
            <GhostBar />
        </div>
    );
}