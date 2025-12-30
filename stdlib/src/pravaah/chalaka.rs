//! # Chālaka - Executors (चालक)
//!
//! Async task executors.
//!
//! > **"चालकः कार्यं निर्वहति"**
//! > *"The driver executes the work"*
//!
//! ## Etymology
//! चालक (chālaka) = driver, executor, one who moves

#![allow(unsafe_code)]

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::collections::VecDeque;
#[cfg(feature = "alloc")]
use alloc::sync::Arc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// SIMPLE WAKER (No-op for synchronous execution)
// ============================================================================

/// Create a no-op waker for synchronous execution
fn noop_waker() -> Waker {
    const VTABLE: RawWakerVTable = RawWakerVTable::new(
        |_| RAW_WAKER, // clone
        |_| {},        // wake
        |_| {},        // wake_by_ref
        |_| {},        // drop
    );
    const RAW_WAKER: RawWaker = RawWaker::new(core::ptr::null(), &VTABLE);

    // Safety: The vtable functions are valid and do nothing
    unsafe { Waker::from_raw(RAW_WAKER) }
}

// ============================================================================
// BLOCK ON (Run future to completion)
// ============================================================================

/// Run a future to completion on current thread (अवरुद्ध चालन)
///
/// # Etymology
/// अवरुद्ध (avaruddha) = blocked, obstructed
/// चालन (chālana) = running, execution
pub fn avaruddha_chalana<F, T>(future: F) -> T
where
    F: Future<Output = T>,
{
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);
    let mut future = core::pin::pin!(future);

    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(val) => return val,
            Poll::Pending => {
                // In a real implementation, we'd wait for a wake event
                // For now, just spin (busy wait)
                core::hint::spin_loop();
            }
        }
    }
}

// ============================================================================
// TASK WRAPPER
// ============================================================================

/// A spawned task (जन्मित कार्य)
#[cfg(feature = "alloc")]
pub struct JanmitaKarya<T> {
    id: u64,
    _phantom: core::marker::PhantomData<T>,
}

#[cfg(feature = "alloc")]
impl<T> JanmitaKarya<T> {
    fn new(id: u64) -> Self {
        Self {
            id,
            _phantom: core::marker::PhantomData,
        }
    }

    /// Get task ID (कार्य पहचान)
    pub fn pehchan(&self) -> u64 {
        self.id
    }
}

// ============================================================================
// SIMPLE SINGLE-THREADED EXECUTOR
// ============================================================================

/// Boxed future type
#[cfg(feature = "alloc")]
type BoxedFuture = Pin<Box<dyn Future<Output = ()> + Send>>;

/// Simple single-threaded executor (सरल चालक)
///
/// Executes futures one at a time on the current thread.
#[cfg(feature = "alloc")]
pub struct SaralaChalaka {
    queue: VecDeque<BoxedFuture>,
    task_counter: u64,
}

#[cfg(feature = "alloc")]
impl SaralaChalaka {
    /// Create new executor (नव चालक)
    pub fn nava() -> Self {
        Self {
            queue: VecDeque::new(),
            task_counter: 0,
        }
    }

    /// Spawn a task (कार्य जन्म)
    pub fn janma<F>(&mut self, future: F) -> JanmitaKarya<F::Output>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.task_counter += 1;
        let id = self.task_counter;
        self.queue.push_back(Box::pin(future));
        JanmitaKarya::new(id)
    }

    /// Run all tasks to completion (सर्व चालन)
    pub fn sarva_chalana(&mut self) {
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        while let Some(mut future) = self.queue.pop_front() {
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(()) => {
                    // Task completed
                }
                Poll::Pending => {
                    // Re-queue for later
                    self.queue.push_back(future);
                }
            }
        }
    }

    /// Run until queue is empty or limit reached (सीमित चालन)
    pub fn simita_chalana(&mut self, iterations: usize) {
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        for _ in 0..iterations {
            if let Some(mut future) = self.queue.pop_front() {
                match future.as_mut().poll(&mut cx) {
                    Poll::Ready(()) => {}
                    Poll::Pending => {
                        self.queue.push_back(future);
                    }
                }
            } else {
                break;
            }
        }
    }

    /// Check if executor is empty (रिक्त जाँच)
    pub fn rikta(&self) -> bool {
        self.queue.is_empty()
    }

    /// Get number of pending tasks (लंबित संख्या)
    pub fn lambit_sankhya(&self) -> usize {
        self.queue.len()
    }
}

