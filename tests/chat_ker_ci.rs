use chat_shards_core::{
    check_response_against_session, ChatKerCheck,
    ChatResponseShardV1, ChatSessionParticleV1, KerMetrics,
};

fn load_example_session() -> ChatSessionParticleV1 {
    ChatSessionParticleV1 {
        session_id_hex: "a3f9c1e4...ddeeff".to_string(),
        session_version: "1.0.0".to_string(),
        created_at_iso8601: "2026-01-20T01:55:00Z".to_string(),
        closed_at_iso8601: Some("2026-01-20T02:10:00Z".to_string()),
        user_primary_bostrom_addr: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
        user_alt_bostrom_addr: None,
        origin_region: "Phoenix-AZ-US".to_string(),
        client_surface: "web".to_string(),
        eco_context_corridor_id: "eco.corridor.cyboquatic.phx.v2".to_string(),
        policy_corridor_id: "policy.corridor.phx.waterreuse.v1".to_string(),
        max_risk_of_harm_allowed: 0.20,
        neuromorphic_role: "advisory-only".to_string(),
        actuation_plane: "none".to_string(),
        session_ker: KerMetrics { k: 0.94, e: 0.90, r: 0.13 },
        session_cybostate_factor: 0.12,
        session_topic_cluster: "cyboquatic-SAT-PilotGate".to_string(),
        pilot_gate_context_id: "PilotGate-Phx-District-01".to_string(),
        primary_corridor_plane: "grammar-spec".to_string(),
        response_shard_ids: vec!["resp-0001".to_string(), "resp-0002".to_string()],
        hex_stamp: "0x8f32ab".to_string(),
        session_did: "did:cyber:chat-session:a3f9c1e4".to_string(),
        created_by_agent_id: "agent.cybercore.brain.v3".to_string(),
        review_required_flag: true,
        review_outcome: "pending".to_string(),
        log_chain_anchor: "tx_0x9c7e21fab4".to_string(),
    }
}

fn load_example_response_ok() -> ChatResponseShardV1 {
    ChatResponseShardV1 {
        response_shard_id: "resp-0001".to_string(),
        session_id_hex: "a3f9c1e4...ddeeff".to_string(),
        turn_index: 0,
        created_at_iso8601: "2026-01-20T01:56:00Z".to_string(),
        primary_topic: "topic-prioritization".to_string(),
        secondary_topic: Some("ecosafety-KER".to_string()),
        corridor_plane: "grammar-spec".to_string(),
        target_module_family: "EcoNet.CyboquaticCorridors".to_string(),
        rust_crate_hint: "cyboquatic_sat_corridors".to_string(),
        aln_contract_hint: "ecosafety.ker.v1".to_string(),
        ker: KerMetrics { k: 0.94, e: 0.90, r: 0.13 },
        survival_band_effect: "tighten".to_string(),
        ker_score_vector: "K=0.94;E=0.90;R=0.13".to_string(),
        affects_hydraulic_corridors_flag: true,
        affects_material_toxicity_flag: false,
        affects_policy_corridors_flag: true,
        requires_new_corridor_proof_flag: true,
        pilot_gate_step: "HydraulicStructural".to_string(),
        hex_stamp: "0x4a3b2c1d9e8f7g6h".to_string(),
        authored_by_agent_id: "agent.cybercore.brain.v3".to_string(),
        bostrom_signer_addr: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
        did_document_ref: "did:cyber:chat-response:resp-0001".to_string(),
        log_chain_anchor: "tx_0x73abff91c2".to_string(),
        contains_sensitive_biomech_flag: false,
        contains_disallowed_cryptoprim_flag: false,
        requires_manual_redaction_flag: false,
        content_pointer_type: "none".to_string(),
        content_pointer_id: None,
    }
}

fn load_example_response_bad() -> ChatResponseShardV1 {
    let mut r = load_example_response_ok();
    r.ker.r = 0.35; // violates session max 0.20
    r
}

#[test]
fn chat_response_must_not_exceed_session_risk_limit() {
    let session = load_example_session();
    let r_ok = load_example_response_ok();
    let r_bad = load_example_response_bad();

    assert_eq!(
        ChatKerCheck::Ok,
        check_response_against_session(&session, &r_ok),
        "expected OK for response within session risk limit"
    );

    assert_eq!(
        ChatKerCheck::RiskExceedsSessionLimit,
        check_response_against_session(&session, &r_bad),
        "CI must fail when response risk_of_harm_r exceeds session max_risk_of_harm_allowed"
    );
}
