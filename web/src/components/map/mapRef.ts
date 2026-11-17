import type L from "leaflet";

/** Acceso compartido al mapa para la barra de herramientas y los comandos. */
export const mapRef: { map: L.Map | null; bounds: L.LatLngBounds | null } = { map: null, bounds: null };

export function fitWhole(): void {
  if (mapRef.map && mapRef.bounds) mapRef.map.fitBounds(mapRef.bounds, { animate: false });
}

export function zoomBy(delta: number): void {
  mapRef.map?.setZoom(mapRef.map.getZoom() + delta);
}
