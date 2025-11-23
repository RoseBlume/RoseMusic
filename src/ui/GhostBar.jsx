import { createMemo } from "solid-js";
import { percent } from "../lib/signals";

export function GhostBar() {
    const clamped = createMemo(() => Math.min(100, Math.max(0, percent())));
    return (
        <div class="ghostbar-container">
            <div
                class="ghostbar-fill"
                style={{ width: clamped() + "%" }}
            >
            </div>
        </div>
    );
}