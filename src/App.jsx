import { createSignal, Show } from "solid-js";
import "./styles/App.css";
import { Radio } from "./Components/features/radio/Radio";
import { listen } from '@tauri-apps/api/event';
import { ExtraSettings } from "./Components/settings/extras";
import { Artists } from "./Components/features/local/lists/artist";
import { Genres } from "./Components/features/local/lists/genre";
import { Tracks } from "./Components/features/local/lists/tracks";
import { Menu } from "./Components/common/Components/menu";
import { DebugSettings } from "./Components/settings/debug";
import {  
  stringTracks,
  debugMode
} from "./Components/common/signals";
import { 
  finishSearch,
  scanMusic
} from "./Components/common/funcs";
import { Player } from "./Components/features/local/player";
function App() {
  scanMusic();

  

  listen('finished-searching', (event) => {
    finishSearch(event.payload.obj);
  });



  return (
    <main>
      <Menu />
      <Radio/>
      <Player />
      <Tracks />
      <Genres />
      <Artists />
      <ExtraSettings />
      <DebugSettings />
    </main>
  );
}

export default App;