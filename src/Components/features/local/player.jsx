import { Show } from "solid-js";
import { playing, player, currentArtist, currentTitle, currentTime, currentDuration, cover } from "../../common/signals";
import { handleInput, handleChange, prev_track, next_track, toggle_play, handleMenuClick } from "../../common/funcs"
export function Player() {

    return(
        <Show when={player()}>
            <div class="music-player">
                <div class="album-art">
                    <p class="spacer" />
                        <img src={cover() || "/covers/default_cover.png"} alt="Album Cover" class="cov" />
                    <p class="spacer" />
                </div>
                <div class="song-info">
                    <h1 class="song-title">{currentTitle()}</h1>
                    <h2 class="artist">By {currentArtist()}</h2>
                    <h3>Current Time: {currentTime()}</h3>
                    <h3>Duration: {currentDuration()}</h3>
                </div>
                <div class="progress-bar">
                    <input
                        type="range"
                        min="0"
                        max={currentDuration()}
                        value={currentTime()}
                        onInput={handleInput}
                        onChange={handleChange}
                        style={{ width: "100%" }}
                    />
                </div>
                <div class="player-buttons">
                    <button class="menbut" onClick={prev_track}>
                        <img class="buttons" src="/buttons/previous-light.webp" style="transform: rotate(0deg);" />
                    </button>
                    <Show when={playing()}>
                        <button class="menbut" onClick={toggle_play}>
                            <img class="buttons" src="/buttons/pause-light.png" />
                        </button>
                    </Show>
                    <Show when={!playing()}>
                        <button class="menbut" onClick={toggle_play}>
                            <img class="buttons" src="/buttons/play-light.png" />
                        </button>
                    </Show>
                    <button class="menbut" onClick={next_track}>
                        <img class="buttons" src="/buttons/previous-light.webp" style="transform: rotate(180deg);" />
                    </button>
                    <div id="menu">
                        <button class="menbut" onClick={handleMenuClick}>
                            <img class="buttons" src="/buttons/Menu-Light.png" />
                        </button>
                </div>
                </div>
            </div>
        </Show>
);}