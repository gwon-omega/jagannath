//! # Anuwanshik - Genetic Algorithms (अनुवांशिक)
//!
//! Genetic algorithm primitives and utilities.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Gene trait for genetic algorithm chromosomes
pub trait Gene: Clone {
    /// Random mutation
    fn uttparivartan(&mut self, dar: f64);

    /// Create random gene
    fn yaadrichik() -> Self;
}

/// Fitness function trait
pub trait Yogyata {
    /// Calculate fitness (higher is better)
    fn yogyata(&self) -> f64;
}

/// Selection method
#[derive(Debug, Clone, Copy)]
pub enum ChayanaVidhi {
    /// Tournament selection
    Pratiyogita(usize),
    /// Roulette wheel selection
    Chakra,
    /// Rank-based selection
    Shrenibaddha,
    /// Elitism (keep top n)
    Shreshtha(usize),
}

/// Crossover method
#[derive(Debug, Clone, Copy)]
pub enum Sankar {
    /// Single point crossover
    EkBindu,
    /// Two point crossover
    DviBindu,
    /// Uniform crossover
    Samana(f64), // probability
}

/// Chromosome (collection of genes)
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct Gunsutra<G: Gene> {
    pub genes: Vec<G>,
    pub yogyata: Option<f64>,
}

#[cfg(feature = "alloc")]
impl<G: Gene> Gunsutra<G> {
    /// Create new chromosome
    pub fn nava(genes: Vec<G>) -> Self {
        Self {
            genes,
            yogyata: None,
        }
    }

    /// Create random chromosome
    pub fn yaadrichik(lambai: usize) -> Self {
        let genes: Vec<G> = (0..lambai).map(|_| G::yaadrichik()).collect();
        Self {
            genes,
            yogyata: None,
        }
    }

    /// Mutate chromosome
    pub fn uttparivartan(&mut self, dar: f64) {
        for gene in &mut self.genes {
            gene.uttparivartan(dar);
        }
        self.yogyata = None;
    }

    /// Single point crossover
    pub fn ek_bindu_sankar(&self, other: &Self) -> (Self, Self) {
        let len = self.genes.len().min(other.genes.len());
        if len == 0 {
            return (self.clone(), other.clone());
        }

        // Simple deterministic crossover point
        let point = len / 2;

        let mut child1 = Vec::with_capacity(len);
        let mut child2 = Vec::with_capacity(len);

        for i in 0..len {
            if i < point {
                child1.push(self.genes[i].clone());
                child2.push(other.genes[i].clone());
            } else {
                child1.push(other.genes[i].clone());
                child2.push(self.genes[i].clone());
            }
        }

        (Self::nava(child1), Self::nava(child2))
    }

    /// Two point crossover
    pub fn dvi_bindu_sankar(&self, other: &Self) -> (Self, Self) {
        let len = self.genes.len().min(other.genes.len());
        if len < 2 {
            return self.ek_bindu_sankar(other);
        }

        let p1 = len / 3;
        let p2 = 2 * len / 3;

        let mut child1 = Vec::with_capacity(len);
        let mut child2 = Vec::with_capacity(len);

        for i in 0..len {
            if i < p1 || i >= p2 {
                child1.push(self.genes[i].clone());
                child2.push(other.genes[i].clone());
            } else {
                child1.push(other.genes[i].clone());
                child2.push(self.genes[i].clone());
            }
        }

        (Self::nava(child1), Self::nava(child2))
    }
}

/// Population of chromosomes
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct Janasankhya<G: Gene> {
    pub jantar: Vec<Gunsutra<G>>,
    pub pidhi: usize,
}

#[cfg(feature = "alloc")]
impl<G: Gene> Janasankhya<G> {
    /// Create new population
    pub fn nava(jantar: Vec<Gunsutra<G>>) -> Self {
        Self { jantar, pidhi: 0 }
    }

    /// Create random population
    pub fn yaadrichik(aakaar: usize, gunsutra_lambai: usize) -> Self {
        let jantar = (0..aakaar)
            .map(|_| Gunsutra::yaadrichik(gunsutra_lambai))
            .collect();
        Self { jantar, pidhi: 0 }
    }

    /// Evaluate fitness for all
    pub fn mulyankan<F>(&mut self, mut yogyata_fn: F)
    where
        F: FnMut(&Gunsutra<G>) -> f64,
    {
        for indiv in &mut self.jantar {
            if indiv.yogyata.is_none() {
                indiv.yogyata = Some(yogyata_fn(indiv));
            }
        }
    }

    /// Get best individual
    pub fn shreshtha(&self) -> Option<&Gunsutra<G>> {
        self.jantar
            .iter()
            .filter(|c| c.yogyata.is_some())
            .max_by(|a, b| {
                a.yogyata
                    .unwrap()
                    .partial_cmp(&b.yogyata.unwrap())
                    .unwrap_or(core::cmp::Ordering::Equal)
            })
    }

    /// Get worst individual
    pub fn nimna(&self) -> Option<&Gunsutra<G>> {
        self.jantar
            .iter()
            .filter(|c| c.yogyata.is_some())
            .min_by(|a, b| {
                a.yogyata
                    .unwrap()
                    .partial_cmp(&b.yogyata.unwrap())
                    .unwrap_or(core::cmp::Ordering::Equal)
            })
    }

