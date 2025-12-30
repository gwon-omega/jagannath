//! # Kriya - Geometric Algorithms (क्रिया)
//!
//! Computational geometry algorithms.

use super::bindu::Bindu2;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Orientation of three points
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disha {
    /// Counter-clockwise
    Vaama,
    /// Clockwise
    Dakshina,
    /// Collinear
    Samrekha,
}

/// Get orientation of three points
pub fn disha(p: &Bindu2, q: &Bindu2, r: &Bindu2) -> Disha {
    let val = (*q - *p).vayu_gunaa(&(*r - *q));

    const EPSILON: f64 = 1e-10;

    if val.abs() < EPSILON {
        Disha::Samrekha
    } else if val > 0.0 {
        Disha::Vaama
    } else {
        Disha::Dakshina
    }
}

/// Signed area of triangle
pub fn hastaksharit_kshetrafal(a: &Bindu2, b: &Bindu2, c: &Bindu2) -> f64 {
    (*b - *a).vayu_gunaa(&(*c - *a)) / 2.0
}

/// Check if point is on line segment
pub fn rekha_par(p: &Bindu2, a: &Bindu2, b: &Bindu2) -> bool {
    if disha(a, b, p) != Disha::Samrekha {
        return false;
    }

    p.x >= a.x.min(b.x) && p.x <= a.x.max(b.x) && p.y >= a.y.min(b.y) && p.y <= a.y.max(b.y)
}

/// Check if two line segments intersect
pub fn praticched(a1: &Bindu2, a2: &Bindu2, b1: &Bindu2, b2: &Bindu2) -> bool {
    let d1 = disha(b1, b2, a1);
    let d2 = disha(b1, b2, a2);
    let d3 = disha(a1, a2, b1);
    let d4 = disha(a1, a2, b2);

    if d1 != d2 && d3 != d4 {
        return true;
    }

    if d1 == Disha::Samrekha && rekha_par(a1, b1, b2) {
        return true;
    }
    if d2 == Disha::Samrekha && rekha_par(a2, b1, b2) {
        return true;
    }
    if d3 == Disha::Samrekha && rekha_par(b1, a1, a2) {
        return true;
    }
    if d4 == Disha::Samrekha && rekha_par(b2, a1, a2) {
        return true;
    }

    false
}

/// Line segment intersection point
pub fn praticched_bindu(a1: &Bindu2, a2: &Bindu2, b1: &Bindu2, b2: &Bindu2) -> Option<Bindu2> {
    let d1 = a2.x - a1.x;
    let d2 = a2.y - a1.y;
    let d3 = b2.x - b1.x;
    let d4 = b2.y - b1.y;

    let denom = d1 * d4 - d2 * d3;

    if denom.abs() < 1e-10 {
        return None;
    }

    let t = ((b1.x - a1.x) * d4 - (b1.y - a1.y) * d3) / denom;
    let u = ((b1.x - a1.x) * d2 - (b1.y - a1.y) * d1) / denom;

    if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
        Some(Bindu2::naya(a1.x + t * d1, a1.y + t * d2))
    } else {
        None
    }
}

/// Distance from point to line
pub fn bindu_rekha_doori(p: &Bindu2, a: &Bindu2, b: &Bindu2) -> f64 {
    let ab = *b - *a;
    let ap = *p - *a;

    let cross = ab.vayu_gunaa(&ap).abs();
    let len = ab.lambai();

    if len < 1e-10 {
        return p.doori(a);
    }

    cross / len
}

/// Closest point on line segment to point
pub fn nikat_bindu(p: &Bindu2, a: &Bindu2, b: &Bindu2) -> Bindu2 {
    let ab = *b - *a;
    let ap = *p - *a;

    let len_sq = ab.bindu_gunaa(&ab);

    if len_sq < 1e-10 {
        return *a;
    }

    let t = (ap.bindu_gunaa(&ab) / len_sq).clamp(0.0, 1.0);

    Bindu2::naya(a.x + t * ab.x, a.y + t * ab.y)
}

