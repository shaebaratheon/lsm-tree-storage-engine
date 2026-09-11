# lsm-tree-storage-engine

Log-Structured Merge-Tree (LSM-tree) persistent key-value storage engine with MemTable and SSTable compaction in Rust.

## Architecture & Design

This project implements a high-reliability distributed architecture designed for production workloads.
### Core Components
- `memtable`: Core subsystem handling specific domain logic, invariants, and performance guarantees.
- `wal`: Core subsystem handling specific domain logic, invariants, and performance guarantees.
- `sstable`: Core subsystem handling specific domain logic, invariants, and performance guarantees.
- `bloom_filter`: Core subsystem handling specific domain logic, invariants, and performance guarantees.
- `compaction`: Core subsystem handling specific domain logic, invariants, and performance guarantees.
- `block_cache`: Core subsystem handling specific domain logic, invariants, and performance guarantees.

## Testing and Verification

Run the test suite via standard tooling.
