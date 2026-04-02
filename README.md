# Thread‑Safe LRU Cache (Rust)

This project implements a **thread‑safe Least Recently Used (LRU) cache** in Rust, as required for the DDN India coding assessment.  
The cache supports concurrent access from multiple threads, maintains correct LRU eviction ordering, and guarantees bounded memory usage.

---

## Features

- **Thread‑safe** (using `RwLock`)
- **Safe Rust only** (no `unsafe`)
- **O(1)** average time complexity for `get()` and `put()`
- **Deterministic eviction** of least‑recently‑used key
- Supports multiple readers & writers
- Predictable and bounded memory usage
- Includes **unit tests + concurrency tests**
- Includes **DESIGN.md** (required)
