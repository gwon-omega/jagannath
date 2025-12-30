//! # Vighatan - Matrix Decompositions (विघटन)
//!
//! LU, QR, Cholesky, and other matrix decompositions.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use super::aavyuha::Aavyuha;
use super::sadish::Sadish;

/// LU Decomposition result
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct LuVighatan {
    pub l: Aavyuha,
    pub u: Aavyuha,
    pub p: Vec<usize>, // Permutation
}

/// QR Decomposition result
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct QrVighatan {
    pub q: Aavyuha,
    pub r: Aavyuha,
}

/// Cholesky Decomposition result
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct CholeskyVighatan {
    pub l: Aavyuha,
}

/// Eigenvalue result
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct SwamaanPhala {
    pub maan: Vec<f64>,  // Eigenvalues
    pub sadish: Aavyuha, // Eigenvectors (columns)
}

/// LU decomposition with partial pivoting
#[cfg(feature = "alloc")]
pub fn lu_vighatan(a: &Aavyuha) -> Option<LuVighatan> {
    if !a.varga_hai() {
        return None;
    }

    let n = a.panki;
    let mut l = Aavyuha::shunya(n, n);
    let mut u = a.clone();
    let mut p: Vec<usize> = (0..n).collect();

    for i in 0..n {
        // Find pivot
        let mut max_row = i;
        let mut max_val = libm::fabs(u.tatva[i * n + i]);

        for k in (i + 1)..n {
            let val = libm::fabs(u.tatva[k * n + i]);
            if val > max_val {
                max_val = val;
                max_row = k;
            }
        }

        // Swap rows in U and P
        if max_row != i {
            for j in 0..n {
                let temp = u.tatva[i * n + j];
                u.tatva[i * n + j] = u.tatva[max_row * n + j];
                u.tatva[max_row * n + j] = temp;
            }

            // Swap in L (only processed columns)
            for j in 0..i {
                let temp = l.tatva[i * n + j];
                l.tatva[i * n + j] = l.tatva[max_row * n + j];
                l.tatva[max_row * n + j] = temp;
            }

            p.swap(i, max_row);
        }

        // Set L diagonal
        l.rakho(i, i, 1.0);

        // Eliminate
        let pivot = u.tatva[i * n + i];
        if pivot.abs() < 1e-10 {
            continue; // Singular
        }

        for k in (i + 1)..n {
            let factor = u.tatva[k * n + i] / pivot;
            l.rakho(k, i, factor);

            for j in i..n {
                u.tatva[k * n + j] -= factor * u.tatva[i * n + j];
            }
        }
    }

    Some(LuVighatan { l, u, p })
}

/// Solve Ax = b using LU decomposition
#[cfg(feature = "alloc")]
pub fn lu_hal(lu: &LuVighatan, b: &Sadish) -> Option<Sadish> {
    let n = lu.l.panki;
    if b.maap() != n {
        return None;
    }

    // Apply permutation to b
    let mut pb = vec![0.0; n];
    for i in 0..n {
        pb[i] = b.tatva[lu.p[i]];
    }

    // Forward substitution: Ly = Pb
    let mut y = vec![0.0; n];
    for i in 0..n {
        let mut sum = pb[i];
        for j in 0..i {
            sum -= lu.l.tatva[i * n + j] * y[j];
        }
        y[i] = sum;
    }

    // Back substitution: Ux = y
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = y[i];
        for j in (i + 1)..n {
            sum -= lu.u.tatva[i * n + j] * x[j];
        }
        let diag = lu.u.tatva[i * n + i];
        if diag.abs() < 1e-10 {
            return None; // Singular
        }
        x[i] = sum / diag;
    }

    Some(Sadish::naya(x))
}

