mod common;

use bsmell::InvocationSurface;
use common::{assert_public_name_contract, assert_rejects};
use std::collections::BTreeSet;

const INTERNAL_SURFACE_FRAGMENTS: [&str; 3] = ["l2a", "l2b", "l2c"];

#[test]
fn invocation_surface_names_cover_exact_closed_set() {
    assert_eq!(5, InvocationSurface::ALL.len());
    assert_public_name_contract(&InvocationSurface::ALL);
}

#[test]
fn invocation_surface_default_is_cli() {
    assert_eq!(InvocationSurface::L2aCli, InvocationSurface::default());
}

#[test]
fn invocation_surface_public_labels_cover_exact_public_closed_set() {
    let expected_labels = BTreeSet::from(["cli", "directus", "payload", "sanity", "strapi"]);
    let actual_labels = InvocationSurface::ALL
        .iter()
        .map(|surface| surface.public_label())
        .collect::<BTreeSet<_>>();

    assert_eq!(expected_labels, actual_labels);
    assert_eq!(InvocationSurface::ALL.len(), actual_labels.len());
}

#[test]
fn invocation_surface_public_labels_are_safe_for_user_facing_output() {
    for surface in InvocationSurface::ALL {
        let label = surface.public_label();

        assert!(!label.is_empty());
        assert_eq!(label.trim(), label);
        assert!(!label.contains(char::is_whitespace));
        assert!(!label.contains(surface.stable_name()));

        for fragment in INTERNAL_SURFACE_FRAGMENTS {
            assert!(!label.contains(fragment), "{surface:?} leaked {fragment}");
        }
    }
}

#[test]
fn invocation_surface_public_labels_are_stable_for_each_surface() {
    for (surface, expected_label) in [
        (InvocationSurface::L2aCli, "cli"),
        (InvocationSurface::L2bPayload, "payload"),
        (InvocationSurface::L2bStrapi, "strapi"),
        (InvocationSurface::L2bSanity, "sanity"),
        (InvocationSurface::L2bDirectus, "directus"),
    ] {
        assert_eq!(expected_label, surface.public_label());
    }
}

#[test]
fn invocation_surface_rejects_names_outside_closed_set() {
    assert_rejects::<InvocationSurface>(&[
        "",
        "l2a",
        "l2a_cli",
        "L2aCli",
        "l2b-wordpress",
        " l2b-payload",
        "l2b-payload ",
    ]);
}
