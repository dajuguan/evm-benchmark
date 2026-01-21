# REVM bench

## Prerequisite
### Set up a node
> Prepare a reth mainnet full-node first.

### Generate block and state dependency data for in-memory execution

```
git clone -b po_bal_dump https://github.com/dajuguan/reth.git
cargo build --release
# for n = from-to blocks (from is included, but to is not included), the following cmd will generate bals_n.json, prestates_n.json, blockHashes_n.json, blocks_n.json
target/release/reth dump --datadir <reth node dir> --from <from> --to <to>
```

## In-memory Pure Execution

### Prerequisite
Run [setup](#setup) and [generate block and state dependency data](#generate-block-and-state-dependency-data-for-in-memory-execution) steps first.

### Pure execution benchmark
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

taskset -c 0-[number of cores - 1] ../../target/release/revme baltest -n [filename suffix, usually it's the number of blocks]  -t [threads] -b [batchsize] -a -p -d --skip-7702 --pre-recover-sender 

# execute with t threads for the n blocks without pre-recoverd sender
taskset -c 0-[number of cores - 1] ../../target/release/revme baltest -n [filename suffix, usually it's the number of blocks]  -t [threads] -b [batchsize] -a -p -d
```

> Use `revme baltest -h` to see more options, such as scheduling with tx's gas limit, debug info for the most time consuming txs.

## Execution with I/O

### Prerequisite
Run [setup](#setup) and [generate block and state dependency data](#generate-block-and-state-dependency-data-for-in-memory-execution) steps first.

### Database migration
Migrate PlainAccountState, PlainStorageState and Bytecodes from MDBX to RocksDB:
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
## rewind the database to snapshot state to block number at from-1 at first. Use rocksdb to get better performance.
## Only need to do it for once, because in the later experiment the code will automatically rewind the database to snapshot state to block number at from-1.
../../target/release/revme baltest -n [filename suffix, usually it's the number of blocks] -a -d --datadir [RocksDB or MDBX path] --db [rocksdb or mdbx] --recover-db

## execute with 1 thread first to generate tx gas used and bal reads, which will be used to measure gas used without 7702 txs
../../target/release/revme baltest -n [filename suffix] -a 
mv balread_[n].json ./data/
mv gasused_[n].json ./data
## parallel I/O
echo 3 | sudo tee /proc/sys/vm/drop_caches && ../../target/release/revme baltest -n [filename suffix]  -t [threads] -b [batchsize] -a -p -d --pre-recover-sender --skip-7702 --datadir [RocksDB or MDBX path] --io par --db [rocksdb or mdbx]
## batched I/O
echo 3 | sudo tee /proc/sys/vm/drop_caches && ../../target/release/revme baltest -n [filename suffix]  -t [threads] -b [batchsize] -a -p -d --pre-recover-sender --skip-7702 --datadir [RocksDB or MDBX path] --io batched --io-threads [batched I/O threads] --db [rocksdb or mdbx]
```

## Execute with worst I/O load case

### Prerequisite
Run [setup](#setup) and [database migration](#database-migration) steps first.

### Execute 
```
cd bins/revme
cargo build --release

## parallel I/O 
echo 3 | sudo tee /proc/sys/vm/drop_caches && taskset -c 0-15 ../../target/release/revme balworst  --datadir [RocksDB path] --io par -b [batchsize: 1-10]

## batched I/O
echo 3 | sudo tee /proc/sys/vm/drop_caches && taskset -c 0-15 ../../target/release/revme balworst  --datadir [RocksDB path] --io batched --io-threads [threads] -b [batchsize: 1-10]
```

### BAL-size measurement
```
../../target/release/revme balsize -n [filename suffix]
```
