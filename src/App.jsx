import { createSignal, Index, Show, createEffect } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
//import { readDir, exists, BaseDirectory, audioDir } from '@tauri-apps/plugin-fs';
//import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';
import { convertFileSrc } from "@tauri-apps/api/core";
import { path as tauriPath } from '@tauri-apps/api';
import "./App.css";

/*
<Show when={showLocal()}>
        <div id="local">
          <ul>
            <Index each={localMusic()}>{(localsong, index) => <button class="LocalList" onClick={() => {
              setCover("covers/default.png");
              setSongTitle(localsong().title);
              setArtist("Unknown");
              setSongSrc(localsong().src);
              setPlaying(true);
              setShowLocal(false);
              setShowPlayer(true);
              setPrefix("");
            }
            }>{localsong().title}</button>}</Index>
          </ul>
          </div>
      </Show>
*/
const [alternative] = createSignal([
    {
        "src": "https://shoutcast.brownrice.com:8002/",
        "title": "KNCE Taos Radio",
        "image": "covers/default.png"
    },
    {
        "src": "https://kathy.torontocast.com:2620/",
        "title": "1 Pure Alternative",
        "image": "covers/default.png"
    },
    {
        "src": "https://jm8n.net:8018/stream",
        "title": "Radio La Innovadora TV",
        "image": "covers/default.png"
    },
    {
        "src": "https://streamconex.com:8134/stream",
        "title": "Grito de Rock Argentina - Uruguay",
        "image": "covers/default.png"
    },
    {
        "src": "https://streamconex.com:8180/stream",
        "title": "FM LA Marea",
        "image": "covers/default.png"
    },
    {
        "src": "https://jm8n.net:8066/stream",
        "title": "Marimba de Guatemala Radio",
        "image": "covers/default.png"
    },
    {
        "src": "https://streamconex.com:8104/stream",
        "title": "KLA Radio 97.1",
        "image": "covers/default.png"
    },
    {
        "src": "https://stream.fm90.hu:8000/",
        "title": "Campus R\u00e1di\u00f3 FM90",
        "image": "covers/default.png"
    },
    {
        "src": "https://cast.radiocast.ch:9000/stream",
        "title": "RadioFoleja",
        "image": "covers/default.png"
    },
    {
        "src": "https://streamconex.com:8118/stream",
        "title": "La 106.1 FM",
        "image": "covers/default.png"
    },
    {
        "src": "https://streamconex.com:8122/stream",
        "title": "Fm Riel Basavilbaso",
        "image": "covers/default.png"
    },
    {
        "src": "https://streamconex.com:8062/stream",
        "title": "Radio Digital San Luis",
        "image": "covers/default.png"
    },
    {
        "src": "https://streamconex.com:8096/stream",
        "title": "Radio NDR FM 103.9",
        "image": "covers/default.png"
    },
    {
        "src": "https://media.dominiocreativo.com:8000/",
        "title": "Radio YSKL 104.1 FM",
        "image": "covers/default.png"
    }
]);

const [pop] = createSignal([
  {
    title: "Antenne Bayern",
    src: "http://stream.antenne.de:80/antenne",
    image: "https://www.antenne.de/media/cache/3/version/18696/streamlogo_simulcast_live_aby_neu_2000x2000-v1.jpg/5e26f53136d9fcbd4f9101a8e1c652ba.webp"
  }
]);
const [rock] = createSignal([
  {
    title: "Bayern Rock",
    src: "http://stream.antenne.de:80/rockantenne",
    image: "https://www.antenne.de/media/cache/3/version/18696/streamlogo_simulcast_live_aby_neu_2000x2000-v1.jpg/5e26f53136d9fcbd4f9101a8e1c652ba.webp"
  }
]);
const [misc] = createSignal([
  {
    "src": "https://cp1.sednastream.com:8014/stream",
    "title": "ClubFM Albania",
    "image": "covers/default.png"
  }
]);

