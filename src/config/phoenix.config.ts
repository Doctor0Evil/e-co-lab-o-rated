import { LCAContext } from "../domain/lca";

export interface PhoenixClimatePreset {
  name: string;
  averageSoilTempC: number;
  averageMoistureIndex: number;
}

export const PHOENIX_CLIMATE: PhoenixClimatePreset = {
  name: "Phoenix, AZ (semi-arid)",
  averageSoilTempC: 25,
  averageMoistureIndex: 0.3
};

export const PHOENIX_LCA_CONTEXT: LCAContext = {
  gridEmissionFactorKgPerKWh: 0.4,
  landfillMethaneKgCO2ePerKgOrganic: 2.0,
  compostMethaneKgCO2ePerKgOrganic: 0.1,
  fossilPlasticGwpKgPerKg: 2.5
};
