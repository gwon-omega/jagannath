# Pancha Kosha Memory Hierarchy
## Five Sheaths of Memory Management

> *"अन्नमयं हि सोम्य मनः"*
> "Mind is indeed made of food" — Chāndogya Upaniṣad 6.5.4

---

## Overview

The Pancha Kosha (five sheaths) model from Vedānta describes five layers of existence, from gross to subtle. Jagannath maps these to a memory hierarchy:

```
┌─────────────────────────────────────────────────────────┐
│                 ĀNANDAMAYA KOSHA                        │
│            (Bliss Sheath - Network/Cloud)               │
│                 Latency: 1-100ms                        │
├─────────────────────────────────────────────────────────┤
│                 VIJÑĀNAMAYA KOSHA                       │
│            (Wisdom Sheath - Disk/SSD)                   │
│                 Latency: 0.1-1ms                        │
├─────────────────────────────────────────────────────────┤
│                 MANOMAYA KOSHA                          │
│            (Mental Sheath - RAM)                        │
│                 Latency: 50-100ns                       │
├─────────────────────────────────────────────────────────┤
│                 PRĀṆAMAYA KOSHA                         │
│            (Vital Sheath - L2/L3 Cache)                 │
│                 Latency: 10-40ns                        │
├─────────────────────────────────────────────────────────┤
│                 ANNAMAYA KOSHA                          │
│            (Physical Sheath - Register/L1)              │
│                 Latency: 1-4ns                          │
└─────────────────────────────────────────────────────────┘
```

---

## 1. Annamaya Kosha (अन्नमयकोश) — Physical Sheath

**Mapping**: CPU Registers and L1 Cache

The "food sheath" — the most physical, immediate memory. Data here is the "food" that feeds computation directly.

```sanskrit
# Allocate in annamaya (register/L1)
saṅkhyā-anna-k: t64 = 42;

# Compiler hint: Keep in register
śīghra counter: t32 = 0;  # śīghra = "swift"
```

### Characteristics
- **Capacity**: 16-64 registers, 32-64KB L1
- **Latency**: 1-4 CPU cycles
- **Use for**: Hot loop variables, accumulators, indices

### Compiler Strategy
```rust
// Allocation decision for annamaya
impl AnnamayaAllocator {
    pub fn should_promote(&self, var: &Variable) -> bool {
        // Promote to register if:
        var.access_count > 10 &&           // Frequently accessed
        var.size <= 8 &&                   // Fits in register
        var.karaka == Some(Karaka::Kartr)  // Agent role (active)
    }
}
```

---

## 2. Prāṇamaya Kosha (प्राणमयकोश) — Vital Sheath

**Mapping**: L2/L3 Cache

The "breath sheath" — energy/vitality. Data that's actively being breathed in and out of computation.

```sanskrit
# Allocate in prāṇamaya (cache-friendly)
sūcī-prāṇa-h: Saṅkhyā[256] = [...];

# Compiler optimizes for cache locality
prāṇa-saṃyukta {  # "breath-connected" block
    # All accesses here optimized for cache
    cala i madhye 0..256 {
        process(array[i]);
    }
}
```

### Characteristics
- **Capacity**: 256KB - 32MB
- **Latency**: 10-40 cycles
- **Use for**: Working sets, arrays being processed, temporary buffers

### Cache-Aware Transformations
```rust
// Loop tiling for prāṇamaya optimization
impl PranamayaOptimizer {
    pub fn tile_for_cache(&self, loop_nest: &LoopNest) -> LoopNest {
        let l2_size = self.cache_info.l2_size;
        let tile_size = self.compute_optimal_tile(loop_nest, l2_size);
        loop_nest.tile(tile_size)
    }
}
```

---

## 3. Manomaya Kosha (मनोमयकोश) — Mental Sheath

**Mapping**: Main Memory (RAM)

The "mind sheath" — the general thinking space. Standard heap allocations live here.

```sanskrit
# Standard heap allocation (manomaya)
upayoktṛ-manas-h = nirmā Upayoktṛ {
    nāma: "Arjuna",
    āyu: 35,
};

# Explicit RAM allocation
smṛti-manas bufara = āvaṇṭana(1024);  # allocate 1KB in RAM
```

### Characteristics
- **Capacity**: GBs - TBs
- **Latency**: 50-100ns
- **Use for**: General data structures, long-lived objects

### Arena Allocation in Manomaya
```rust
// Arena allocator for manomaya
pub struct ManomayaArena {
    chunks: Vec<Chunk>,
    current: *mut u8,
    end: *mut u8,
}

impl ManomayaArena {
    pub fn alloc<T>(&mut self) -> *mut T {
        let size = std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();

        // Align pointer
        let aligned = self.current.align_up(align);

        if aligned.add(size) > self.end {
            self.new_chunk(size);
        }

        let ptr = self.current as *mut T;
        self.current = self.current.add(size);
        ptr
    }
}
```

---

## 4. Vijñānamaya Kosha (विज्ञानमयकोश) — Wisdom Sheath

**Mapping**: Persistent Storage (Disk/SSD)

