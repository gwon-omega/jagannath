//! # Gati - Motion (गति)
//!
//! Kinematics and motion equations.
//!
//! > **"गतिः जीवनस्य लक्षणम्"**
//! > *"Motion is the sign of life"*

use super::bala::sthirank;
use super::sadish::{Sadish2, Sadish3};

// ============================================================================
// 1D KINEMATICS
// ============================================================================

/// Position from uniform motion (s = s₀ + vt)
pub fn ekasaman_sthiti(s0: f64, v: f64, t: f64) -> f64 {
    s0 + v * t
}

/// Position from uniformly accelerated motion (s = s₀ + v₀t + ½at²)
pub fn tvarit_sthiti(s0: f64, v0: f64, a: f64, t: f64) -> f64 {
    s0 + v0 * t + 0.5 * a * t * t
}

/// Velocity from acceleration (v = v₀ + at)
pub fn tvarit_veg(v0: f64, a: f64, t: f64) -> f64 {
    v0 + a * t
}

/// Velocity from displacement (v² = v₀² + 2as)
pub fn visthaapan_veg(v0: f64, a: f64, s: f64) -> f64 {
    let v_sq = v0 * v0 + 2.0 * a * s;
    if v_sq >= 0.0 {
        libm::sqrt(v_sq)
    } else {
        -libm::sqrt(-v_sq)
    }
}

/// Time to reach position
pub fn sthiti_samay(s0: f64, s: f64, v: f64) -> f64 {
    if v == 0.0 {
        return f64::INFINITY;
    }
    (s - s0) / v
}

/// Free fall position (h = h₀ - ½gt²)
pub fn mukta_patan_oonchaayi(h0: f64, t: f64) -> f64 {
    h0 - 0.5 * sthirank::MANAKA_GURUTVA * t * t
}

/// Free fall velocity (v = gt)
pub fn mukta_patan_veg(t: f64) -> f64 {
    sthirank::MANAKA_GURUTVA * t
}

/// Time to fall height h (t = √(2h/g))
pub fn patan_samay(h: f64) -> f64 {
    if h <= 0.0 {
        return 0.0;
    }
    libm::sqrt(2.0 * h / sthirank::MANAKA_GURUTVA)
}

// ============================================================================
// PROJECTILE MOTION
// ============================================================================

/// Projectile state at time t
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrakshepyaSthiti {
    /// Position (x, y)
    pub sthiti: Sadish2,
    /// Velocity (vx, vy)
    pub veg: Sadish2,
    /// Time
    pub samay: f64,
}

/// Calculate projectile state at time t
/// Initial velocity v₀ at angle θ
pub fn prakshepya_sthiti(v0: f64, kon: f64, t: f64) -> PrakshepyaSthiti {
    let vx = v0 * libm::cos(kon);
    let vy = v0 * libm::sin(kon);
    let g = sthirank::MANAKA_GURUTVA;

    PrakshepyaSthiti {
        sthiti: Sadish2::nava(vx * t, vy * t - 0.5 * g * t * t),
        veg: Sadish2::nava(vx, vy - g * t),
        samay: t,
    }
}

/// Maximum height of projectile
pub fn prakshepya_adhiktam_oonchaayi(v0: f64, kon: f64) -> f64 {
    let vy = v0 * libm::sin(kon);
    vy * vy / (2.0 * sthirank::MANAKA_GURUTVA)
}

/// Range of projectile (horizontal distance)
pub fn prakshepya_paridhi(v0: f64, kon: f64) -> f64 {
    let sin2theta = libm::sin(2.0 * kon);
    v0 * v0 * sin2theta / sthirank::MANAKA_GURUTVA
}

/// Time of flight
pub fn prakshepya_udan_samay(v0: f64, kon: f64) -> f64 {
    let vy = v0 * libm::sin(kon);
    2.0 * vy / sthirank::MANAKA_GURUTVA
}

/// Optimal angle for maximum range (45°)
pub const ADHIKTAM_PARIDHI_KON: f64 = core::f64::consts::FRAC_PI_4;

// ============================================================================
// CIRCULAR MOTION
// ============================================================================

/// Angular velocity (ω = v/r)
pub fn koniya_veg(v: f64, r: f64) -> f64 {
    if r == 0.0 {
        return 0.0;
    }
    v / r
}

