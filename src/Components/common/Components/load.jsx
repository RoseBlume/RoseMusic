import { createSignal, onCleanup } from "solid-js";
import { loading } from "../signals";
export function LoadingAnimation() {
  const [dots, setDots] = createSignal("");

  // Cycle through "", ".", "..", "..."
  let count = 0;
  const interval = setInterval(() => {
    count = (count + 1) % 4; // 0,1,2,3
    setDots(".".repeat(count));
  }, 500);

  onCleanup(() => clearInterval(interval));

  return (
    <Show when={loading()}>
      <h2>Loading Tracks From Disk{dots()}</h2>
    </Show>
  );
}
