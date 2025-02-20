import { createSignal, createEffect } from "solid-js";

export const ProgressBar = (props) => {
  const [currentTime, setCurrentTime] = createSignal(props.currentTime());
  const duration = () => props.currentDuration();

  // Sync state with props
  createEffect(() => {
    setCurrentTime(props.currentTime());
  });

  const handleInput = (event) => {
    setCurrentTime(Number(event.target.value));
  };

  const handleChange = (event) => {
    props.onSeek(Number(event.target.value)); // Call the parent function to update playback time
  };

  return (
    <input
      type="range"
      min="0"
      max={duration()}
      value={currentTime()}
      onInput={handleInput}
      onChange={handleChange}
      style={{ width: "100%" }}
    />
  );
};