/// Linear velocity from angular (v = ωr)
pub fn rekhik_veg(omega: f64, r: f64) -> f64 {
    omega * r
}

/// Centripetal acceleration (a = v²/r = ω²r)
pub fn kendrabimukhi_tvarana(v: f64, r: f64) -> f64 {
    if r == 0.0 {
        return 0.0;
    }
    v * v / r
}

/// Period of circular motion (T = 2πr/v = 2π/ω)
pub fn avadhi(omega: f64) -> f64 {
    if omega == 0.0 {
        return f64::INFINITY;
    }
    2.0 * core::f64::consts::PI / omega
}

/// Frequency (f = 1/T = ω/2π)
pub fn avritti(omega: f64) -> f64 {
    omega / (2.0 * core::f64::consts::PI)
}

/// Position in circular motion
pub fn vritiya_sthiti(r: f64, omega: f64, t: f64, phi: f64) -> Sadish2 {
    let theta = omega * t + phi;
    Sadish2::nava(r * libm::cos(theta), r * libm::sin(theta))
}

/// Velocity in circular motion
pub fn vritiya_veg(r: f64, omega: f64, t: f64, phi: f64) -> Sadish2 {
    let theta = omega * t + phi;
    Sadish2::nava(-r * omega * libm::sin(theta), r * omega * libm::cos(theta))
}

// ============================================================================
// SIMPLE HARMONIC MOTION
// ============================================================================

/// Position in SHM (x = A cos(ωt + φ))
pub fn shm_sthiti(aayaam: f64, omega: f64, t: f64, phi: f64) -> f64 {
    aayaam * libm::cos(omega * t + phi)
}

/// Velocity in SHM (v = -Aω sin(ωt + φ))
pub fn shm_veg(aayaam: f64, omega: f64, t: f64, phi: f64) -> f64 {
    -aayaam * omega * libm::sin(omega * t + phi)
}

/// Acceleration in SHM (a = -Aω² cos(ωt + φ) = -ω²x)
pub fn shm_tvarana(aayaam: f64, omega: f64, t: f64, phi: f64) -> f64 {
    -aayaam * omega * omega * libm::cos(omega * t + phi)
}

/// Period of spring-mass system (T = 2π√(m/k))
pub fn spring_avadhi(druvyaman: f64, sthirank_k: f64) -> f64 {
    2.0 * core::f64::consts::PI * libm::sqrt(druvyaman / sthirank_k)
}

/// Period of simple pendulum (T = 2π√(L/g))
pub fn dolak_avadhi(lambai: f64) -> f64 {
    2.0 * core::f64::consts::PI * libm::sqrt(lambai / sthirank::MANAKA_GURUTVA)
}

// ============================================================================
// 2D/3D MOTION
// ============================================================================

/// 2D particle state
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Kana2Sthiti {
    pub sthiti: Sadish2,
    pub veg: Sadish2,
    pub tvarana: Sadish2,
}

impl Kana2Sthiti {
    /// Create new particle state
    pub fn nava(sthiti: Sadish2, veg: Sadish2, tvarana: Sadish2) -> Self {
        Self {
            sthiti,
            veg,
            tvarana,
        }
    }

    /// Create at rest
    pub fn vishram(sthiti: Sadish2) -> Self {
        Self {
            sthiti,
            veg: Sadish2::shunya(),
            tvarana: Sadish2::shunya(),
        }
    }

    /// Update state after time dt (Euler integration)
    pub fn agami(&self, dt: f64) -> Self {
        Self {
            sthiti: self.sthiti + self.veg * dt + self.tvarana * (0.5 * dt * dt),
            veg: self.veg + self.tvarana * dt,
            tvarana: self.tvarana,
        }
    }

    /// Get speed
    pub fn drutata(&self) -> f64 {
        self.veg.parimaan()
    }

    /// Get kinetic energy
    pub fn gatij_oorja(&self, druvyaman: f64) -> f64 {
        0.5 * druvyaman * self.veg.parimaan_varg()
    }
}

/// 3D particle state
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Kana3Sthiti {
    pub sthiti: Sadish3,
    pub veg: Sadish3,
    pub tvarana: Sadish3,
}

