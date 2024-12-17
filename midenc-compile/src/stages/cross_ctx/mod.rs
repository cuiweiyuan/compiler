//! Translation between Wasm CM CABI and Miden cross-context ABI (Miden CCABI).

mod flat;
pub(super) mod lift_exports;
pub(super) mod lower_imports;
