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
  const [showMenu, setShowMenu] = createSignal(true);
  const [showPlayer, setShowPlayer] = createSignal(false);
  const [showList, setShowList] = createSignal(false);
  const [showRadio, setShowRadio] = createSignal(false);
  const [showLocal, setShowLocal] = createSignal(false);

  const [progress, setProgress] = createSignal(0);
  let audio;


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
  return (
    <div id="area">
      <h2>{localMusic()}</h2>
      <h2>{localNames()}</h2>
      <Show when={showMenu()}>
        <ul><li  onClick={() => {
          setShowMenu(false);
          setShowRadio(true);
        }}><h2>Radio Stations</h2>
        </li>
        <li onClick={() => {
          setShowMenu(false);
          setShowList(true);
        }}><h2>Included Songs</h2>
        </li>
        </ul>
      </Show>
      <Show when={showRadio()}>
        <div id="radio">
          <ul>
            <Index each={stations()}>{(station, index) => <li class="RadioList" onClick={() => {
              setCover(station().image);
              setSongTitle(station().title);
              setArtist("");
              setSongSrc(station().src);
              setPlaying(true);
              setShowRadio(false);
              setRadioPlay(true);
              setShowPlayer(true);
              setPrefix("");
              read();
            }}>{station().title}</li>}</Index>
          </ul>
        </div>
      </Show>

      <Show when={showList()}>
        <div id="list">
          <ul>
            <Index each={songs()}>{(song, index) => <li class="SongList" onClick={() => {
              setCover(song().cover);
              setSongTitle(song().title);
              setArtist(song().artist);
              setSongSrc(song().src);
              setPlaying(true);
              setShowList(false);
              setShowPlayer(true);
              setPrefix("By: ");
            }}>{song().title}</li>}</Index>
          </ul>
        </div>
      </Show>
      
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
                        setShowMenu(true);
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
