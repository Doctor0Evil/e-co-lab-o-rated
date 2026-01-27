import { TrayMaterial } from "../domain/materials";
import {
  Environment,
  halfLifeFromWindow,
  computeDecay,
  DecayResult
} from "../domain/decay";
import { PHOENIX_CLIMATE } from "../config/phoenix.config";

export interface LocalDecayResult extends DecayResult {
  environment: Environment;
  effectiveHalfLifeDays: number;
}

export function estimateLocalDecay(
  material: TrayMaterial,
  environment: Environment,
  days: number
): LocalDecayResult {
  const profile = material.biodegradation.find(
    (p) => p.environment === environment
  );
  if (!profile) {
    throw new Error(
      `No biodegradation profile for environment ${environment}`
    );
  }

  const baseHalfLife = halfLifeFromWindow(
    profile.minDays,
    profile.maxDays
  );

  let climateFactor = 1.0;
  if (PHOENIX_CLIMATE.averageSoilTempC >= 25) {
    climateFactor *= 0.9;
  }
  if (PHOENIX_CLIMATE.averageMoistureIndex < 0.4) {
    climateFactor *= 1.1;
  }

  const effectiveHalfLifeDays = baseHalfLife * climateFactor;
  const decay = computeDecay(
    { halfLifeDays: effectiveHalfLifeDays, environment },
    days
  );

  return {
    environment,
    effectiveHalfLifeDays,
    remainingFraction: decay.remainingFraction,
    degradedFraction: decay.degradedFraction
  };
}