/// Convex hull using Graham scan
#[cfg(feature = "alloc")]
pub fn uttal_aavarana(bindu: &[Bindu2]) -> Vec<Bindu2> {
    if bindu.len() < 3 {
        return bindu.to_vec();
    }

    // Find bottom-left point
    let mut start_idx = 0;
    for i in 1..bindu.len() {
        if bindu[i].y < bindu[start_idx].y
            || (bindu[i].y == bindu[start_idx].y && bindu[i].x < bindu[start_idx].x)
        {
            start_idx = i;
        }
    }

    let pivot = bindu[start_idx];

    // Sort by polar angle
    let mut points: Vec<Bindu2> = bindu.to_vec();
    points.swap(0, start_idx);

    points[1..].sort_by(|a, b| {
        let cross = (*a - pivot).vayu_gunaa(&(*b - pivot));

        if cross.abs() < 1e-10 {
            let da = pivot.doori(a);
            let db = pivot.doori(b);
            da.partial_cmp(&db).unwrap()
        } else if cross > 0.0 {
            core::cmp::Ordering::Less
        } else {
            core::cmp::Ordering::Greater
        }
    });

    // Build hull
    let mut hull: Vec<Bindu2> = Vec::new();

    for p in points {
        while hull.len() >= 2 {
            let n = hull.len();
            let cross = (hull[n - 1] - hull[n - 2]).vayu_gunaa(&(p - hull[n - 1]));
            if cross <= 0.0 {
                hull.pop();
            } else {
                break;
            }
        }
        hull.push(p);
    }

    hull
}

/// Polygon area (shoelace formula)
#[cfg(feature = "alloc")]
pub fn bahubhuj_kshetrafal(sheersh: &[Bindu2]) -> f64 {
    let n = sheersh.len();
    if n < 3 {
        return 0.0;
    }

    let mut area = 0.0;
    for i in 0..n {
        let j = (i + 1) % n;
        area += sheersh[i].x * sheersh[j].y;
        area -= sheersh[j].x * sheersh[i].y;
    }

    libm::fabs(area) / 2.0
}

/// Check if polygon is convex
#[cfg(feature = "alloc")]
pub fn uttal_hai(sheersh: &[Bindu2]) -> bool {
    let n = sheersh.len();
    if n < 3 {
        return false;
    }

    let mut sign = 0i32;

    for i in 0..n {
        let j = (i + 1) % n;
        let k = (i + 2) % n;

        let cross = (sheersh[j] - sheersh[i]).vayu_gunaa(&(sheersh[k] - sheersh[j]));

        if cross > 0.0 {
            if sign < 0 {
                return false;
            }
            sign = 1;
        } else if cross < 0.0 {
            if sign > 0 {
                return false;
            }
            sign = -1;
        }
    }

    true
}

/// Point in polygon (ray casting)
#[cfg(feature = "alloc")]
pub fn bahubhuj_me(p: &Bindu2, sheersh: &[Bindu2]) -> bool {
    let n = sheersh.len();
    if n < 3 {
        return false;
    }

    let mut inside = false;
    let mut j = n - 1;

    for i in 0..n {
        let vi = &sheersh[i];
        let vj = &sheersh[j];

        if ((vi.y > p.y) != (vj.y > p.y))
            && (p.x < (vj.x - vi.x) * (p.y - vi.y) / (vj.y - vi.y) + vi.x)
        {
            inside = !inside;
        }

        j = i;
    }

    inside
}

/// Triangulate a simple polygon (ear clipping)
#[cfg(feature = "alloc")]
pub fn tribhuj_vibhajan(sheersh: &[Bindu2]) -> Vec<[usize; 3]> {
    let n = sheersh.len();
    if n < 3 {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut remaining: Vec<usize> = (0..n).collect();

    while remaining.len() > 3 {
        let m = remaining.len();
        let mut found = false;

        for i in 0..m {
            let prev = remaining[(i + m - 1) % m];
            let curr = remaining[i];
            let next = remaining[(i + 1) % m];

            // Check if ear
            let cross =
                (sheersh[curr] - sheersh[prev]).vayu_gunaa(&(sheersh[next] - sheersh[curr]));

            if cross <= 0.0 {
                continue;
            }

            // Check no other vertex inside
            let mut valid = true;
            for j in 0..m {
                if j == (i + m - 1) % m || j == i || j == (i + 1) % m {
                    continue;
                }

                let p = &sheersh[remaining[j]];
                let tri = super::aakrti::Tribhuj::naya(sheersh[prev], sheersh[curr], sheersh[next]);
                if tri.shamil(p) {
                    valid = false;
                    break;
                }
            }

            if valid {
                result.push([prev, curr, next]);
                remaining.remove(i);
                found = true;
                break;
            }
        }

        if !found {
            break;
        }
    }

    if remaining.len() == 3 {
        result.push([remaining[0], remaining[1], remaining[2]]);
    }

    result
}

/// Minkowski sum of two convex polygons
#[cfg(feature = "alloc")]
pub fn minkowski_yoga(a: &[Bindu2], b: &[Bindu2]) -> Vec<Bindu2> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }

    // Find bottom-left points
    let mut ai = 0;
    let mut bi = 0;

    for i in 1..a.len() {
        if a[i].y < a[ai].y || (a[i].y == a[ai].y && a[i].x < a[ai].x) {
            ai = i;
        }
    }

    for i in 1..b.len() {
        if b[i].y < b[bi].y || (b[i].y == b[bi].y && b[i].x < b[bi].x) {
            bi = i;
        }
    }

    let na = a.len();
    let nb = b.len();
    let mut result = Vec::new();

    let mut i = 0;
    let mut j = 0;

    while i < na || j < nb {
        let pa = a[(ai + i) % na];
        let pb = b[(bi + j) % nb];

        result.push(pa + pb);

        if i >= na {
            j += 1;
            continue;
        }
        if j >= nb {
            i += 1;
            continue;
        }

        let ea = a[(ai + i + 1) % na] - a[(ai + i) % na];
        let eb = b[(bi + j + 1) % nb] - b[(bi + j) % nb];

        let cross = ea.vayu_gunaa(&eb);

        if cross > 0.0 {
            i += 1;
        } else if cross < 0.0 {
            j += 1;
        } else {
            i += 1;
            j += 1;
        }
    }

    result
}

