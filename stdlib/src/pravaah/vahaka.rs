//! # Vāhaka - Channels (वाहक)
//!
//! Async communication channels for message passing.
//!
//! > **"वाहकः सन्देशं वहति"**
//! > *"The carrier carries the message"*
//!
//! ## Etymology
//! वाहक (vāhaka) = carrier, conveyor, channel

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};

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

#[cfg(feature = "std")]
use std::sync::Mutex;

// ============================================================================
// CHANNEL ERRORS
// ============================================================================

/// Send error (प्रेषण त्रुटि)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreshanaDosha<T> {
    /// The value that failed to send
    pub vastu: T,
}

/// Receive error (ग्रहण त्रुटि)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrahanaDosha {
    /// Channel is empty (रिक्त)
    Rikta,
    /// Channel is disconnected (विच्छिन्न)
    Vichchhinna,
}

/// Try receive error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrayasGrahanaDosha {
    /// Would block (अवरुद्ध)
    Avaruddha,
    /// Disconnected (विच्छिन्न)
    Vichchhinna,
}

// ============================================================================
// ONESHOT CHANNEL
// ============================================================================

/// One-time channel state
#[cfg(feature = "std")]
struct EkakalikaState<T> {
    value: Option<T>,
    sender_dropped: bool,
    receiver_dropped: bool,
    waker: Option<Waker>,
}

/// Oneshot sender (एककालिक प्रेषक)
#[cfg(feature = "std")]
pub struct EkakalikaPreshaka<T> {
    state: Arc<Mutex<EkakalikaState<T>>>,
}

/// Oneshot receiver (एककालिक ग्राहक)
#[cfg(feature = "std")]
pub struct EkakalikaGrahaka<T> {
    state: Arc<Mutex<EkakalikaState<T>>>,
}

/// Create oneshot channel (एककालिक वाहक)
#[cfg(feature = "std")]
pub fn ekakalika<T>() -> (EkakalikaPreshaka<T>, EkakalikaGrahaka<T>) {
    let state = Arc::new(Mutex::new(EkakalikaState {
        value: None,
        sender_dropped: false,
        receiver_dropped: false,
        waker: None,
    }));

    (
        EkakalikaPreshaka {
            state: state.clone(),
        },
        EkakalikaGrahaka { state },
    )
}

#[cfg(feature = "std")]
impl<T> EkakalikaPreshaka<T> {
    /// Send value (प्रेषण)
    pub fn preshana(self, vastu: T) -> Result<(), PreshanaDosha<T>> {
        let mut state = self.state.lock().unwrap();

        if state.receiver_dropped {
            return Err(PreshanaDosha { vastu });
        }

        state.value = Some(vastu);

        if let Some(waker) = state.waker.take() {
            waker.wake();
        }

        Ok(())
    }

    /// Check if receiver is still connected
    pub fn sandhit(&self) -> bool {
        !self.state.lock().unwrap().receiver_dropped
    }
}

#[cfg(feature = "std")]
impl<T> Drop for EkakalikaPreshaka<T> {
    fn drop(&mut self) {
        let mut state = self.state.lock().unwrap();
        state.sender_dropped = true;
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
    }
}

#[cfg(feature = "std")]
impl<T> Future for EkakalikaGrahaka<T> {
    type Output = Result<T, GrahanaDosha>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();

        if let Some(value) = state.value.take() {
            return Poll::Ready(Ok(value));
        }

        if state.sender_dropped {
            return Poll::Ready(Err(GrahanaDosha::Vichchhinna));
        }

