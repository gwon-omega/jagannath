//! Non-Lexical Lifetimes (NLL) Analysis - Kāla Mukti (काल मुक्ति)
//!
//! Implements MIR-based borrow checking with non-lexical lifetimes.
//! This allows borrows to end earlier than lexical scope boundaries,
//! enabling more flexible and ergonomic code.
//!
//! Key concepts mapped to Sanskrit:
//! - काल (Kāla) - Time/Region - Lifetime region
//! - मुक्ति (Mukti) - Liberation - Borrow ending
//! - जीवन (Jīvana) - Life - Liveness of a value
//! - धारण (Dhāraṇa) - Holding - Active borrow
//!
//! Algorithm Overview:
//! 1. Build region inference variables for each borrow
//! 2. Generate constraints from MIR statements
//! 3. Compute liveness for all locals
//! 4. Propagate constraints to fixed point
//! 5. Check for violations

use super::types::*;
use std::collections::{HashMap, HashSet, VecDeque};

// ============================================================================
// Region Variables and Constraints
// ============================================================================

/// Region variable identifier (lifetime variable)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegionVar(pub usize);

impl std::fmt::Display for RegionVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'_{}", self.0)
    }
}

/// A region represents the lifetime of a borrow
#[derive(Debug, Clone)]
pub struct Region {
    /// Points in the CFG where this region is live
    pub points: HashSet<LocationPoint>,
    /// Other regions this region must outlive
    pub outlives: HashSet<RegionVar>,
}

/// A point in the MIR CFG
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocationPoint {
    /// Basic block index
    pub block: usize,
    /// Statement index within block (usize::MAX = terminator)
    pub statement: usize,
}

impl LocationPoint {
    pub fn new(block: usize, statement: usize) -> Self {
        Self { block, statement }
    }

    pub fn terminator(block: usize) -> Self {
        Self {
            block,
            statement: usize::MAX,
        }
    }
}

/// Constraint on regions
#[derive(Debug, Clone)]
pub enum RegionConstraint {
    /// R1: R2 - Region R1 must outlive region R2
    Outlives { sup: RegionVar, sub: RegionVar },
    /// R: P - Region R must contain point P
    ContainsPoint {
        region: RegionVar,
        point: LocationPoint,
    },
    /// R1 = R2 - Regions are equal
    Equal { r1: RegionVar, r2: RegionVar },
}

// ============================================================================
// Liveness Analysis
// ============================================================================

/// Liveness information for MIR locals
#[derive(Debug, Clone)]
pub struct LivenessInfo {
    /// Live-in for each block: which locals are live at block entry
    pub live_in: HashMap<usize, HashSet<usize>>,
    /// Live-out for each block: which locals are live at block exit
    pub live_out: HashMap<usize, HashSet<usize>>,
    /// Def points: where each local is defined (block, statement)
    pub def_points: HashMap<usize, Vec<LocationPoint>>,
    /// Use points: where each local is used (block, statement)
    pub use_points: HashMap<usize, Vec<LocationPoint>>,
}

impl LivenessInfo {
    pub fn new() -> Self {
        Self {
            live_in: HashMap::new(),
            live_out: HashMap::new(),
            def_points: HashMap::new(),
            use_points: HashMap::new(),
        }
    }

    /// Check if a local is live at a point
    pub fn is_live_at(&self, local: usize, point: &LocationPoint) -> bool {
        // A local is live at point P if there's a path from P to a use
        // without passing through a definition
        self.live_out
            .get(&point.block)
            .map_or(false, |set| set.contains(&local))
    }
}

