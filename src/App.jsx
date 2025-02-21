import { createEffect, createSignal, Index, Show } from "solid-js";
import "./App.css";
import i18n from "./i18n";
import { alternative, pop, rock, electronic, reggae, misc } from "./signals";
import { invoke } from "@tauri-apps/api/core";
import { Radio } from "./Radio";
import { listen } from '@tauri-apps/api/event';
import { Settings } from "./Components/settings";
import { ProgressBar } from "./Components/ProgressBar";
function App() {
  const [home, setHome] = createSignal(true);
  const [tracksShow, setTracksShow] = createSignal(false);
  const [tracks, setTracks] = createSignal();
  const [genre, setGenre] = createSignal(false);
  const [artist, setArtist] = createSignal(false);
  const [radio, setRadio] = createSignal(false);
  const [player, setPlayer] = createSignal(false);
  const [playing, setPlaying] = createSignal(false);
  const [paused, setPaused] = createSignal(false);
  const [muse, setMuse] = createSignal(false);
  const [stringTracks, setStringTracks] = createSignal("");
  const [genres, setGenres] = createSignal();
  const [currentLocation, setCurrentLocation] = createSignal("C:\\Users\\James\\Music\\(I Can_t Get No) Satisfaction.mp3");
  const [currentArtist, setCurrentArtist] = createSignal("The Rolling Stones");
  const [currentTitle, setCurrentTitle] = createSignal("(I Can't Get No) Satisfaction");
  const [currentGenre, setCurrentGenre] = createSignal("Rock");
  const [cover, setCover] = createSignal("");
  const [currentDuration, setCurrentDuration] = createSignal(0);
  const [currentTime, setCurrentTime] = createSignal(0);
  const [stopInterval, setStopInterval] = createSignal(false);
  const [debugMode, setDebugMode] = createSignal(false);
  const [settings, setSettings] = createSignal(false);
  const [recurLevel, setRecurLevel] = createSignal(0);

  setStopInterval(true);

  const scanMusic = async () => {
    setTracks(await invoke("scan_music_files"));
    await invoke("save_music_data", {data: tracks()});
  };


  async function get_progress() {
    const progress = await invoke("get_song_progress");
    setCurrentTime(progress);
  }
  const handleMenuClick = () => {
    setHome(true);
    setTracksShow(false);
    setGenre(false);
    setArtist(false);
    setRadio(false);
    setPlayer(false);
    setDebugMode(false);
    setSettings(false);
    invoke("log", {message: "Menu Clicked"});
    stop_track();
  };
  async function clear() {
    setSettings(false);
    setHome(false);
    setTracksShow(false);
    setGenre(false);
    setArtist(false);
    setRadio(false);
    
  }

  const togglePlay = () => {
    setPlaying(!playing());
    toggle_play();
  };

  async function check_playing() {
      const status = await invoke("is_playing");
      setPlaying(status);
  }
  async function start_timer() {
    const interval = setInterval(() => {
      get_progress();
      if (currentTime() >= currentDuration()) {
        next_track();
      }
    }, 1000);
  }

  async function next_track() {
    await invoke("stop");
    const trackList = tracks();
    if (!trackList || trackList.length === 0) return;
    const currentIndex = trackList.findIndex(t => t.location === currentLocation());
    const nextIndex = currentIndex === -1 || currentIndex === trackList.length - 1 ? 0 : currentIndex + 1;
    const prevIndex = currentIndex <= 0 ? trackList.length - 1 : currentIndex - 1;
    const next = trackList[nextIndex];
    const prev = trackList[prevIndex];
    setCurrentLocation(next.location);
    setCurrentArtist(next.artist);
    setCurrentTitle(next.title);
    setCurrentGenre(next.genre);
    setCurrentDuration(next.duration);
    setCurrentTime(next.duration)
    setCover(next.cover);
    start_track();
  }

  async function prev_track() {
    await invoke("stop");
    const trackList = tracks();
    if (!trackList || trackList.length === 0) return;
    const currentIndex = trackList.findIndex(t => t.location === currentLocation());
    const nextIndex = currentIndex === -1 || currentIndex === trackList.length - 1 ? 0 : currentIndex + 1;
    const prevIndex = currentIndex <= 0 ? trackList.length - 1 : currentIndex - 1;
    const prev = trackList[prevIndex];
    const next = trackList[nextIndex];
    setCurrentLocation(prev.location);
    setCurrentArtist(prev.artist);
    setCurrentTitle(prev.title);
    setCurrentGenre(prev.genre);
    setCurrentTime(prev.duration);
    setCover(prev.cover);
    start_track();
  }
  async function get_tracks() {
    // const tracksData = await invoke("scan_music_files");
    const tracksData = await invoke("get_scan_file");
    setTracks(tracksData);
    setStringTracks(JSON.stringify(tracksData));
  }
  get_tracks();
  async function get_genres() {

  }
  async function start_track() {
    await invoke("play_song", {location: currentLocation()})
    setPlaying(true);
    setCurrentTime(0);
    setStopInterval(false);
    start_timer();
  }

  const onSeek = async (time) => {
    await invoke("seek_to", {timeMs: time});
  };

  const handleInput = (event) => {
    onSeek(Number(event.target.value));
  };

  const handleChange = (event) => {
    onSeek(Number(event.target.value)); // Call the parent function to update playback time
  };

  async function stop_track() {
    await invoke("stop")
    setStopInterval(true);
    setPlaying(false);
    setPaused(false);
  }
  stop_track();
  async function toggle_play() {
    setPlaying(!playing());
    await invoke("toggle_playing");
  }

  async function get_duration() {
    const duration = await invoke("convert_to_milli", {seconds: currentDuration()});
  }
  async function main_loop() {
    const timer = setInterval(() => {
      get_progress();
      get_duration();
      if (currrentProgress() >= currentDuration()) {
        next_track();
      }
    }, 100);
  }
  main_loop();
  createEffect(() => {
    get_duration();
    get_progress();
    check_playing();
  }
  );

  return (
    <main>
      <Show when={home()}>
        <ul>
          <li
            onClick={() => {
              clear();
              setArtist(true);
            }}
          >
            <h2>Artists</h2>
          </li>
          <li
            onClick={() => {
              clear();
              setGenre(true);
            }}
          >
            <h2>Genres</h2>
          </li>
          <li
            onClick={() => {
              get_tracks();
              clear();
              setTracksShow(true);
            }}
          >
            <h2>Tracks</h2>
          </li>
          <li
            onClick={() => {
              clear();
              setRadio(true);
            }}
          >
            <h2>Radio</h2>
          </li>
          <li onClick={() => {
            clear();
            setSettings(true);
          }}><h2>Settings</h2></li>
          <li
            onClick={() => {
              clear();
                setDebugMode(true);
              }}
              >
              <h2>Debug</h2>
              </li>
            </ul>
            </Show>
            <Show when={radio()}>
            <Radio MenuClick={handleMenuClick} />
            </Show>
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
            <Show when={tracksShow()}>
            <ul>
              <li onClick={() => handleMenuClick()}>
              <h2 onClick={handleMenuClick}>&larr; Back</h2>
              </li>
              <Index each={[...(tracks() || [])].sort((a, b) => a.title.localeCompare(b.title))}>
              {(track) => (
                <li
                onClick={() => {
                  setCurrentLocation(track().location);
                  setCurrentArtist(track().artist);
                  setCurrentTitle(track().title);
                  setCurrentGenre(track().genre);
                  setCurrentDuration(track().duration);
                  setCover(track().cover);
                  get_duration();
                  setPlayer(true);
                  setMuse(true);
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
            <Show when={genre()}>
            <h2 onClick={handleMenuClick}>&larr; Back</h2>
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
                    setTracks(filtered);
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
              <Show when={artist()}>
              <div class="artist-section">
              <h2 onClick={handleMenuClick}>&larr; Back</h2>
                <ul>
                {(() => {
              const fullList = stringTracks() ? JSON.parse(stringTracks()) : [];
              const uniqueArtists = Array.from(new Set(fullList.map(t => t.artist))).sort();
              return uniqueArtists.map(a => (
                <li
                  onClick={() => {
                    const filtered = fullList.filter(t => t.artist === a);
                    setTracks(filtered);
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
      <Show when={debugMode()}>
        <div class="debug-section">
          <ul>
            <li onClick={handleMenuClick}><h3>&larr; Back</h3></li>
          </ul>
          <h2>Debug Info</h2>
          <p>{stringTracks()}</p>
        </div>
      </Show>
      <Show when={settings()}>
        <Settings MenuClick={handleMenuClick} Scan={scanMusic}/>
      </Show>
    </main>
  );
}

export default App;