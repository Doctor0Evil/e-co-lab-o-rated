export interface LineTelemetrySnapshot {
  lineId: string;
  timestamp: string;
  energyKWh: number;
  waterLiters: number;
  materialKg: number;
  traysProduced: number;
}

export interface TelemetryShard {
  shardId: string;
  lineId: string;
  timestamp: string;
  payload: {
    energyKWhPerTray: number;
    waterLitersPerTray: number;
    materialKgPerTray: number;
  };
}
