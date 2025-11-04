# REVM bench

## Setup

### Generate block and state dependency data for in-memory executing

```
git clone -b po_bal_dump git@github.com:dajuguan/reth.git
cargo build --release
# for n = to-from blocks, the following cmd will generate bals_n.json, prestates_n.json, blockHashes_n.json, blocks_n.json
target/release/reth dump --datadir ~/test_nodes/ethereum/execution/reth_full --from 23600500 --to 23601000 
```

### In-memory execute the dumped blocks
```
git clone -b po_bal_pure_mem https://github.com/dajuguan/revm.git
cd bins/revme
cargo build --release

# Move the above generate json file to bins/revme/data directory. If you don't have the blocks data, there is a default json files for block 23601108.

# execute with t threads for the n blocks
../../target/release/revme baltest -t t -n n
```

> Use revme -h to see more options, such as scheduling with tx's gas limit, debug info for the most time consuming txs.