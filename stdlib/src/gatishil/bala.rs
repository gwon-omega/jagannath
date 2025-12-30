//! # Bala - Forces (बल)
//!
//! Physical forces and mechanics.
//!
//! > **"बलं क्रियायाः कारणम्"**
//! > *"Force is the cause of action"*

use super::sadish::{Sadish2, Sadish3};

// ============================================================================
// PHYSICAL CONSTANTS
// ============================================================================

/// Physical constants (स्थिरांक)
pub mod sthirank {
    /// Gravitational constant G (m³/kg·s²)
    pub const GURUTVA_STHIRANK: f64 = 6.67430e-11;

    /// Speed of light c (m/s)
    pub const PRAKASH_GATI: f64 = 299_792_458.0;

    /// Planck constant h (J·s)
    pub const PLANCK_STHIRANK: f64 = 6.62607015e-34;

    /// Boltzmann constant k (J/K)
    pub const BOLTZMANN_STHIRANK: f64 = 1.380649e-23;

    /// Elementary charge e (C)
    pub const PRATHAMIK_AVESH: f64 = 1.602176634e-19;

    /// Electron mass (kg)
    pub const ELECTRON_DRUVYAMAN: f64 = 9.1093837015e-31;

    /// Proton mass (kg)
    pub const PROTON_DRUVYAMAN: f64 = 1.67262192369e-27;

    /// Avogadro constant (1/mol)
    pub const AVOGADRO_STHIRANK: f64 = 6.02214076e23;

    /// Gas constant R (J/mol·K)
    pub const GAS_STHIRANK: f64 = 8.314462618;

    /// Standard gravity (m/s²)
    pub const MANAKA_GURUTVA: f64 = 9.80665;

    /// Atmospheric pressure (Pa)
    pub const VATAVARAN_DAB: f64 = 101_325.0;

    /// Stefan-Boltzmann constant (W/m²·K⁴)
    pub const STEFAN_BOLTZMANN: f64 = 5.670374419e-8;

    /// Vacuum permittivity ε₀ (F/m)
    pub const NIRVATANUSHILTA: f64 = 8.8541878128e-12;

    /// Vacuum permeability μ₀ (H/m)
    pub const CHUMBAKIYA_ANUSHILTA: f64 = 1.25663706212e-6;
}

// ============================================================================
// FORCE TYPES
// ============================================================================

/// 2D force
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Bala2 {
    pub sadish: Sadish2,
}

impl Bala2 {
    /// Create new force
    pub fn nava(x: f64, y: f64) -> Self {
        Self {
            sadish: Sadish2::nava(x, y),
        }
    }

    /// Zero force
    pub fn shunya() -> Self {
        Self {
            sadish: Sadish2::shunya(),
        }
    }

    /// Create from magnitude and angle
    pub fn parimaan_kon(parimaan: f64, kon: f64) -> Self {
        Self {
            sadish: Sadish2::kon_se(kon) * parimaan,
        }
    }

    /// Get magnitude (N)
    pub fn parimaan(&self) -> f64 {
        self.sadish.parimaan()
    }

    /// Get direction angle (radians)
    pub fn disha(&self) -> f64 {
        self.sadish.kon()
    }

    /// Add forces
    pub fn jod(&self, other: &Self) -> Self {
        Self {
            sadish: self.sadish + other.sadish,
        }
    }

    /// Scale force
    pub fn guna(&self, factor: f64) -> Self {
        Self {
            sadish: self.sadish * factor,
        }
    }

    /// Get acceleration (a = F/m)
    pub fn tvarana(&self, druvyaman: f64) -> Sadish2 {
        self.sadish / druvyaman
    }
}

/// 3D force
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Bala3 {
    pub sadish: Sadish3,
}

impl Bala3 {
    /// Create new force
    pub fn nava(x: f64, y: f64, z: f64) -> Self {
        Self {
            sadish: Sadish3::nava(x, y, z),
        }
    }

    /// Zero force
    pub fn shunya() -> Self {
        Self {
            sadish: Sadish3::shunya(),
        }
    }

    /// Get magnitude (N)
    pub fn parimaan(&self) -> f64 {
        self.sadish.parimaan()
    }

    /// Add forces
    pub fn jod(&self, other: &Self) -> Self {
        Self {
            sadish: self.sadish + other.sadish,
        }
    }

    /// Get acceleration
    pub fn tvarana(&self, druvyaman: f64) -> Sadish3 {
        self.sadish / druvyaman
    }

    /// Calculate torque about origin
    pub fn bal_aghurn(&self, baahu: &Sadish3) -> Sadish3 {
        baahu.kross_gunaa(&self.sadish)
    }
}

// ============================================================================
// COMMON FORCES
// ============================================================================

/// Gravitational force between two masses
/// F = G * m1 * m2 / r²
pub fn gurutviya_bala(m1: f64, m2: f64, doori: f64) -> f64 {
    if doori == 0.0 {
        return 0.0;
    }
    sthirank::GURUTVA_STHIRANK * m1 * m2 / (doori * doori)
}

/// Weight force (F = mg)
pub fn bhar(druvyaman: f64) -> f64 {
    druvyaman * sthirank::MANAKA_GURUTVA
}