/*
<img src={cover()} id="cover" />
      <h2 class="songtitle">{songTitle()}</h2>
      <h3 class="artist">{artist()}</h3>
      <audio id="audio" src={songSrc()} controls autoPlay></audio>




       <button onClick={() => {
          setShowMenu(false);
          setShowLocal(true);
          genMusic();
        }}><h2>Local Music</h2></button>
*/
function App() {
  const [localMusic, setLocalMusic] = createSignal("");
  const [localNames, setLocalNames] = createSignal("");
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");
  const [cover, setCover] = createSignal("covers/Leaning_Towers_Of_Babylon.png");
  const [songTitle, setSongTitle] = createSignal("");
  const [artist, setArtist] = createSignal("");
  const [songSrc, setSongSrc] = createSignal("");
  const [prefix, setPrefix] = createSignal("By: ");
  const [playing, setPlaying] = createSignal(false);
  const [radioPlay, setRadioPlay] = createSignal(false);
  const [music, SetMusic] = createSignal("");
  // Show Variables
  // Genre Variables
  const [alternativeShow, setAlternativeShow] = createSignal(false);
  const [bluesShow, setBluesShow] = createSignal(false);
  const [classicalShow, setClassicalShow] = createSignal(false);
  const [countryShow, setCountryShow] = createSignal(false);
  const [easyListeningShow, setEasyListeningShow] = createSignal(false);
  const [electronicShow, setElectronicShow] = createSignal(false);
  const [folkShow, setFolkShow] = createSignal(false);
  const [themesShow, setThemesShow] = createSignal(false);
  const [rapShow, setRapShow] = createSignal(false);
  const [inspirationalShow, setInspirationalShow] = createSignal(false);
  const [internationalShow, setInternationalShow] = createSignal(false);
  const [jazzShow, setJazzShow] = createSignal(false);
  const [latinShow, setLatinShow] = createSignal(false);
  const [metalShow, setMetalShow] = createSignal(false);
  const [newAgeShow, setNewAgeShow] = createSignal(false);
  const [decadesShow, setDecadesShow] = createSignal(false);
  const [popShow, setPopShow] = createSignal(false);
  const [rbUrbanShow, setRbUrbanShow] = createSignal(false);
  const [reggaeShow, setReggaeShow] = createSignal(false);
  const [rockShow, setRockShow] = createSignal(false);
  const [seasonalHolidayShow, setSeasonalHolidayShow] = createSignal(false);
  const [soundtracksShow, setSoundtracksShow] = createSignal(false);
  const [talkShow, setTalkShow] = createSignal(false);
  const [miscShow, setMiscShow] = createSignal(false);

  // Area Variables
  const [showPlayer, setShowPlayer] = createSignal(false);
  const [showRadio, setShowRadio] = createSignal(true);

  const [progress, setProgress] = createSignal(0);
  let audio;
  async function test() {
    const audioDir = await tauriPath.audioDir();
    const path = await tauriPath.join(audioDir, "Fire_And_Salt.mp3");
    const url = convertFileSrc(path);
    setSongSrc(url);
  };

/*  let path = await window._TAURI_.path.join(await window._TAURI_.path.audioDir(),"file.mp3")
let url = convertFileSrc(path)
  */


// Play or pause the song
  const togglePlay = () => {
    if (playing()) {
      audio.pause();
    } else {
      audio.play();
    }
    setPlaying(!playing());
  };
  // Switch to the previous song
  const prevSong = () => {
    const currentIndex = songs().findIndex(song => song.src === songSrc());
    const prevIndex = (currentIndex - 1 + songs().length) % songs().length;
    const prevSong = songs()[prevIndex];
    setCover(prevSong.cover);
    setSongTitle(prevSong.title);
    setArtist(prevSong.artist);
    setSongSrc(prevSong.src);
    setPlaying(true);
  };
  // Switch to the next song
  const nextSong = () => {
    const currentIndex = songs().findIndex(song => song.src === songSrc());
    const nextIndex = (currentIndex + 1) % songs().length;
    const nextSong = songs()[nextIndex];
    setCover(nextSong.cover);
    setSongTitle(nextSong.title);
    setArtist(nextSong.artist);
    setSongSrc(nextSong.src);
    setPlaying(true);
  };
  // Update progress bar
  const updateProgress = () => {
    const progress = (audio.currentTime / audio.duration) * 100;
    setProgress(progress);
  };
  async function genMusic() {
    const music = await invoke('scan_music');
    setLocalMusic(JSON.parse(music));
  }
  async function clear() {
    setAlternativeShow(false);
    setBluesShow(false);
    setClassicalShow(false);
    setCountryShow(false);
    setEasyListeningShow(false);
    setElectronicShow(false);
    setFolkShow(false);
    setThemesShow(false);
    setRapShow(false);
    setInspirationalShow(false);
    setInternationalShow(false);
    setJazzShow(false);
    setLatinShow(false);
    setMetalShow(false);
    setNewAgeShow(false);
    setDecadesShow(false);
    setPopShow(false);
    setRbUrbanShow(false);
    setReggaeShow(false);
    setRockShow(false);
    setSeasonalHolidayShow(false);
    setSoundtracksShow(false);
    setTalkShow(false);
    setMiscShow(false);
    setShowRadio(false);
  }
  return (
    <div id="area">
      <Show when={showRadio()}>
        <li onClick={() => {
          clear();
          setAlternativeShow(true);
        }}>
          <h2>Alternative</h2>
        </li>
        <li onClick={() => {
          clear();
          setPopShow(true);
        }}>
          <h2>Pop</h2>
        </li>
        <li onClick={() => {
          clear();
          setRockShow(true);
        }}>
          <h2>Rock</h2>
        </li>
        <li onClick={() => {
          clear();
          setMiscShow(true);
        }}>
          <h2>Misc</h2>
        </li>

      </Show>
        <div id="radio">
          <ul>
            <Show when = {alternativeShow()}>
            <Index each={alternative()}>{(station, index) => <li class="RadioList" onClick={() => {
              setCover(station().image);
              setSongTitle(station().title);
              setArtist("");
              setSongSrc(station().src);
              setPlaying(true);
              setShowRadio(false);
              setRadioPlay(true);
              setShowPlayer(true);
              setPrefix("");
              clear();
            }}><h2>{station().title}</h2></li>}</Index>
            </Show>
            <Show when = {popShow()}>
            <Index each={pop()}>{(station, index) => <li class="RadioList" onClick={() => {
              setCover(station().image);
              setSongTitle(station().title);
              setArtist("");
              setSongSrc(station().src);
              setPlaying(true);
              setShowRadio(false);
              setRadioPlay(true);
              setShowPlayer(true);
              setPrefix("");
              clear();
            }}><h2>{station().title}</h2></li>}</Index>
            </Show>
            <Show when = {rockShow()}>
            <Index each={rock()}>{(station, index) => <li class="RadioList" onClick={() => {
              setCover(station().image);
              setSongTitle(station().title);
              setArtist("");
              setSongSrc(station().src);
              setPlaying(true);
              setShowRadio(false);
              setRadioPlay(true);
              setShowPlayer(true);
              setPrefix("");
              clear();
            }}><h2>{station().title}</h2></li>}</Index>
            </Show>
            <Show when = {miscShow()}>
            <Index each={misc()}>{(station, index) => <li class="RadioList" onClick={() => {
              setCover(station().image);
              setSongTitle(station().title);
              setArtist("");
              setSongSrc(station().src);
              setPlaying(true);
              setShowRadio(false);
              setRadioPlay(true);
              setShowPlayer(true);
              setPrefix("");
              clear();
            }}>{station().title}</li>}</Index>
          </Show>
          </ul>
        </div>
      
     <Show when={showPlayer()}> 
      <div class="music-player">
        <div class="album-art">
          <img src={cover()} alt="Album Cover" />
        </div>

        <div class="song-info">
          <h2 class="song-title">{songTitle()}</h2>
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
              <table>
                <tbody>
                <tr>
                  <td>
                    <button onClick={prevSong}>
                    {!radioPlay() && <img class="buttons" src="buttons/previous-light.webp"/>}
                    </button>
                  </td>
                  <td>
                  <button onClick={togglePlay}>
                    {playing() ? <img class="buttons" src="/buttons/pause-light.png"></img> : <img class="buttons" src="buttons/play-light.png"></img>}
                  </button>
                  </td>
                  <td>
                    <button onClick={nextSong}>
                    {!radioPlay() && <img class="buttons" src="buttons/previous-light.webp" id="prevbutton"/>}
                    </button>
                  </td>
                  <td>
                    <div id="menu">
                      <button onClick={() => {
                        setShowRadio(true);
                        setShowPlayer(false);
                        setRadioPlay(false);
                      }}>
                    <img class="buttons" src="buttons/Menu-Light.png"/>
                    </button>
                    </div>
                  </td>
                </tr>
                </tbody>
              </table>
                

          </div>
          
     </div>
     </div>
     </Show>
    </div>
  );
}

export default App;