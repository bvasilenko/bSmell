use bsmell::{EvidenceState, InvocationSurface, SmellCategory, routing_key};

struct PendingDetector;

impl PendingDetector {
    fn detect(&self) -> EvidenceState {
        unimplemented!("not yet implemented")
    }
}

#[test]
fn library_reexports_public_contract_types() {
    assert_eq!(15, SmellCategory::ALL.len());
    assert_eq!(3, EvidenceState::ALL.len());
    assert_eq!(5, InvocationSurface::ALL.len());
}

#[test]
fn routing_key_uses_bsmell_core_entry_point() {
    assert_eq!(bsuite_core::RoutingKey::bsmell(), routing_key());
}

#[test]
#[should_panic(expected = "not yet implemented")]
fn placeholder_detector_is_explicitly_pending() {
    let detector = PendingDetector;

    let _ = detector.detect();
}