/// Weight force with custom gravity
pub fn bhar_anyatra(druvyaman: f64, g: f64) -> f64 {
    druvyaman * g
}

/// Spring force (Hooke's law: F = -kx)
pub fn spring_bala(sthirank_k: f64, visthaapan: f64) -> f64 {
    -sthirank_k * visthaapan
}

/// Spring force 2D
pub fn spring_bala_2d(sthirank_k: f64, visthaapan: Sadish2) -> Bala2 {
    Bala2 {
        sadish: visthaapan * (-sthirank_k),
    }
}

/// Friction force
/// F = μN
pub fn gharshan_bala(mu: f64, normal_bala: f64) -> f64 {
    mu * normal_bala
}

/// Drag force (air resistance)
/// F = 0.5 * ρ * v² * Cd * A
pub fn vayu_pratirrodh(
    ghantva: f64,     // ρ (density)
    veg: f64,         // v (velocity)
    drag_gunank: f64, // Cd
    kshetraphal: f64, // A (area)
) -> f64 {
    0.5 * ghantva * veg * veg * drag_gunank * kshetraphal
}

/// Centripetal force
/// F = mv²/r
pub fn kendrabimukhi_bala(druvyaman: f64, veg: f64, trijya: f64) -> f64 {
    if trijya == 0.0 {
        return 0.0;
    }
    druvyaman * veg * veg / trijya
}

/// Coulomb force between charges
/// F = k * q1 * q2 / r²
pub fn coulomb_bala(q1: f64, q2: f64, doori: f64) -> f64 {
    if doori == 0.0 {
        return 0.0;
    }
    let k = 1.0 / (4.0 * core::f64::consts::PI * sthirank::NIRVATANUSHILTA);
    k * q1 * q2 / (doori * doori)
}

// ============================================================================
// ENERGY
// ============================================================================

/// Kinetic energy (KE = 0.5 * m * v²)
pub fn gatij_oorja(druvyaman: f64, veg: f64) -> f64 {
    0.5 * druvyaman * veg * veg
}

/// Potential energy (gravitational, PE = mgh)
pub fn sthitij_oorja(druvyaman: f64, oonchaayi: f64) -> f64 {
    druvyaman * sthirank::MANAKA_GURUTVA * oonchaayi
}

/// Spring potential energy (PE = 0.5 * k * x²)
pub fn spring_sthitij_oorja(sthirank_k: f64, visthaapan: f64) -> f64 {
    0.5 * sthirank_k * visthaapan * visthaapan
}

/// Work done (W = F · d)
pub fn karya(bala: f64, doori: f64, kon: f64) -> f64 {
    bala * doori * libm::cos(kon)
}

/// Power (P = W/t = F·v)
pub fn shakti(bala: f64, veg: f64) -> f64 {
    bala * veg
}

// ============================================================================
// MOMENTUM
// ============================================================================

/// Linear momentum (p = mv)
pub fn sanveg(druvyaman: f64, veg: f64) -> f64 {
    druvyaman * veg
}

/// Impulse (J = F·Δt = Δp)
pub fn aaveg(bala: f64, samay: f64) -> f64 {
    bala * samay
}

/// Velocity after elastic collision
pub fn elastic_takkar_veg(m1: f64, v1: f64, m2: f64, v2: f64) -> (f64, f64) {
    let total_m = m1 + m2;
    let v1_final = ((m1 - m2) * v1 + 2.0 * m2 * v2) / total_m;
    let v2_final = ((m2 - m1) * v2 + 2.0 * m1 * v1) / total_m;
    (v1_final, v2_final)
}

// ============================================================================
// PRESSURE
// ============================================================================

/// Pressure (P = F/A)
pub fn daab(bala: f64, kshetraphal: f64) -> f64 {
    if kshetraphal == 0.0 {
        return 0.0;
    }
    bala / kshetraphal
}

/// Hydrostatic pressure (P = ρgh)
pub fn dravit_daab(ghantva: f64, gehrai: f64) -> f64 {
    ghantva * sthirank::MANAKA_GURUTVA * gehrai
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weight() {
        let w = bhar(1.0);
        assert!((w - 9.80665).abs() < 1e-5);
    }

    #[test]
    fn test_spring() {
        let f = spring_bala(100.0, 0.1);
        assert!((f - (-10.0)).abs() < 1e-10);
    }

    #[test]
    fn test_kinetic_energy() {
        let ke = gatij_oorja(2.0, 3.0);
        assert!((ke - 9.0).abs() < 1e-10);
    }

    #[test]
    fn test_potential_energy() {
        let pe = sthitij_oorja(1.0, 10.0);
        assert!((pe - 98.0665).abs() < 1e-3);
    }

    #[test]
    fn test_momentum() {
        let p = sanveg(2.0, 5.0);
        assert!((p - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_centripetal() {
        let f = kendrabimukhi_bala(1.0, 10.0, 5.0);
        assert!((f - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_force_addition() {
        let f1 = Bala2::nava(3.0, 0.0);
        let f2 = Bala2::nava(0.0, 4.0);
        let total = f1.jod(&f2);
        assert!((total.parimaan() - 5.0).abs() < 1e-10);
    }
}
