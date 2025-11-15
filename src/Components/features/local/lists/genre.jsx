import { Show } from "solid-js";

import { setGenre, setPlaylist, setTracksShow, stringTracks, genre } from "../../../common/signals";
import { handleMenuClick } from "../../../common/funcs";
import { Back } from "../../../common/Components/back";
export function Genres() {
    return (
        <Show when={genre()}>
            <Back />
            <div class="genre-section">
                <ul>
                {(() => {
                    const fullList = stringTracks() ? JSON.parse(stringTracks()) : [];
                    const uniqueGenres = [...new Set(fullList.map(t => t.genre))];
                    const sortedGenres = uniqueGenres.sort((a, b) => a.localeCompare(b));
                    return sortedGenres.map(g => (
                        <li
                        onClick={() => {
                        const filtered = fullList.filter(t => t.genre === g);
                        setPlaylist(filtered);
                        setGenre(false);
                        setTracksShow(true);
                        }}
                        >
                            <h2>{g}</h2>
                        </li>
                    ));
                })()}
                </ul>
            </div>
        </Show>
    );
}