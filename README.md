# REVM bench

## Setup
> Prepare a reth mainnet full-node first.

### Generate block and state dependency data for in-memory execution

```
git clone -b po_bal_dump https://github.com/dajuguan/reth.git
cargo build --release
# for n = to-from blocks, the following cmd will generate bals_n.json, prestates_n.json, blockHashes_n.json, blocks_n.json
target/release/reth dump --datadir <reth node dir> --from <from> --to <to>
```

### In-memory execute the dumped blocks
```
git clone -b po_bal_pure_mem https://github.com/dajuguan/revm.git
cd bins/revme
cargo build --release

# Move the above generated json file to bins/revme/data directory. If you don't have the blocks data, there is a default json files for block 23601108.

# execute with 1 thread first to generate tx gas used, which will be used to measure gas used without 7702 txs
../../target/release/revme baltest -n [number of blocks] -a 
mv balread_[n].json ./data/
mv gasused_[n].json ./data

# execute with t threads for the n blocks with pre-recoverd sender

taskset -c 0-[number of cores - 1] ../../target/release/revme baltest -n [number of blocks]  -t [threads] -b [batchsize] -a -p -d --skip-7702 --pre-recover-sender 

# execute with t threads for the n blocks without pre-recoverd sender
taskset -c 0-[number of cores - 1] ../../target/release/revme baltest -n [number of blocks]  -t [threads] -b [batchsize] -a -p -d
```

> Use `revme baltest -h` to see more options, such as scheduling with tx's gas limit, debug info for the most time consuming txs.

## Execution with I/O

### Prerequisite
Run [setup](#setup) and [generate block and state dependency data](#generate-block-and-state-dependency-data-for-in-memory-execution) steps first.

### Migrate PlainAccountState, PlainStorageState and Bytecodes from MDBX to RocksDB
```
git clone -b po_bal_pure_mem https://github.com/dajuguan/revm.git
cd bins/dbtool
cargo build --release
../../target/release/dbtool migration --src [reth MDBX path] --dst [RocksDB path]
```

### Execute the dumped block with I/O
```
cd bins/revme
cargo build --release
## execute with 1 thread first to generate tx gas used and bal reads, which will be used to measure gas used without 7702 txs
../../target/release/revme baltest -n [number of blocks] -a 
mv balread_[n].json ./data/
mv gasused_[n].json ./data
## parallel I/O
echo 3 | sudo tee /proc/sys/vm/drop_caches && ../../target/release/revme baltest -n [number of blocks]  -t [threads] -b [batchsize] -a -p -d --pre-recover-sender --skip-7702 --datadir [RocksDB or MDBX path] --io par --db [rocksdb or mdbx]
## batched I/O
echo 3 | sudo tee /proc/sys/vm/drop_caches && ../../target/release/revme baltest -n [number of blocks]  -t [threads] -b [batchsize] -a -p -d --pre-recover-sender --skip-7702 --datadir [RocksDB or MDBX path] --io batched --io-threads [batched I/O threads] --db [rocksdb or mdbx]
```

### BAL-size measurement
```
../../target/release/revme balsize -n [number of blocks]
```
