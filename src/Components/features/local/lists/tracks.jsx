import { Show, Index } from "solid-js";
import { Back } from "../../../common/Components/back";
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

} from "../../../common/signals";
import { clear, start_track } from "../../../common/funcs";
import { LoadingAnimation } from "../../../common/Components/load";
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