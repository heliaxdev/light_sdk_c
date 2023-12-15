pub mod account;

use std::str::FromStr;

use light_sdk::namada_core::types::chain::ChainId;
use light_sdk::namada_core::types::hash::Hash;
use light_sdk::namada_core::types::time::DateTimeUtc;
use light_sdk::transaction;

use crate::utils::{CString, FormatErr};

#[repr(C)]
pub struct GlobalArgs {
    expiration: CString,
    code_hash: [u8; 32],
    chain_id: CString,
}

impl TryFrom<GlobalArgs> for transaction::GlobalArgs {
    type Error = CString;

    fn try_from(global_args: GlobalArgs) -> Result<Self, Self::Error> {
        let expiration = global_args.expiration.to_string();
        let expiration = if &expiration == "" {
            None
        } else {
            Some(DateTimeUtc::from_str(&expiration).format_err()?)
        };
        let chain_id = ChainId::from_str(&global_args.chain_id.to_string()).format_err()?;
        Ok(transaction::GlobalArgs {
            expiration,
            code_hash: Hash(global_args.code_hash),
            chain_id,
        })
    }
}

#[repr(C)]
pub enum TxKind {
    RevealPk,
}
