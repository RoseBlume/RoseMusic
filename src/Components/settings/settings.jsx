import { Show, createSignal } from "solid-js";
import { setArtist, setGenre, setPlaylist, setTracksShow, home, setSettings, setRadio, settingsMenu, tracks } from "../common/signals";
import { clear } from "../common/funcs";
import "../../styles/App.css";
export function Settings() {
  return (
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
                  clear();
                  setTracksShow(true);
                  setPlaylist(tracks());
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
              <Show when={settingsMenu()}>
                <li onClick={() => {
                  clear();
                  setSettings(true);
                }}>
                  <h2>Settings</h2>
                </li>
              </Show>
            </ul>
          </Show>
  );
}