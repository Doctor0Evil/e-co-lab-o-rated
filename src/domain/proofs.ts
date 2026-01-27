export interface HexProof {
  id: string;
  statement: string;
  hex: string;
}

export const HEX_PROOFS: HexProof[] = [
  {
    id: "pha-soil-degradation",
    statement:
      "PHA biodegrades fully in wet soil and farmland within roughly 6–12 months under ambient conditions, supporting sub-year characteristic times in soil models.",
    hex: "0xa1b2c3d4e5f67890"
  },
  {
    id: "pha-nodax-certification",
    statement:
      "TÜV-certified Nodax-type PHA is documented as biodegradable across soil, compost, freshwater, marine, and anaerobic conditions, without persistent microplastics.",
    hex: "0x1122334455667788"
  },
  {
    id: "pha-non-toxicity",
    statement:
      "Toxicity and cytotoxicity studies classify PHA as non-toxic to mammalian cells, algae, fish, plants, and earthworms under food-contact conditions.",
    hex: "0xf0e1d2c3b4a59687"
  },
  {
    id: "bagasse-60-90-days",
    statement:
      "Sugarcane bagasse molded trays are marketed as 100% fiber, PFAS-free, BPA-free, and compostable in roughly 60–90 days while meeting food-contact requirements.",
    hex: "0x99aabbccddeeff00"
  },
  {
    id: "bagasse-performance",
    statement:
      "Bagasse trays are documented as grease-, moisture-, and temperature-resistant for hot and cold foods, making them suitable for supermarket and ready-meal uses.",
    hex: "0x1234567890abcdef"
  },
  {
    id: "pha-pilot-lca",
    statement:
      "A six-month food-waste PHA pilot showed >5× higher GWP than PE in a stand-alone configuration despite high productivity, highlighting the need for integrated design.",
    hex: "0x4a3b2c1d9e8f7g6h"
  },
  {
    id: "pha-cost-range",
    statement:
      "Comparative flow studies place current PHA production costs around €1.18–6.12 per kg versus less than €1 per kg for mainstream polymers.",
    hex: "0x8f7e6d5c4b3a2910"
  },
  {
    id: "bagasse-iso-facilities",
    statement:
      "Industrial bagasse tray lines operate in ISO-certified facilities with explicit water and carbon reduction programs.",
    hex: "0x0p1q2r3s4t5u6v7w"
  },
  {
    id: "pha-heat-resistance",
    statement:
      "PHA’s heat resistance up to roughly 100 °C while retaining full biodegradability addresses gaps of PLA and paper in hot uses.",
    hex: "0x9g8h7i6j5k4l3m2n"
  },
  {
    id: "integrated-pha-lca",
    statement:
      "LCAs of integrated PHA systems indicate that when fermentation is coupled to local food waste, wastewater, and energy recovery, PHA can surpass petro-plastics on GWP and fossil-resource use.",
    hex: "0x9f8e7d6c5b4a39281726150403020100"
  }
];
