import { createSignal, Show } from "solid-js";
import "./styles/App.css";
import { invoke } from "@tauri-apps/api/core";
import { Radio } from "./Components/features/radio/Radio";
import { listen } from '@tauri-apps/api/event';
import { ExtraSettings } from "./Components/settings/extras";
import { LoadingAnimation } from "./Components/common/Components/load";
import { Artists } from "./Components/features/local/lists/artist";
import { Genres } from "./Components/features/local/lists/genre";
import { Tracks } from "./Components/features/local/lists/tracks";
import { Settings } from "./Components/settings/settings";
import {  
  tracks,
  setTracksShow, 
  setPlaylist,
  home,
  setArtist,
  setGenre,
  settings,
  setSettings,
  setRadio,
  debugMode
} from "./Components/common/signals";
import { 
  clear,
  finishSearch,
  scanMusic
} from "./Components/common/funcs";
import { Player } from "./Components/features/local/player";
function App() {
  const [settingsMenu] = createSignal(false);
  scanMusic();

  

  listen('finished-searching', (event) => {
    finishSearch(event.payload.obj);
  });



  return (
    <main>
      <Settings />

      <Show when={debugMode()}>
        <div class="debug-section">
          <h2>Debug Info</h2>
          <p>{stringTracks()}</p>
        </div>
      </Show>
      
      <ExtraSettings />
      
      <Radio/>
      <Player />
      <Tracks />
      <Genres />
      <Artists />
    </main>
  );
}

export default App;