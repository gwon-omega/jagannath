//! # Prakriti - Nature-Inspired Algorithms (प्रकृति)
//!
//! Swarm intelligence and other nature-inspired optimization algorithms.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// 2D position for swarm algorithms
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sthiti {
    pub x: f64,
    pub y: f64,
}

impl Sthiti {
    pub const fn nava(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn doori(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        libm::sqrt(dx * dx + dy * dy)
    }
}

/// Particle for PSO
#[derive(Debug, Clone)]
pub struct Kana {
    pub sthiti: Sthiti,
    pub veg: Sthiti,
    pub shreshtha_sthiti: Sthiti,
    pub shreshtha_yogyata: f64,
}

impl Kana {
    /// Create new particle
    pub fn nava(sthiti: Sthiti) -> Self {
        Self {
            sthiti,
            veg: Sthiti::nava(0.0, 0.0),
            shreshtha_sthiti: sthiti,
            shreshtha_yogyata: f64::NEG_INFINITY,
        }
    }

    /// Update personal best
    pub fn navikaran(&mut self, yogyata: f64) {
        if yogyata > self.shreshtha_yogyata {
            self.shreshtha_yogyata = yogyata;
            self.shreshtha_sthiti = self.sthiti;
        }
    }
}

/// Particle Swarm Optimization
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct KanaJhund {
    pub kana: Vec<Kana>,
    pub vaishvik_shreshtha: Sthiti,
    pub vaishvik_yogyata: f64,

    // Parameters
    pub omega: f64,                  // Inertia weight
    pub c1: f64,                     // Cognitive parameter
    pub c2: f64,                     // Social parameter
    pub seema: (f64, f64, f64, f64), // x_min, x_max, y_min, y_max
}

#[cfg(feature = "alloc")]
impl KanaJhund {
    /// Create PSO swarm
    pub fn nava(aakaar: usize, seema: (f64, f64, f64, f64)) -> Self {
        let (x_min, x_max, y_min, y_max) = seema;

        // Initialize particles in grid pattern
        let kana: Vec<Kana> = (0..aakaar)
            .map(|i| {
                let t = i as f64 / aakaar as f64;
                let x = x_min + t * (x_max - x_min);
                let y = y_min + t * (y_max - y_min);
                Kana::nava(Sthiti::nava(x, y))
            })
            .collect();

        Self {
            kana,
            vaishvik_shreshtha: Sthiti::nava(0.0, 0.0),
            vaishvik_yogyata: f64::NEG_INFINITY,
            omega: 0.7,
            c1: 1.5,
            c2: 1.5,
            seema,
        }
    }

    /// Set parameters
    pub fn params(mut self, omega: f64, c1: f64, c2: f64) -> Self {
        self.omega = omega;
        self.c1 = c1;
        self.c2 = c2;
        self
    }

    /// Update swarm
    pub fn charana<F>(&mut self, yogyata_fn: F)
    where
        F: Fn(f64, f64) -> f64,
    {
        let (x_min, x_max, y_min, y_max) = self.seema;

        // Evaluate and update personal/global best
        for kana in &mut self.kana {
            let fit = yogyata_fn(kana.sthiti.x, kana.sthiti.y);
            kana.navikaran(fit);

            if fit > self.vaishvik_yogyata {
                self.vaishvik_yogyata = fit;
                self.vaishvik_shreshtha = kana.sthiti;
            }
        }

        // Update velocities and positions
        for (i, kana) in self.kana.iter_mut().enumerate() {
            // Deterministic "random" factors based on index
            let r1 = (i as f64 * 0.1).fract();
            let r2 = (i as f64 * 0.17).fract();

            // Velocity update
            kana.veg.x = self.omega * kana.veg.x
                + self.c1 * r1 * (kana.shreshtha_sthiti.x - kana.sthiti.x)
                + self.c2 * r2 * (self.vaishvik_shreshtha.x - kana.sthiti.x);

            kana.veg.y = self.omega * kana.veg.y
                + self.c1 * r1 * (kana.shreshtha_sthiti.y - kana.sthiti.y)
                + self.c2 * r2 * (self.vaishvik_shreshtha.y - kana.sthiti.y);

            // Position update
            kana.sthiti.x = (kana.sthiti.x + kana.veg.x).clamp(x_min, x_max);
            kana.sthiti.y = (kana.sthiti.y + kana.veg.y).clamp(y_min, y_max);
        }
    }

    /// Run for iterations
    pub fn chalao<F>(&mut self, iterations: usize, yogyata_fn: F)
    where
        F: Fn(f64, f64) -> f64,
    {
        for _ in 0..iterations {
            self.charana(&yogyata_fn);
        }
    }
}

/// Ant for ACO
#[derive(Debug, Clone)]
pub struct Pipilika {
    pub marga: Vec<usize>,
    pub doori: f64,
}

impl Pipilika {
    pub fn nava() -> Self {
        Self {
            marga: Vec::new(),
            doori: 0.0,
        }
    }
}

/// Ant Colony Optimization (for TSP-like problems)
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct PipilikaVasahat {
    pub pheromone: Vec<Vec<f64>>,
    pub doori: Vec<Vec<f64>>,
    pub n: usize,

    // Parameters
    pub alpha: f64, // Pheromone importance
    pub beta: f64,  // Distance importance
    pub rho: f64,   // Evaporation rate
    pub q: f64,     // Pheromone deposit factor
}

#[cfg(feature = "alloc")]
impl PipilikaVasahat {
    /// Create ACO colony
    pub fn nava(doori: Vec<Vec<f64>>) -> Self {
        let n = doori.len();
        let pheromone = vec![vec![1.0; n]; n];

        Self {
            pheromone,
            doori,
            n,
            alpha: 1.0,
            beta: 2.0,
            rho: 0.5,
            q: 100.0,
        }
    }

