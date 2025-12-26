//! Pancha Kosha Allocator (पञ्च कोश स्मृति प्रबन्धन)
//!
//! Multi-tier memory allocator based on Pancha Kosha philosophy.
//! Integrates with Garuda Purana's Preta detection for leak tracking.
//!
//! ## The Five Koshas (Sheaths)
//! 1. **Annamaya** (अन्नमय) - Physical/Food sheath → Registers/L1 Cache
//! 2. **Prāṇamaya** (प्राणमय) - Vital energy sheath → L2/L3 Cache
//! 3. **Manomaya** (मनोमय) - Mental sheath → Main RAM
//! 4. **Vijñānamaya** (विज्ञानमय) - Wisdom sheath → SSD/NVMe
//! 5. **Ānandamaya** (आनन्दमय) - Bliss sheath → Network/Cloud
//!
//! ## Preta Detection (Garuda Purana v5.0)
//! Tracks allocations to detect "hungry ghosts" - memory that is allocated
//! but never freed, trapped between worlds (allocated but unreachable).

#[cfg(feature = "std")]
use std::alloc::{GlobalAlloc, Layout, System};
#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(feature = "std")]
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
#[cfg(feature = "std")]
use std::sync::Mutex;

/// Memory tier (Kosha/कोश)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Kosha {
    /// Annamaya (अन्नमय) - Fastest (registers/L1) - The physical body
    Anna = 0,
    /// Prāṇamaya (प्राणमय) - Fast (L2/L3) - The vital breath
    Prana = 1,
    /// Manomaya (मनोमय) - Medium (RAM) - The mind
    Manas = 2,
    /// Vijñānamaya (विज्ञानमय) - Slow (SSD) - The intellect
    Vijnana = 3,
    /// Ānandamaya (आनन्दमय) - Slowest (Network) - Pure bliss
    Ananda = 4,
}

impl Kosha {
    /// Get expected latency in nanoseconds
    pub const fn latency_ns(&self) -> u64 {
        match self {
            Self::Anna => 1,           // ~1ns for L1
            Self::Prana => 10,         // ~10ns for L2/L3
            Self::Manas => 100,        // ~100ns for RAM
            Self::Vijnana => 10_000,   // ~10μs for SSD
            Self::Ananda => 1_000_000, // ~1ms for network
        }
    }

    /// Get Sanskrit name with diacritics
    pub const fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Anna => "अन्नमय (Annamaya)",
            Self::Prana => "प्राणमय (Prāṇamaya)",
            Self::Manas => "मनोमय (Manomaya)",
            Self::Vijnana => "विज्ञानमय (Vijñānamaya)",
            Self::Ananda => "आनन्दमय (Ānandamaya)",
        }
    }

    /// Get recommended max allocation size for this tier
    pub const fn max_size(&self) -> usize {
        match self {
            Self::Anna => 64,                    // Cache line
            Self::Prana => 4096,                 // Page
            Self::Manas => 16 * 1024 * 1024,     // 16 MB
            Self::Vijnana => 1024 * 1024 * 1024, // 1 GB
            Self::Ananda => usize::MAX,          // Unlimited
        }
    }
}

/// Preta State (प्रेत - Hungry Ghost)
/// From Garuda Purana: souls trapped between death and rebirth
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PretaState {
    /// Jīvita (जीवित) - Alive and reachable
    Jivita,
    /// Mṛta (मृत) - Dead but not yet freed (potential leak)
    Mrta,
    /// Preta (प्रेत) - Trapped ghost (definite leak)
    Preta,
    /// Mukta (मुक्त) - Liberated (properly freed)
    Mukta,
}

/// Allocation tracking entry for Preta detection
#[cfg(feature = "std")]
#[derive(Debug)]
pub struct AllocationEntry {
    /// Memory address
    pub ptr: usize,
    /// Size in bytes
    pub size: usize,
    /// Which Kosha tier
    pub kosha: Kosha,
    /// Current state
    pub state: PretaState,
    /// Allocation timestamp (for age tracking)
    pub allocated_at: u64,
    /// Call site (if available)
    pub call_site: Option<&'static str>,
}

/// Statistics for allocator
#[derive(Debug, Default)]
pub struct AllocatorStats {
    /// Total bytes allocated
    pub total_allocated: usize,
    /// Total bytes freed
    pub total_freed: usize,
    /// Current live bytes
    pub live_bytes: usize,
    /// Peak memory usage
    pub peak_bytes: usize,
    /// Allocation count per kosha
    pub kosha_allocations: [usize; 5],
    /// Detected Preta (leaks)
    pub preta_count: usize,
}

/// Initialize allocator
pub fn init() {
    #[cfg(feature = "std")]
    {
        // Initialize Preta detector by acquiring and releasing lock
        drop(PRETA_DETECTOR.lock().unwrap());
    }
}