/// Rotating calipers - diameter of convex polygon
#[cfg(feature = "alloc")]
pub fn uttal_vyaas(hull: &[Bindu2]) -> f64 {
    let n = hull.len();
    if n < 2 {
        return 0.0;
    }
    if n == 2 {
        return hull[0].doori(&hull[1]);
    }

    let mut max_dist = 0.0f64;
    let mut j = 1;

    for i in 0..n {
        let next_i = (i + 1) % n;
        let edge = hull[next_i] - hull[i];

        while {
            let next_j = (j + 1) % n;
            let cross1 = edge.vayu_gunaa(&(hull[j] - hull[i]));
            let cross2 = edge.vayu_gunaa(&(hull[next_j] - hull[i]));
            cross2 > cross1
        } {
            j = (j + 1) % n;
        }

        let dist = hull[i].doori(&hull[j]);
        max_dist = max_dist.max(dist);

        let dist2 = hull[next_i].doori(&hull[j]);
        max_dist = max_dist.max(dist2);
    }

    max_dist
}

/// Closest pair of points
#[cfg(feature = "alloc")]
pub fn nikat_joda(bindu: &[Bindu2]) -> Option<(Bindu2, Bindu2, f64)> {
    if bindu.len() < 2 {
        return None;
    }

    // Simple O(n^2) for now
    let mut min_dist = f64::INFINITY;
    let mut result = (bindu[0], bindu[1], min_dist);

    for i in 0..bindu.len() {
        for j in (i + 1)..bindu.len() {
            let dist = bindu[i].doori(&bindu[j]);
            if dist < min_dist {
                min_dist = dist;
                result = (bindu[i], bindu[j], dist);
            }
        }
    }

    Some(result)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orientation() {
        let a = Bindu2::naya(0.0, 0.0);
        let b = Bindu2::naya(1.0, 0.0);
        let c = Bindu2::naya(1.0, 1.0);

        assert_eq!(disha(&a, &b, &c), Disha::Vaama);
    }

    #[test]
    fn test_segment_intersection() {
        let a1 = Bindu2::naya(0.0, 0.0);
        let a2 = Bindu2::naya(2.0, 2.0);
        let b1 = Bindu2::naya(0.0, 2.0);
        let b2 = Bindu2::naya(2.0, 0.0);

        assert!(praticched(&a1, &a2, &b1, &b2));

        let point = praticched_bindu(&a1, &a2, &b1, &b2).unwrap();
        assert!((point.x - 1.0).abs() < 1e-10);
        assert!((point.y - 1.0).abs() < 1e-10);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_convex_hull() {
        let points = vec![
            Bindu2::naya(0.0, 0.0),
            Bindu2::naya(1.0, 1.0),
            Bindu2::naya(2.0, 0.0),
            Bindu2::naya(1.0, 2.0),
            Bindu2::naya(1.0, 0.5),
        ];

        let hull = uttal_aavarana(&points);
        // Hull is triangle: (0,0), (2,0), (1,2) - (1,1) and (1,0.5) are interior
        assert!(hull.len() >= 3);
        assert!(hull.len() <= 4);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_polygon_area() {
        let square = vec![
            Bindu2::naya(0.0, 0.0),
            Bindu2::naya(2.0, 0.0),
            Bindu2::naya(2.0, 2.0),
            Bindu2::naya(0.0, 2.0),
        ];

        assert_eq!(bahubhuj_kshetrafal(&square), 4.0);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_point_in_polygon() {
        let square = vec![
            Bindu2::naya(0.0, 0.0),
            Bindu2::naya(2.0, 0.0),
            Bindu2::naya(2.0, 2.0),
            Bindu2::naya(0.0, 2.0),
        ];

        assert!(bahubhuj_me(&Bindu2::naya(1.0, 1.0), &square));
        assert!(!bahubhuj_me(&Bindu2::naya(3.0, 3.0), &square));
    }
}