        state.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

#[cfg(feature = "std")]
impl<T> Drop for EkakalikaGrahaka<T> {
    fn drop(&mut self) {
        self.state.lock().unwrap().receiver_dropped = true;
    }
}

// ============================================================================
// BOUNDED CHANNEL
// ============================================================================

/// Bounded channel state
#[cfg(feature = "std")]
struct SimaitaState<T> {
    queue: VecDeque<T>,
    capacity: usize,
    sender_count: usize,
    receiver_alive: bool,
    send_wakers: Vec<Waker>,
    recv_waker: Option<Waker>,
}

/// Bounded sender (सीमित प्रेषक)
#[cfg(feature = "std")]
pub struct SimaitaPreshaka<T> {
    state: Arc<Mutex<SimaitaState<T>>>,
}

/// Bounded receiver (सीमित ग्राहक)
#[cfg(feature = "std")]
pub struct SimaitaGrahaka<T> {
    state: Arc<Mutex<SimaitaState<T>>>,
}

/// Create bounded channel (सीमित वाहक)
#[cfg(feature = "std")]
pub fn simaita<T>(capacity: usize) -> (SimaitaPreshaka<T>, SimaitaGrahaka<T>) {
    let state = Arc::new(Mutex::new(SimaitaState {
        queue: VecDeque::with_capacity(capacity),
        capacity,
        sender_count: 1,
        receiver_alive: true,
        send_wakers: Vec::new(),
        recv_waker: None,
    }));

    (
        SimaitaPreshaka {
            state: state.clone(),
        },
        SimaitaGrahaka { state },
    )
}

#[cfg(feature = "std")]
impl<T> SimaitaPreshaka<T> {
    /// Try to send without blocking
    pub fn prayas_preshana(&self, vastu: T) -> Result<(), PreshanaDosha<T>> {
        let mut state = self.state.lock().unwrap();

        if !state.receiver_alive {
            return Err(PreshanaDosha { vastu });
        }

        if state.queue.len() >= state.capacity {
            return Err(PreshanaDosha { vastu });
        }

        state.queue.push_back(vastu);

        if let Some(waker) = state.recv_waker.take() {
            waker.wake();
        }

        Ok(())
    }

    /// Create async send future
    pub fn preshana(&self, vastu: T) -> SimaitaPreshanaFuture<T> {
        SimaitaPreshanaFuture {
            state: self.state.clone(),
            value: Some(vastu),
        }
    }

    /// Check if receiver is alive
    pub fn sandhit(&self) -> bool {
        self.state.lock().unwrap().receiver_alive
    }
}

#[cfg(feature = "std")]
impl<T> Clone for SimaitaPreshaka<T> {
    fn clone(&self) -> Self {
        self.state.lock().unwrap().sender_count += 1;
        Self {
            state: self.state.clone(),
        }
    }
}

#[cfg(feature = "std")]
impl<T> Drop for SimaitaPreshaka<T> {
    fn drop(&mut self) {
        let mut state = self.state.lock().unwrap();
        state.sender_count -= 1;

        if state.sender_count == 0 {
            if let Some(waker) = state.recv_waker.take() {
                waker.wake();
            }
        }
    }
}

/// Send future for bounded channel
#[cfg(feature = "std")]
pub struct SimaitaPreshanaFuture<T> {
    state: Arc<Mutex<SimaitaState<T>>>,
    value: Option<T>,
}

// Allow mutable access through Pin
#[cfg(feature = "std")]
impl<T> Unpin for SimaitaPreshanaFuture<T> {}

#[cfg(feature = "std")]
impl<T> Future for SimaitaPreshanaFuture<T> {
    type Output = Result<(), PreshanaDosha<T>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Clone Arc to avoid borrow conflict
        let state_arc = self.state.clone();
        let mut state = state_arc.lock().unwrap();

        if !state.receiver_alive {
            let vastu = self.value.take().unwrap();
            return Poll::Ready(Err(PreshanaDosha { vastu }));
        }

        if state.queue.len() < state.capacity {
            let vastu = self.value.take().unwrap();
            state.queue.push_back(vastu);

            if let Some(waker) = state.recv_waker.take() {
                waker.wake();
            }

            return Poll::Ready(Ok(()));
        }

        state.send_wakers.push(cx.waker().clone());
        Poll::Pending
    }
}

