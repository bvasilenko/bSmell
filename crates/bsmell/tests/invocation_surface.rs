mod common;

use bsmell::InvocationSurface;
use common::{assert_public_name_contract, assert_rejects};

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
