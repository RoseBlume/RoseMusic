import { Show } from "solid-js";

import { setArtist, setPlaylist, setTracksShow, stringTracks, artist } from "../../../lib/signals";


import { Back } from "../../../ui/back";
export function Artists() {
    return(
        <Show when={artist()}>
            <div class="artist-section">
                <Back />
                <ul>
                    {(() => {
                    const fullList = stringTracks() ? JSON.parse(stringTracks()) : [];
                    const uniqueArtists = Array.from(new Set(fullList.map(t => t.artist))).sort();
                    return uniqueArtists.map(a => (
                    <li
                        onClick={() => {
                        const filtered = fullList.filter(t => t.artist === a);
                        setPlaylist(filtered);
                        setArtist(false);
                        setTracksShow(true);
                        }}
                    >
                        <h2>{a}</h2>
                    </li>
                    ));
                    })()}
                </ul>
            </div>
        </Show>
    );
}