/// Pancha Kosha Allocator (पञ्च कोश आवंटक)
pub struct PanchaKoshaAllocator {
    /// Total bytes allocated per tier
    #[cfg(feature = "std")]
    anna_bytes: AtomicUsize,
    #[cfg(feature = "std")]
    prana_bytes: AtomicUsize,
    #[cfg(feature = "std")]
    manas_bytes: AtomicUsize,
    #[cfg(feature = "std")]
    vijnana_bytes: AtomicUsize,
    #[cfg(feature = "std")]
    ananda_bytes: AtomicUsize,

    /// Allocation counter for unique IDs
    #[cfg(feature = "std")]
    allocation_counter: AtomicU64,

    /// Peak memory tracker
    #[cfg(feature = "std")]
    peak_bytes: AtomicUsize,

    /// Total live bytes
    #[cfg(feature = "std")]
    live_bytes: AtomicUsize,
}

// Global Preta detector (leak tracker)
#[cfg(feature = "std")]
lazy_static::lazy_static! {
    static ref PRETA_DETECTOR: Mutex<HashMap<usize, AllocationEntry>> = Mutex::new(HashMap::new());
}

impl PanchaKoshaAllocator {
    /// Create new allocator
    pub const fn new() -> Self {
        Self {
            #[cfg(feature = "std")]
            anna_bytes: AtomicUsize::new(0),
            #[cfg(feature = "std")]
            prana_bytes: AtomicUsize::new(0),
            #[cfg(feature = "std")]
            manas_bytes: AtomicUsize::new(0),
            #[cfg(feature = "std")]
            vijnana_bytes: AtomicUsize::new(0),
            #[cfg(feature = "std")]
            ananda_bytes: AtomicUsize::new(0),
            #[cfg(feature = "std")]
            allocation_counter: AtomicU64::new(0),
            #[cfg(feature = "std")]
            peak_bytes: AtomicUsize::new(0),
            #[cfg(feature = "std")]
            live_bytes: AtomicUsize::new(0),
        }
    }

    /// Select tier based on size and access pattern
    /// Applies Pāṇini's context-sensitive rules for optimal tier selection
    pub fn select_tier(&self, size: usize, hint: Option<Kosha>) -> Kosha {
        // If explicit hint provided, respect it (like Pāṇini's explicit rules)
        if let Some(h) = hint {
            return h;
        }

        // Apply Pāṇini's principle: context determines transformation
        // Size determines the most appropriate kosha
        match size {
            0..=64 => Kosha::Anna,          // Cache-line sized → register/L1
            65..=4096 => Kosha::Prana,      // Page-sized → L2/L3
            4097..=1048576 => Kosha::Manas, // Up to 1MB → RAM
            _ => Kosha::Vijnana,            // Large → SSD tier
        }
    }

    /// Get bytes counter for tier
    #[cfg(feature = "std")]
    fn tier_counter(&self, kosha: Kosha) -> &AtomicUsize {
        match kosha {
            Kosha::Anna => &self.anna_bytes,
            Kosha::Prana => &self.prana_bytes,
            Kosha::Manas => &self.manas_bytes,
            Kosha::Vijnana => &self.vijnana_bytes,
            Kosha::Ananda => &self.ananda_bytes,
        }
    }

    /// Track allocation for Preta detection
    #[cfg(feature = "std")]
    fn track_allocation(&self, ptr: *mut u8, size: usize, kosha: Kosha) {
        if let Ok(mut detector) = PRETA_DETECTOR.try_lock() {
            let entry = AllocationEntry {
                ptr: ptr as usize,
                size,
                kosha,
                state: PretaState::Jivita,
                allocated_at: self.allocation_counter.fetch_add(1, Ordering::Relaxed),
                call_site: None,
            };
            detector.insert(ptr as usize, entry);
        }
    }

    /// Track deallocation - grant Mukti (liberation)
    #[cfg(feature = "std")]
    fn track_deallocation(&self, ptr: *mut u8) {
        if let Ok(mut detector) = PRETA_DETECTOR.try_lock() {
            if let Some(entry) = detector.get_mut(&(ptr as usize)) {
                entry.state = PretaState::Mukta;
            }
            detector.remove(&(ptr as usize));
        }
    }

    /// Get current statistics
    #[cfg(feature = "std")]
    pub fn get_stats(&self) -> AllocatorStats {
        AllocatorStats {
            total_allocated: 0, // Would need additional tracking
            total_freed: 0,
            live_bytes: self.live_bytes.load(Ordering::Relaxed),
            peak_bytes: self.peak_bytes.load(Ordering::Relaxed),
            kosha_allocations: [
                self.anna_bytes.load(Ordering::Relaxed),
                self.prana_bytes.load(Ordering::Relaxed),
                self.manas_bytes.load(Ordering::Relaxed),
                self.vijnana_bytes.load(Ordering::Relaxed),
                self.ananda_bytes.load(Ordering::Relaxed),
            ],
            preta_count: PRETA_DETECTOR.lock().map(|d| d.len()).unwrap_or(0),
        }
    }