// ============================================================================
// ROUND-ROBIN EXECUTOR
// ============================================================================

/// Round-robin executor with fairness (चक्रीय चालक)
///
/// Gives each task equal opportunity to make progress.
#[cfg(feature = "alloc")]
pub struct ChakriyaChalaka {
    tasks: Vec<Option<BoxedFuture>>,
    current: usize,
    task_counter: u64,
}

#[cfg(feature = "alloc")]
impl ChakriyaChalaka {
    /// Create new round-robin executor
    pub fn nava() -> Self {
        Self {
            tasks: Vec::new(),
            current: 0,
            task_counter: 0,
        }
    }

    /// Spawn a task
    pub fn janma<F>(&mut self, future: F) -> JanmitaKarya<F::Output>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.task_counter += 1;
        let id = self.task_counter;

        // Find empty slot or add new
        let slot = self.tasks.iter().position(|t| t.is_none());
        match slot {
            Some(idx) => self.tasks[idx] = Some(Box::pin(future)),
            None => self.tasks.push(Some(Box::pin(future))),
        }

        JanmitaKarya::new(id)
    }

    /// Poll one task (एक चालन)
    pub fn eka_chalana(&mut self) -> bool {
        if self.tasks.is_empty() || self.tasks.iter().all(|t| t.is_none()) {
            return false;
        }

        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        // Find next task
        let start = self.current;
        loop {
            if let Some(ref mut future) = self.tasks[self.current] {
                match future.as_mut().poll(&mut cx) {
                    Poll::Ready(()) => {
                        self.tasks[self.current] = None;
                    }
                    Poll::Pending => {}
                }
                self.current = (self.current + 1) % self.tasks.len();
                return true;
            }

            self.current = (self.current + 1) % self.tasks.len();
            if self.current == start {
                break;
            }
        }

        false
    }

    /// Run all tasks to completion
    pub fn sarva_chalana(&mut self) {
        while self.eka_chalana() {}
    }

    /// Get active task count
    pub fn sakriya_sankhya(&self) -> usize {
        self.tasks.iter().filter(|t| t.is_some()).count()
    }
}

// ============================================================================
// PRIORITY EXECUTOR
// ============================================================================

/// Task with priority (प्राथमिकता कार्य)
#[cfg(feature = "alloc")]
struct PrathamikataKarya {
    priority: u32,
    future: BoxedFuture,
}

/// Priority-based executor (प्राथमिकता चालक)
///
/// Executes higher priority tasks first.
#[cfg(feature = "alloc")]
pub struct PrathamikataChalaka {
    tasks: Vec<PrathamikataKarya>,
    task_counter: u64,
}

#[cfg(feature = "alloc")]
impl PrathamikataChalaka {
    /// Create new priority executor
    pub fn nava() -> Self {
        Self {
            tasks: Vec::new(),
            task_counter: 0,
        }
    }

    /// Spawn with priority (higher = more important)
    pub fn janma<F>(&mut self, future: F, priority: u32) -> JanmitaKarya<F::Output>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.task_counter += 1;
        let id = self.task_counter;

        let task = PrathamikataKarya {
            priority,
            future: Box::pin(future),
        };

        // Insert in sorted order (highest priority first)
        let pos = self.tasks.iter().position(|t| t.priority < priority);
        match pos {
            Some(idx) => self.tasks.insert(idx, task),
            None => self.tasks.push(task),
        }