/// QR decomposition using Gram-Schmidt
#[cfg(feature = "alloc")]
pub fn qr_vighatan(a: &Aavyuha) -> Option<QrVighatan> {
    let m = a.panki;
    let n = a.stambh;

    if m < n {
        return None; // Need m >= n
    }

    let mut q = Aavyuha::shunya(m, n);
    let mut r = Aavyuha::shunya(n, n);

    // Extract columns
    let mut cols: Vec<Sadish> = Vec::with_capacity(n);
    for j in 0..n {
        cols.push(a.stambh_pao(j)?);
    }

    // Modified Gram-Schmidt
    for j in 0..n {
        let mut v = cols[j].clone();

        for i in 0..j {
            let qi = q.stambh_pao(i)?;
            let rij = qi.bindu_gunaa(&cols[j]);
            r.rakho(i, j, rij);
            v = v.ghatao(&qi.mapa(rij));
        }

        let rjj = v.lambai();
        r.rakho(j, j, rjj);

        if rjj.abs() < 1e-10 {
            // Column is linearly dependent
            for i in 0..m {
                q.rakho(i, j, 0.0);
            }
        } else {
            let qj = v.mapa(1.0 / rjj);
            for i in 0..m {
                q.rakho(i, j, qj.tatva[i]);
            }
        }
    }

    Some(QrVighatan { q, r })
}

/// Solve Ax = b using QR decomposition
#[cfg(feature = "alloc")]
pub fn qr_hal(qr: &QrVighatan, b: &Sadish) -> Option<Sadish> {
    let n = qr.r.panki;

    // Compute Q^T * b
    let qt = qr.q.parivart();
    let qtb = qt.sadish_gunaa(b)?;

    // Back substitution: Rx = Q^T*b
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = qtb.tatva[i];
        for j in (i + 1)..n {
            sum -= qr.r.tatva[i * n + j] * x[j];
        }
        let diag = qr.r.tatva[i * n + i];
        if diag.abs() < 1e-10 {
            return None;
        }
        x[i] = sum / diag;
    }

    Some(Sadish::naya(x))
}

/// Cholesky decomposition (A = LL^T)
/// Only for symmetric positive-definite matrices
#[cfg(feature = "alloc")]
pub fn cholesky_vighatan(a: &Aavyuha) -> Option<CholeskyVighatan> {
    if !a.varga_hai() {
        return None;
    }

    let n = a.panki;
    let mut l = Aavyuha::shunya(n, n);

    for i in 0..n {
        for j in 0..=i {
            let mut sum = a.tatva[i * n + j];

            for k in 0..j {
                sum -= l.tatva[i * n + k] * l.tatva[j * n + k];
            }

            if i == j {
                if sum <= 0.0 {
                    return None; // Not positive definite
                }
                l.rakho(i, j, libm::sqrt(sum));
            } else {
                let diag = l.tatva[j * n + j];
                if diag.abs() < 1e-10 {
                    return None;
                }
                l.rakho(i, j, sum / diag);
            }
        }
    }

    Some(CholeskyVighatan { l })
}

/// Solve Ax = b using Cholesky
#[cfg(feature = "alloc")]
pub fn cholesky_hal(chol: &CholeskyVighatan, b: &Sadish) -> Option<Sadish> {
    let n = chol.l.panki;
    if b.maap() != n {
        return None;
    }

    // Forward substitution: Ly = b
    let mut y = vec![0.0; n];
    for i in 0..n {
        let mut sum = b.tatva[i];
        for j in 0..i {
            sum -= chol.l.tatva[i * n + j] * y[j];
        }
        let diag = chol.l.tatva[i * n + i];
        if diag.abs() < 1e-10 {
            return None;
        }
        y[i] = sum / diag;
    }

    // Back substitution: L^T x = y
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = y[i];
        for j in (i + 1)..n {
            sum -= chol.l.tatva[j * n + i] * x[j];
        }
        let diag = chol.l.tatva[i * n + i];
        if diag.abs() < 1e-10 {
            return None;
        }
        x[i] = sum / diag;
    }

    Some(Sadish::naya(x))
}

/// Power iteration for dominant eigenvalue
#[cfg(feature = "alloc")]
pub fn shakti_punaraavriti(a: &Aavyuha, max_iter: usize, tol: f64) -> Option<(f64, Sadish)> {
    if !a.varga_hai() {
        return None;
    }

    let n = a.panki;
    let mut v = Sadish::ekak(n).ekikaran();
    let mut lambda = 0.0;

    for _ in 0..max_iter {
        let av = a.sadish_gunaa(&v)?;
        let new_lambda = v.bindu_gunaa(&av);

        v = av.ekikaran();

        if (new_lambda - lambda).abs() < tol {
            return Some((new_lambda, v));
        }

        lambda = new_lambda;
    }

    Some((lambda, v))
}