#[cfg(feature = "std")]
impl<T> SimaitaGrahaka<T> {
    /// Try to receive without blocking
    pub fn prayas_grahana(&self) -> Result<T, PrayasGrahanaDosha> {
        let mut state = self.state.lock().unwrap();

        if let Some(value) = state.queue.pop_front() {
            // Wake a waiting sender
            if let Some(waker) = state.send_wakers.pop() {
                waker.wake();
            }
            return Ok(value);
        }

        if state.sender_count == 0 {
            return Err(PrayasGrahanaDosha::Vichchhinna);
        }

        Err(PrayasGrahanaDosha::Avaruddha)
    }

    /// Create async receive future
    pub fn grahana(&self) -> SimaitaGrahanaFuture<T> {
        SimaitaGrahanaFuture {
            state: self.state.clone(),
        }
    }
}

#[cfg(feature = "std")]
impl<T> Drop for SimaitaGrahaka<T> {
    fn drop(&mut self) {
        let mut state = self.state.lock().unwrap();
        state.receiver_alive = false;

        // Wake all waiting senders
        for waker in state.send_wakers.drain(..) {
            waker.wake();
        }
    }
}

/// Receive future for bounded channel
#[cfg(feature = "std")]
pub struct SimaitaGrahanaFuture<T> {
    state: Arc<Mutex<SimaitaState<T>>>,
}

#[cfg(feature = "std")]
impl<T> Future for SimaitaGrahanaFuture<T> {
    type Output = Result<T, GrahanaDosha>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();

        if let Some(value) = state.queue.pop_front() {
            // Wake a waiting sender
            if let Some(waker) = state.send_wakers.pop() {
                waker.wake();
            }
            return Poll::Ready(Ok(value));
        }

        if state.sender_count == 0 {
            return Poll::Ready(Err(GrahanaDosha::Vichchhinna));
        }

        state.recv_waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

// ============================================================================
// UNBOUNDED CHANNEL
// ============================================================================

/// Unbounded channel state
#[cfg(feature = "std")]
struct AsimaitaState<T> {
    queue: VecDeque<T>,
    sender_count: usize,
    receiver_alive: bool,
    recv_waker: Option<Waker>,
}

/// Unbounded sender (असीमित प्रेषक)
#[cfg(feature = "std")]
pub struct AsimaitaPreshaka<T> {
    state: Arc<Mutex<AsimaitaState<T>>>,
}

/// Unbounded receiver (असीमित ग्राहक)
#[cfg(feature = "std")]
pub struct AsimaitaGrahaka<T> {
    state: Arc<Mutex<AsimaitaState<T>>>,
}

/// Create unbounded channel (असीमित वाहक)
#[cfg(feature = "std")]
pub fn asimaita<T>() -> (AsimaitaPreshaka<T>, AsimaitaGrahaka<T>) {
    let state = Arc::new(Mutex::new(AsimaitaState {
        queue: VecDeque::new(),
        sender_count: 1,
        receiver_alive: true,
        recv_waker: None,
    }));

    (
        AsimaitaPreshaka {
            state: state.clone(),
        },
        AsimaitaGrahaka { state },
    )
}

#[cfg(feature = "std")]
impl<T> AsimaitaPreshaka<T> {
    /// Send value (never blocks, can OOM)
    pub fn preshana(&self, vastu: T) -> Result<(), PreshanaDosha<T>> {
        let mut state = self.state.lock().unwrap();

        if !state.receiver_alive {
            return Err(PreshanaDosha { vastu });
        }

        state.queue.push_back(vastu);

        if let Some(waker) = state.recv_waker.take() {
            waker.wake();
        }

        Ok(())
    }

