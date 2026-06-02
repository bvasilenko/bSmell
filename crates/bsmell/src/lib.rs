pub mod cli;
pub mod error;
pub mod evidence_state;
pub mod invocation_surface;
pub mod scan;
pub mod substrate_input;
pub mod taxonomy;

pub use cli::{BsmellCli, Command, ScanArgs};
pub use error::BsmellError;
pub use evidence_state::EvidenceState;
pub use invocation_surface::InvocationSurface;
pub use taxonomy::SmellCategory;

pub fn routing_key() -> bsuite_core::RoutingKey {
    bsuite_core::RoutingKey::bsmell()
}
