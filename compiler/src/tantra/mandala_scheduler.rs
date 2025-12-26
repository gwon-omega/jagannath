//! Maṇḍala Scheduler - Work Distribution (मण्डल)
//!
//! Maṇḍala (circle) represents the cosmos in structured form.
//! The scheduler distributes work across threads in concentric circles:
//!
//! - Center: Critical path (single thread, sequential)
//! - Inner ring: Parallel independent tasks
//! - Middle rings: Dependent task chains
//! - Outer ring: Background/low-priority tasks

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Maṇḍala work scheduler
pub struct MandalaScheduler {
    /// Task rings (0 = center, outer = lower priority)
    rings: Vec<TaskRing>,
    /// Task dependencies
    dependencies: HashMap<TaskId, Vec<TaskId>>,
    /// Task ready queue
    ready_queue: VecDeque<TaskId>,
    /// Completed tasks
    completed: HashSet<TaskId>,
    /// Next task ID
    next_id: AtomicUsize,
    /// Configuration
    config: MandalaConfig,
}

/// Task ring (concentric circle of the maṇḍala)
#[derive(Debug)]
pub struct TaskRing {
    /// Ring level (0 = center)
    pub level: usize,
    /// Tasks in this ring
    pub tasks: Vec<TaskId>,
    /// Ring priority
    pub priority: RingPriority,
    /// Parallelism allowed
    pub max_parallelism: usize,
}

/// Ring priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RingPriority {
    /// Bindu (center point) - Critical, sequential
    Bindu,
    /// Lotus petals - High priority parallel
    Padma,
    /// Square enclosure - Normal parallel
    Bhupura,
    /// Outer circle - Background
    Chakra,
}

/// Task identifier
pub type TaskId = usize;

/// Task definition
#[derive(Debug, Clone)]
pub struct Task {
    pub id: TaskId,
    pub name: String,
    pub ring: RingPriority,
    pub estimated_cost: usize,
    pub dependencies: Vec<TaskId>,
    pub can_parallelize: bool,
}

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Ready,
    Running,
    Completed,
    Failed,
}

/// Scheduler configuration
#[derive(Debug, Clone)]
pub struct MandalaConfig {
    /// Number of worker threads
    pub num_workers: usize,
    /// Enable work stealing?
    pub work_stealing: bool,
    /// Load balance threshold
    pub balance_threshold: f64,
}

impl Default for MandalaConfig {
    fn default() -> Self {
        Self {
            num_workers: num_cpus::get().unwrap_or(4),
            work_stealing: true,
            balance_threshold: 0.2,
        }
    }
}

// Provide num_cpus inline since we might not have the crate
mod num_cpus {
    pub fn get() -> Option<usize> {
        std::thread::available_parallelism()
            .map(|p| p.get())
            .ok()
    }
}

impl MandalaScheduler {
    pub fn new(config: MandalaConfig) -> Self {
        Self {
            rings: vec![
                TaskRing { level: 0, tasks: vec![], priority: RingPriority::Bindu, max_parallelism: 1 },
                TaskRing { level: 1, tasks: vec![], priority: RingPriority::Padma, max_parallelism: config.num_workers },
                TaskRing { level: 2, tasks: vec![], priority: RingPriority::Bhupura, max_parallelism: config.num_workers },
                TaskRing { level: 3, tasks: vec![], priority: RingPriority::Chakra, max_parallelism: config.num_workers / 2 },
            ],
            dependencies: HashMap::new(),
            ready_queue: VecDeque::new(),
            completed: HashSet::new(),
            next_id: AtomicUsize::new(1),
            config,
        }
    }

    /// Add a task to the maṇḍala
    pub fn add_task(&mut self, task: Task) -> TaskId {
        let id = task.id;

        // Store dependencies
        self.dependencies.insert(id, task.dependencies.clone());

        // Assign to appropriate ring
        let ring_idx = match task.ring {
            RingPriority::Bindu => 0,
            RingPriority::Padma => 1,
            RingPriority::Bhupura => 2,
            RingPriority::Chakra => 3,
        };

        self.rings[ring_idx].tasks.push(id);

        // Check if ready (no dependencies)
        if task.dependencies.is_empty() {
            self.ready_queue.push_back(id);
        }

        id
    }

