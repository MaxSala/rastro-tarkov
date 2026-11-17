// Catálogo del mercado (precios y munición). Fuente: src/market/mod.rs (Catalog, Item, Offer, Ammo).
// Llega por HTTP (/api/market) porque pesa demasiado para el WebSocket; se valida con zod.
import { z } from "zod";
import { create } from "zustand";
import { api } from "./api";

const num = z.number().finite();
const name = z.string().max(160);

const offer = z.object({
  vendor: z.string().max(80),
  vendorName: name,
  priceRub: z.number().int().nonnegative(),
  original: z.tuple([num, z.string().max(8)]).optional(),
  level: z.number().int().min(1).max(10).optional(),
});

const ammo = z.object({
  caliber: name,
  damage: num,
  projectiles: z.number().int().min(1),
  penetration: num,
  armorDamage: num,
  fragmentation: num,
  speed: num,
  tracer: z.boolean(),
});

const item = z.object({
  id: z.string().regex(/^[0-9a-f]{24}$/),
  name,
  short: name,
  slug: z.string().max(80),
  width: z.number().int().min(1).max(20),
  height: z.number().int().min(1).max(20),
  types: z.array(z.string().max(20)).max(12),
  fleaLow: z.number().int().nullable(),
  fleaAvg: z.number().int().nullable(),
  change48h: num.nullable(),
  sell: z.array(offer).max(20),
  buy: z.array(offer).max(40),
  bestSell: offer.nullable(),
  perSlot: z.number().int().nullable(),
  cheapestBuy: offer.nullable(),
  ammo: ammo.optional(),
});

const catalogSchema = z.object({
  mode: z.string().max(10),
  lang: z.string().max(5),
  fetchedAt: z.number().int(),
  items: z.array(item).max(20000),
});

export type Offer = z.infer<typeof offer>;
export type Item = z.infer<typeof item>;
export type Catalog = z.infer<typeof catalogSchema>;

interface CatalogState {
  catalog: Catalog | null;
  /** fetchedAt del catálogo cargado (para saber cuándo hay uno nuevo en el backend). */
  loadedAt: number | null;
  loading: boolean;
  error: string | null;
  load: (fetchedAt: number) => Promise<void>;
}

export const useCatalog = create<CatalogState>((set, get) => ({
  catalog: null,
  loadedAt: null,
  loading: false,
  error: null,
  load: async (fetchedAt) => {
    if (get().loading || get().loadedAt === fetchedAt) return;
    set({ loading: true, error: null });
    try {
      const raw: unknown = await api.json("/api/market");
      const parsed = catalogSchema.safeParse(raw);
      if (!parsed.success) throw new Error(parsed.error.issues[0]?.message ?? "invalid catalog");
      set({ catalog: parsed.data, loadedAt: parsed.data.fetchedAt, loading: false });
    } catch (e) {
      set({ loading: false, error: e instanceof Error ? e.message : String(e) });
    }
  },
}));

/** Categorías de la interfaz a partir de los tipos de tarkov.dev. */
export const CATEGORIES = {
  all: [],
  barter: ["barter"],
  keys: ["keys"],
  ammo: ["ammo", "ammoBox"],
  meds: ["meds", "injectors"],
  provisions: ["provisions"],
  gear: ["armor", "armorPlate", "backpack", "rig", "helmet", "headphones", "glasses", "wearable"],
  gun: ["gun"],
  mods: ["mods", "suppressor", "pistolGrip"],
  container: ["container"],
} as const;
export type Category = keyof typeof CATEGORIES;

export function inCategory(it: Item, c: Category): boolean {
  const types: readonly string[] = CATEGORIES[c];
  return types.length === 0 || it.types.some((ty) => types.includes(ty));
}

const indexes = new WeakMap<Catalog, Map<string, Item>>();
/** Búsqueda por id en O(1); el índice se crea una vez por catálogo. */
export function itemById(catalog: Catalog | null, id: string | undefined): Item | undefined {
  if (!catalog || !id) return undefined;
  let idx = indexes.get(catalog);
  if (!idx) {
    idx = new Map(catalog.items.map((i) => [i.id, i]));
    indexes.set(catalog, idx);
  }
  return idx.get(id);
}
