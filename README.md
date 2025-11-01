# REVM bench

## Setup
- 1 blocks with block, pre-block state, bal, blockHashes loaded into memory

```
git clone -b po_bal_pure_mem https://github.com/dajuguan/revm.git
cd bins/revme
cargo build --release
# execute in n threads
../../target/release/revme baltest -t n
```