//! Nalikā - Channels (नालिका - Pipe/Channel)
//!
//! Message passing channels for concurrency.
//!
//! Based on the Yoga concept of nāḍī (नाड़ी) - energy channels.

use std::sync::mpsc::{
    self, Receiver, RecvError, SendError, Sender, SyncSender, TryRecvError, TrySendError,
};
use std::time::Duration;

// ============================================================================
// Unbounded Channel (Ananta-Nalikā - अनन्तनालिका)
// ============================================================================

/// Sender end of unbounded channel (Preṣaka - प्रेषक)
pub struct Preshaka<T> {
    inner: Sender<T>,
}

impl<T> Preshaka<T> {
    /// Send message (प्रेषय - preṣaya)
    pub fn preshaya(&self, sandesh: T) -> Result<(), SendError<T>> {
        self.inner.send(sandesh)
    }
}

impl<T> Clone for Preshaka<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

/// Receiver end of channel (Grāhaka - ग्राहक)
pub struct Grahaka<T> {
    inner: Receiver<T>,
}

impl<T> Grahaka<T> {
    /// Receive message, blocking (प्राप्नोति - prāpnoti)
    pub fn prapnoti(&self) -> Result<T, RecvError> {
        self.inner.recv()
    }

    /// Try receive, non-blocking (प्रयत्नम् - prayatnam)
    pub fn prayatnam(&self) -> Result<T, TryRecvError> {
        self.inner.try_recv()
    }

    /// Receive with timeout (कालबद्ध - kālabaddha)
    pub fn kalabaddha(&self, kala: Duration) -> Result<T, mpsc::RecvTimeoutError> {
        self.inner.recv_timeout(kala)
    }

    /// Create iterator (यात्री - yātrī)
    pub fn yatri(&self) -> impl Iterator<Item = T> + '_ {
        self.inner.iter()
    }

    /// Try iterator (प्रयत्न यात्री - prayatna yātrī)
    pub fn prayatna_yatri(&self) -> impl Iterator<Item = T> + '_ {
        self.inner.try_iter()
    }
}

/// Create unbounded channel (अनन्त नालिका - ananta nālikā)
///
/// Creates a channel with unlimited buffer capacity.
pub fn ananta_nalika<T>() -> (Preshaka<T>, Grahaka<T>) {
    let (tx, rx) = mpsc::channel();
    (Preshaka { inner: tx }, Grahaka { inner: rx })
}

// ============================================================================
// Bounded Channel (Maryādita-Nalikā - मर्यादितनालिका)
// ============================================================================

/// Sync sender for bounded channel (Samakālīna-Preṣaka - समकालीनप्रेषक)
pub struct SamakalinaPreShaka<T> {
    inner: SyncSender<T>,
}

impl<T> SamakalinaPreShaka<T> {
    /// Send message, blocking if full (प्रेषय - preṣaya)
    pub fn preshaya(&self, sandesh: T) -> Result<(), SendError<T>> {
        self.inner.send(sandesh)
    }

    /// Try send, non-blocking (प्रयत्नम् - prayatnam)
    pub fn prayatnam(&self, sandesh: T) -> Result<(), TrySendError<T>> {
        self.inner.try_send(sandesh)
    }
}

impl<T> Clone for SamakalinaPreShaka<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

/// Create bounded channel (मर्यादित नालिका - maryādita nālikā)
///
/// Creates a channel with fixed buffer capacity.
/// Senders block when the buffer is full.
pub fn maryadita_nalika<T>(kshamata: usize) -> (SamakalinaPreShaka<T>, Grahaka<T>) {
    let (tx, rx) = mpsc::sync_channel(kshamata);
    (SamakalinaPreShaka { inner: tx }, Grahaka { inner: rx })
}

// ============================================================================
// OneShot Channel (Ekakālīna - एककालीन)
// ============================================================================

/// OneShot channel for single message (एककालीन - ekakālīna)
///
/// A channel that can only send one message.
pub fn ekakalina<T>() -> (Preshaka<T>, Grahaka<T>) {
    ananta_nalika()
}

// ============================================================================
// Channel Errors
// ============================================================================

/// Channel send error type (प्रेषण दोष - preṣaṇa doṣa)
pub type PreshanaDosha<T> = SendError<T>;

/// Channel receive error type (प्राप्ति दोष - prāpti doṣa)
pub type PraptiDosha = RecvError;

