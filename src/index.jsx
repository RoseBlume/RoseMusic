/* @refresh reload */
import { render } from "solid-js/web";
import { Router, Route } from "@solidjs/router";

import { Home, Splashscreen } from "./pages";
import { scanMusic, finishSearch } from "./lib/funcs";
import "./styles/App.css"
import { listen } from "@tauri-apps/api/event";
listen('finished-searching', (event) => {
        finishSearch(event.payload.obj);
});
scanMusic();
render(
  () => (
    <Router>
        <Route path="/" component={Home} />
        <Route path="/splashscreen" component={Splashscreen} />
    </Router>
  ),
  document.getElementById("root")
);