impl Kana3Sthiti {
    /// Create new particle state
    pub fn nava(sthiti: Sadish3, veg: Sadish3, tvarana: Sadish3) -> Self {
        Self {
            sthiti,
            veg,
            tvarana,
        }
    }

    /// Update state after time dt
    pub fn agami(&self, dt: f64) -> Self {
        Self {
            sthiti: self.sthiti + self.veg * dt + self.tvarana * (0.5 * dt * dt),
            veg: self.veg + self.tvarana * dt,
            tvarana: self.tvarana,
        }
    }

    /// Apply force
    pub fn bala_lagu(&self, bala: Sadish3, druvyaman: f64, dt: f64) -> Self {
        let tvarana = bala / druvyaman;
        Self {
            sthiti: self.sthiti + self.veg * dt + tvarana * (0.5 * dt * dt),
            veg: self.veg + tvarana * dt,
            tvarana,
        }
    }
}

// ============================================================================
// INTEGRATION METHODS
// ============================================================================

/// Verlet integration step
pub fn verlet_charana(
    sthiti: Sadish2,
    purav_sthiti: Sadish2,
    tvarana: Sadish2,
    dt: f64,
) -> Sadish2 {
    sthiti * 2.0 - purav_sthiti + tvarana * (dt * dt)
}

/// RK4 integration for 1D (returns new position and velocity)
pub fn rk4_charana(x: f64, v: f64, tvarana_fn: impl Fn(f64, f64) -> f64, dt: f64) -> (f64, f64) {
    let k1v = tvarana_fn(x, v);
    let k1x = v;

    let k2v = tvarana_fn(x + 0.5 * dt * k1x, v + 0.5 * dt * k1v);
    let k2x = v + 0.5 * dt * k1v;

    let k3v = tvarana_fn(x + 0.5 * dt * k2x, v + 0.5 * dt * k2v);
    let k3x = v + 0.5 * dt * k2v;

    let k4v = tvarana_fn(x + dt * k3x, v + dt * k3v);
    let k4x = v + dt * k3v;

    let new_x = x + (dt / 6.0) * (k1x + 2.0 * k2x + 2.0 * k3x + k4x);
    let new_v = v + (dt / 6.0) * (k1v + 2.0 * k2v + 2.0 * k3v + k4v);

    (new_x, new_v)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_motion() {
        let s = ekasaman_sthiti(0.0, 10.0, 5.0);
        assert!((s - 50.0).abs() < 1e-10);
    }

    #[test]
    fn test_accelerated_motion() {
        // v = 0, a = 10, t = 2 => s = 0.5 * 10 * 4 = 20
        let s = tvarit_sthiti(0.0, 0.0, 10.0, 2.0);
        assert!((s - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_free_fall() {
        let t = patan_samay(10.0);
        // t = √(2h/g) ≈ 1.43s
        assert!((t - 1.4278431).abs() < 0.001);
    }

    #[test]
    fn test_projectile_range() {
        let range = prakshepya_paridhi(20.0, ADHIKTAM_PARIDHI_KON);
        // R = v²/g ≈ 40.77m at 45°
        assert!((range - 40.77).abs() < 0.1);
    }

    #[test]
    fn test_circular_motion() {
        let omega = koniya_veg(10.0, 2.0);
        assert!((omega - 5.0).abs() < 1e-10);

        let v = rekhik_veg(5.0, 2.0);
        assert!((v - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_shm() {
        // At t=0, φ=0: x = A, v = 0
        let x = shm_sthiti(5.0, 2.0, 0.0, 0.0);
        let v = shm_veg(5.0, 2.0, 0.0, 0.0);

        assert!((x - 5.0).abs() < 1e-10);
        assert!(v.abs() < 1e-10);
    }

    #[test]
    fn test_particle_update() {
        let p = Kana2Sthiti::nava(
            Sadish2::shunya(),
            Sadish2::nava(10.0, 0.0),
            Sadish2::shunya(),
        );

        let p2 = p.agami(1.0);
        assert!((p2.sthiti.x - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_pendulum_period() {
        let t = dolak_avadhi(1.0);
        // T ≈ 2.006s for 1m pendulum
        assert!((t - 2.006).abs() < 0.01);
    }
}
