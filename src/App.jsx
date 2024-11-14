import { createSignal, Index, Show } from "solid-js";
import "./App.css";
import i18n from "./i18n";
import { alternative, pop, rock, electronic, reggae, misc } from "./signals";


try {
  i18n.locale(navigator.language.slice(0, 2));
}
catch {
  i18n.locale("es");
}


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
  // Loading Variables
  const [playing, setPlaying] = createSignal(false);
  const [loading, setLoading] = createSignal(false);

  const [radioPlay, setRadioPlay] = createSignal(false);
  const [music, SetMusic] = createSignal("");
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
  // Update progress bar
  const updateProgress = () => {
    const progress = (audio.currentTime / audio.duration) * 100;
    setProgress(progress);
  };

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
          <h2>{i18n.t("alternative")}</h2>
        </li>
        <li onClick={() => {
          clear();
          setPopShow(true);
        }}>
          <h2>{i18n.t("pop")}</h2>
        </li>
        <li onClick={() => {
          clear();
          setRockShow(true);
        }}>
          <h2>{i18n.t("rock")}</h2>
        </li>
        <li onClick={() => {
          clear();
          setElectronicShow(true);
        }}>
          <h2>{i18n.t("electronic")}</h2>
        </li>
        <li onClick={() => {
          clear();
          setReggaeShow(true);
        }}>
          <h2>{i18n.t("reggae")}</h2>
        </li>
        <li onClick={() => {
          clear();
          setMiscShow(true);
        }}>
          <h2>{i18n.t("misc")}</h2>
        </li>

      </Show>
        <div id="radio">
          <ul>
            <Show when = {alternativeShow()}>
            <Index each={alternative()}>{(station, index) => <li class="RadioList" onClick={() => {
              setLoading(true); // Start loading animation
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
              audio.addEventListener('canplaythrough', () => setLoading(false), { once: true });
            }}><h2>{station().title}</h2></li>}</Index>
            </Show>
            <Show when = {popShow()}>
            <Index each={pop()}>{(station, index) => <li class="RadioList" onClick={() => {
              setLoading(true); // Start loading animation
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
              audio.addEventListener('canplaythrough', () => setLoading(false), { once: true });
            }}><h2>{station().title}</h2></li>}</Index>
            </Show>
            <Show when = {rockShow()}>
            <Index each={rock()}>{(station, index) => <li class="RadioList" onClick={() => {
              setLoading(true); // Start loading animation
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
              audio.addEventListener('canplaythrough', () => setLoading(false), { once: true });
            }}><h2>{station().title}</h2></li>}</Index>
            </Show>
            <Show when = {electronicShow()}>
            <Index each={electronic()}>{(station, index) => <li class="RadioList" onClick={() => {
              setLoading(true); // Start loading animation
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
              audio.addEventListener('canplaythrough', () => setLoading(false), { once: true });
            }}><h2>{station().title}</h2></li>}</Index>
            </Show>
            <Show when={reggaeShow()}>
            <Index each={reggae()}>{(station, index) => <li class="RadioList" onClick={() => {
              setLoading(true); // Start loading animation
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
              audio.addEventListener('canplaythrough', () => setLoading(false), { once: true });
            }}><h2>{station().title}</h2></li>}</Index>
            </Show>


            <Show when = {miscShow()}>
            <Index each={misc()}>{(station, index) => <li class="RadioList" onClick={() => {
              setLoading(true); // Start loading animation
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
              audio.addEventListener('canplaythrough', () => setLoading(false), { once: true });
            }}><h2>{station().title}</h2></li>}</Index>
          </Show>
          </ul>
        </div>
      
     <Show when={showPlayer()}> 
      <div class="music-player">
        <div class="album-art">
          <p class="spacer"/>
          { loading() ? <img src="/covers/pulsing_dot_loader.gif" alt="Album Cover" class="cov" /> : <img src={cover()} class="cov" alt="Album Cover" />}
          <p class="spacer"/>
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
                setRadioPlay(false);
              }}>
            <img class="buttons" src="buttons/Menu-Light.png"/>
            </button>
            </div>
          </div>
          
     </div>
     </div>
     </Show>
    </div>
  );
}

export default App;