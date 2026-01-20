use crate::KerMetrics;

#[derive(Clone, Debug)]
pub struct ChatResponseShardV1 {
    pub response_shard_id: String,
    pub session_id_hex: String,
    pub turn_index: u32,
    pub created_at_iso8601: String,

    pub primary_topic: String,
    pub secondary_topic: Option<String>,
    pub corridor_plane: String,
    pub target_module_family: String,
    pub rust_crate_hint: String,
    pub aln_contract_hint: String,

    pub ker: KerMetrics,
    pub survival_band_effect: String, // "tighten" | "widen" | "no-change"
    pub ker_score_vector: String,

    pub affects_hydraulic_corridors_flag: bool,
    pub affects_material_toxicity_flag: bool,
    pub affects_policy_corridors_flag: bool,
    pub requires_new_corridor_proof_flag: bool,
    pub pilot_gate_step: String,

    pub hex_stamp: String,
    pub authored_by_agent_id: String,
    pub bostrom_signer_addr: String,
    pub did_document_ref: String,
    pub log_chain_anchor: String,

    pub contains_sensitive_biomech_flag: bool,
    pub contains_disallowed_cryptoprim_flag: bool,
    pub requires_manual_redaction_flag: bool,
    pub content_pointer_type: String,        // "none" | "ipfs" | "internal-blob-id"
    pub content_pointer_id: Option<String>,  // never holds raw text
}

impl ChatResponseShardV1 {
    pub fn is_well_formed(&self) -> bool {
        self.ker.is_well_formed()
            && !self.response_shard_id.is_empty()
            && !self.session_id_hex.is_empty()
            && self.content_pointer_type != "inline"
    }
}
