// src/components/GenreList.jsx
import { Show } from "solid-js";

const GenreList = ({ genres, clear, setGenreShow, i18n }) => (
  <Show when={true}>
    {genres.map((genre) => (
      <li onClick={() => {
        clear();
        setGenreShow(genre.show);
      }}>
        <h2>{i18n.t(genre.name)}</h2>
      </li>
    ))}
  </Show>
);

export default GenreList;