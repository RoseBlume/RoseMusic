import { createSignal, Index, Show } from "solid-js";
//import { invoke } from "@tauri-apps/api/core";
//import { readDir, exists, BaseDirectory, audioDir } from '@tauri-apps/plugin-fs';
//import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';
//import { convertFileSrc } from "@tauri-apps/api/core";
//import { path as tauriPath } from '@tauri-apps/api';
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
    "image": "covers/9.avif"
  },
  {
    "src": "https://kathy.torontocast.com:2620/",
    "title": "1 Pure Alternative",
    "image": "covers/13.avif"
  },
  {
    "src": "https://jm8n.net:8018/stream",
    "title": "Radio La Innovadora TV",
    "image": "covers/15.avif"
  },
  {
    "src": "https://streamconex.com:8134/stream",
    "title": "Grito de Rock Argentina - Uruguay",
    "image": "covers/16.avif"
  },
  {
    "src": "https://streamconex.com:8180/stream",
    "title": "FM LA Marea",
    "image": "covers/3.avif"
  },
  {
    "src": "https://jm8n.net:8066/stream",
    "title": "Marimba de Guatemala Radio",
    "image": "covers/11.avif"
  },
  {
    "src": "https://streamconex.com:8104/stream",
    "title": "KLA Radio 97.1",
    "image": "covers/13.avif"
  },
  {
    "src": "https://stream.fm90.hu:8000/",
    "title": "Campus R\u00e1di\u00f3 FM90",
    "image": "covers/11.avif"
  },
  {
    "src": "https://cast.radiocast.ch:9000/stream",
    "title": "RadioFoleja",
    "image": "covers/14.avif"
  },
  {
    "src": "https://streamconex.com:8118/stream",
    "title": "La 106.1 FM",
    "image": "covers/16.avif"
  },
  {
    "src": "https://streamconex.com:8122/stream",
    "title": "Fm Riel Basavilbaso",
    "image": "covers/1.avif"
  },
  {
    "src": "https://streamconex.com:8062/stream",
    "title": "Radio Digital San Luis",
    "image": "covers/16.avif"
  },
  {
    "src": "https://streamconex.com:8096/stream",
    "title": "Radio NDR FM 103.9",
    "image": "covers/4.avif"
  },
  {
    "src": "https://media.dominiocreativo.com:8000/",
    "title": "Radio YSKL 104.1 FM",
    "image": "covers/5.avif"
  }
]);

