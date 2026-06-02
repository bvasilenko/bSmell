mod common;

use bsmell::EvidenceState;
use bsuite_core::ExitCode;
use common::{assert_public_name_contract, assert_rejects};
use proptest::prelude::*;
use std::str::FromStr;

proptest! {
    #[test]
    fn evidence_state_round_trip(index in 0usize..EvidenceState::ALL.len()) {
        let state = EvidenceState::ALL[index];
        let parsed = EvidenceState::from_str(&state.to_string()).expect("state must parse");
        prop_assert_eq!(state, parsed);
    }
}

#[test]
fn evidence_state_names_cover_exact_closed_set() {
    assert_eq!(3, EvidenceState::ALL.len());
    assert_public_name_contract(&EvidenceState::ALL);
}

#[test]
fn evidence_state_exit_codes_match_contract() {
    let cases = [
        (EvidenceState::Clean, ExitCode::Success, 0),
        (EvidenceState::SmellDetected, ExitCode::Finding, 1),
        (EvidenceState::Malformed, ExitCode::InternalError, 2),
    ];

    for (state, expected_exit_code, expected_raw_code) in cases {
        let exit_code: ExitCode = state.into();

        assert_eq!(expected_exit_code, exit_code);
        assert_eq!(expected_raw_code, exit_code.as_i32());
    }
}

#[test]
fn evidence_state_rejects_names_outside_closed_set() {
    assert_rejects::<EvidenceState>(&[
        "finding",
        "",
        "clean ",
        " clean",
        "Clean",
        "smell_detected",
        "internal-error",
    ]);
}
