// src/components/MusicPlayer.jsx
import { Show } from "solid-js";

const MusicPlayer = ({ cover, songTitle, artist, prefix, songSrc, playing, togglePlay, progress, updateProgress, setPlaying, setShowRadio, setShowPlayer }) => {
  let audio;

  return (
    <Show when={true}>
      <div class="music-player">
        <div class="album-art">
          <p class="spacer" />
          <img src={cover()} class="cov" alt="Album Cover" />
          <p class="spacer" />
        </div>

        <div class="song-info">
          <h1 class="song-title">{songTitle()}</h1>
          <h3 class="artist">{prefix()}{artist()}</h3>
        </div>

        <div class="controls">
          <audio
            ref={(el) => (audio = el)}
            src={songSrc()}
            onTimeUpdate={updateProgress}
            autoPlay
            onEnded={() => setPlaying(false)}
          ></audio>

          <div class="progress-container">
            <input
              type="range"
              class="progress-bar"
              value={progress()}
              min="0"
              max="100"
              onInput={(e) => {
                const seekTime = (e.target.value / 100) * audio.duration;
                audio.currentTime = seekTime;
              }}
            />
          </div>

          <div class="player-buttons">
            {playing() ? <button class="menbut" onClick={togglePlay}><img class="buttons" src="/buttons/pause-light.png"></img></button> : <button onClick={togglePlay}><img class="buttons" src="buttons/play-light.png"></img></button>}
            <div id="menu">
              <button class="menbut" onClick={() => {
                setShowRadio(true);
                setShowPlayer(false);
              }}>
                <img class="buttons" src="buttons/Menu-Light.png" />
              </button>
            </div>
          </div>
        </div>
      </div>
    </Show>
  );
};

export default MusicPlayer;