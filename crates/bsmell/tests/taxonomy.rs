mod common;

use bsmell::SmellCategory;
use common::{assert_public_name_contract, assert_rejects};
use proptest::prelude::*;
use std::str::FromStr;

proptest! {
    #[test]
    fn category_round_trip(index in 0usize..SmellCategory::ALL.len()) {
        let category = SmellCategory::ALL[index];
        let parsed = SmellCategory::from_str(&category.to_string()).expect("category must parse");
        prop_assert_eq!(category, parsed);
    }
}

#[test]
fn category_names_cover_exact_closed_set() {
    assert_eq!(15, SmellCategory::ALL.len());
    assert_public_name_contract(&SmellCategory::ALL);
}

#[test]
fn category_rejects_names_outside_closed_set() {
    assert_rejects::<SmellCategory>(&[
        "unknown",
        "off-brand-voice-creep",
        "",
        "scope",
        "silent_success",
        "SilentSuccess",
        " silent-success",
        "silent-success ",
        "schema-violation",
    ]);
}
