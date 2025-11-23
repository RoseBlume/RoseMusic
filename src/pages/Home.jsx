
import { listen } from '@tauri-apps/api/event';
import { Radio } from "../features/radio/Radio";
import { Tracks, Genres, Artists } from "../features/local/lists";
import { Menu, Player } from "../ui";
import { ExtraSettings, DebugSettings } from "../settings";

// import { 
//   finishSearch
// } from "./lib/funcs";
// import { Player } from "./Components/features/player";
export function Home() {

  // listen('finished-searching', (event) => {
  //   finishSearch(event.payload.obj);
  // });



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
