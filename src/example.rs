use revm::{Context, MainContext, context};
use revm::{
    context::{CfgEnv, ContextTr},
    database::State,
    primitives::ruint::Uint,
};

#[cfg(test)]
mod tests {
    use revm::{
        Database, ExecuteEvm, MainBuilder,
        bytecode::opcode,
        context::TxEnv,
        database::{CacheDB, EmptyDB, StorageWithOriginalValues},
        interpreter::instructions::utility::IntoAddress,
        primitives::{
            self, Address, HashMap, KECCAK_EMPTY, StorageKey, StorageValue, TxKind, U256, address,
            hex::{self, FromHex},
        },
        state::{AccountInfo, Bytecode, EvmStorageSlot},
    };

    use super::*;

    const RUNTIME_BYTECODE: &[u8] = &[opcode::PUSH0, opcode::SLOAD];

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

        // set mock account and storage
        let address = Address::from_hex("0x4838B106FCe9647Bdf1E7877BF73cE8B0BAD5f97").unwrap();

        let info = AccountInfo {
            balance: U256::from(10),
            nonce: 1,
            code_hash: primitives::keccak256(&RUNTIME_BYTECODE),
            code: Some(Bytecode::new_raw(RUNTIME_BYTECODE.into())),
        };
        let mut storage = HashMap::<StorageKey, StorageValue>::default();
        let key1 = StorageKey::from(0);
        let slot1 = StorageValue::from(0xff);
        storage.insert(key1, slot1);
        state.insert_account_with_storage(address, info.clone(), storage.clone());

        let acct_info = state.basic(address).unwrap().unwrap();
        println!("account info: {:?}", acct_info);

        let slot_val = storage.get(&key1).unwrap();
        println!("slot value: {:?}", hex::encode(slot_val.to_be_bytes_vec()));

        // build tx
        let caller = address!("0000000000000000000000000000000000000000");
        let tx = TxEnv::builder()
            .caller(caller)
            .kind(TxKind::Call(address))
            .data(Default::default())
            .value(U256::from(0))
            .build()
            .unwrap();
        // start tx
        // let mut db = CacheDB::<EmptyDB>::default();
        // db.insert_account_info(address, info.clone());
        // db.insert_account_storage(address, key1, slot1).unwrap();
        let ctx = Context::mainnet().with_db(&mut state);
        let mut evm = ctx.build_mainnet();

        let result = evm.transact(tx).unwrap();
        let Some(storage0) = result
            .state
            .get(&address)
            .unwrap()
            .storage
            .get::<StorageValue>(&Default::default())
        else {
            panic!("not exists")
        };

        println!("storage U256(0) at {address}:  {storage0:#?}");
    }
}