/// Compute liveness for all locals in a MIR function
pub fn compute_liveness(func: &MirFunction) -> LivenessInfo {
    let mut info = LivenessInfo::new();

    // Initialize all blocks
    for block in &func.blocks {
        info.live_in.insert(block.id, HashSet::new());
        info.live_out.insert(block.id, HashSet::new());
    }

    // Collect def and use points
    for block in &func.blocks {
        for (stmt_idx, inst) in block.instructions.iter().enumerate() {
            let point = LocationPoint::new(block.id, stmt_idx);

            // Record definitions
            if let Some(def_local) = get_instruction_def(inst) {
                info.def_points.entry(def_local).or_default().push(point);
            }

            // Record uses
            for use_local in get_instruction_uses(inst) {
                info.use_points.entry(use_local).or_default().push(point);
            }
        }

        // Handle terminator
        let term_point = LocationPoint::terminator(block.id);
        for use_local in get_terminator_uses(&block.terminator) {
            info.use_points
                .entry(use_local)
                .or_default()
                .push(term_point);
        }
    }

    // Build CFG successors map
    let mut successors: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut predecessors: HashMap<usize, Vec<usize>> = HashMap::new();

    for block in &func.blocks {
        let succs = get_terminator_successors(&block.terminator);
        successors.insert(block.id, succs.clone());
        for succ in succs {
            predecessors.entry(succ).or_default().push(block.id);
        }
    }

    // Build gen/kill sets for each block
    let mut gen: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut kill: HashMap<usize, HashSet<usize>> = HashMap::new();

    for block in &func.blocks {
        let mut block_gen = HashSet::new();
        let mut block_kill = HashSet::new();

        // Process in reverse order for gen (upward exposed uses)
        for inst in block.instructions.iter().rev() {
            // If defined in this block, not gen'd from above
            if let Some(def) = get_instruction_def(inst) {
                block_kill.insert(def);
                block_gen.remove(&def);
            }
            // If used, it's gen'd (unless killed first going backwards)
            for use_local in get_instruction_uses(inst) {
                if !block_kill.contains(&use_local) {
                    block_gen.insert(use_local);
                }
            }
        }

        // Terminator uses
        for use_local in get_terminator_uses(&block.terminator) {
            if !block_kill.contains(&use_local) {
                block_gen.insert(use_local);
            }
        }

        gen.insert(block.id, block_gen);
        kill.insert(block.id, block_kill);
    }

    // Fixed-point iteration (backwards dataflow)
    let mut changed = true;
    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 1000;

    while changed && iterations < MAX_ITERATIONS {
        changed = false;
        iterations += 1;

        // Process blocks in reverse order (for backwards analysis)
        for block in func.blocks.iter().rev() {
            let block_id = block.id;

            // live_out[B] = ∪ live_in[S] for all successors S of B
            let mut new_live_out = HashSet::new();
            if let Some(succs) = successors.get(&block_id) {
                for succ in succs {
                    if let Some(succ_in) = info.live_in.get(succ) {
                        new_live_out.extend(succ_in.iter().copied());
                    }
                }
            }

            // live_in[B] = gen[B] ∪ (live_out[B] - kill[B])
            let block_gen = gen.get(&block_id).cloned().unwrap_or_default();
            let block_kill = kill.get(&block_id).cloned().unwrap_or_default();
            let mut new_live_in: HashSet<usize> =
                new_live_out.difference(&block_kill).copied().collect();
            new_live_in.extend(block_gen);

            // Check for changes
            if info.live_in.get(&block_id) != Some(&new_live_in) {
                info.live_in.insert(block_id, new_live_in);
                changed = true;
            }
            if info.live_out.get(&block_id) != Some(&new_live_out) {
                info.live_out.insert(block_id, new_live_out);
                changed = true;
            }
        }
    }

    info
}

