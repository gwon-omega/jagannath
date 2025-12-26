//! Vedic Constant Folder (वेदगणित अचर-संक्षेपक)
//!
//! Compile-time constant folding using Vedic math sūtras.
//! Applies algebraic optimizations and pattern matching to
//! reduce expressions to constants where possible.

use super::{Sutra, VedicMath};

/// Vedic constant folder - optimizes expressions at compile time
pub struct VedicConstantFolder {
    /// Statistics: how many folds performed
    pub folds_performed: usize,
    /// Which sūtras were used
    pub sutras_used: Vec<Sutra>,
}

/// An expression that can be constant-folded
#[derive(Debug, Clone)]
pub enum Expr {
    /// Integer literal
    Int(i64),
    /// Float literal
    Float(f64),
    /// Boolean literal
    Bool(bool),
    /// Binary operation
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    /// Unary operation
    UnaryOp(UnaryOp, Box<Expr>),
    /// Variable (cannot fold)
    Var(String),
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    And,
    Or,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
    Sqrt,
}

impl VedicConstantFolder {
    pub fn new() -> Self {
        Self {
            folds_performed: 0,
            sutras_used: Vec::new(),
        }
    }

    /// Fold an expression to a constant if possible
    pub fn fold(&mut self, expr: &Expr) -> Expr {
        match expr {
            Expr::Int(_) | Expr::Float(_) | Expr::Bool(_) | Expr::Var(_) => expr.clone(),

            Expr::BinOp(left, op, right) => {
                let left_folded = self.fold(left);
                let right_folded = self.fold(right);
                self.fold_binop(&left_folded, *op, &right_folded)
            }

            Expr::UnaryOp(op, inner) => {
                let inner_folded = self.fold(inner);
                self.fold_unary(*op, &inner_folded)
            }
        }
    }

    /// Fold binary operation
    fn fold_binop(&mut self, left: &Expr, op: BinOp, right: &Expr) -> Expr {
        match (left, op, right) {
            // Integer arithmetic
            (Expr::Int(a), BinOp::Add, Expr::Int(b)) => {
                self.folds_performed += 1;
                Expr::Int(a + b)
            }
            (Expr::Int(a), BinOp::Sub, Expr::Int(b)) => {
                self.folds_performed += 1;
                Expr::Int(a - b)
            }
            (Expr::Int(a), BinOp::Mul, Expr::Int(b)) => {
                self.folds_performed += 1;
                self.vedic_multiply(*a, *b)
            }
            (Expr::Int(a), BinOp::Div, Expr::Int(b)) if *b != 0 => {
                self.folds_performed += 1;
                Expr::Int(a / b)
            }
            (Expr::Int(a), BinOp::Mod, Expr::Int(b)) if *b != 0 => {
                self.folds_performed += 1;
                Expr::Int(a % b)
            }
            (Expr::Int(a), BinOp::Pow, Expr::Int(b)) if *b >= 0 => {
                self.folds_performed += 1;
                self.vedic_power(*a, *b as u32)
            }

            // Float arithmetic
            (Expr::Float(a), BinOp::Add, Expr::Float(b)) => {
                self.folds_performed += 1;
                Expr::Float(a + b)
            }
            (Expr::Float(a), BinOp::Sub, Expr::Float(b)) => {
                self.folds_performed += 1;
                Expr::Float(a - b)
            }
            (Expr::Float(a), BinOp::Mul, Expr::Float(b)) => {
                self.folds_performed += 1;
                Expr::Float(a * b)
            }
            (Expr::Float(a), BinOp::Div, Expr::Float(b)) if *b != 0.0 => {
                self.folds_performed += 1;
                Expr::Float(a / b)
            }

            // Boolean operations
            (Expr::Bool(a), BinOp::And, Expr::Bool(b)) => {
                self.folds_performed += 1;
                Expr::Bool(*a && *b)
            }
            (Expr::Bool(a), BinOp::Or, Expr::Bool(b)) => {
                self.folds_performed += 1;
                Expr::Bool(*a || *b)
            }

            // Comparisons
            (Expr::Int(a), BinOp::Eq, Expr::Int(b)) => {
                self.folds_performed += 1;
                Expr::Bool(a == b)
            }
            (Expr::Int(a), BinOp::Ne, Expr::Int(b)) => {
                self.folds_performed += 1;
                Expr::Bool(a != b)
            }
            (Expr::Int(a), BinOp::Lt, Expr::Int(b)) => {
                self.folds_performed += 1;
                Expr::Bool(a < b)
            }
            (Expr::Int(a), BinOp::Le, Expr::Int(b)) => {
                self.folds_performed += 1;
                Expr::Bool(a <= b)
            }
            (Expr::Int(a), BinOp::Gt, Expr::Int(b)) => {
                self.folds_performed += 1;
                Expr::Bool(a > b)
            }
            (Expr::Int(a), BinOp::Ge, Expr::Int(b)) => {
                self.folds_performed += 1;
                Expr::Bool(a >= b)
            }

            // Algebraic identities (Sūtra 15: Guṇitasamuccayaḥ)
            // x + 0 = x
            (x, BinOp::Add, Expr::Int(0)) | (Expr::Int(0), BinOp::Add, x) => {
                self.folds_performed += 1;
                self.record_sutra(Sutra::GunitaSamuccayah);
                x.clone()
            }
            // x - 0 = x
            (x, BinOp::Sub, Expr::Int(0)) => {
                self.folds_performed += 1;
                self.record_sutra(Sutra::GunitaSamuccayah);
                x.clone()
            }
            // x * 1 = x
            (x, BinOp::Mul, Expr::Int(1)) | (Expr::Int(1), BinOp::Mul, x) => {
                self.folds_performed += 1;
                self.record_sutra(Sutra::GunitaSamuccayah);
                x.clone()
            }
            // x * 0 = 0
            (_, BinOp::Mul, Expr::Int(0)) | (Expr::Int(0), BinOp::Mul, _) => {
                self.folds_performed += 1;
                self.record_sutra(Sutra::SunyamSamyasamuccaye);
                Expr::Int(0)
            }
            // x / 1 = x
            (x, BinOp::Div, Expr::Int(1)) => {
                self.folds_performed += 1;
                x.clone()
            }
            // x ^ 0 = 1
            (_, BinOp::Pow, Expr::Int(0)) => {
                self.folds_performed += 1;
                Expr::Int(1)
            }
            // x ^ 1 = x
            (x, BinOp::Pow, Expr::Int(1)) => {
                self.folds_performed += 1;
                x.clone()
            }

            // Cannot fold
            _ => Expr::BinOp(Box::new(left.clone()), op, Box::new(right.clone())),
        }
    }

