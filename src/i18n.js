// src/i18n.js
import rosetta from "rosetta";

const i18n = new rosetta({
  en: {
    alternative: "Alternative",
    pop: "Pop",
    rock: "Rock",
    electronic: "Electronic",
    reggae: "Reggae",
    misc: "Misc",
  },
  es: {
    alternative: "Alternativa",
    pop: "Pop",
    rock: "Rock",
    electronic: "Electrónica",
    reggae: "Reggae",
    misc: "Varios",
  },
  de: {
    alternative: "Alternative",
    pop: "Pop",
    rock: "Rock",
    electronic: "Elektronisch",
    reggae: "Reggae",
    misc: "Verschiedenes",
  },
  fr: {
    alternative: "Alternative",
    pop: "Pop",
    rock: "Rock",
    electronic: "Électronique",
    reggae: "Reggae",
    misc: "Divers",
  }
});

try {
  const locale = navigator.language.slice(0, 2).toLowerCase();
  if (!["fr", "en", "es", "de"].includes(locale)) {
    throw new Error("Unsupported language");
  }
  i18n.locale(locale);
} catch {
  i18n.locale("en");
}

export default i18n;