    /// Generate a new task ID
    pub fn new_task_id(&self) -> TaskId {
        self.next_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Mark task as completed
    pub fn complete_task(&mut self, id: TaskId) {
        self.completed.insert(id);

        // Check if any dependent tasks are now ready
        let mut newly_ready = Vec::new();

        for (task_id, deps) in &self.dependencies {
            if !self.completed.contains(task_id) {
                let all_deps_done = deps.iter().all(|d| self.completed.contains(d));
                if all_deps_done && !self.ready_queue.contains(task_id) {
                    newly_ready.push(*task_id);
                }
            }
        }

        for task_id in newly_ready {
            self.ready_queue.push_back(task_id);
        }
    }

    /// Get next task for a worker
    pub fn get_next_task(&mut self, worker_id: usize) -> Option<TaskId> {
        // Process rings from center outward (highest priority first)
        for ring in &self.rings {
            // Check if this ring has ready tasks
            let ring_ready: Vec<_> = ring.tasks.iter()
                .filter(|id| self.ready_queue.contains(id))
                .cloned()
                .collect();

            if !ring_ready.is_empty() {
                // Get task based on worker assignment
                // Bindu ring: only worker 0
                if ring.priority == RingPriority::Bindu && worker_id != 0 {
                    continue;
                }

                // Find and remove from ready queue
                if let Some(pos) = self.ready_queue.iter().position(|id| ring_ready.contains(id)) {
                    return self.ready_queue.remove(pos);
                }
            }
        }

        None
    }

    /// Calculate critical path length
    pub fn critical_path_length(&self) -> usize {
        // DFS to find longest path
        let mut max_length = 0;
        let mut memo: HashMap<TaskId, usize> = HashMap::new();

        for ring in &self.rings {
            for &task_id in &ring.tasks {
                let length = self.dfs_path_length(task_id, &mut memo);
                max_length = max_length.max(length);
            }
        }

        max_length
    }

    fn dfs_path_length(&self, task_id: TaskId, memo: &mut HashMap<TaskId, usize>) -> usize {
        if let Some(&len) = memo.get(&task_id) {
            return len;
        }

        let deps = self.dependencies.get(&task_id).cloned().unwrap_or_default();
        let max_dep_length = deps.iter()
            .map(|&dep| self.dfs_path_length(dep, memo))
            .max()
            .unwrap_or(0);

        let length = max_dep_length + 1;
        memo.insert(task_id, length);
        length
    }

    /// Get parallelism opportunity
    pub fn max_parallelism(&self) -> usize {
        // Count tasks that could run in parallel
        let ready_count = self.ready_queue.len();
        let available_workers = self.config.num_workers;

        ready_count.min(available_workers)
    }

    /// Generate execution schedule
    pub fn generate_schedule(&self) -> Schedule {
        let mut steps = Vec::new();
        let mut remaining: HashSet<TaskId> = self.rings.iter()
            .flat_map(|r| r.tasks.iter().cloned())
            .collect();
        let mut completed: HashSet<TaskId> = HashSet::new();

        while !remaining.is_empty() {
            // Find all tasks that can run (deps satisfied)
            let ready: Vec<TaskId> = remaining.iter()
                .filter(|&id| {
                    self.dependencies.get(id)
                        .map(|deps| deps.iter().all(|d| completed.contains(d)))
                        .unwrap_or(true)
                })
                .cloned()
                .collect();

            if ready.is_empty() {
                break; // Cycle detected or error
            }

            // Group by ring and respect parallelism limits
            let mut step_tasks = Vec::new();
            for ring in &self.rings {
                let ring_tasks: Vec<_> = ready.iter()
                    .filter(|id| ring.tasks.contains(id))
                    .take(ring.max_parallelism)
                    .cloned()
                    .collect();
                step_tasks.extend(ring_tasks);
            }

            // Mark as completed
            for id in &step_tasks {
                remaining.remove(id);
                completed.insert(*id);
            }

            steps.push(ScheduleStep {
                tasks: step_tasks,
                parallelism: self.config.num_workers,
            });
        }

        Schedule { steps }
    }

    /// Get status report
    pub fn report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Maṇḍala Scheduler Report ===\n\n");
        report.push_str(&format!("Workers: {}\n", self.config.num_workers));
        report.push_str(&format!("Critical Path: {} steps\n", self.critical_path_length()));
        report.push_str(&format!("Max Parallelism: {}\n\n", self.max_parallelism()));

        report.push_str("Task Rings:\n");
        for ring in &self.rings {
            report.push_str(&format!("  {:?} (level {}): {} tasks, max {} parallel\n",
                ring.priority, ring.level, ring.tasks.len(), ring.max_parallelism
            ));
        }

        report.push_str(&format!("\nReady: {}, Completed: {}\n",
            self.ready_queue.len(), self.completed.len()
        ));

        report
    }
}

/// Execution schedule
#[derive(Debug)]
pub struct Schedule {
    pub steps: Vec<ScheduleStep>,
}

/// Single step in schedule
#[derive(Debug)]
pub struct ScheduleStep {
    pub tasks: Vec<TaskId>,
    pub parallelism: usize,
}

impl Default for MandalaScheduler {
    fn default() -> Self {
        Self::new(MandalaConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_scheduling() {
        let mut scheduler = MandalaScheduler::default();

        let id1 = scheduler.new_task_id();
        let id2 = scheduler.new_task_id();
        let id3 = scheduler.new_task_id();

        // id3 depends on id1 and id2
        scheduler.add_task(Task {
            id: id1,
            name: "task1".to_string(),
            ring: RingPriority::Padma,
            estimated_cost: 10,
            dependencies: vec![],
            can_parallelize: true,
        });

        scheduler.add_task(Task {
            id: id2,
            name: "task2".to_string(),
            ring: RingPriority::Padma,
            estimated_cost: 10,
            dependencies: vec![],
            can_parallelize: true,
        });

        scheduler.add_task(Task {
            id: id3,
            name: "task3".to_string(),
            ring: RingPriority::Bhupura,
            estimated_cost: 20,
            dependencies: vec![id1, id2],
            can_parallelize: false,
        });

        // task1 and task2 should be ready
        assert!(scheduler.get_next_task(0).is_some());
    }

    #[test]
    fn test_critical_path() {
        let mut scheduler = MandalaScheduler::default();

        let id1 = scheduler.new_task_id();
        let id2 = scheduler.new_task_id();

        scheduler.add_task(Task {
            id: id1,
            name: "task1".to_string(),
            ring: RingPriority::Bindu,
            estimated_cost: 10,
            dependencies: vec![],
            can_parallelize: false,
        });

        scheduler.add_task(Task {
            id: id2,
            name: "task2".to_string(),
            ring: RingPriority::Padma,
            estimated_cost: 10,
            dependencies: vec![id1],
            can_parallelize: true,
        });

        assert_eq!(scheduler.critical_path_length(), 2);
    }
}
