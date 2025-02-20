import { invoke } from '@tauri-apps/api/core';
import "../App.css";
export function Settings(Props) {
  return (
    <div>
        <ul>
            <li onClick={() => {Props.MenuClick();}}>
                <h2>Home</h2>
            </li>
            <li onClick={() => {
                Props.Scan();
                Props.MenuClick();
                }}>
                <h2>Scan Music Folder</h2>
            </li>
        </ul>
    </div>
  );
}