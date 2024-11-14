// src/components/RadioList.jsx
import { Index } from "solid-js";

const RadioList = ({ stations, onClick }) => (
  <ul>
    <Index each={stations}>{(station, index) => (
      <li class="RadioList" onClick={() => onClick(station)}>
        <h2>{station().title}</h2>
      </li>
    )}</Index>
  </ul>
);

export default RadioList;