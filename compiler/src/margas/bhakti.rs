//! # Bhakti Mārga - Path of Devotion
//!
//! Optimization strategy for domain-specific, devoted code.
//!
//! ## Philosophy
//!
//! "भक्त्या त्वनन्यया शक्य" (Bhagavad Gita 11.54)
//! "By single-minded devotion alone can I be known"
//!
//! Bhakti Marga focuses on *devotion* to a single domain - GPU kernels,
//! embedded systems, DSLs. The compiler becomes devoted to that domain's needs.

use super::{Domain, Marga, MargaOptimizer, MargaResult};
use crate::mir::types::{MirFunction, MirInstruction, MirTerminator, MirRvalue, MirOperand, PlaceProjection, MirConstant};
use std::collections::HashMap;

/// Bhakti Marga optimizer for domain-specific devoted optimization
pub struct BhaktiMarga {
    /// The domain we are devoted to
    domain: Domain,
}

/// Memory access pattern for coalescing analysis
#[derive(Debug, Clone)]
struct MemoryPattern {
    /// Base local
    base_local: usize,
    /// Stride between accesses (in elements)
    stride: Option<usize>,
    /// Total accesses in pattern
    access_count: usize,
    /// Is sequential access
    is_sequential: bool,
}

/// Parallelism opportunity
#[derive(Debug)]
struct ParallelOpportunity {
    /// Block ID where parallelism is possible
    block: usize,
    /// Instructions that can be parallelized
    instructions: Vec<usize>,
    /// Type of parallelism
    kind: ParallelKind,
}

#[derive(Debug)]
enum ParallelKind {
    /// SIMD vectorization
    Simd { width: usize },
    /// Thread-level parallelism
    Thread,
    /// GPU kernel
    Gpu,
}

impl BhaktiMarga {
    /// Create a new Bhakti Marga optimizer devoted to a domain
    pub fn new(domain: Domain) -> Self {
        Self { domain }
    }

    /// Set the domain of devotion
    pub fn devote_to(&mut self, domain: Domain) {
        self.domain = domain;
    }

    /// Optimize for GPU execution - vectorization and memory coalescing
    fn vectorize_for_gpu(&self, func: &mut MirFunction) {
        // Find vectorizable loops
        let vector_candidates = self.find_vectorizable_operations(func);

        for candidate in vector_candidates {
            // Transform scalar operations to SIMD
            self.apply_simd_transformation(func, &candidate);
        }
    }

    /// Find operations that can be vectorized
    fn find_vectorizable_operations(&self, func: &MirFunction) -> Vec<ParallelOpportunity> {
        let mut opportunities = Vec::new();

        for block in &func.blocks {
            // Look for repeated similar operations on array elements
            let mut consecutive_ops: Vec<usize> = Vec::new();
            let mut last_op_kind: Option<&MirRvalue> = None;

            for (idx, inst) in block.instructions.iter().enumerate() {
                if let MirInstruction::Assign { value, .. } = inst {
                    // Check if this is a vectorizable operation
                    if self.is_vectorizable_rvalue(value) {
                        if let Some(last) = last_op_kind {
                            if self.same_operation_kind(last, value) {
                                consecutive_ops.push(idx);
                            } else {
                                // Different operation, flush if we have enough
                                if consecutive_ops.len() >= 4 {
                                    opportunities.push(ParallelOpportunity {
                                        block: block.id,
                                        instructions: consecutive_ops.clone(),
                                        kind: ParallelKind::Simd { width: consecutive_ops.len().min(8) },
                                    });
                                }
                                consecutive_ops.clear();
                                consecutive_ops.push(idx);
                            }
                        } else {
                            consecutive_ops.push(idx);
                        }
                        last_op_kind = Some(value);
                    }
                }
            }

            // Flush remaining
            if consecutive_ops.len() >= 4 {
                opportunities.push(ParallelOpportunity {
                    block: block.id,
                    instructions: consecutive_ops,
                    kind: ParallelKind::Simd { width: 4 },
                });
            }
        }

        opportunities
    }