const [pop] = createSignal([
  {
    title: "Antenne Bayern",
    src: "http://stream.antenne.de:80/antenne",
    image: "https://www.antenne.de/media/cache/3/version/18696/streamlogo_simulcast_live_aby_neu_2000x2000-v1.jpg/5e26f53136d9fcbd4f9101a8e1c652ba.webp"
  },
  {
    "src": "http://popradiostream.se/stream/1:8001/",
    "title": "Popradio Stockholm",
    "image": "covers/2.avif"
  },
  {
    "src": "https://stream.drukciji.ba:9998/stream",
    "title": "Drukciji Radio",
    "image": "covers/10.avif"
  },
  {
    "src": "https://a1.asurahosting.com:8020/radio.mp3",
    "title": "Ka'u Radio Station",
    "image": "covers/11.avif"
  },
  {
    "src": "https://broadcast.radioponiente.org:8036/",
    "title": "RADIO PONIENTE",
    "image": "covers/6.avif"
  },
  {
    "src": "https://yayin.firatfm.net:8016/stream",
    "title": "Radio Firat Fm Pop",
    "image": "covers/2.avif"
  },
  {
    "src": "https://streams.radiomast.io:443/ba864be6-11d1-4e13-aa74-f8e8781f64f6",
    "title": "BraidwoodFM",
    "image": "covers/13.avif"
  },
  {
    "src": "https://media.srb-streaming.com:8002/stream",
    "title": "Radio Sumadinac EX YU",
    "image": "covers/17.avif"
  },
  {
    "src": "https://radio.persianvip.com:8888/",
    "title": "Persian VIP",
    "image": "covers/13.avif"
  },
  {
    "src": "https://server-27.stream-server.nl:8140/stream",
    "title": "Rivierenland Radio",
    "image": "covers/1.avif"
  },
  {
    "src": "http://stream.antenne.de:80/greatest-hits",
    "title": "ANTENNE BAYERN Greatest Hits",
    "image": "https://www.antenne.de/media/cache/3/version/18696/streamlogo_simulcast_live_aby_neu_2000x2000-v1.jpg/5e26f53136d9fcbd4f9101a8e1c652ba.webp"
  },
  {
    "src": "https://radio.mediacp.eu:8072/stream",
    "title": "LOVE MOMENTS RADIO",
    "image": "covers/9.avif"
  },
  {
    "src": "https://radio.en.rs:8000/radio.mp3",
    "title": "996FM Kraljevo",
    "image": "covers/6.avif"
  },
  {
    "src": "https://radiohuis.com:9200/ElisaFM",
    "title": "Elisa FM Belgi\u00eb",
    "image": "covers/2.avif"
  },
  {
    "src": "https://broadcast.radioponiente.org:8034/",
    "title": "DALIAS RADIOLUZ CLASSIC",
    "image": "covers/16.avif"
  },
  {
    "src": "https://stream.radio-mit-herz.de:8000/radio.mp3",
    "title": "Radio mit Herz",
    "image": "covers/9.avif"
  },
  {
    "src": "https://streams.radiomast.io:443/abe5f558-c883-47de-91ea-e04a06ed1fd4",
    "title": "lumiRadio - 24/7 Unofficial Homestuck Radio",
    "image": "covers/1.avif"
  },
  {
    "src": "https://stream.badratunfm.com:8000/live",
    "title": "Radio Badratun FM Sigli",
    "image": "covers/14.avif"
  },
  {
    "src": "https://masterdomains.be:8000/",
    "title": "Radio Superstar2",
    "image": "covers/1.avif"
  },
  {
    "src": "https://cp1.sednastream.com:8014/stream",
    "title": "ClubFM Albania",
    "image": "covers/17.avif"
  },
  {
    "src": "https://broadcast.radioponiente.org:8038/",
    "title": "ALMERIA TRENDY FM",
    "image": "covers/16.avif"
  },
  {
    "src": "https://stm12.xcast.com.br:9904/",
    "title": "Mais Ouvinda",
    "image": "covers/2.avif"
  },
  {
    "src": "https://stream.servidorm24.net:7004/stream",
    "title": "Radio Patagonia",
    "image": "covers/2.avif"
  },
  {
    "src": "https://broadcast.radioponiente.org:8030/",
    "title": "ALMERIA SI FM",
    "image": "covers/14.avif"
  },
  {
    "src": "https://server.mixify.in:8040/radio.mp3",
    "title": "Mixify English Hits - Tune in to Listen - www.mixify.in - Play it Loud",
    "image": "covers/1.avif"
  },
  {
    "src": "https://s08.w3bserver.com:8170/radio.mp3",
    "title": "Stuffmix",
    "image": "covers/13.avif"
  },
  {
    "src": "https://b1.alhastream.com:4090/radio",
    "title": "107,8 New Prasasty FM Banyuwangi",
    "image": "covers/7.avif"
  },
  {
    "src": "https://jm8n.net:8160/stream",
    "title": "La Poderosa de Rancho",
    "image": "covers/11.avif"
  },
  {
    "src": "http://stream.antenne.de:80/2010er-hits",
    "title": "ANTENNE BAYERN 2010er Hits",
    "image": "https://www.antenne.de/media/cache/3/version/18696/streamlogo_simulcast_live_aby_neu_2000x2000-v1.jpg/5e26f53136d9fcbd4f9101a8e1c652ba.webp"
  },
  {
    "src": "https://broadcast.radioponiente.org:8072/",
    "title": "RADIO BERJA",
    "image": "covers/5.avif"
  },
  {
    "src": "https://cheetah.streemlion.com:2490/",
    "title": "Radio Mondo Italia - La Radio delle Comunit\u00e0 Italiane nel mondo",
    "image": "covers/1.avif"
  },
  {
    "src": "https://mpc1.mediacp.eu:8036/stream",
    "title": "Power FM Breda",
    "image": "covers/9.avif"
  },
  {
    "src": "https://liveradio.co.il:7000/",
    "title": "Radio-Kol Ramla",
    "image": "covers/1.avif"
  },
  {
    "src": "https://protvradiostream.com:8280/stream",
    "title": "Tentaci\u00f2n FM",
    "image": "covers/16.avif"
  },
  {
    "src": "https://radio.manelemania.ro:8020/shoutcast",
    "title": "Radio Manele Mania",
    "image": "covers/3.avif"
  },
  {
    "src": "https://stream.nobarriersradio.com:8000/nobarriers",
    "title": "No Barriers Radio Station",
    "image": "covers/12.avif"
  },
  {
    "src": "https://media.fmmradio.com:9065/radio.mp3",
    "title": "90's, 2000's and Today's Hits",
    "image": "covers/4.avif"
  },
  {
    "src": "https://pop.mamasfm.com:4500/",
    "title": "Pop Radyo Mamas FM",
    "image": "covers/4.avif"
  },
  {
    "src": "https://stm4.xcast.com.br:8938/2",
    "title": "FM Clube Teresina",
    "image": "covers/14.avif"
  },
  {
    "src": "https://c2.auracast.net:8022/radio.mp3",
    "title": "Best Spain",
    "image": "covers/7.avif"
  },
  {
    "src": "https://radio.streemlion.com:2760/",
    "title": "Radio ITALY",
    "image": "covers/2.avif"
  },
  {
    "src": "https://radio.trucksim.fm:8000/radio.mp3",
    "title": "TruckSimFM",
    "image": "covers/9.avif"
  }
]);

