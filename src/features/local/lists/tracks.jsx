import { Show, Index } from "solid-js";
import { Back } from "../../../ui/back";
import {  
    tracksShow,
    setCurrentLocation,
    setCurrentGenre,
    setCurrentArtist,
    setCurrentTitle,
    setCurrentDuration,
    setCover,
    setPlayer,
    playlist

} from "../../../lib/signals";
import { clear, start_track } from "../../../lib/funcs";
import { LoadingAnimation } from "../../../ui/load";
export function Tracks() {
    return (
        <Show when={tracksShow()}>
            <Back />
            <LoadingAnimation />
            <ul>
            <Index each={[...(playlist() || [])].sort((a, b) => a.title.localeCompare(b.title))}>
            {(track) => (
                <li
                onClick={() => {
                    setCurrentLocation(track().location);
                    setCurrentArtist(track().artist);
                    setCurrentTitle(track().title);
                    setCurrentGenre(track().genre);
                    setCurrentDuration(track().duration);
                    setCover(track().cover);
                    setPlayer(true);
                    clear();
                    start_track();
                    }}
                >
                <h2>{track().title}</h2>
                </li>
            )}
            </Index>
            </ul>
        </Show>
    );
}