    /// Get queue length
    pub fn lambai(&self) -> usize {
        self.state.lock().unwrap().queue.len()
    }
}

#[cfg(feature = "std")]
impl<T> Clone for AsimaitaPreshaka<T> {
    fn clone(&self) -> Self {
        self.state.lock().unwrap().sender_count += 1;
        Self {
            state: self.state.clone(),
        }
    }
}

#[cfg(feature = "std")]
impl<T> Drop for AsimaitaPreshaka<T> {
    fn drop(&mut self) {
        let mut state = self.state.lock().unwrap();
        state.sender_count -= 1;

        if state.sender_count == 0 {
            if let Some(waker) = state.recv_waker.take() {
                waker.wake();
            }
        }
    }
}

#[cfg(feature = "std")]
impl<T> AsimaitaGrahaka<T> {
    /// Try to receive
    pub fn prayas_grahana(&self) -> Result<T, PrayasGrahanaDosha> {
        let mut state = self.state.lock().unwrap();

        if let Some(value) = state.queue.pop_front() {
            return Ok(value);
        }

        if state.sender_count == 0 {
            return Err(PrayasGrahanaDosha::Vichchhinna);
        }

        Err(PrayasGrahanaDosha::Avaruddha)
    }

    /// Create async receive future
    pub fn grahana(&self) -> AsimaitaGrahanaFuture<T> {
        AsimaitaGrahanaFuture {
            state: self.state.clone(),
        }
    }
}

#[cfg(feature = "std")]
impl<T> Drop for AsimaitaGrahaka<T> {
    fn drop(&mut self) {
        self.state.lock().unwrap().receiver_alive = false;
    }
}

/// Receive future for unbounded channel
#[cfg(feature = "std")]
pub struct AsimaitaGrahanaFuture<T> {
    state: Arc<Mutex<AsimaitaState<T>>>,
}

#[cfg(feature = "std")]
impl<T> Future for AsimaitaGrahanaFuture<T> {
    type Output = Result<T, GrahanaDosha>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();

        if let Some(value) = state.queue.pop_front() {
            return Poll::Ready(Ok(value));
        }

        if state.sender_count == 0 {
            return Poll::Ready(Err(GrahanaDosha::Vichchhinna));
        }

        state.recv_waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

// ============================================================================
// WATCH CHANNEL (Broadcast last value)
// ============================================================================

/// Watch channel state
#[cfg(feature = "std")]
struct DrishtiState<T: Clone> {
    value: Option<T>,
    version: u64,
    sender_alive: bool,
    wakers: Vec<(u64, Waker)>, // (last_seen_version, waker)
}

/// Watch sender (दृष्टि प्रेषक)
#[cfg(feature = "std")]
pub struct DrishtiPreshaka<T: Clone> {
    state: Arc<Mutex<DrishtiState<T>>>,
}

/// Watch receiver (दृष्टि ग्राहक)
#[cfg(feature = "std")]
pub struct DrishtiGrahaka<T: Clone> {
    state: Arc<Mutex<DrishtiState<T>>>,
    last_version: u64,
}

/// Create watch channel (दृष्टि वाहक)
#[cfg(feature = "std")]
pub fn drishti<T: Clone>(initial: T) -> (DrishtiPreshaka<T>, DrishtiGrahaka<T>) {
    let state = Arc::new(Mutex::new(DrishtiState {
        value: Some(initial),
        version: 0,
        sender_alive: true,
        wakers: Vec::new(),
    }));

    (
        DrishtiPreshaka {
            state: state.clone(),
        },
        DrishtiGrahaka {
            state,
            last_version: 0,
        },
    )
}

#[cfg(feature = "std")]
impl<T: Clone> DrishtiPreshaka<T> {
    /// Send/update value
    pub fn preshana(&self, vastu: T) {
        let mut state = self.state.lock().unwrap();
        state.value = Some(vastu);
        state.version += 1;

        // Wake all receivers
        for (_, waker) in state.wakers.drain(..) {
            waker.wake();
        }
    }

    /// Get current value
    pub fn borrow(&self) -> Option<T> {
        self.state.lock().unwrap().value.clone()
    }
}

#[cfg(feature = "std")]
impl<T: Clone> Drop for DrishtiPreshaka<T> {
    fn drop(&mut self) {
        let mut state = self.state.lock().unwrap();
        state.sender_alive = false;
        for (_, waker) in state.wakers.drain(..) {
            waker.wake();
        }
    }
}

#[cfg(feature = "std")]
impl<T: Clone> DrishtiGrahaka<T> {
    /// Get current value without waiting for change
    pub fn borrow(&self) -> Option<T> {
        let state = self.state.lock().unwrap();
        self.last_version; // Track that we've seen this version
        state.value.clone()
    }

