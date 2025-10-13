use revm::{Context, MainContext, context};
use revm::{
    context::{CfgEnv, ContextTr},
    database::State,
    primitives::ruint::Uint,
};

#[cfg(test)]
mod tests {
    use revm::{
        Database,
        database::StorageWithOriginalValues,
        primitives::{Address, HashMap, KECCAK_EMPTY, U256, hex::FromHex},
        state::AccountInfo,
    };

    use super::*;

    #[test]
    fn test_block_ctx() {
        let mut block_env = context::BlockEnv::default();
        block_env.number = Uint::from(100);
        println!("blockenv: {:?}", block_env);
        println!("blockNumber: {:?}", block_env.number);

        let mut cfg = CfgEnv::new();
        // Create database with initial state
        let mut state = State::builder().build();
        let evm_context = Context::mainnet()
            .with_block(&block_env)
            .with_cfg(&cfg)
            .with_db(&mut state);
        println!("evm_context: {:?}", evm_context);

        let address = Address::from_hex("0x4838B106FCe9647Bdf1E7877BF73cE8B0BAD5f97").unwrap();

        let info = AccountInfo {
            balance: U256::from(10),
            nonce: 1,
            code_hash: KECCAK_EMPTY,
            code: None,
        };
        let storage = HashMap::default();
        state.insert_account_with_storage(address, info, storage);

        let acct_info = state.basic(address).unwrap().unwrap();
        print!("account info: {:?}", acct_info)
    }
}