    /// Fold unary operation
    fn fold_unary(&mut self, op: UnaryOp, inner: &Expr) -> Expr {
        match (op, inner) {
            (UnaryOp::Neg, Expr::Int(n)) => {
                self.folds_performed += 1;
                Expr::Int(-n)
            }
            (UnaryOp::Neg, Expr::Float(f)) => {
                self.folds_performed += 1;
                Expr::Float(-f)
            }
            (UnaryOp::Not, Expr::Bool(b)) => {
                self.folds_performed += 1;
                Expr::Bool(!b)
            }
            (UnaryOp::Sqrt, Expr::Int(n)) if *n >= 0 => {
                let root = VedicMath::integer_sqrt(*n);
                if root * root == *n {
                    self.folds_performed += 1;
                    Expr::Int(root)
                } else {
                    self.folds_performed += 1;
                    Expr::Float((*n as f64).sqrt())
                }
            }
            (UnaryOp::Sqrt, Expr::Float(f)) if *f >= 0.0 => {
                self.folds_performed += 1;
                Expr::Float(f.sqrt())
            }
            // Double negation: --x = x
            (UnaryOp::Neg, Expr::UnaryOp(UnaryOp::Neg, inner)) => {
                self.folds_performed += 1;
                (**inner).clone()
            }
            // Double not: !!x = x
            (UnaryOp::Not, Expr::UnaryOp(UnaryOp::Not, inner)) => {
                self.folds_performed += 1;
                (**inner).clone()
            }
            _ => Expr::UnaryOp(op, Box::new(inner.clone())),
        }
    }

    /// Vedic multiplication with sūtra selection
    fn vedic_multiply(&mut self, a: i64, b: i64) -> Expr {
        // Select best sūtra
        let (result, sutra) = self.select_multiply_sutra(a, b);
        self.record_sutra(sutra);
        Expr::Int(result)
    }