    /// Wait for value to change
    pub fn parivartan(&mut self) -> DrishtiParivartanFuture<T> {
        DrishtiParivartanFuture {
            state: self.state.clone(),
            last_version: self.last_version,
        }
    }

    /// Check if sender is still alive
    pub fn sandhit(&self) -> bool {
        self.state.lock().unwrap().sender_alive
    }
}

#[cfg(feature = "std")]
impl<T: Clone> Clone for DrishtiGrahaka<T> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            last_version: self.last_version,
        }
    }
}

/// Future for watching value changes
#[cfg(feature = "std")]
pub struct DrishtiParivartanFuture<T: Clone> {
    state: Arc<Mutex<DrishtiState<T>>>,
    last_version: u64,
}

// Allow mutable access through Pin
#[cfg(feature = "std")]
impl<T: Clone> Unpin for DrishtiParivartanFuture<T> {}

#[cfg(feature = "std")]
impl<T: Clone> Future for DrishtiParivartanFuture<T> {
    type Output = Result<T, GrahanaDosha>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let state = self.state.lock().unwrap();

        if state.version > self.last_version {
            let new_version = state.version;
            let value = state.value.clone();
            let sender_alive = state.sender_alive;
            drop(state); // Release lock before modifying self

            self.last_version = new_version;
            if let Some(v) = value {
                return Poll::Ready(Ok(v));
            }

            if !sender_alive {
                return Poll::Ready(Err(GrahanaDosha::Vichchhinna));
            }

            // Re-acquire lock to add waker
            let mut state = self.state.lock().unwrap();
            state.wakers.push((self.last_version, cx.waker().clone()));
            return Poll::Pending;
        }

        if !state.sender_alive {
            return Poll::Ready(Err(GrahanaDosha::Vichchhinna));
        }

        let last_ver = self.last_version;
        let mut state = state; // Rebind to allow push
        state.wakers.push((last_ver, cx.waker().clone()));
        Poll::Pending
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    #[test]
    fn test_oneshot_basic() {
        let (tx, rx) = ekakalika();

        assert!(tx.preshana(42).is_ok());

        // Would need executor to await rx
    }

    #[test]
    fn test_bounded_try_send() {
        let (tx, rx) = simaita(2);

        assert!(tx.prayas_preshana(1).is_ok());
        assert!(tx.prayas_preshana(2).is_ok());
        assert!(tx.prayas_preshana(3).is_err()); // Full

        assert_eq!(rx.prayas_grahana().unwrap(), 1);
        assert!(tx.prayas_preshana(3).is_ok()); // Space now
    }

    #[test]
    fn test_unbounded_basic() {
        let (tx, rx) = asimaita();

        for i in 0..100 {
            assert!(tx.preshana(i).is_ok());
        }

        assert_eq!(tx.lambai(), 100);

        for i in 0..100 {
            assert_eq!(rx.prayas_grahana().unwrap(), i);
        }
    }

    #[test]
    fn test_watch_basic() {
        let (tx, rx) = drishti(0);

        assert_eq!(rx.borrow(), Some(0));

        tx.preshana(42);
        assert_eq!(rx.borrow(), Some(42));

        tx.preshana(100);
        assert_eq!(rx.borrow(), Some(100));
    }

    #[test]
    fn test_channel_disconnect() {
        let (tx, rx) = simaita::<i32>(1);
        drop(rx);

        assert!(tx.prayas_preshana(1).is_err());
        assert!(!tx.sandhit());
    }

    #[test]
    fn test_sender_disconnect() {
        let (tx, rx) = simaita::<i32>(1);
        drop(tx);

        assert!(matches!(
            rx.prayas_grahana(),
            Err(PrayasGrahanaDosha::Vichchhinna)
        ));
    }
}
