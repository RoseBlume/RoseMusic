import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import {
    tracks, 
    setTracks, 
    setTracksShow,
    setCurrentGenre, 
    setCurrentTime, 
    setCurrentArtist,
    setCurrentTitle,
    setCover,
    setPlaying,
    loading,
    setLoading,
    setHome,
    setStringTracks,
    setStopInterval,
    currentLocation,
    setArtist,
    setGenre,
    setRadio,
    setPlayer,
    setDebugMode,
    setSettings,
    currentTime,
    currentDuration,
    setCurrentDuration,
    setCurrentLocation,
    playing

    
} from "./signals";

let timer;

export async function scanMusic() {
    await invoke("scan_music_files");
    // await invoke("save_music_data", {data: tracks()});
  };

async function main_loop() {
    if (timer) {
        clearInterval(timer);
    }
    timer = setInterval(() => {
        get_duration();
        get_progress();
        let current_time = currentTime();
        let current_duration = currentDuration();
        if (current_time >= current_duration) {
            next_track();
        }
        }, 100);
}


export async function prev_track() {
    await invoke("stop");
    const trackList = tracks();
    if (!trackList || trackList.length === 0) return;
    const currentIndex = trackList.findIndex(t => t.location === currentLocation());
    const nextIndex = currentIndex === -1 || currentIndex === trackList.length - 1 ? 0 : currentIndex + 1;
    const prevIndex = currentIndex <= 0 ? trackList.length - 1 : currentIndex - 1;
    const prev = trackList[prevIndex];
    const next = trackList[nextIndex];
    setCurrentLocation(prev.location);
    setCurrentArtist(prev.artist);
    setCurrentTitle(prev.title);
    setCurrentGenre(prev.genre);
    setCurrentTime(prev.duration);
    setCover(prev.cover);
    start_track();
}
export async function start_track() {
    await invoke("play_song", {location: currentLocation()})
    setPlaying(true);
    setCurrentTime(0);
    setStopInterval(false);
    main_loop();
}



export async function get_duration() {
    const duration = await invoke("get_song_duration");
}
export async function get_progress() {
    const progress = await invoke("get_song_progress");
    setCurrentTime(progress);
}

export async function stop_track() {
    await invoke("stop")
    setStopInterval(true);
    setPlaying(false);
  }
stop_track();
export async function toggle_play() {
    setPlaying(!playing());
    await invoke("toggle_playing");
}


export async function next_track() {
    await invoke("stop");
    const trackList = tracks();
    if (!trackList || trackList.length === 0) return;
    const currentIndex = trackList.findIndex(t => t.location === currentLocation());
    const nextIndex = currentIndex === -1 || currentIndex === trackList.length - 1 ? 0 : currentIndex + 1;
    const next = trackList[nextIndex];
    setCurrentLocation(next.location);
    setCurrentArtist(next.artist);
    setCurrentTitle(next.title);
    setCurrentGenre(next.genre);
    setCurrentDuration(next.duration);
    setCurrentTime(next.duration)
    setCover(next.cover);
    start_track();
  }

export const handleInput = (event) => {
    onSeek(Number(event.target.value));
};

export const handleChange = (event) => {
    onSeek(Number(event.target.value)); // Call the parent function to update playback time
};


export const onSeek = async (time) => {
    await invoke("seek_to", {timeMs: time});
  };

export const handleMenuClick = () => {
    setHome(true);
    setTracksShow(false);
    setGenre(false);
    setArtist(false);
    setRadio(false);
    setPlayer(false);
    setDebugMode(false);
    setSettings(false);
    invoke("log", {message: "Menu Clicked"});
    stop_track();
};
export async function clear() {
    setSettings(false);
    setHome(false);
    setTracksShow(false);
    setGenre(false);
    setArtist(false);
    setRadio(false);

}

export const togglePlay = () => {
    setPlaying(!playing());
    toggle_play();
};

export async function finishSearch(object) {
    console.log(`downloading`, object);

    // Update signals
    setTracks(object);

    // Create string directly from the new object
    const jsonString = JSON.stringify(object, null, 2);
    setStringTracks(jsonString);
    setLoading(false);
    // Log immediately using the fresh value
    console.log(jsonString);
}