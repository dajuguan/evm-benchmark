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

# execute with t threads for the n blocks

taskset -c 0-[number of cores - 1] ../../target/release/revme baltest -n [number of blocks]  -t [threads] -b [batchsize] -a -p -d --skip-7702
```

> Use `revme baltest -h` to see more options, such as scheduling with tx's gas limit, debug info for the most time consuming txs.
