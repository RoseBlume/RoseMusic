import { createSignal, For, Show } from "solid-js";
import i18n from "../../i18n";
import { radio } from "../../lib/signals";
import { stations} from "./stations"; // single array with all stations
import { Back } from "../../ui";
export function Radio(Props) {
  const [cover, setCover] = createSignal("covers/Leaning_Towers_Of_Babylon.png");
  const [songTitle, setSongTitle] = createSignal("");
  const [artist, setArtist] = createSignal("");
  const [songSrc, setSongSrc] = createSignal("");
  const [prefix, setPrefix] = createSignal("By: ");
  const [playing, setPlaying] = createSignal(false);
  const [loading, setLoading] = createSignal(false);
  const [showPlayer, setShowPlayer] = createSignal(false);
  const [showRadio, setShowRadio] = createSignal(true);
  const [progress, setProgress] = createSignal(0);
  const [sortKey, setSortKey] = createSignal("title");
  const [sortOrder, setSortOrder] = createSignal("asc");
  const [genreFilter, setGenreFilter] = createSignal("All");

  let audio;

  const togglePlay = () => {
    if (playing()) audio.pause();
    else audio.play();
    setPlaying(!playing());
  };

  const updateProgress = () => {
    const progress = (audio.currentTime / audio.duration) * 100;
    setProgress(progress);
  };

  const handleStationClick = (station) => {
    setLoading(true);
    setCover(station.image);
    setSongTitle(station.title);
    setArtist("");
    setSongSrc(station.src);
    setShowRadio(false);
    setShowPlayer(true);
    setPrefix("");
    audio.addEventListener("canplaythrough", () => setLoading(false), { once: true });
  };

  const sortedStations = () => {
    let list = stations();
    if (genreFilter() !== "All") {
      list = list.filter((s) => s.genre === genreFilter());
    }
    return [...list].sort((a, b) => {
      const key = sortKey();
      const order = sortOrder() === "asc" ? 1 : -1;
      if (key === "bitrate") return (parseInt(a.bitrate) - parseInt(b.bitrate)) * order;
      return a.title.localeCompare(b.title) * order;
    });
  };

  const toggleSort = (key) => {
    if (sortKey() === key) {
      setSortOrder(sortOrder() === "asc" ? "desc" : "asc");
    } else {
      setSortKey(key);
      setSortOrder("asc");
    }
  };

  const genres = () => {
    const unique = new Set(stations().map((s) => s.genre));
    return ["all", ...Array.from(unique)];
  };

  return (
    <Show when={radio()}>
      <Back />
      {showRadio() && (
        <>
          <div class="genre-filter">
            <label>{i18n.t("Genre")}:</label>
            <select onInput={(e) => setGenreFilter(e.target.value)}>
              <For each={genres()}>
                {(g) => <option value={g}>{g}</option>}
              </For>
            </select>
          </div>

          <table class="radio-table">
            <thead>
              <tr>
                <th onClick={() => toggleSort("title")}>{i18n.t("Title")}</th>
                <th>{i18n.t("Genre")}</th>
                <th onClick={() => toggleSort("bitrate")}>{i18n.t("Bitrate")}</th>
              </tr>
            </thead>
            <tbody>
              <For each={sortedStations()}>
                {(station) => (
                  <tr
                    class="RadioList"
                    onClick={() => handleStationClick(station)}
                    style={{ cursor: "pointer" }}
                  >
                    <td>{station.title}</td>
                    <td>{station.genre}</td>
                    <td>{station.bitrate}</td>
                  </tr>
                )}
              </For>
            </tbody>
          </table>
        </>
      )}

      {showPlayer() && (
        <div class="music-player">
          <div class="album-art">
            <p class="spacer" />
            {loading() ? (
              <img src="/covers/pulsing_dot_loader.gif" alt="Album Cover" class="cov" />
            ) : (
              <img src={cover()} class="cov" alt="Album Cover" />
            )}
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
              {playing() ? (
                <button class="menbut" onClick={togglePlay}>
                  <img class="buttons" src="/buttons/pause-light.png" />
                </button>
              ) : (
                <button onClick={togglePlay}>
                  <img class="buttons" src="buttons/play-light.png" />
                </button>
              )}
              <div id="menu">
                <button
                  class="menbut"
                  onClick={() => {
                    setShowRadio(true);
                    setShowPlayer(false);
                    setPlaying(false);
                    Props.MenuClick();
                  }}
                >
                  <img class="buttons" src="buttons/Menu-Light.png" />
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </Show>
  );
}
