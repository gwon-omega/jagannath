//! # Dhārā - Streams (धारा)
//!
//! Async stream types and combinators.
//!
//! > **"धारा नदी इव प्रवहति"**
//! > *"The stream flows like a river"*
//!
//! ## Etymology
//! धारा (dhārā) = stream, flow, current

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// STREAM TRAIT
// ============================================================================

/// Async stream trait (धारा लक्षण)
///
/// Like Iterator but asynchronous - yields items over time.
pub trait Dhara {
    /// Item type yielded by the stream
    type Vastu;

    /// Poll for next item (अग्रिम मतदान)
    fn agrim_matadana(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Vastu>>;

    /// Get size hint (आकार संकेत)
    fn akar_sanketa(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

// ============================================================================
// STREAM EXTENSION TRAIT
// ============================================================================

/// Extension methods for streams (धारा विस्तार)
pub trait DharaVistar: Dhara + Sized {
    /// Map over stream items
    fn rupantarana<F, B>(self, f: F) -> Rupantarana<Self, F>
    where
        F: FnMut(Self::Vastu) -> B,
    {
        Rupantarana { dhara: self, f }
    }

    /// Filter stream items
    fn channana<P>(self, predicate: P) -> Channana<Self, P>
    where
        P: FnMut(&Self::Vastu) -> bool,
    {
        Channana {
            dhara: self,
            predicate,
        }
    }

    /// Take first n items
    fn prathama_n(self, n: usize) -> PrathamaN<Self> {
        PrathamaN {
            dhara: self,
            remaining: n,
        }
    }

    /// Skip first n items
    fn tyaja_n(self, n: usize) -> TyajaN<Self> {
        TyajaN {
            dhara: self,
            remaining: n,
        }
    }

    /// Chain two streams
    fn shrinkhala<S>(self, other: S) -> Shrinkhala<Self, S>
    where
        S: Dhara<Vastu = Self::Vastu>,
    {
        Shrinkhala {
            first: Some(self),
            second: other,
        }
    }

    /// Enumerate stream items
    fn ganana(self) -> Ganana<Self> {
        Ganana {
            dhara: self,
            count: 0,
        }
    }

    /// Collect into Vec
    #[cfg(feature = "alloc")]
    fn sangraha(self) -> Sangraha<Self> {
        Sangraha {
            dhara: self,
            items: Vec::new(),
        }
    }

    /// Fold stream into single value
    fn valana<B, F>(self, init: B, f: F) -> Valana<Self, B, F>
    where
        F: FnMut(B, Self::Vastu) -> B,
    {
        Valana {
            dhara: self,
            accumulator: Some(init),
            f,
        }
    }

    /// Get next item as future
    fn agrim(self) -> Agrim<Self> {
        Agrim { dhara: Some(self) }
    }
}

impl<S: Dhara + Sized> DharaVistar for S {}

// ============================================================================
// MAP STREAM
// ============================================================================

/// Mapped stream (रूपांतरित धारा)
pub struct Rupantarana<S, F> {
    dhara: S,
    f: F,
}

impl<S, F, B> Dhara for Rupantarana<S, F>
where
    S: Dhara + Unpin,
    F: FnMut(S::Vastu) -> B + Unpin,
{
    type Vastu = B;

    fn agrim_matadana(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Vastu>> {
        match Pin::new(&mut self.dhara).agrim_matadana(cx) {
            Poll::Ready(Some(item)) => Poll::Ready(Some((self.f)(item))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

// ============================================================================
// FILTER STREAM
// ============================================================================

/// Filtered stream (छन्नित धारा)
pub struct Channana<S, P> {
    dhara: S,
    predicate: P,
}

impl<S, P> Dhara for Channana<S, P>
where
    S: Dhara + Unpin,
    P: FnMut(&S::Vastu) -> bool + Unpin,
{
    type Vastu = S::Vastu;

    fn agrim_matadana(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Vastu>> {
        loop {
            match Pin::new(&mut self.dhara).agrim_matadana(cx) {
                Poll::Ready(Some(item)) => {
                    if (self.predicate)(&item) {
                        return Poll::Ready(Some(item));
                    }
                    // Item filtered out, continue polling
                }
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

// ============================================================================
// TAKE N STREAM
// ============================================================================

/// Take first N items (प्रथम N)
pub struct PrathamaN<S> {
    dhara: S,
    remaining: usize,
}

impl<S> Dhara for PrathamaN<S>
where
    S: Dhara + Unpin,
{
    type Vastu = S::Vastu;

    fn agrim_matadana(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Vastu>> {
        if self.remaining == 0 {
            return Poll::Ready(None);
        }

        match Pin::new(&mut self.dhara).agrim_matadana(cx) {
            Poll::Ready(Some(item)) => {
                self.remaining -= 1;
                Poll::Ready(Some(item))
            }
            other => other,
        }
    }

    fn akar_sanketa(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.dhara.akar_sanketa();
        let lower = core::cmp::min(lower, self.remaining);
        let upper = upper.map(|u| core::cmp::min(u, self.remaining));
        (lower, upper)
    }
}

// ============================================================================
// SKIP N STREAM
// ============================================================================

/// Skip first N items (त्याग N)
pub struct TyajaN<S> {
    dhara: S,
    remaining: usize,
}

impl<S> Dhara for TyajaN<S>
where
    S: Dhara + Unpin,
{
    type Vastu = S::Vastu;

    fn agrim_matadana(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Vastu>> {
        while self.remaining > 0 {
            match Pin::new(&mut self.dhara).agrim_matadana(cx) {
                Poll::Ready(Some(_)) => {
                    self.remaining -= 1;
                }
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => return Poll::Pending,
            }
        }

        Pin::new(&mut self.dhara).agrim_matadana(cx)
    }
}

// ============================================================================
// CHAIN STREAM
// ============================================================================

/// Chained streams (श्रृंखला धारा)
pub struct Shrinkhala<S1, S2> {
    first: Option<S1>,
    second: S2,
}

impl<S1, S2> Dhara for Shrinkhala<S1, S2>
where
    S1: Dhara + Unpin,
    S2: Dhara<Vastu = S1::Vastu> + Unpin,
{
    type Vastu = S1::Vastu;

    fn agrim_matadana(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Vastu>> {
        if let Some(ref mut first) = self.first {
            match Pin::new(first).agrim_matadana(cx) {
                Poll::Ready(Some(item)) => return Poll::Ready(Some(item)),
                Poll::Ready(None) => {
                    self.first = None;
                }
                Poll::Pending => return Poll::Pending,
            }
        }

        Pin::new(&mut self.second).agrim_matadana(cx)
    }
}

// ============================================================================
// ENUMERATE STREAM
// ============================================================================

/// Enumerated stream (गणित धारा)
pub struct Ganana<S> {
    dhara: S,
    count: usize,
}

impl<S> Dhara for Ganana<S>
where
    S: Dhara + Unpin,
{
    type Vastu = (usize, S::Vastu);

    fn agrim_matadana(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Vastu>> {
        match Pin::new(&mut self.dhara).agrim_matadana(cx) {
            Poll::Ready(Some(item)) => {
                let idx = self.count;
                self.count += 1;
                Poll::Ready(Some((idx, item)))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

// ============================================================================
// COLLECT STREAM
// ============================================================================

/// Collect stream into Vec (संग्रह)
#[cfg(feature = "alloc")]
pub struct Sangraha<S: Dhara> {
    dhara: S,
    items: Vec<S::Vastu>,
}

// Allow mutable access through Pin when stream is Unpin
#[cfg(feature = "alloc")]
impl<S: Dhara + Unpin> Unpin for Sangraha<S> {}

#[cfg(feature = "alloc")]
impl<S> Future for Sangraha<S>
where
    S: Dhara + Unpin,
{
    type Output = Vec<S::Vastu>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match Pin::new(&mut self.dhara).agrim_matadana(cx) {
                Poll::Ready(Some(item)) => {
                    self.items.push(item);
                }
                Poll::Ready(None) => {
                    return Poll::Ready(core::mem::take(&mut self.items));
                }
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

// ============================================================================
// FOLD STREAM
// ============================================================================

/// Fold stream into single value (वलन)
pub struct Valana<S, B, F> {
    dhara: S,
    accumulator: Option<B>,
    f: F,
}

impl<S, B, F> Future for Valana<S, B, F>
where
    S: Dhara + Unpin,
    F: FnMut(B, S::Vastu) -> B + Unpin,
    B: Unpin,
{
    type Output = B;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match Pin::new(&mut self.dhara).agrim_matadana(cx) {
                Poll::Ready(Some(item)) => {
                    let acc = self.accumulator.take().unwrap();
                    self.accumulator = Some((self.f)(acc, item));
                }
                Poll::Ready(None) => {
                    return Poll::Ready(self.accumulator.take().unwrap());
                }
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

// ============================================================================
// NEXT ITEM FUTURE
// ============================================================================

/// Get next item as future (अग्रिम)
pub struct Agrim<S> {
    dhara: Option<S>,
}

impl<S> Future for Agrim<S>
where
    S: Dhara + Unpin,
{
    type Output = Option<(S::Vastu, S)>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut dhara = self.dhara.take().expect("Agrim polled after completion");

        match Pin::new(&mut dhara).agrim_matadana(cx) {
            Poll::Ready(Some(item)) => Poll::Ready(Some((item, dhara))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => {
                self.dhara = Some(dhara);
                Poll::Pending
            }
        }
    }
}

// ============================================================================
// CONCRETE STREAMS
// ============================================================================

/// Stream from iterator (पुनरावृत्ति धारा)
pub struct PunaravrittiDhara<I> {
    iter: I,
}

impl<I> PunaravrittiDhara<I> {
    pub fn nava(iter: I) -> Self {
        Self { iter }
    }
}

impl<I> Dhara for PunaravrittiDhara<I>
where
    I: Iterator + Unpin,
{
    type Vastu = I::Item;

    fn agrim_matadana(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Vastu>> {
        Poll::Ready(self.iter.next())
    }

    fn akar_sanketa(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

/// Create stream from iterator
pub fn iter_dhara<I: IntoIterator>(iter: I) -> PunaravrittiDhara<I::IntoIter> {
    PunaravrittiDhara::nava(iter.into_iter())
}

/// Empty stream (रिक्त धारा)
pub struct RiktaDhara<T> {
    _phantom: core::marker::PhantomData<T>,
}

impl<T> RiktaDhara<T> {
    pub fn nava() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<T> Dhara for RiktaDhara<T> {
    type Vastu = T;

    fn agrim_matadana(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Vastu>> {
        Poll::Ready(None)
    }

    fn akar_sanketa(&self) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}

/// Create empty stream (रिक्त)
pub fn rikta<T>() -> RiktaDhara<T> {
    RiktaDhara::nava()
}

/// Single item stream (एक धारा)
pub struct EkaDhara<T> {
    item: Option<T>,
}

impl<T> EkaDhara<T> {
    pub fn nava(item: T) -> Self {
        Self { item: Some(item) }
    }
}

impl<T: Unpin> Dhara for EkaDhara<T> {
    type Vastu = T;

    fn agrim_matadana(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Vastu>> {
        Poll::Ready(self.item.take())
    }

    fn akar_sanketa(&self) -> (usize, Option<usize>) {
        let n = if self.item.is_some() { 1 } else { 0 };
        (n, Some(n))
    }
}

/// Create single-item stream (एक)
pub fn eka<T>(item: T) -> EkaDhara<T> {
    EkaDhara::nava(item)
}

/// Repeat stream (पुनरावृत्त धारा)
pub struct PunarvartaDhara<T: Clone> {
    item: T,
}

impl<T: Clone> PunarvartaDhara<T> {
    pub fn nava(item: T) -> Self {
        Self { item }
    }
}

impl<T: Clone + Unpin> Dhara for PunarvartaDhara<T> {
    type Vastu = T;

    fn agrim_matadana(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Vastu>> {
        Poll::Ready(Some(self.item.clone()))
    }
}

/// Create repeating stream (पुनरावृत्त)
pub fn punarvarta<T: Clone>(item: T) -> PunarvartaDhara<T> {
    PunarvartaDhara::nava(item)
}

/// Range stream (परास धारा)
pub struct ParasDhara {
    current: i64,
    end: i64,
    step: i64,
}

impl ParasDhara {
    pub fn nava(start: i64, end: i64) -> Self {
        Self {
            current: start,
            end,
            step: 1,
        }
    }

    pub fn with_step(start: i64, end: i64, step: i64) -> Self {
        Self {
            current: start,
            end,
            step,
        }
    }
}

impl Dhara for ParasDhara {
    type Vastu = i64;

    fn agrim_matadana(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Vastu>> {
        if (self.step > 0 && self.current >= self.end)
            || (self.step < 0 && self.current <= self.end)
            || self.step == 0
        {
            return Poll::Ready(None);
        }

        let val = self.current;
        self.current += self.step;
        Poll::Ready(Some(val))
    }
}

/// Create range stream (परास)
pub fn paras(start: i64, end: i64) -> ParasDhara {
    ParasDhara::nava(start, end)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use core::task::{RawWaker, RawWakerVTable, Waker};

    fn noop_waker() -> Waker {
        const VTABLE: RawWakerVTable = RawWakerVTable::new(|_| RAW_WAKER, |_| {}, |_| {}, |_| {});
        const RAW_WAKER: RawWaker = RawWaker::new(core::ptr::null(), &VTABLE);
        unsafe { Waker::from_raw(RAW_WAKER) }
    }

    fn poll_dhara<S: Dhara + Unpin>(dhara: &mut S) -> Option<S::Vastu> {
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);
        match Pin::new(dhara).agrim_matadana(&mut cx) {
            Poll::Ready(item) => item,
            Poll::Pending => None,
        }
    }

    #[test]
    fn test_iter_dhara() {
        let mut s = iter_dhara(vec![1, 2, 3]);

        assert_eq!(poll_dhara(&mut s), Some(1));
        assert_eq!(poll_dhara(&mut s), Some(2));
        assert_eq!(poll_dhara(&mut s), Some(3));
        assert_eq!(poll_dhara(&mut s), None);
    }

    #[test]
    fn test_map_stream() {
        let mut s = iter_dhara(vec![1, 2, 3]).rupantarana(|x| x * 2);

        assert_eq!(poll_dhara(&mut s), Some(2));
        assert_eq!(poll_dhara(&mut s), Some(4));
        assert_eq!(poll_dhara(&mut s), Some(6));
    }

    #[test]
    fn test_filter_stream() {
        let mut s = iter_dhara(vec![1, 2, 3, 4, 5]).channana(|x| x % 2 == 0);

        assert_eq!(poll_dhara(&mut s), Some(2));
        assert_eq!(poll_dhara(&mut s), Some(4));
        assert_eq!(poll_dhara(&mut s), None);
    }

    #[test]
    fn test_take_stream() {
        let mut s = iter_dhara(vec![1, 2, 3, 4, 5]).prathama_n(3);

        assert_eq!(poll_dhara(&mut s), Some(1));
        assert_eq!(poll_dhara(&mut s), Some(2));
        assert_eq!(poll_dhara(&mut s), Some(3));
        assert_eq!(poll_dhara(&mut s), None);
    }

    #[test]
    fn test_skip_stream() {
        let mut s = iter_dhara(vec![1, 2, 3, 4, 5]).tyaja_n(2);

        assert_eq!(poll_dhara(&mut s), Some(3));
        assert_eq!(poll_dhara(&mut s), Some(4));
        assert_eq!(poll_dhara(&mut s), Some(5));
    }

    #[test]
    fn test_enumerate_stream() {
        let mut s = iter_dhara(vec!["a", "b", "c"]).ganana();

        assert_eq!(poll_dhara(&mut s), Some((0, "a")));
        assert_eq!(poll_dhara(&mut s), Some((1, "b")));
        assert_eq!(poll_dhara(&mut s), Some((2, "c")));
    }

    #[test]
    fn test_empty_stream() {
        let mut s: RiktaDhara<i32> = rikta();
        assert_eq!(poll_dhara(&mut s), None);
    }

    #[test]
    fn test_single_stream() {
        let mut s = eka(42);
        assert_eq!(poll_dhara(&mut s), Some(42));
        assert_eq!(poll_dhara(&mut s), None);
    }

    #[test]
    fn test_range_stream() {
        let mut s = paras(1, 4);

        assert_eq!(poll_dhara(&mut s), Some(1));
        assert_eq!(poll_dhara(&mut s), Some(2));
        assert_eq!(poll_dhara(&mut s), Some(3));
        assert_eq!(poll_dhara(&mut s), None);
    }
}