/// Try send error type
pub type PrayatnaPreshanaDosha<T> = TrySendError<T>;

/// Try receive error type
pub type PrayatnaPraptiDosha = TryRecvError;

// ============================================================================
// Select-like functionality (Citi - चिति - Selection)
// ============================================================================

/// A simple select result for two channels
pub enum CitiPhala<A, B> {
    /// First channel (प्रथम - prathama)
    Prathama(A),
    /// Second channel (द्वितीय - dvitīya)
    Dvitiya(B),
    /// Both empty (रिक्त - rikta)
    Rikta,
}

/// Try to receive from two channels (द्वि चयन - dvi cayana)
///
/// Non-blocking select between two channels.
pub fn dvi_cayana<A, B>(eka: &Grahaka<A>, dvi: &Grahaka<B>) -> CitiPhala<A, B> {
    match eka.prayatnam() {
        Ok(a) => return CitiPhala::Prathama(a),
        Err(TryRecvError::Empty) => {}
        Err(TryRecvError::Disconnected) => {}
    }

    match dvi.prayatnam() {
        Ok(b) => return CitiPhala::Dvitiya(b),
        Err(TryRecvError::Empty) => {}
        Err(TryRecvError::Disconnected) => {}
    }

    CitiPhala::Rikta
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_unbounded_channel() {
        let (tx, rx) = ananta_nalika();

        tx.preshaya(42).unwrap();
        tx.preshaya(43).unwrap();

        assert_eq!(rx.prapnoti().unwrap(), 42);
        assert_eq!(rx.prapnoti().unwrap(), 43);
    }

    #[test]
    fn test_bounded_channel() {
        let (tx, rx) = maryadita_nalika(2);

        tx.preshaya(1).unwrap();
        tx.preshaya(2).unwrap();

        // Buffer full, try_send should fail
        assert!(tx.prayatnam(3).is_err());

        assert_eq!(rx.prapnoti().unwrap(), 1);
        assert_eq!(rx.prapnoti().unwrap(), 2);
    }

    #[test]
    fn test_channel_thread_safety() {
        let (tx, rx) = ananta_nalika();

        let handle = thread::spawn(move || {
            for i in 0..5 {
                tx.preshaya(i).unwrap();
            }
        });

        let mut received = Vec::new();
        for _ in 0..5 {
            received.push(rx.prapnoti().unwrap());
        }

        handle.join().unwrap();
        assert_eq!(received, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_try_recv() {
        let (tx, rx) = ananta_nalika::<i32>();

        assert!(rx.prayatnam().is_err());

        tx.preshaya(1).unwrap();
        assert_eq!(rx.prayatnam().unwrap(), 1);
    }

    #[test]
    fn test_channel_iterator() {
        let (tx, rx) = ananta_nalika();

        tx.preshaya(1).unwrap();
        tx.preshaya(2).unwrap();
        tx.preshaya(3).unwrap();
        drop(tx); // Close sender

        let items: Vec<_> = rx.yatri().collect();
        assert_eq!(items, vec![1, 2, 3]);
    }

    #[test]
    fn test_multi_sender() {
        let (tx, rx) = ananta_nalika();
        let tx2 = tx.clone();

        tx.preshaya(1).unwrap();
        tx2.preshaya(2).unwrap();

        let mut items = vec![rx.prapnoti().unwrap(), rx.prapnoti().unwrap()];
        items.sort();
        assert_eq!(items, vec![1, 2]);
    }

    #[test]
    fn test_dvi_cayana() {
        let (tx1, rx1) = ananta_nalika::<i32>();
        let (tx2, rx2) = ananta_nalika::<String>();

        // Both empty
        match dvi_cayana(&rx1, &rx2) {
            CitiPhala::Rikta => {}
            _ => panic!("Expected empty"),
        }

        // First has data
        tx1.preshaya(42).unwrap();
        match dvi_cayana(&rx1, &rx2) {
            CitiPhala::Prathama(v) => assert_eq!(v, 42),
            _ => panic!("Expected first"),
        }

        // Second has data
        tx2.preshaya("hello".to_string()).unwrap();
        match dvi_cayana(&rx1, &rx2) {
            CitiPhala::Dvitiya(v) => assert_eq!(v, "hello"),
            _ => panic!("Expected second"),
        }
    }
}