    /// Check if an rvalue is vectorizable
    fn is_vectorizable_rvalue(&self, rv: &MirRvalue) -> bool {
        matches!(rv,
            MirRvalue::BinaryOp { .. } |
            MirRvalue::FloatOp { .. } |
            MirRvalue::UnaryOp { .. }
        )
    }

    /// Check if two rvalues represent the same operation kind
    fn same_operation_kind(&self, a: &MirRvalue, b: &MirRvalue) -> bool {
        match (a, b) {
            (MirRvalue::BinaryOp { op: op1, .. }, MirRvalue::BinaryOp { op: op2, .. }) => op1 == op2,
            (MirRvalue::FloatOp { op: op1, .. }, MirRvalue::FloatOp { op: op2, .. }) => op1 == op2,
            (MirRvalue::UnaryOp { op: op1, .. }, MirRvalue::UnaryOp { op: op2, .. }) => op1 == op2,
            _ => false,
        }
    }

    /// Apply SIMD transformation to a set of operations
    fn apply_simd_transformation(&self, func: &mut MirFunction, opportunity: &ParallelOpportunity) {
        if let ParallelKind::Simd { width } = opportunity.kind {
            // In real implementation:
            // 1. Gather operands into SIMD vectors
            // 2. Replace N scalar ops with 1 SIMD op
            // 3. Scatter results back

            // For now, mark as transformed by noting the optimization
            let _ = (func, width);
        }
    }

    /// Optimize memory coalescing for GPU
    fn optimize_memory_coalescing(&self, func: &mut MirFunction) {
        // Analyze memory access patterns
        let patterns = self.analyze_memory_patterns(func);

        for pattern in patterns {
            if pattern.is_sequential && pattern.access_count >= 4 {
                // Mark for coalesced load/store
                self.mark_coalesced_access(func, &pattern);
            }
        }
    }