    /// Detect Preta (leaked allocations)
    #[cfg(feature = "std")]
    pub fn detect_preta(&self) -> Vec<AllocationEntry> {
        let mut pretas = Vec::new();
        if let Ok(detector) = PRETA_DETECTOR.lock() {
            for entry in detector.values() {
                if entry.state != PretaState::Mukta {
                    pretas.push(AllocationEntry {
                        ptr: entry.ptr,
                        size: entry.size,
                        kosha: entry.kosha,
                        state: PretaState::Preta, // Mark as definite leak
                        allocated_at: entry.allocated_at,
                        call_site: entry.call_site,
                    });
                }
            }
        }
        pretas
    }

    /// Grant Mukti (liberation) - explicitly free memory
    /// This is the Sanskrit way to say "free"
    #[cfg(feature = "std")]
    pub unsafe fn mukti(&self, ptr: *mut u8, layout: Layout) {
        self.dealloc(ptr, layout);
    }

    /// Allocate with explicit Kosha tier (Advaita-aware allocation)
    #[cfg(feature = "std")]
    pub unsafe fn allocate_in_kosha(&self, layout: Layout, kosha: Kosha) -> *mut u8 {
        let ptr = System.alloc(layout);
        if !ptr.is_null() {
            // Update tier statistics
            self.tier_counter(kosha)
                .fetch_add(layout.size(), Ordering::Relaxed);
            let new_live =
                self.live_bytes.fetch_add(layout.size(), Ordering::Relaxed) + layout.size();
            self.peak_bytes.fetch_max(new_live, Ordering::Relaxed);

            // Track for Preta detection
            self.track_allocation(ptr, layout.size(), kosha);
        }
        ptr
    }
}

#[cfg(feature = "std")]
unsafe impl GlobalAlloc for PanchaKoshaAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Select tier based on size
        let kosha = self.select_tier(layout.size(), None);

        // Allocate from system (future: tier-specific pools)
        let ptr = System.alloc(layout);

        if !ptr.is_null() {
            // Update statistics
            self.tier_counter(kosha)
                .fetch_add(layout.size(), Ordering::Relaxed);
            let new_live =
                self.live_bytes.fetch_add(layout.size(), Ordering::Relaxed) + layout.size();
            self.peak_bytes.fetch_max(new_live, Ordering::Relaxed);

            // Track for Preta detection
            self.track_allocation(ptr, layout.size(), kosha);
        }

        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // Track deallocation - grant Mukti
        self.track_deallocation(ptr);

        // Update live bytes
        self.live_bytes.fetch_sub(layout.size(), Ordering::Relaxed);

        // Actually free the memory
        System.dealloc(ptr, layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // Track old allocation removal
        self.track_deallocation(ptr);
        self.live_bytes.fetch_sub(layout.size(), Ordering::Relaxed);

        // Reallocate
        let new_ptr = System.realloc(ptr, layout, new_size);

        if !new_ptr.is_null() {
            // Select tier for new size
            let kosha = self.select_tier(new_size, None);

            // Track new allocation
            let new_live = self.live_bytes.fetch_add(new_size, Ordering::Relaxed) + new_size;
            self.peak_bytes.fetch_max(new_live, Ordering::Relaxed);
            self.track_allocation(new_ptr, new_size, kosha);
        }

        new_ptr
    }
}

/// Global allocator instance
/// Note: Disabled by default to allow stdlib usage
/// Enable with: #[global_allocator] in your main.rs
#[cfg(feature = "std")]
pub static PANCHA_KOSHA_ALLOCATOR: PanchaKoshaAllocator = PanchaKoshaAllocator::new();

// ============================================================================
// Sanskrit API (संस्कृत एपीआई)
// ============================================================================

/// स्मृति आवंटन (smṛti āvaṇṭana) - Memory allocation
/// Allocate memory in the specified Kosha tier
#[cfg(feature = "std")]
pub fn smriti_avantana(size: usize, kosha: Kosha) -> Option<*mut u8> {
    use std::alloc::Layout;
    let layout = Layout::from_size_align(size, 8).ok()?;
    let ptr = unsafe { PANCHA_KOSHA_ALLOCATOR.allocate_in_kosha(layout, kosha) };
    if ptr.is_null() {
        None
    } else {
        Some(ptr)
    }
}

/// मुक्ति (mukti) - Liberation/Free
/// Grant liberation to memory (free it)
#[cfg(feature = "std")]
pub unsafe fn smriti_mukti(ptr: *mut u8, size: usize) {
    use std::alloc::Layout;
    if let Ok(layout) = Layout::from_size_align(size, 8) {
        PANCHA_KOSHA_ALLOCATOR.mukti(ptr, layout);
    }
}

/// प्रेत पता लगाना (preta patā lagānā) - Detect Pretas/Leaks
#[cfg(feature = "std")]
pub fn preta_pata_lagana() -> Vec<AllocationEntry> {
    PANCHA_KOSHA_ALLOCATOR.detect_preta()
}

/// आंकड़े प्राप्त करें (āṅkaṛe prāpt kareṁ) - Get statistics
#[cfg(feature = "std")]
pub fn ankare_prapt_karem() -> AllocatorStats {
    PANCHA_KOSHA_ALLOCATOR.get_stats()
}