/// Inverse power iteration (for smallest eigenvalue)
#[cfg(feature = "alloc")]
pub fn vyutkram_shakti(a: &Aavyuha, max_iter: usize, tol: f64) -> Option<(f64, Sadish)> {
    let inv = a.vyutkram()?;
    let (inv_lambda, v) = shakti_punaraavriti(&inv, max_iter, tol)?;
    Some((1.0 / inv_lambda, v))
}

/// Simple eigenvalue estimation using QR algorithm (few iterations)
#[cfg(feature = "alloc")]
pub fn swamaan_aagathan(a: &Aavyuha, max_iter: usize) -> Option<Vec<f64>> {
    if !a.varga_hai() {
        return None;
    }

    let n = a.panki;
    let mut ak = a.clone();

    for _ in 0..max_iter {
        let qr = qr_vighatan(&ak)?;
        ak = qr.r.gunaa(&qr.q)?;
    }

    // Extract diagonal (approximate eigenvalues)
    let mut eigenvalues = Vec::with_capacity(n);
    for i in 0..n {
        eigenvalues.push(ak.tatva[i * n + i]);
    }

    Some(eigenvalues)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_lu() {
        let a = Aavyuha::naya(vec![2.0, 1.0, 1.0, 4.0], 2, 2);
        let lu = lu_vighatan(&a).unwrap();

        // L * U should equal PA
        let lu_prod = lu.l.gunaa(&lu.u).unwrap();

        // Apply permutation to A
        let mut pa = Aavyuha::shunya(2, 2);
        for i in 0..2 {
            for j in 0..2 {
                pa.rakho(i, j, a.tatva[lu.p[i] * 2 + j]);
            }
        }

        for i in 0..4 {
            assert!((lu_prod.tatva[i] - pa.tatva[i]).abs() < 1e-10);
        }
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_lu_solve() {
        let a = Aavyuha::naya(vec![2.0, 1.0, 1.0, 3.0], 2, 2);
        let b = Sadish::naya(vec![5.0, 5.0]);

        let lu = lu_vighatan(&a).unwrap();
        let x = lu_hal(&lu, &b).unwrap();

        // Check Ax = b
        let ax = a.sadish_gunaa(&x).unwrap();
        assert!((ax.tatva[0] - 5.0).abs() < 1e-10);
        assert!((ax.tatva[1] - 5.0).abs() < 1e-10);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_qr() {
        let a = Aavyuha::naya(vec![1.0, 1.0, 0.0, 1.0, 0.0, 1.0], 3, 2);
        let qr = qr_vighatan(&a).unwrap();

        // Q should be orthogonal
        let qt = qr.q.parivart();
        let qtq = qt.gunaa(&qr.q).unwrap();

        // QtQ should be identity
        assert!((qtq.tatva[0] - 1.0).abs() < 1e-10);
        assert!((qtq.tatva[3] - 1.0).abs() < 1e-10);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_cholesky() {
        // Symmetric positive definite matrix
        let a = Aavyuha::naya(vec![4.0, 2.0, 2.0, 5.0], 2, 2);
        let chol = cholesky_vighatan(&a).unwrap();

        // L * L^T should equal A
        let lt = chol.l.parivart();
        let llt = chol.l.gunaa(&lt).unwrap();

        for i in 0..4 {
            assert!((llt.tatva[i] - a.tatva[i]).abs() < 1e-10);
        }
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_power_iteration() {
        let a = Aavyuha::naya(vec![2.0, 1.0, 1.0, 2.0], 2, 2);

        let (lambda, _v) = shakti_punaraavriti(&a, 100, 1e-10).unwrap();

        // Dominant eigenvalue should be 3
        assert!((lambda - 3.0).abs() < 1e-5);
    }
}