    /// Select multiplication sūtra based on operands
    fn select_multiply_sutra(&self, a: i64, b: i64) -> (i64, Sutra) {
        // Check for squaring numbers ending in 5
        if a == b && a % 10 == 5 {
            return (VedicMath::ekadhikena_square(a), Sutra::EkadhikenaPurvena);
        }

        // Check for numbers near 100
        if (90..=110).contains(&a) && (90..=110).contains(&b) {
            return (VedicMath::nikhilam_multiply(a, b, 100), Sutra::NikhilamNavatascaramam);
        }

        // Check for numbers near 1000
        if (900..=1100).contains(&a) && (900..=1100).contains(&b) {
            return (VedicMath::nikhilam_multiply(a, b, 1000), Sutra::NikhilamNavatascaramam);
        }

        // Check for multiplication by nines
        if b == 9 || b == 99 || b == 999 {
            let nines_count = b.to_string().len();
            return (VedicMath::multiply_by_nines(a, nines_count), Sutra::EkanyunenaPurvena);
        }

        // Default: ūrdhva-tiryagbhyām
        if a.abs() < 100 && b.abs() < 100 {
            (VedicMath::urdhva_multiply_2digit(a, b), Sutra::UrdhvaTiryagbhyam)
        } else {
            (a * b, Sutra::UrdhvaTiryagbhyam)
        }
    }

    /// Vedic power computation
    fn vedic_power(&mut self, base: i64, exp: u32) -> Expr {
        if exp == 2 {
            // Squaring - try yāvadūnam
            let (result, sutra) = if base % 10 == 5 {
                (VedicMath::ekadhikena_square(base), Sutra::EkadhikenaPurvena)
            } else if (90..=110).contains(&base) {
                (VedicMath::yavadunam_square(base, 100), Sutra::Yavadunam)
            } else {
                (base * base, Sutra::UrdhvaTiryagbhyam)
            };
            self.record_sutra(sutra);
            Expr::Int(result)
        } else {
            // General power (binary exponentiation)
            let mut result = 1i64;
            let mut b = base;
            let mut e = exp;
            while e > 0 {
                if e & 1 == 1 {
                    result *= b;
                }
                b *= b;
                e >>= 1;
            }
            Expr::Int(result)
        }
    }

    /// Record which sūtra was used
    fn record_sutra(&mut self, sutra: Sutra) {
        if !self.sutras_used.contains(&sutra) {
            self.sutras_used.push(sutra);
        }
    }

    /// Get statistics
    pub fn stats(&self) -> FoldStats {
        FoldStats {
            folds_performed: self.folds_performed,
            sutras_used: self.sutras_used.len(),
        }
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.folds_performed = 0;
        self.sutras_used.clear();
    }
}

/// Folding statistics
#[derive(Debug)]
pub struct FoldStats {
    pub folds_performed: usize,
    pub sutras_used: usize,
}

impl Default for VedicConstantFolder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_fold() {
        let mut folder = VedicConstantFolder::new();

        // 2 + 3 = 5
        let expr = Expr::BinOp(
            Box::new(Expr::Int(2)),
            BinOp::Add,
            Box::new(Expr::Int(3)),
        );

        let result = folder.fold(&expr);
        assert!(matches!(result, Expr::Int(5)));
    }

    #[test]
    fn test_vedic_multiply() {
        let mut folder = VedicConstantFolder::new();

        // 97 × 96 using nikhilam
        let expr = Expr::BinOp(
            Box::new(Expr::Int(97)),
            BinOp::Mul,
            Box::new(Expr::Int(96)),
        );

        let result = folder.fold(&expr);
        assert!(matches!(result, Expr::Int(9312)));
    }

    #[test]
    fn test_square_ending_5() {
        let mut folder = VedicConstantFolder::new();

        // 25² = 625
        let expr = Expr::BinOp(
            Box::new(Expr::Int(25)),
            BinOp::Pow,
            Box::new(Expr::Int(2)),
        );

        let result = folder.fold(&expr);
        assert!(matches!(result, Expr::Int(625)));
    }

    #[test]
    fn test_identity_folds() {
        let mut folder = VedicConstantFolder::new();

        // x + 0 should not fully fold (x is variable)
        // But we test with constant: 5 + 0 = 5
        let expr = Expr::BinOp(
            Box::new(Expr::Int(5)),
            BinOp::Add,
            Box::new(Expr::Int(0)),
        );

        let result = folder.fold(&expr);
        assert!(matches!(result, Expr::Int(5)));
    }

    #[test]
    fn test_nested_fold() {
        let mut folder = VedicConstantFolder::new();

        // (2 + 3) * 4 = 20
        let expr = Expr::BinOp(
            Box::new(Expr::BinOp(
                Box::new(Expr::Int(2)),
                BinOp::Add,
                Box::new(Expr::Int(3)),
            )),
            BinOp::Mul,
            Box::new(Expr::Int(4)),
        );

        let result = folder.fold(&expr);
        assert!(matches!(result, Expr::Int(20)));
    }
}
