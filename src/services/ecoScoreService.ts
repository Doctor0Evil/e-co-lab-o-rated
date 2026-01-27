import {
  TrayConfiguration,
  LCAContext,
  EcoScore,
  computeEcoScore
} from "../domain/lca";
import { computeDisplacementMetrics } from "./displacementService";

export interface EcoScoreInputs {
  config: TrayConfiguration;
  ctx: LCAContext;
  landfillFractionBaseline: number;
  compostFractionScenario: number;
  gridKWhPerTray: number;
}

export interface EcoScoreReport {
  config: TrayConfiguration;
  ecoScore: EcoScore;
}

export function calculateEcoScore(
  inputs: EcoScoreInputs
): EcoScoreReport {
  const { config, ctx, landfillFractionBaseline, compostFractionScenario } =
    inputs;

  const { displacement, methane } = computeDisplacementMetrics({
    config,
    ctx,
    landfillFractionBaseline,
    compostFractionScenario
  });

  const ecoScore = computeEcoScore(
    displacement,
    methane,
    inputs.gridKWhPerTray,
    ctx,
    config.materialId,
    config.integratedSystem
  );

  return { config, ecoScore };
}
