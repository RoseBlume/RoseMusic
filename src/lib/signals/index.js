import { createSignal } from "solid-js";
export const [percent, setPercent] = createSignal(0);
export const [loading, setLoading] = createSignal(true);
export const [tracks, setTracks] = createSignal();
export const [currentLocation, setCurrentLocation] = createSignal("C:\\Users\\James\\Music\\(I Can_t Get No) Satisfaction.mp3");
export const [currentArtist, setCurrentArtist] = createSignal("The Rolling Stones");
export const [currentTitle, setCurrentTitle] = createSignal("(I Can't Get No) Satisfaction");
export const [currentGenre, setCurrentGenre] = createSignal("Rock");
export const [cover, setCover] = createSignal("");
export const [currentDuration, setCurrentDuration] = createSignal(0);
export const [currentTime, setCurrentTime] = createSignal(0);
export const [playlist, setPlaylist] = createSignal();
export const [player, setPlayer] = createSignal(false);
export const [playing, setPlaying] = createSignal(false);

export const [stringTracks, setStringTracks] = createSignal("");

export const [tracksShow, setTracksShow] = createSignal(false);


export const [home, setHome] = createSignal(true);

export const [stopInterval, setStopInterval] = createSignal(false);

export const [genre, setGenre] = createSignal(false);
export const [artist, setArtist] = createSignal(false);
export const [radio, setRadio] = createSignal(false);
export const [settings, setSettings] = createSignal(false);

export const [debugMode, setDebugMode] = createSignal(false);


export const [settingsMenu] = createSignal(false);