    /// Analyze memory access patterns
    fn analyze_memory_patterns(&self, func: &MirFunction) -> Vec<MemoryPattern> {
        let mut patterns: HashMap<usize, MemoryPattern> = HashMap::new();

        for block in &func.blocks {
            for inst in &block.instructions {
                match inst {
                    MirInstruction::Load { ptr, .. } | MirInstruction::Store { ptr, .. } => {
                        if let MirOperand::Copy(place) | MirOperand::Move(place) = ptr {
                            let entry = patterns.entry(place.local).or_insert(MemoryPattern {
                                base_local: place.local,
                                stride: None,
                                access_count: 0,
                                is_sequential: true,
                            });
                            entry.access_count += 1;

                            // Check for sequential access pattern
                            for proj in &place.projection {
                                if let PlaceProjection::ConstIndex { offset } = proj {
                                    if let Some(stride) = entry.stride {
                                        if *offset != stride + 1 {
                                            entry.is_sequential = false;
                                        }
                                    }
                                    entry.stride = Some(*offset);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        patterns.into_values().collect()
    }

    /// Mark memory accesses for coalesced access
    fn mark_coalesced_access(&self, _func: &mut MirFunction, _pattern: &MemoryPattern) {
        // In real implementation, would add hints for codegen
    }

    /// Maximize parallelism
    fn maximize_parallelism(&self, func: &mut MirFunction) {
        // Find parallel opportunities
        let opportunities = self.find_parallel_opportunities(func);

        for opp in opportunities {
            match opp.kind {
                ParallelKind::Simd { width } => {
                    // Already handled by vectorize_for_gpu
                    let _ = width;
                }
                ParallelKind::Thread => {
                    // Mark for thread-level parallelism
                    self.mark_thread_parallel(func, &opp);
                }
                ParallelKind::Gpu => {
                    // Mark as GPU kernel candidate
                    self.mark_gpu_kernel(func, &opp);
                }
            }
        }
    }

    /// Find opportunities for parallelism
    fn find_parallel_opportunities(&self, func: &MirFunction) -> Vec<ParallelOpportunity> {
        let mut opportunities = Vec::new();

        // Look for independent operations (no data dependencies)
        for block in &func.blocks {
            let deps = self.compute_data_dependencies(block);

            // Find instructions with no inter-dependencies
            let mut independent: Vec<usize> = Vec::new();
            for (idx, _) in block.instructions.iter().enumerate() {
                let has_deps = deps.iter().any(|(from, to)| *from == idx || *to == idx);
                if !has_deps {
                    independent.push(idx);
                }
            }

            if independent.len() >= 2 {
                opportunities.push(ParallelOpportunity {
                    block: block.id,
                    instructions: independent,
                    kind: ParallelKind::Thread,
                });
            }
        }

        opportunities
    }

    /// Compute data dependencies in a block
    fn compute_data_dependencies(&self, block: &crate::mir::types::MirBasicBlock) -> Vec<(usize, usize)> {
        let mut deps = Vec::new();
        let mut last_def: HashMap<usize, usize> = HashMap::new();

        for (idx, inst) in block.instructions.iter().enumerate() {
            // Get uses and defs
            let (uses, defs) = self.get_uses_and_defs(inst);

            // Add dependencies for each use
            for use_local in &uses {
                if let Some(&def_idx) = last_def.get(use_local) {
                    deps.push((def_idx, idx));
                }
            }

            // Record defs
            for def_local in defs {
                last_def.insert(def_local, idx);
            }
        }

        deps
    }

    /// Get uses and definitions from an instruction
    fn get_uses_and_defs(&self, inst: &MirInstruction) -> (Vec<usize>, Vec<usize>) {
        let mut uses = Vec::new();
        let mut defs = Vec::new();

        match inst {
            MirInstruction::Assign { dest, value } => {
                defs.push(dest.local);
                self.collect_rvalue_uses(value, &mut uses);
            }
            MirInstruction::Store { ptr, value } => {
                self.collect_operand_uses(ptr, &mut uses);
                self.collect_operand_uses(value, &mut uses);
            }
            MirInstruction::Load { dest, ptr } => {
                defs.push(dest.local);
                self.collect_operand_uses(ptr, &mut uses);
            }
            _ => {}
        }

        (uses, defs)
    }

    fn collect_rvalue_uses(&self, rv: &MirRvalue, uses: &mut Vec<usize>) {
        match rv {
            MirRvalue::Use(op) => self.collect_operand_uses(op, uses),
            MirRvalue::BinaryOp { left, right, .. } | MirRvalue::FloatOp { left, right, .. } => {
                self.collect_operand_uses(left, uses);
                self.collect_operand_uses(right, uses);
            }
            MirRvalue::UnaryOp { operand, .. } | MirRvalue::Cast { operand, .. } => {
                self.collect_operand_uses(operand, uses);
            }
            MirRvalue::Aggregate { operands, .. } | MirRvalue::SimdOp { operands, .. } => {
                for op in operands {
                    self.collect_operand_uses(op, uses);
                }
            }
            MirRvalue::Ref { place, .. } | MirRvalue::AddressOf { place, .. }
            | MirRvalue::Discriminant(place) | MirRvalue::Len(place) => {
                uses.push(place.local);
            }
            MirRvalue::Field { base, .. } => self.collect_operand_uses(base, uses),
            MirRvalue::Index { base, index } => {
                self.collect_operand_uses(base, uses);
                self.collect_operand_uses(index, uses);
            }
        }
    }

    fn collect_operand_uses(&self, op: &MirOperand, uses: &mut Vec<usize>) {
        if let MirOperand::Copy(place) | MirOperand::Move(place) = op {
            uses.push(place.local);
        }
    }

    /// Mark instructions for thread-level parallelism
    fn mark_thread_parallel(&self, _func: &mut MirFunction, _opp: &ParallelOpportunity) {
        // In real implementation, would add parallel execution hints
    }

    /// Mark as GPU kernel candidate
    fn mark_gpu_kernel(&self, _func: &mut MirFunction, _opp: &ParallelOpportunity) {
        // In real implementation, would mark for GPU codegen
    }

    /// Optimize for embedded systems - minimize code size
    fn minimize_code_size(&self, func: &mut MirFunction) {
        // Remove redundant instructions
        self.remove_redundant_instructions(func);

        // Use compact instruction forms where possible
        self.use_compact_forms(func);

        // Inline small constants
        self.inline_small_constants(func);
    }

    /// Remove redundant instructions
    fn remove_redundant_instructions(&self, func: &mut MirFunction) {
        // Find and mark redundant loads (same value already in register)
        let mut value_map: HashMap<usize, usize> = HashMap::new(); // local -> source

        for block in &mut func.blocks {
            value_map.clear();
            let mut to_remove: Vec<usize> = Vec::new();

            for (idx, inst) in block.instructions.iter().enumerate() {
                if let MirInstruction::Assign { dest, value } = inst {
                    // Check if we're assigning a copy of an existing value
                    if let MirRvalue::Use(MirOperand::Copy(src)) = value {
                        if let Some(&existing) = value_map.get(&src.local) {
                            if existing == dest.local {
                                // Redundant copy to self
                                to_remove.push(idx);
                                continue;
                            }
                        }
                        value_map.insert(dest.local, src.local);
                    }
                }
            }

            // Remove redundant instructions (in reverse order to preserve indices)
            for idx in to_remove.into_iter().rev() {
                block.instructions[idx] = MirInstruction::Nop;
            }
        }
    }

    /// Use compact instruction forms
    fn use_compact_forms(&self, _func: &mut MirFunction) {
        // In real implementation, would transform to compact forms
        // e.g., inc/dec instead of add/sub 1
    }

    /// Inline small constants to avoid constant pool
    fn inline_small_constants(&self, _func: &mut MirFunction) {
        // In real implementation, would identify small constants
        // and mark them for immediate encoding
    }

    /// Eliminate dynamic allocation for embedded
    fn eliminate_dynamic_allocation(&self, func: &mut MirFunction) {
        // Find heap allocations
        let allocs = self.find_allocations(func);

        for alloc in allocs {
            // Check if allocation size is statically known
            if let Some(size) = self.get_static_alloc_size(func, &alloc) {
                if size <= 1024 {
                    // Convert to stack allocation
                    self.convert_to_stack_alloc(func, &alloc, size);
                }
            }
        }
    }

    /// Find allocation calls
    fn find_allocations(&self, func: &MirFunction) -> Vec<(usize, usize)> {
        let mut allocs = Vec::new();

        for block in &func.blocks {
            if let MirTerminator::Call { func: callee, .. } = &block.terminator {
                if let MirOperand::Constant(MirConstant::String(name)) = callee {
                    if name.contains("alloc") || name.contains("malloc") {
                        allocs.push((block.id, 0));
                    }
                }
            }
        }

        allocs
    }

    /// Get static allocation size if known
    fn get_static_alloc_size(&self, _func: &MirFunction, _alloc: &(usize, usize)) -> Option<usize> {
        // In real implementation, would analyze allocation argument
        None
    }

    /// Convert heap allocation to stack
    fn convert_to_stack_alloc(&self, _func: &mut MirFunction, _alloc: &(usize, usize), _size: usize) {
        // In real implementation, would replace alloc call with stack alloca
    }

    /// Optimize for low power
    fn optimize_for_low_power(&self, func: &mut MirFunction) {
        // Reduce memory accesses (most power-hungry operation)
        self.minimize_memory_accesses(func);

        // Use efficient instructions
        self.prefer_efficient_instructions(func);
    }

    /// Minimize memory accesses
    fn minimize_memory_accesses(&self, _func: &mut MirFunction) {
        // In real implementation:
        // - Promote frequently accessed values to registers
        // - Combine multiple small loads into larger loads
        // - Cache computed values
    }

    /// Prefer power-efficient instructions
    fn prefer_efficient_instructions(&self, _func: &mut MirFunction) {
        // In real implementation:
        // - Use shifts instead of multiplies where possible
        // - Avoid floating point when integers suffice
        // - Use conditional moves instead of branches
    }

    /// Apply domain knowledge
    fn apply_domain_knowledge(&self, func: &mut MirFunction) {
        // Apply domain-specific patterns and idioms
        match self.domain {
            Domain::GPU => {
                // GPU-specific patterns
                self.apply_gpu_patterns(func);
            }
            Domain::Embedded => {
                // Embedded-specific patterns
                self.apply_embedded_patterns(func);
            }
            Domain::MachineLearning => {
                // ML-specific patterns (matmul, convolution, etc.)
                self.apply_ml_patterns(func);
            }
            Domain::DSL => {
                // DSL-specific optimizations
                self.apply_dsl_patterns(func);
            }
            _ => {}
        }
    }

    fn apply_gpu_patterns(&self, _func: &mut MirFunction) {
        // Recognize and optimize GPU patterns:
        // - Parallel reduction
        // - Scan/prefix sum
        // - Matrix operations
    }

    fn apply_embedded_patterns(&self, _func: &mut MirFunction) {
        // Recognize and optimize embedded patterns:
        // - Interrupt handlers
        // - State machines
        // - Bit manipulation
    }

    fn apply_ml_patterns(&self, _func: &mut MirFunction) {
        // Recognize and optimize ML patterns:
        // - Matrix multiplication
        // - Convolution
        // - Activation functions
    }

    fn apply_dsl_patterns(&self, _func: &mut MirFunction) {
        // Apply DSL-specific patterns based on domain hints
    }

    /// Use domain-specific optimizations
    fn use_domain_specific_optimizations(&self, func: &mut MirFunction) {
        // Pattern matching for domain-specific idioms
        self.apply_domain_knowledge(func);
    }

    /// Detect the domain from function attributes
    fn detect_domain(&self, func: &MirFunction) -> Domain {
        // Check for domain indicators in function name
        let name_lower = func.name.to_lowercase();

        // GPU indicators
        if name_lower.contains("gpu") || name_lower.contains("kernel")
           || name_lower.contains("cuda") || name_lower.contains("compute") {
            return Domain::GPU;
        }

        // Embedded indicators
        if name_lower.contains("embedded") || name_lower.contains("mcu")
           || name_lower.contains("irq") || name_lower.contains("interrupt") {
            return Domain::Embedded;
        }

        // ML indicators
        if name_lower.contains("tensor") || name_lower.contains("neural")
           || name_lower.contains("conv") || name_lower.contains("matmul") {
            return Domain::MachineLearning;
        }

        // Network indicators
        if name_lower.contains("socket") || name_lower.contains("async")
           || name_lower.contains("network") || name_lower.contains("http") {
            return Domain::Network;
        }

        Domain::General
    }
}

impl MargaOptimizer for BhaktiMarga {
    fn marga(&self) -> Marga {
        Marga::Bhakti
    }

    fn optimize(&self, func: &mut MirFunction) -> MargaResult {
        // Devotion to single purpose/domain
        let domain = self.detect_domain(func);

        match domain {
            Domain::GPU => {
                // Devoted to GPU execution
                self.vectorize_for_gpu(func);
                self.optimize_memory_coalescing(func);
                self.maximize_parallelism(func);
            }

            Domain::Embedded => {
                // Devoted to embedded systems
                self.minimize_code_size(func);
                self.eliminate_dynamic_allocation(func);
                self.optimize_for_low_power(func);
            }

            Domain::DSL => {
                // Devoted to domain-specific language
                self.apply_domain_knowledge(func);
                self.use_domain_specific_optimizations(func);
            }

            Domain::MachineLearning => {
                // Devoted to ML workloads
                self.vectorize_for_gpu(func);
                self.apply_domain_knowledge(func);
            }

            Domain::Network => {
                // Devoted to network/async code
                self.apply_domain_knowledge(func);
            }

            Domain::General => {
                // General optimization
                self.apply_domain_knowledge(func);
            }
        }

        MargaResult::success(
            Marga::Bhakti,
            format!("Devoted optimization for {:?} domain", domain),
        )
    }

    fn is_suitable_for(&self, func: &MirFunction) -> bool {
        // Bhakti is suitable when we can identify a specific domain
        self.detect_domain(func) != Domain::General
    }

    fn mantra(&self) -> &'static str {
        // "By single-minded devotion alone can I be known"
        "भक्त्या त्वनन्यया शक्य"
    }
}
