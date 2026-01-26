// BioTrayEcoImpactRust_2026v1.rs
// Sovereign eco-impact calculator for biodegradable tray pathways
// Uses simple mass-balance & decomposition model; extendable to integral forms
// Hex-stamp: 0x9f8e7d6c5b4a39281726150403020100 (authored node: bostrom-eco-snowglobe-tray-research)

use std::f64::consts::E;

#[derive(Debug, Clone)]
struct TrayPathway {
    name: &'static str,
    mass_grams: f64,               // tray mass
    plastic_displaced_grams: f64,  // fossil-plastic equivalent displaced
    decomposition_days_soil: f64,  // half-life approximation in soil
    energy_kwh_per_kg: f64,        // production energy
    water_liters_per_kg: f64,      // production water
}

#[derive(Debug)]
struct EcoImpact {
    displacement_score: f64,       // fossil-plastic avoided (0-1 normalized)
    decomp_score: f64,             // decomposition completeness in 365 days
    energy_penalty: f64,           // relative to fossil baseline
    net_eco_score: f64,            // composite (higher = better)
}

impl TrayPathway {
    fn compute_impact(&self, days: f64) -> EcoImpact {
        // Exponential decay approximation: fraction remaining = e^(-t / tau)
        // tau = half-life / ln(2)
        let tau = self.decomposition_days_soil / (E.ln() / 2.0_f64.ln());
        let remaining_frac = E.powf(-days / tau);
        let decomp_frac = 1.0 - remaining_frac.clamp(0.0, 1.0);

        // Displacement: mass displaced / reference plastic mass
        let disp_score = (self.plastic_displaced_grams / 50.0).clamp(0.0, 1.0); // ref 50g plastic tray

        // Energy penalty: lower is better; normalize to 1.0 = fossil avg
        let energy_penalty = (self.energy_kwh_per_kg / 2.5).clamp(0.0, 2.0); // fossil ~2.5 kWh/kg

        let net = (disp_score * 0.5) + (decomp_frac * 0.4) - (energy_penalty * 0.1);

        EcoImpact {
            displacement_score: disp_score,
            decomp_score: decomp_frac,
            energy_penalty,
            net_eco_score: net.clamp(0.0, 1.0),
        }
    }
}

fn main() {
    let pathways = vec![
        TrayPathway {
            name: "PHA Microbial",
            mass_grams: 35.0,
            plastic_displaced_grams: 35.0,
            decomposition_days_soil: 180.0,   // approx half-life soil/marine
            energy_kwh_per_kg: 3.2,
            water_liters_per_kg: 15.0,
        },
        TrayPathway {
            name: "Bagasse Pulp",
            mass_grams: 45.0,
            plastic_displaced_grams: 45.0,
            decomposition_days_soil: 90.0,    // faster in compost/soil
            energy_kwh_per_kg: 1.8,
            water_liters_per_kg: 40.0,
        },
    ];

    let days_sim = 365.0; // one-year soil exposure

    for p in pathways {
        let impact = p.compute_impact(days_sim);
        println!("Pathway: {}", p.name);
        println!("  Net Eco-Score (0-1): {:.3}", impact.net_eco_score);
        println!("  Displacement: {:.3}", impact.displacement_score);
        println!("  Decomp Fraction (1yr): {:.3}", impact.decomp_score);
        println!("  Energy Penalty: {:.3}", impact.energy_penalty);
        println!("---");
    }
}