    /// Average fitness
    pub fn madhya_yogyata(&self) -> Option<f64> {
        let valid: Vec<_> = self.jantar.iter().filter_map(|c| c.yogyata).collect();

        if valid.is_empty() {
            return None;
        }

        Some(valid.iter().sum::<f64>() / valid.len() as f64)
    }

    /// Sort by fitness (descending)
    pub fn kramit(&mut self) {
        self.jantar.sort_by(|a, b| {
            b.yogyata
                .unwrap_or(f64::NEG_INFINITY)
                .partial_cmp(&a.yogyata.unwrap_or(f64::NEG_INFINITY))
                .unwrap_or(core::cmp::Ordering::Equal)
        });
    }

    /// Selection - tournament
    pub fn pratiyogita_chayan(&self, _tournament_size: usize) -> Option<&Gunsutra<G>> {
        // Simple: return best (without randomness)
        self.shreshtha()
    }

    /// Evolution step
    pub fn vikas<F>(
        &mut self,
        yogyata_fn: F,
        uttparivartan_dar: f64,
        sankar_vidhi: Sankar,
        shreshtha_sankhya: usize,
    ) where
        F: FnMut(&Gunsutra<G>) -> f64 + Clone,
    {
        // Evaluate
        self.mulyankan(yogyata_fn.clone());
        self.kramit();

        let pop_size = self.jantar.len();

        // Keep elites
        let mut new_pop: Vec<Gunsutra<G>> = self
            .jantar
            .iter()
            .take(shreshtha_sankhya)
            .cloned()
            .collect();

        // Generate offspring
        while new_pop.len() < pop_size {
            // Simple selection: pick from top half
            let idx1 = (new_pop.len() % (pop_size / 2).max(1));
            let idx2 = ((new_pop.len() + 1) % (pop_size / 2).max(1));

            let parent1 = &self.jantar[idx1];
            let parent2 = &self.jantar[idx2];

            let (mut child1, mut child2) = match sankar_vidhi {
                Sankar::EkBindu => parent1.ek_bindu_sankar(parent2),
                Sankar::DviBindu => parent1.dvi_bindu_sankar(parent2),
                Sankar::Samana(_) => parent1.ek_bindu_sankar(parent2),
            };

            child1.uttparivartan(uttparivartan_dar);
            child2.uttparivartan(uttparivartan_dar);

            new_pop.push(child1);
            if new_pop.len() < pop_size {
                new_pop.push(child2);
            }
        }

        self.jantar = new_pop;
        self.pidhi += 1;
    }
}

/// Binary gene implementation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DvitvGene(pub bool);

impl Gene for DvitvGene {
    fn uttparivartan(&mut self, dar: f64) {
        // Deterministic based on rate threshold
        if dar > 0.5 {
            self.0 = !self.0;
        }
    }

    fn yaadrichik() -> Self {
        Self(true) // Default
    }
}

/// Float gene (0.0 to 1.0)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DashamlavaGene(pub f64);

impl Gene for DashamlavaGene {
    fn uttparivartan(&mut self, dar: f64) {
        // Simple perturbation
        let delta = dar * 0.1;
        self.0 = (self.0 + delta).clamp(0.0, 1.0);
    }

    fn yaadrichik() -> Self {
        Self(0.5) // Default to middle
    }
}

/// Integer gene
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PurnankGene {
    pub mana: i64,
    pub nyunatam: i64,
    pub adhikatam: i64,
}

impl PurnankGene {
    pub fn nava(mana: i64, nyunatam: i64, adhikatam: i64) -> Self {
        Self {
            mana,
            nyunatam,
            adhikatam,
        }
    }
}

impl Gene for PurnankGene {
    fn uttparivartan(&mut self, dar: f64) {
        let range = self.adhikatam - self.nyunatam;
        let delta = (dar * range as f64) as i64;
        self.mana = (self.mana + delta).clamp(self.nyunatam, self.adhikatam);
    }

    fn yaadrichik() -> Self {
        Self {
            mana: 0,
            nyunatam: 0,
            adhikatam: 100,
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
    fn test_chromosome_crossover() {
        let c1 = Gunsutra::nava(vec![
            DvitvGene(true),
            DvitvGene(true),
            DvitvGene(true),
            DvitvGene(true),
        ]);
        let c2 = Gunsutra::nava(vec![
            DvitvGene(false),
            DvitvGene(false),
            DvitvGene(false),
            DvitvGene(false),
        ]);

        let (child1, child2) = c1.ek_bindu_sankar(&c2);

        assert_eq!(child1.genes.len(), 4);
        assert_eq!(child2.genes.len(), 4);
    }

    #[test]
    fn test_population() {
        let mut pop = Janasankhya::<DvitvGene>::yaadrichik(10, 5);

        // Simple fitness: count true genes
        pop.mulyankan(|c| c.genes.iter().filter(|g| g.0).count() as f64);

        let best = pop.shreshtha().unwrap();
        assert!(best.yogyata.is_some());
    }
}