/// Get the local defined by an instruction (if any)
fn get_instruction_def(inst: &MirInstruction) -> Option<usize> {
    match inst {
        MirInstruction::Assign { dest, .. } => {
            if dest.projection.is_empty() {
                Some(dest.local)
            } else {
                None
            }
        }
        MirInstruction::Load { dest, .. } => {
            if dest.projection.is_empty() {
                Some(dest.local)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Get locals used by an instruction
fn get_instruction_uses(inst: &MirInstruction) -> Vec<usize> {
    let mut uses = Vec::new();

    match inst {
        MirInstruction::Assign { value, .. } => {
            collect_rvalue_uses(value, &mut uses);
        }
        MirInstruction::Store { ptr, value } => {
            collect_operand_uses(ptr, &mut uses);
            collect_operand_uses(value, &mut uses);
        }
        MirInstruction::Load { ptr, .. } => {
            collect_operand_uses(ptr, &mut uses);
        }
        MirInstruction::Drop { place } => {
            uses.push(place.local);
        }
        _ => {}
    }

    uses
}

/// Get locals used by a terminator
fn get_terminator_uses(term: &MirTerminator) -> Vec<usize> {
    let mut uses = Vec::new();

    match term {
        MirTerminator::SwitchInt { discriminant, .. } => {
            collect_operand_uses(discriminant, &mut uses);
        }
        MirTerminator::Call { func, args, .. } => {
            collect_operand_uses(func, &mut uses);
            for arg in args {
                collect_operand_uses(arg, &mut uses);
            }
        }
        _ => {}
    }

    uses
}

/// Get successor block IDs from a terminator
fn get_terminator_successors(term: &MirTerminator) -> Vec<usize> {
    match term {
        MirTerminator::Goto { target } => vec![*target],
        MirTerminator::SwitchInt {
            targets, otherwise, ..
        } => {
            let mut succs: Vec<usize> = targets.iter().map(|(_, t)| *t).collect();
            succs.push(*otherwise);
            succs
        }
        MirTerminator::Return | MirTerminator::Unreachable | MirTerminator::Unwind => vec![],
        MirTerminator::Call { target, .. } => vec![*target],
    }
}

/// Collect uses from an rvalue
fn collect_rvalue_uses(rvalue: &MirRvalue, uses: &mut Vec<usize>) {
    match rvalue {
        MirRvalue::Use(op) => collect_operand_uses(op, uses),
        MirRvalue::Ref { place, .. } => uses.push(place.local),
        MirRvalue::BinaryOp { left, right, .. } => {
            collect_operand_uses(left, uses);
            collect_operand_uses(right, uses);
        }
        MirRvalue::UnaryOp { operand, .. } => collect_operand_uses(operand, uses),
        MirRvalue::Aggregate { operands, .. } => {
            for op in operands {
                collect_operand_uses(op, uses);
            }
        }
        MirRvalue::Cast { operand, .. } => collect_operand_uses(operand, uses),
        MirRvalue::Len(place) => uses.push(place.local),
        MirRvalue::Discriminant(place) => uses.push(place.local),
        MirRvalue::AddressOf { place, .. } => uses.push(place.local),
        MirRvalue::Field { base, .. } => collect_operand_uses(base, uses),
        MirRvalue::Index { base, index } => {
            collect_operand_uses(base, uses);
            collect_operand_uses(index, uses);
        }
        MirRvalue::FloatOp { left, right, .. } => {
            collect_operand_uses(left, uses);
            collect_operand_uses(right, uses);
        }
        MirRvalue::SimdOp { operands, .. } => {
            for op in operands {
                collect_operand_uses(op, uses);
            }
        }
    }
}

/// Collect uses from an operand
fn collect_operand_uses(op: &MirOperand, uses: &mut Vec<usize>) {
    match op {
        MirOperand::Copy(place) | MirOperand::Move(place) => {
            uses.push(place.local);
        }
        MirOperand::Constant(_) => {}
    }
}

// ============================================================================
// NLL Borrow Checker
// ============================================================================

/// Borrow information for NLL analysis
#[derive(Debug, Clone)]
pub struct BorrowData {
    /// The region variable for this borrow
    pub region: RegionVar,
    /// What is being borrowed
    pub borrowed_place: MirPlace,
    /// Is this a mutable borrow?
    pub mutable: bool,
    /// Location where borrow starts
    pub activation_point: LocationPoint,
    /// Location where borrow is first used (may differ from activation)
    pub reserve_point: Option<LocationPoint>,
}

/// NLL borrow checker state
pub struct NllChecker {
    /// Region variables
    regions: HashMap<RegionVar, Region>,
    /// Next region variable ID
    next_region: usize,
    /// Constraints collected
    constraints: Vec<RegionConstraint>,
    /// Borrow data
    borrows: Vec<BorrowData>,
    /// Liveness information
    liveness: Option<LivenessInfo>,
    /// Errors found
    errors: Vec<NllError>,
}

/// NLL error kinds
#[derive(Debug, Clone)]
pub enum NllError {
    /// Use after move
    UseAfterMove {
        moved_at: LocationPoint,
        used_at: LocationPoint,
        local: usize,
    },
    /// Conflicting borrows
    ConflictingBorrow {
        first_borrow: LocationPoint,
        second_borrow: LocationPoint,
        local: usize,
        first_mutable: bool,
        second_mutable: bool,
    },
    /// Borrow used after value modified
    InvalidatedBorrow {
        borrow_point: LocationPoint,
        invalidation_point: LocationPoint,
        use_point: LocationPoint,
    },
    /// Moved value still borrowed
    MovedWhileBorrowed {
        borrow_point: LocationPoint,
        move_point: LocationPoint,
    },
}

impl NllChecker {
    pub fn new() -> Self {
        Self {
            regions: HashMap::new(),
            next_region: 0,
            constraints: Vec::new(),
            borrows: Vec::new(),
            liveness: None,
            errors: Vec::new(),
        }
    }

    /// Create a fresh region variable
    pub fn fresh_region(&mut self) -> RegionVar {
        let rv = RegionVar(self.next_region);
        self.next_region += 1;
        self.regions.insert(
            rv,
            Region {
                points: HashSet::new(),
                outlives: HashSet::new(),
            },
        );
        rv
    }

    /// Add a constraint
    pub fn add_constraint(&mut self, constraint: RegionConstraint) {
        self.constraints.push(constraint);
    }

    /// Check a MIR function for borrow violations
    pub fn check_function(&mut self, func: &MirFunction) -> Result<(), Vec<NllError>> {
        // Phase 1: Compute liveness
        self.liveness = Some(compute_liveness(func));

        // Phase 2: Collect borrows and generate constraints
        self.collect_borrows(func);

        // Phase 3: Propagate constraints to fixed point
        self.propagate_constraints(func);

        // Phase 4: Check for violations
        self.check_violations(func);

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    /// Collect all borrows from MIR
    fn collect_borrows(&mut self, func: &MirFunction) {
        for block in &func.blocks {
            for (stmt_idx, inst) in block.instructions.iter().enumerate() {
                let point = LocationPoint::new(block.id, stmt_idx);

                if let MirInstruction::Assign {
                    value: MirRvalue::Ref { place, mutable },
                    ..
                } = inst
                {
                    let region = self.fresh_region();
                    self.borrows.push(BorrowData {
                        region,
                        borrowed_place: place.clone(),
                        mutable: *mutable,
                        activation_point: point,
                        reserve_point: None,
                    });

                    // Add constraint: borrow region contains activation point
                    self.add_constraint(RegionConstraint::ContainsPoint { region, point });
                }
            }
        }
    }

    /// Propagate constraints to compute final regions
    fn propagate_constraints(&mut self, func: &MirFunction) {
        let mut changed = true;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 1000;

        while changed && iterations < MAX_ITERATIONS {
            changed = false;
            iterations += 1;

            for constraint in self.constraints.clone() {
                match constraint {
                    RegionConstraint::ContainsPoint { region, point } => {
                        if let Some(r) = self.regions.get_mut(&region) {
                            if r.points.insert(point) {
                                changed = true;
                            }
                        }
                    }
                    RegionConstraint::Outlives { sup, sub } => {
                        // sup must contain all points in sub
                        if let Some(sub_region) = self.regions.get(&sub).cloned() {
                            if let Some(sup_region) = self.regions.get_mut(&sup) {
                                for point in sub_region.points {
                                    if sup_region.points.insert(point) {
                                        changed = true;
                                    }
                                }
                            }
                        }
                    }
                    RegionConstraint::Equal { r1, r2 } => {
                        // Both must have same points
                        let (points1, points2) = {
                            let r1_reg = self.regions.get(&r1).cloned();
                            let r2_reg = self.regions.get(&r2).cloned();
                            (
                                r1_reg.map(|r| r.points).unwrap_or_default(),
                                r2_reg.map(|r| r.points).unwrap_or_default(),
                            )
                        };

                        if let Some(r1_reg) = self.regions.get_mut(&r1) {
                            for p in &points2 {
                                if r1_reg.points.insert(*p) {
                                    changed = true;
                                }
                            }
                        }
                        if let Some(r2_reg) = self.regions.get_mut(&r2) {
                            for p in &points1 {
                                if r2_reg.points.insert(*p) {
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Check for borrow violations
    fn check_violations(&mut self, func: &MirFunction) {
        // Check for conflicting borrows
        for i in 0..self.borrows.len() {
            for j in (i + 1)..self.borrows.len() {
                let b1 = &self.borrows[i];
                let b2 = &self.borrows[j];

                // Check if they borrow the same place
                if b1.borrowed_place.local == b2.borrowed_place.local {
                    // Check if regions overlap
                    if let (Some(r1), Some(r2)) =
                        (self.regions.get(&b1.region), self.regions.get(&b2.region))
                    {
                        let overlap: HashSet<_> = r1.points.intersection(&r2.points).collect();

                        // Conflict if: both mutable, or one mutable and one shared
                        if !overlap.is_empty() && (b1.mutable || b2.mutable) {
                            self.errors.push(NllError::ConflictingBorrow {
                                first_borrow: b1.activation_point,
                                second_borrow: b2.activation_point,
                                local: b1.borrowed_place.local,
                                first_mutable: b1.mutable,
                                second_mutable: b2.mutable,
                            });
                        }
                    }
                }
            }
        }

        // Check for moves while borrowed
        // This requires tracking all moves and checking against active borrows
        for block in &func.blocks {
            for (stmt_idx, inst) in block.instructions.iter().enumerate() {
                let point = LocationPoint::new(block.id, stmt_idx);

                // Check if this instruction moves a borrowed value
                if let Some(moved_local) = get_moved_local(inst) {
                    for borrow in &self.borrows {
                        if borrow.borrowed_place.local == moved_local {
                            if let Some(region) = self.regions.get(&borrow.region) {
                                // If the borrow is still active at this point, error
                                if region.points.contains(&point) {
                                    self.errors.push(NllError::MovedWhileBorrowed {
                                        borrow_point: borrow.activation_point,
                                        move_point: point,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Get errors
    pub fn errors(&self) -> &[NllError] {
        &self.errors
    }
}

/// Check if an instruction moves a local
fn get_moved_local(inst: &MirInstruction) -> Option<usize> {
    match inst {
        MirInstruction::Assign { value, .. } => {
            if let MirRvalue::Use(MirOperand::Move(place)) = value {
                if place.projection.is_empty() {
                    return Some(place.local);
                }
            }
            None
        }
        _ => None,
    }
}

impl Default for NllChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for LivenessInfo {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_point() {
        let p1 = LocationPoint::new(0, 5);
        let p2 = LocationPoint::terminator(0);

        assert_eq!(p1.block, 0);
        assert_eq!(p1.statement, 5);
        assert_eq!(p2.statement, usize::MAX);
    }

    #[test]
    fn test_region_var_display() {
        let rv = RegionVar(42);
        assert_eq!(format!("{}", rv), "'_42");
    }

    #[test]
    fn test_fresh_region() {
        let mut checker = NllChecker::new();
        let r1 = checker.fresh_region();
        let r2 = checker.fresh_region();

        assert_eq!(r1.0, 0);
        assert_eq!(r2.0, 1);
    }

    #[test]
    fn test_liveness_info_creation() {
        let info = LivenessInfo::new();
        assert!(info.live_in.is_empty());
        assert!(info.live_out.is_empty());
    }
}