        JanmitaKarya::new(id)
    }

    /// Poll highest priority task
    pub fn eka_chalana(&mut self) -> bool {
        if self.tasks.is_empty() {
            return false;
        }

        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        // Poll first (highest priority) task
        let completed = {
            let task = &mut self.tasks[0];
            matches!(task.future.as_mut().poll(&mut cx), Poll::Ready(()))
        };

        if completed {
            self.tasks.remove(0);
        }

        true
    }

    /// Run all tasks
    pub fn sarva_chalana(&mut self) {
        while !self.tasks.is_empty() {
            self.eka_chalana();
        }
    }
}

// ============================================================================
// LOCAL EXECUTOR (for thread-local tasks)
// ============================================================================

/// Local executor for non-Send futures (स्थानीय चालक)
#[cfg(feature = "alloc")]
pub struct SthaniyaChalaka {
    queue: VecDeque<Pin<Box<dyn Future<Output = ()>>>>,
}

#[cfg(feature = "alloc")]
impl SthaniyaChalaka {
    pub fn nava() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    /// Spawn a local (non-Send) task
    pub fn janma<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        self.queue.push_back(Box::pin(future));
    }

    /// Poll once
    pub fn eka_chalana(&mut self) -> bool {
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        if let Some(mut future) = self.queue.pop_front() {
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(()) => {}
                Poll::Pending => {
                    self.queue.push_back(future);
                }
            }
            true
        } else {
            false
        }
    }

    /// Run all
    pub fn sarva_chalana(&mut self) {
        while self.eka_chalana() {}
    }
}

// ============================================================================
// SPAWN FUNCTION (for use with executor)
// ============================================================================

/// Spawn helper that returns a future wrapper
#[cfg(feature = "alloc")]
pub fn spawn_wrapper<F, T>(future: F) -> impl Future<Output = ()>
where
    F: Future<Output = T>,
    T: 'static,
{
    async move {
        let _ = future.await;
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_on_ready() {
        let result = avaruddha_chalana(async { 42 });
        assert_eq!(result, 42);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_simple_executor() {
        use core::sync::atomic::{AtomicUsize, Ordering};
        static COUNTER: AtomicUsize = AtomicUsize::new(0);

        let mut executor = SaralaChalaka::nava();

        executor.janma(async {
            COUNTER.fetch_add(1, Ordering::SeqCst);
        });

        executor.janma(async {
            COUNTER.fetch_add(2, Ordering::SeqCst);
        });

        executor.sarva_chalana();

        assert_eq!(COUNTER.load(Ordering::SeqCst), 3);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_round_robin_executor() {
        let mut executor = ChakriyaChalaka::nava();

        executor.janma(async {});
        executor.janma(async {});

        assert_eq!(executor.sakriya_sankhya(), 2);

        executor.sarva_chalana();

        assert_eq!(executor.sakriya_sankhya(), 0);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_priority_executor() {
        use core::sync::atomic::{AtomicU32, Ordering};
        static ORDER: AtomicU32 = AtomicU32::new(0);

        let mut executor = PrathamikataChalaka::nava();

        // Lower priority
        executor.janma(
            async {
                let order = ORDER.fetch_add(1, Ordering::SeqCst);
                assert_eq!(order, 1); // Should be second
            },
            1,
        );

        // Higher priority
        executor.janma(
            async {
                let order = ORDER.fetch_add(1, Ordering::SeqCst);
                assert_eq!(order, 0); // Should be first
            },
            10,
        );

        executor.sarva_chalana();
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_local_executor() {
        let mut executor = SthaniyaChalaka::nava();

        // Can spawn non-Send futures
        let local_data = core::cell::RefCell::new(0);
        executor.janma(async {
            // This wouldn't work with Send requirement
        });

        executor.sarva_chalana();
    }
}
