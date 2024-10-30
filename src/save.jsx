/*import { createSignal, Index, Show } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { readDir, exists, BaseDirectory, audioDir } from '@tauri-apps/plugin-fs';
import { path as tauriPath } from '@tauri-apps/api';
import { convertFileSrc } from "@tauri-apps/api/core";
//import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';
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
/*
const [stations] = createSignal([
  {
    title: "Antenne Bayern",
    src: "http://stream.antenne.de:80/antenne",
    image: "https://www.antenne.de/media/cache/3/version/18696/streamlogo_simulcast_live_aby_neu_2000x2000-v1.jpg/5e26f53136d9fcbd4f9101a8e1c652ba.webp"
  },
  {
    title: "Bayern Rock",
    src: "http://stream.antenne.de:80/rockantenne",
    image: "https://www.antenne.de/media/cache/3/version/18696/streamlogo_simulcast_live_aby_neu_2000x2000-v1.jpg/5e26f53136d9fcbd4f9101a8e1c652ba.webp"
  },
  {
    "src": "http://stream.antenne.de:80/chillout",
    "title": "ANTENNE BAYERN Chillout (Germany)",
    "image": "covers/default.png"
  },
  {
    "src": "https://cp1.sednastream.com:8014/stream",
    "title": "ClubFM Albania",
    "image": "covers/default.png"
  },
  {
    "src": "https://broadcast.radioponiente.org:8038/",
    "title": "ALMERIA TRENDY FM",
    "image": "covers/default.png"
  }
]);
const [songs] = createSignal([
  {
    title: "Leaning Tower Of Babylon",
    src: "music/Leaning_Tower_Of_Babylon.mp3",
    artist: "Various",
    cover: "covers/Leaning_Towers_Of_Babylon.png"
  },
  {
    title: "Life Jobs Greatest Trial",
    src: "music/Life_Jobs_Greatest_Trial.mp3",
    artist: "Various",
    cover: "covers/Jobs_Greatest_Trial.png"
  },
  {
    title: "Preacher In The End Times",
    src: "music/Preacher_In_The_End_Times.mp3",
    artist: "Various",
    cover: "covers/Preacher_In_The_End_Times.png"
  },
  {
    title: "Red Sea Division",
    src: "music/Red_Sea_Division.mp3",
    artist: "Various",
    cover: "covers/Red_Sea_Division.png"
  },
  {
    title: "Riders Of Revelations",
    src: "music/Riders_Of_Revelations.mp3",
    artist: "Various",
    cover: "covers/Riders_Of_Revelations.png"
  },
  {
    title: "Samsons Roar",
    src: "music/Samsons_Roar.mp3",
    artist: "Various",
    cover: "covers/Lions_Roar.png"
  },
  {
    title: "Temptations",
    src: "music/Temptations.mp3",
    artist: "Various",
    cover: "covers/Touch_Of_Gold.png"
  },
  {
    title: "The Martyr Of North Korea",
    src: "music/The_Martyr_Of_North_Korea.mp3",
    artist: "Various",
    cover: "covers/Martyr_Of_North_Korea.png"
  },
  {
    title: "The Prodigal Sons Journey",
    src: "music/The_Prodigal_Sons_Journey.mp3",
    artist: "Various",
    cover: "covers/Prodigal_Songs_Journey.png"
  },
  {
    title: "Daniel And The Devils Den",
    src: "music/Daniel_And_The_Devils_Den.mp3",
    artist: "Various",
    cover: "covers/The_Devils_Den.png"
  },
  {
    title: "David Slayer Of The Mighty",
    src: "music/David_Slayer_Of_The_Mighty.mp3",
    artist: "Various",
    cover: "covers/David_Slayer_Of_The_Mighty.png"
  },
  {
    title: "Fire And Salt",
    src: "music/Fire_And_Salt.mp3",
    artist: "Various",
    cover: "covers/Fire_And_Salt.png"
  },
  {
    title: "King Sauls Downfall",
    src: "music/King_Sauls_Downfall.mp3",
    artist: "Various",
    cover: "covers/King_Sauls_Downfall.png"
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
/*
function App() {
  const [audioSrc, setAudioSrc] = createSignal("");

  createEffect(async () => {
    const audioDir = await tauriPath.audioDir();
    const path = await tauriPath.join(audioDir, "Fire_And_Salt.mp3");
    const url = convertFileSrc(path);
    setAudioSrc(url);
  });

  return (
    <div>
      <h1>Audio Player</h1>
      <Show when={audioSrc()}>
        <audio src={audioSrc()} controls />
      </Show>
    </div>
  );
};

export default App;

import { createSignal, Index, Show } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
//import { readDir, exists, BaseDirectory } from '@tauri-apps/plugin-fs';
//import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';
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