const [rock] = createSignal([
  {
    title: "Bayern Rock",
    src: "http://stream.antenne.de:80/rockantenne",
    image: "https://www.antenne.de/media/cache/3/version/18696/streamlogo_simulcast_live_aby_neu_2000x2000-v1.jpg/5e26f53136d9fcbd4f9101a8e1c652ba.webp"
  },
    {
        "src": "http://stream.antenne.de:80/live-rock",
        "title": "ROCK ANTENNE Live Rock",
        "image": "https://www.antenne.de/media/cache/3/version/18696/streamlogo_simulcast_live_aby_neu_2000x2000-v1.jpg/5e26f53136d9fcbd4f9101a8e1c652ba.webp"
    },
    {
      "src": "http://stream.antenne.de:80/80er-rock",
      "title": "ROCK ANTENNE 80er Rock",
      "image": "https://www.antenne.de/media/cache/3/version/18696/streamlogo_simulcast_live_aby_neu_2000x2000-v1.jpg/5e26f53136d9fcbd4f9101a8e1c652ba.webp"
  },
    {
        "src": "http://185.157.233.163:8014/mainstream",
        "title": "Barnet Community Radio",
        "image": "covers/1.avif"
    },
    {
        "src": "https://cast.magicstreams.gr:9037/stream",
        "title": "Sound of Pluto",
        "image": "covers/11.avif"
    },
    {
        "src": "https://stm14.xcast.com.br:10632/",
        "title": "COSTAO FM",
        "image": "covers/10.avif"
    },
    {
        "src": "https://secordradio.com:8000/radio.mp3",
        "title": "Secord Lake Radio",
        "image": "covers/10.avif"
    },
    {
        "src": "https://my-radio.live:10022/stream",
        "title": "BalkanRock",
        "image": "covers/10.avif"
    },
    {
        "src": "https://media.fmmradio.com:9095/radio.mp3",
        "title": "Pure Rock Hits Radio",
        "image": "covers/1.avif"
    },
    {
        "src": "https://stream.serviciospararadios.es:8060/rockstar.mp3",
        "title": "RockStar (Espa\u00f1a)",
        "image": "covers/14.avif"
    },
    {
        "src": "https://server5.mediasector.es:8050/rocksatelite.mp3",
        "title": "rockSateliteONE HQ",
        "image": "covers/8.avif"
    },
    {
        "src": "https://maggie.torontocast.com:8036/stream",
        "title": "VIBRAZIONI ROCK RADIO",
        "image": "covers/12.avif"
    },
    {
        "src": "https://radio.limnosfm100.gr:9998/limnosfm100",
        "title": "LimnosFm",
        "image": "covers/15.avif"
    },
    {
        "src": "https://streaming.radioestacion4.com:8082/stream",
        "title": "Kocodrilo radio",
        "image": "covers/12.avif"
    },
    {
        "src": "https://rocklive.radionoise.ro:9130/stream",
        "title": "Radio Noise Rock",
        "image": "covers/8.avif"
    },
    {
        "src": "https://a1.asurahosting.com:8930/radio.mp3",
        "title": "Nu Rock Radio",
        "image": "covers/17.avif"
    },
    {
        "src": "https://streamconex.com:8106/stream",
        "title": "Estacion del Valle",
        "image": "covers/1.avif"
    }
]);

