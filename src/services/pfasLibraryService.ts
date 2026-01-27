export type BarrierId =
  | "NONE"
  | "BIO_WAX"
  | "PLA_COATING"
  | "ALGINATE"
  | "STARCH_DERIVATIVE";

export interface BarrierCoating {
  id: BarrierId;
  name: string;
  description: string;
  pfasFree: boolean;
  compostable: boolean;
  compostStandards: string[];
  foodContactApprovals: string[];
  typicalUseCases: string[];
}

export const PFAS_FREE_BARRIER_LIBRARY: BarrierCoating[] = [
  {
    id: "NONE",
    name: "Uncoated Bagasse",
    description:
      "Bare molded sugarcane fiber tray with inherent grease and moisture resistance.",
    pfasFree: true,
    compostable: true,
    compostStandards: ["EN13432", "ASTM D6400"],
    foodContactApprovals: ["FDA", "EU food-contact"],
    typicalUseCases: ["meat trays", "produce trays", "ready-meal bases"]
  },
  {
    id: "BIO_WAX",
    name: "Bio-based Wax Coating",
    description:
      "Thin bio-wax layer to enhance water and oil barrier while retaining compostability.",
    pfasFree: true,
    compostable: true,
    compostStandards: ["EN13432", "ASTM D6400"],
    foodContactApprovals: ["FDA", "EU food-contact"],
    typicalUseCases: ["saucy dishes", "oily ready meals"]
  },
  {
    id: "PLA_COATING",
    name: "PLA-based Compostable Coating",
    description:
      "Compostable PLA film or dispersion coating that improves liquid hold-up for high-moisture foods.",
    pfasFree: true,
    compostable: true,
    compostStandards: ["EN13432", "ASTM D6400"],
    foodContactApprovals: ["FDA", "EU food-contact"],
    typicalUseCases: ["high-moisture entrees", "cold chain applications"]
  },
  {
    id: "ALGINATE",
    name: "Alginate Barrier",
    description:
      "Seaweed-derived barrier system used to boost grease resistance without fluorochemicals.",
    pfasFree: true,
    compostable: true,
    compostStandards: ["EN13432"],
    foodContactApprovals: ["EU food-contact"],
    typicalUseCases: ["greasy finger foods", "snack trays"]
  },
  {
    id: "STARCH_DERIVATIVE",
    name: "Starch-Derived Barrier",
    description:
      "Modified starch barrier system that maintains fiber compostability while adding moisture control.",
    pfasFree: true,
    compostable: true,
    compostStandards: ["EN13432"],
    foodContactApprovals: ["FDA"],
    typicalUseCases: ["produce", "short-life retail packaging"]
  }
];

export function listPfasFreeCompostableBarriers(): BarrierCoating[] {
  return PFAS_FREE_BARRIER_LIBRARY.filter(
    (b) => b.pfasFree && b.compostable
  );
}