    /// Build solution for one ant
    pub fn marga_banao(&self, praarambh: usize) -> Pipilika {
        let mut ant = Pipilika::nava();
        let mut bhraman = vec![false; self.n];

        let mut vartamaan = praarambh;
        ant.marga.push(vartamaan);
        bhraman[vartamaan] = true;

        while ant.marga.len() < self.n {
            // Find next city (deterministic: highest probability)
            let mut best_prob = -1.0;
            let mut best_city = 0;

            for j in 0..self.n {
                if !bhraman[j] {
                    let tau = self.pheromone[vartamaan][j];
                    let eta = if self.doori[vartamaan][j] > 0.0 {
                        1.0 / self.doori[vartamaan][j]
                    } else {
                        1.0
                    };

                    let prob = libm::pow(tau, self.alpha) * libm::pow(eta, self.beta);

                    if prob > best_prob {
                        best_prob = prob;
                        best_city = j;
                    }
                }
            }

            ant.doori += self.doori[vartamaan][best_city];
            vartamaan = best_city;
            ant.marga.push(vartamaan);
            bhraman[vartamaan] = true;
        }

        // Return to start
        ant.doori += self.doori[vartamaan][praarambh];

        ant
    }

    /// Update pheromones
    pub fn pheromone_navikaran(&mut self, ants: &[Pipilika]) {
        // Evaporation
        for i in 0..self.n {
            for j in 0..self.n {
                self.pheromone[i][j] *= 1.0 - self.rho;
            }
        }

        // Deposit
        for ant in ants {
            let deposit = self.q / ant.doori;
            for w in ant.marga.windows(2) {
                let i = w[0];
                let j = w[1];
                self.pheromone[i][j] += deposit;
                self.pheromone[j][i] += deposit;
            }
        }
    }

    /// Run iteration
    pub fn charana(&mut self, ant_count: usize) -> Pipilika {
        let mut ants = Vec::with_capacity(ant_count);

        for i in 0..ant_count {
            let start = i % self.n;
            ants.push(self.marga_banao(start));
        }

        let best = ants
            .iter()
            .min_by(|a, b| a.doori.partial_cmp(&b.doori).unwrap())
            .cloned()
            .unwrap();

        self.pheromone_navikaran(&ants);

        best
    }
}

/// Simulated Annealing state
#[derive(Debug, Clone)]
pub struct TaapaAnukrit {
    pub taapamaan: f64,     // Current temperature
    pub shitalan_dar: f64,  // Cooling rate
    pub nyunatam_taap: f64, // Minimum temperature
}

impl TaapaAnukrit {
    /// Create SA state
    pub fn nava(praarambhik_taap: f64, shitalan_dar: f64) -> Self {
        Self {
            taapamaan: praarambhik_taap,
            shitalan_dar,
            nyunatam_taap: 1e-10,
        }
    }

    /// Acceptance probability
    pub fn svikrti_sambhavya(&self, vartamaan_oorja: f64, naya_oorja: f64) -> f64 {
        if naya_oorja < vartamaan_oorja {
            1.0
        } else {
            libm::exp((vartamaan_oorja - naya_oorja) / self.taapamaan)
        }
    }

    /// Cool down
    pub fn shitalan(&mut self) {
        self.taapamaan *= self.shitalan_dar;
    }

    /// Check if frozen
    pub fn jamit(&self) -> bool {
        self.taapamaan < self.nyunatam_taap
    }
}

/// Hill climbing state
#[derive(Debug, Clone)]
pub struct PahaadiCharhai<T> {
    pub vartamaan: T,
    pub vartamaan_yogyata: f64,
    pub shreshtha: T,
    pub shreshtha_yogyata: f64,
}

impl<T: Clone> PahaadiCharhai<T> {
    /// Create hill climbing state
    pub fn nava(praarambh: T, yogyata: f64) -> Self {
        Self {
            vartamaan: praarambh.clone(),
            vartamaan_yogyata: yogyata,
            shreshtha: praarambh,
            shreshtha_yogyata: yogyata,
        }
    }

    /// Try move to neighbor
    pub fn prayas(&mut self, padosi: T, padosi_yogyata: f64) -> bool {
        if padosi_yogyata > self.vartamaan_yogyata {
            self.vartamaan = padosi.clone();
            self.vartamaan_yogyata = padosi_yogyata;

            if padosi_yogyata > self.shreshtha_yogyata {
                self.shreshtha = padosi;
                self.shreshtha_yogyata = padosi_yogyata;
            }
            true
        } else {
            false
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
#[cfg(feature = "alloc")]
mod tests {
    use super::*;

    #[test]
    fn test_pso() {
        let mut swarm = KanaJhund::nava(10, (-5.0, 5.0, -5.0, 5.0));

        // Sphere function (minimum at origin)
        let fitness = |x: f64, y: f64| -(x * x + y * y);

        swarm.chalao(50, fitness);

        // Should converge near origin
        assert!(swarm.vaishvik_shreshtha.x.abs() < 2.0);
        assert!(swarm.vaishvik_shreshtha.y.abs() < 2.0);
    }

    #[test]
    fn test_sa_acceptance() {
        let sa = TaapaAnukrit::nava(100.0, 0.9);

        // Better solution always accepted
        assert!((sa.svikrti_sambhavya(10.0, 5.0) - 1.0).abs() < 1e-10);

        // Worse solution has probability < 1
        let prob = sa.svikrti_sambhavya(5.0, 10.0);
        assert!(prob < 1.0);
        assert!(prob > 0.0);
    }
}
