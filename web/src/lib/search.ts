import { locale } from "../locales";

/** Texto normalizado para buscar: minúsculas y sin acentos («cámara» → «camara»). */
export function fold(s: string): string {
  return s
    .normalize("NFD")
    .replace(/\p{Diacritic}/gu, "")
    .toLocaleLowerCase(locale);
}

/** true si todas las palabras de la consulta aparecen en alguno de los textos. */
export function matches(query: string, ...fields: string[]): boolean {
  const words = fold(query).split(/\s+/).filter(Boolean);
  if (words.length === 0) return true;
  const hay = fold(fields.join(" "));
  return words.every((w) => hay.includes(w));
}