The "wisdom sheath" — accumulated knowledge. Data that persists beyond program execution.

```sanskrit
# Persistent storage (vijñānamaya)
vijñāna-sañcaya db = khola("users.db");

# Memory-mapped file
vijñāna-mānacitra config = mānacitra("/etc/jag.conf");

# Persistent data structure
@vijñāna
māna CachedModel {
    weights: Tensor-vijñāna,  # Stored on disk
    version: t32,
}
```

### Characteristics
- **Capacity**: TBs - PBs
- **Latency**: 0.1-1ms (SSD), 5-10ms (HDD)
- **Use for**: Databases, config files, ML models, caches

### Memory-Mapped Optimization
```rust
// Vijñānamaya memory mapping
impl VijnanamayaMapper {
    pub fn map_file(&self, path: &Path) -> MappedRegion {
        let file = File::open(path)?;
        let mmap = unsafe {
            memmap2::MmapOptions::new()
                .map(&file)?
        };
        MappedRegion {
            ptr: mmap.as_ptr(),
            len: mmap.len(),
            kosha: Kosha::Vijnanamaya,
        }
    }
}
```

---

## 5. Ānandamaya Kosha (आनन्दमयकोश) — Bliss Sheath

**Mapping**: Network/Distributed Storage

The "bliss sheath" — the subtlest, most expansive layer. Data distributed across the network, approaching infinite storage.

```sanskrit
# Network storage (ānandamaya)
ānanda-sthāna cloud = sambandha("s3://bucket/data");

# Distributed cache
@ānanda
māna GlobalCache {
    nodes: Sūcī<Saṅketa>,
    data: Sāraṇī<Kuñji, Mūlya>,
}

# Async network fetch
ānanda-prāpti result = await fetch_remote(key);
```

### Characteristics
- **Capacity**: Unlimited (distributed)
- **Latency**: 1-100ms+
- **Use for**: Cloud storage, distributed caches, CDNs

### Distributed Data Handling
```rust
// Ānandamaya distributed access
impl AnandamayaClient {
    pub async fn fetch(&self, key: &Key) -> Result<Value> {
        // Check local cache first (descend through koshas)
        if let Some(v) = self.manomaya_cache.get(key) {
            return Ok(v);
        }

        // Fetch from network
        let response = self.client.get(key).await?;

        // Promote to local cache
        self.manomaya_cache.insert(key, response.clone());

        Ok(response)
    }
}
```

---

## Automatic Kosha Selection

The compiler automatically selects the appropriate kosha based on:

```rust
pub fn select_kosha(var: &Variable, ctx: &Context) -> Kosha {
    // 1. Explicit annotation wins
    if let Some(k) = var.kosha_annotation {
        return k;
    }

    // 2. Size-based heuristics
    if var.size <= 8 && var.is_hot {
        return Kosha::Annamaya;  // Register
    }

    if var.size <= ctx.cache_line_size * 4 && var.locality_score > 0.8 {
        return Kosha::Pranamaya;  // Cache
    }

    if var.is_persistent {
        return Kosha::Vijnanamaya;  // Disk
    }

    if var.is_distributed {
        return Kosha::Anandamaya;  // Network
    }

    // Default
    Kosha::Manomaya  // RAM
}
```

---

## Kosha Migration

Data can migrate between koshas based on access patterns:

```sanskrit
# Automatic promotion
sūcī-k data = paṭha_kosha("large_file.dat");  # Starts in vijñāna
# After frequent access, hot portions migrate to prāṇa/manas

# Explicit demotion
vimuñca data -> vijñāna;  # Force back to disk
```

### Migration Strategy
```rust
impl KoshaMigrator {
    pub fn maybe_promote(&mut self, var: &Variable) {
        let current = var.kosha;
        let access_count = self.access_tracker.get(var.id);

        match current {
            Kosha::Vijnanamaya if access_count > 100 => {
                self.migrate(var, Kosha::Manomaya);
            }
            Kosha::Manomaya if access_count > 1000 => {
                self.migrate(var, Kosha::Pranamaya);
            }
            Kosha::Pranamaya if access_count > 10000 => {
                self.migrate(var, Kosha::Annamaya);
            }
            _ => {}
        }
    }
}
```

---

## Performance Comparison

| Kosha | Latency | Bandwidth | Typical Size | Use Case |
|-------|---------|-----------|--------------|----------|
| Annamaya | 1ns | 1TB/s | 64 bytes | Loop counters |
| Prāṇamaya | 10ns | 100GB/s | 256KB | Arrays |
| Manomaya | 100ns | 50GB/s | 32GB | General heap |
| Vijñānamaya | 100μs | 5GB/s | 1TB | Persistent data |
| Ānandamaya | 10ms | 1GB/s | ∞ | Cloud storage |

---

## See Also

- [Sāṃkhya Pipeline](samkhya_pipeline.md) — How memory tiers integrate with compilation
- [Chakra Architecture](../yoga/chakra_architecture.md) — Software layer mapping
- [Guṇa Optimization](../README.md) — How guṇas affect memory decisions
