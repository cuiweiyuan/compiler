//! Function types and lowering for tx kernel API functions
#![allow(missing_docs)]

use std::sync::OnceLock;

use miden_hir::FunctionType;
use miden_hir_type::Type::*;

use super::ModuleFunctionTypeMap;
use crate::miden_abi::FunctionTypeMap;

pub const NOTE_MODULE_NAME: &str = "miden:tx_kernel/note";
pub const NOTE_GET_INPUTS: &str = "get_inputs";

pub const ACCOUNT_MODULE_NAME: &str = "miden:tx_kernel/account";
pub const ACCOUNT_ADD_ASSET: &str = "add_asset";
pub const ACCOUNT_GET_ID: &str = "get_id";

pub(crate) fn signatures() -> &'static ModuleFunctionTypeMap {
    static TYPES: OnceLock<ModuleFunctionTypeMap> = OnceLock::new();
    TYPES.get_or_init(|| {
        let mut m: ModuleFunctionTypeMap = Default::default();

        let mut note: FunctionTypeMap = Default::default();
        note.insert(NOTE_GET_INPUTS, FunctionType::new([Felt], [I32, Felt]));
        m.insert(NOTE_MODULE_NAME, note);

        let mut account: FunctionTypeMap = Default::default();
        account.insert(
            ACCOUNT_ADD_ASSET,
            // Accepts and returns word
            FunctionType::new([Felt, Felt, Felt, Felt], [Felt, Felt, Felt, Felt]),
        );
        account.insert(ACCOUNT_GET_ID, FunctionType::new([], [Felt]));
        m.insert(ACCOUNT_MODULE_NAME, account);

        m
    })
}