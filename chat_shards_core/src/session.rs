use crate::KerMetrics;

#[derive(Clone, Debug)]
pub struct ChatSessionParticleV1 {
    pub session_id_hex: String,
    pub session_version: String,
    pub created_at_iso8601: String,
    pub closed_at_iso8601: Option<String>,
    pub user_primary_bostrom_addr: String,
    pub user_alt_bostrom_addr: Option<String>,
    pub origin_region: String,
    pub client_surface: String,
    pub eco_context_corridor_id: String,
    pub policy_corridor_id: String,
    pub max_risk_of_harm_allowed: f64,
    pub neuromorphic_role: String,
    pub actuation_plane: String, // must be "none" for this project

    pub session_ker: KerMetrics,
    pub session_cybostate_factor: f64,
    pub session_topic_cluster: String,

    pub pilot_gate_context_id: String,
    pub primary_corridor_plane: String,
    pub response_shard_ids: Vec<String>,

    pub hex_stamp: String,
    pub session_did: String,
    pub created_by_agent_id: String,
    pub review_required_flag: bool,
    pub review_outcome: String,
    pub log_chain_anchor: String,
}

impl ChatSessionParticleV1 {
    pub fn is_well_formed(&self) -> bool {
        fn in_01(x: f64) -> bool {
            x.is_finite() && x >= 0.0 && x <= 1.0
        }

        self.session_ker.is_well_formed()
            && in_01(self.max_risk_of_harm_allowed)
            && !self.session_id_hex.is_empty()
            && self.actuation_plane == "none"
    }
}