const [electronic] = createSignal([
  {
    "src": "http://stream.antenne.de:80/chillout",
    "title": "ANTENNE BAYERN Chillout (Germany)",
    "image": "https://www.antenne.de/media/cache/3/version/18696/streamlogo_simulcast_live_aby_neu_2000x2000-v1.jpg/5e26f53136d9fcbd4f9101a8e1c652ba.webp"
},
  {
    "src": "https://stream.soundstorm-radio.com:8000/radio.mp3",
    "title": "Soundstorm Radio",
    "image": "covers/15.avif"
},
{
    "src": "https://s1.slotex.pl:7494/",
    "title": "Radio MRS",
    "image": "covers/11.avif"
},
{
    "src": "http://amoris.sknt.ru:80/trance",
    "title": "Anima Amoris [Trance]",
    "image": "covers/8.avif"
},
{
    "src": "https://cast4.magicstreams.gr:10159/stream",
    "title": "Music Galaxy Web Radio",
    "image": "covers/5.avif"
},
{
    "src": "https://streams.radiomast.io:443/tune1",
    "title": "Tune1 - All Digital",
    "image": "covers/7.avif"
},
{
    "src": "https://shoutcast.protonradio.com:7000/stream",
    "title": "Proton Radio Live",
    "image": "covers/5.avif"
},
{
    "src": "http://80.85.84.114:8024/stream",
    "title": "Dance UK Radio danceradiouk aac+",
    "image": "covers/13.avif"
},
{
    "src": "https://s3.slotex.pl:7354/",
    "title": "DiscoParty.pl - Club",
    "image": "covers/4.avif"
},
{
    "src": "https://live.m40radio.fr:8520/mixxone-128.mp3",
    "title": "MIXX?ONE",
    "image": "covers/16.avif"
},
{
    "src": "https://live.radiovibefm.eu:8052/stream",
    "title": "Vibe FM Romania",
    "image": "covers/9.avif"
},
{
    "src": "https://sc1.radioheaven.pl:8000/",
    "title": "Radio Heaven",
    "image": "covers/2.avif"
},
{
    "src": "https://streams.radiomast.io:443/0cef93cd-5974-43b1-868e-c739e81f4f2b",
    "title": "HAPPY HARDCORE",
    "image": "covers/6.avif"
},
{
    "src": "https://stream.electroradio.fm:80/192k",
    "title": "electroradio.fm",
    "image": "covers/4.avif"
}
]);



const [misc] = createSignal([
  {
    "src": "https://cp1.sednastream.com:8014/stream",
    "title": "ClubFM Albania",
    "image": "covers/default.avif"
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
  /*
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
  */
  // Update progress bar
  const updateProgress = () => {
    const progress = (audio.currentTime / audio.duration) * 100;
    setProgress(progress);
  };
  /*
  async function genMusic() {
    const music = await invoke('scan_music');
    setLocalMusic(JSON.parse(music));
  }
    */
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
          setElectronicShow(true);
        }}>
          <h2>Electronic</h2>
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