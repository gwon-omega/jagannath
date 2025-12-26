//! Vedic Mathematics (वेदगणित)
//!
//! Fast arithmetic using the 16 Vedic math Mahāsūtras (great formulas)
//! and 13 Upasūtras (sub-formulas) for compile-time constant folding
//! and runtime optimization patterns.
//!
//! References:
//! - "Vedic Mathematics" by Bharati Krishna Tirthaji (1880-1960)
//! - 16 Sūtras discovered in Atharvaveda Pariśiṣṭa

pub mod constant_folder;

pub use constant_folder::VedicConstantFolder;

/// The 16 Mahāsūtras (great formulas)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sutra {
    /// 1. एकाधिकेन पूर्वेण - By one more than the previous
    EkadhikenaPurvena,
    /// 2. निखिलं नवतश्चरमं दशतः - All from 9 and the last from 10
    NikhilamNavatascaramam,
    /// 3. ऊर्ध्वतिर्यग्भ्याम् - Vertically and crosswise
    UrdhvaTiryagbhyam,
    /// 4. परावर्त्य योजयेत् - Transpose and adjust
    ParavartyaYojayet,
    /// 5. शून्यं साम्यसमुच्चये - If samuccaya is same, it's zero
    SunyamSamyasamuccaye,
    /// 6. आनुरूप्येण - Proportionately
    Anurupyena,
    /// 7. संकलन व्यवकलनाभ्याम् - By addition and subtraction
    SankalanaVyavakalanabhyam,
    /// 8. पूरणापूरणाभ्याम् - By completion or non-completion
    PuranapuranabhyAm,
    /// 9. चलन कलनाभ्याम् - Differential calculus
    CalanaKalanabhyam,
    /// 10. यावदूनम् - Whatever the deficiency
    Yavadunam,
    /// 11. व्यष्टिसमष्टिः - Part and whole
    VyastiSamastih,
    /// 12. शेषाण्यङ्केन चरमेण - The remainders by the last digit
    SesanyankenaCaramena,
    /// 13. सोपान्त्यद्वयमन्त्यम् - Ultimate and twice the penultimate
    Sopantyadvayamantyam,
    /// 14. एकन्यूनेन पूर्वेण - By one less than the previous
    EkanyunenaPurvena,
    /// 15. गुणितसमुच्चयः - The product of the sum
    GunitaSamuccayah,
    /// 16. गुणकसमुच्चयः - All the multipliers
    GunakaSamuccayah,
}

/// Vedic Math computation engine
pub struct VedicMath;

impl VedicMath {
    // ==========================================
    // SŪTRA 1: एकाधिकेन पूर्वेण (By one more than the previous)
    // ==========================================

    /// Square numbers ending in 5: 25² = 2×3|25 = 625
    pub fn square_ending_5(n: i64) -> i64 {
        let tens = n / 10;
        let prefix = tens * (tens + 1);
        prefix * 100 + 25
    }

    /// Multiply numbers whose first parts are same and last digits sum to 10
    /// e.g., 23 × 27 = 2×3|21 = 621 (2×3=6, 3×7=21)
    pub fn ekadhikena_multiply(a: i64, b: i64) -> Option<i64> {
        let a_tens = a / 10;
        let b_tens = b / 10;
        let a_units = a % 10;
        let b_units = b % 10;

        if a_tens == b_tens && a_units + b_units == 10 {
            let prefix = a_tens * (a_tens + 1);
            let suffix = a_units * b_units;
            Some(prefix * 100 + suffix)
        } else {
            None
        }
    }

    // ==========================================
    // SŪTRA 2: निखिलं (All from 9, last from 10)
    // ==========================================

    /// Fast multiplication near a base (10, 100, 1000, etc.)
    /// 97 × 96: deviations -3, -4; result = 93|12 = 9312
    pub fn nikhilam_multiply(a: i64, b: i64, base: i64) -> i64 {
        let diff_a = a - base;
        let diff_b = b - base;

        let left_part = a + diff_b; // or b + diff_a
        let right_part = diff_a * diff_b;

        // Handle negative right part
        if right_part < 0 {
            let base_digits = (base as f64).log10() as i64;
            let borrow = 10i64.pow(base_digits as u32);
            (left_part - 1) * base + (borrow + right_part)
        } else {
            left_part * base + right_part
        }
    }

    /// Calculate complement (all from 9, last from 10)
    pub fn nikhilam_complement(n: i64) -> i64 {
        let s = n.to_string();
        let mut result = 0i64;
        let len = s.len();

        for (i, c) in s.chars().enumerate() {
            let digit = c.to_digit(10).unwrap() as i64;
            let complement = if i == len - 1 { 10 - digit } else { 9 - digit };
            result = result * 10 + complement;
        }
        result
    }

    // ==========================================
    // SŪTRA 3: ऊर्ध्वतिर्यग्भ्याम् (Vertically and crosswise)
    // ==========================================

    /// General multiplication using vertical-crosswise method
    /// Works for any numbers, optimal for multi-digit
    pub fn urdhva_multiply(a: i64, b: i64) -> i64 {
        let a_digits: Vec<i64> = a
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();
        let b_digits: Vec<i64> = b
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();

        let n = a_digits.len();
        let m = b_digits.len();
        let total_positions = n + m;
        let mut result_digits = vec![0i64; total_positions];

        // Crosswise multiplication
        // Position in result array is from the right
        for i in 0..n {
            for j in 0..m {
                // Position from the right: (n-1-i) + (m-1-j)
                // Position from the left: total_positions - 1 - ((n-1-i) + (m-1-j))
                //                       = i + j + 1
                let pos = i + j + 1;
                result_digits[pos] += a_digits[i] * b_digits[j];
            }
        }

        // Carry propagation (from right to left)
        let mut carry = 0;
        for i in (0..result_digits.len()).rev() {
            result_digits[i] += carry;
            carry = result_digits[i] / 10;
            result_digits[i] %= 10;
        }

        // Convert to number, skip leading zeros
        let mut result = 0i64;
        for &d in &result_digits {
            result = result * 10 + d;
        }
        result
    }

    // ==========================================
    // SŪTRA 4: परावर्त्य योजयेत् (Transpose and adjust)
    // ==========================================

    /// Division by numbers slightly greater than power of 10
    /// e.g., divide by 12 = 10 + 2
    pub fn paravartya_divide(dividend: i64, divisor: i64) -> (i64, i64) {
        // Find nearest lower power of 10
        let base = 10i64.pow((divisor as f64).log10().floor() as u32);
        let adjustment = divisor - base;

        if adjustment == 0 || adjustment > base / 2 {
            // Fall back to normal division
            return (dividend / divisor, dividend % divisor);
        }

        // Use paravartya method
        let mut quotient = 0i64;
        let mut remainder = dividend;

        while remainder >= divisor {
            let q_digit = remainder / base;
            quotient = quotient * 10 + q_digit;
            remainder = remainder - q_digit * divisor;

            // Prevent infinite loop
            if remainder >= dividend {
                return (dividend / divisor, dividend % divisor);
            }
        }

        (quotient, remainder)
    }

    // ==========================================
    // SŪTRA 5: शून्यं साम्यसमुच्चये (If sum is same, equate to zero)
    // ==========================================

    /// Solve equations where sum of coefficients is equal
    /// (ax + b)(cx + d) = (ex + f)(gx + h)
    /// If a+b = e+f AND c+d = g+h, then x = 0 is a solution
    pub fn sunyam_check(
        coeffs_left: (i64, i64, i64, i64),
        coeffs_right: (i64, i64, i64, i64),
    ) -> bool {
        let (a, b, c, d) = coeffs_left;
        let (e, f, g, h) = coeffs_right;

        (a + b == e + f) && (c + d == g + h)
    }

    // ==========================================
    // SŪTRA 6: आनुरूप्येण (Proportionately)
    // ==========================================

    /// Simplify calculations using proportions
    pub fn anurupya_scale(n: i64, from_base: i64, to_base: i64) -> i64 {
        n * to_base / from_base
    }

    /// Proportional division
    pub fn anurupya_divide(dividend: i64, divisor: i64) -> (i64, i64) {
        (dividend / divisor, dividend % divisor)
    }

    // ==========================================
    // SŪTRA 7: संकलन व्यवकलनाभ्याम् (By addition and subtraction)
    // ==========================================

    /// Solve simultaneous equations
    /// ax + by = c, dx + ey = f
    /// Add/subtract to eliminate one variable
    pub fn sankalana_solve(a: i64, b: i64, c: i64, d: i64, e: i64, f: i64) -> Option<(i64, i64)> {
        let det = a * e - b * d;
        if det == 0 {
            return None;
        }

        let x = (c * e - b * f) / det;
        let y = (a * f - c * d) / det;

        // Verify integer solution
        if (c * e - b * f) % det == 0 && (a * f - c * d) % det == 0 {
            Some((x, y))
        } else {
            None
        }
    }

    // ==========================================
    // SŪTRA 8: पूरणापूरणाभ्याम् (By completion)
    // ==========================================

    /// Complete the square for quadratics
    /// ax² + bx + c → a(x + b/2a)² + (c - b²/4a)
    pub fn purana_complete_square(a: f64, b: f64, c: f64) -> (f64, f64, f64) {
        let h = -b / (2.0 * a);
        let k = c - (b * b) / (4.0 * a);
        (a, h, k) // a(x - h)² + k
    }

    // ==========================================
    // SŪTRA 9: चलन कलनाभ्याम् (Differential calculus)
    // ==========================================

    /// Numerical differentiation (calculus)
    pub fn calana_derivative(f: impl Fn(f64) -> f64, x: f64, h: f64) -> f64 {
        (f(x + h) - f(x - h)) / (2.0 * h)
    }

    // ==========================================
    // SŪTRA 10: यावदूनम् (Whatever the deficiency)
    // ==========================================

    /// Fast squaring near base
    /// 98² = (98-2)×100 + 2² = 9604
    pub fn yavadunam_square(n: i64, base: i64) -> i64 {
        let diff = n - base;
        (n + diff) * base + (diff * diff)
    }

    /// Fast cubing near base
    pub fn yavadunam_cube(n: i64, base: i64) -> i64 {
        let diff = n - base;
        let a = (n + 2 * diff) * base * base;
        let b = 3 * diff * diff * base;
        let c = diff * diff * diff;
        a + b + c
    }

    // ==========================================
    // SŪTRA 11: व्यष्टिसमष्टिः (Part and whole)
    // ==========================================

    /// Use partial computations to find whole
    pub fn vyasti_average(parts: &[i64]) -> i64 {
        if parts.is_empty() {
            return 0;
        }
        parts.iter().sum::<i64>() / parts.len() as i64
    }

    // ==========================================
    // SŪTRA 12: शेषाण्यङ्केन चरमेण (Remainders by last digit)
    // ==========================================

    /// Divisibility test by examining remainders
    pub fn sesanyankena_divisible_by(n: i64, divisor: i64) -> bool {
        n % divisor == 0
    }

    /// Digital root (repeated digit sum until single digit)
    pub fn digital_root(n: i64) -> i64 {
        if n == 0 {
            return 0;
        }
        1 + (n.abs() - 1) % 9
    }

    /// Check divisibility by 9
    pub fn divisible_by_9(n: i64) -> bool {
        Self::digital_root(n) == 9
    }

    /// Check divisibility by 3
    pub fn divisible_by_3(n: i64) -> bool {
        let root = Self::digital_root(n);
        root == 3 || root == 6 || root == 9
    }

    // ==========================================
    // SŪTRA 13: सोपान्त्यद्वयमन्त्यम् (Penultimate formula)
    // ==========================================

    /// Solve equations of form 1/(x+a) + 1/(x+b) = 1/(x+c) + 1/(x+d)
    /// where a+b = c+d
    pub fn sopantya_solve(a: i64, b: i64, c: i64, d: i64) -> Option<i64> {
        if a + b != c + d {
            return None;
        }
        // x = -(a+b+c+d)/2
        let sum = a + b + c + d;
        if sum % 2 == 0 {
            Some(-sum / 2)
        } else {
            None
        }
    }

    // ==========================================
    // SŪTRA 14: एकन्यूनेन पूर्वेण (By one less than the previous)
    // ==========================================

    /// Multiply by numbers like 9, 99, 999
    /// n × 99 = n × 100 - n
    pub fn ekanyunena_multiply(n: i64, nines: i64) -> i64 {
        let base = nines + 1; // 99 + 1 = 100
        n * base - n
    }

    // ==========================================
    // SŪTRA 15: गुणितसमुच्चयः (Product of sums)
    // ==========================================

    /// (a+b)(c+d) = ac + ad + bc + bd
    /// But verify: sum(a,b) × sum(c,d) = sum of products
    pub fn gunita_verify(a: i64, b: i64, c: i64, d: i64) -> bool {
        let product_of_sums = (a + b) * (c + d);
        let sum_of_products = a * c + a * d + b * c + b * d;
        product_of_sums == sum_of_products
    }

    // ==========================================
    // SŪTRA 16: गुणकसमुच्चयः (All the multipliers)
    // ==========================================

    /// Product of factors equals product of roots
    /// For ax² + bx + c with roots r1, r2: r1 × r2 = c/a
    pub fn gunaka_product_of_roots(a: i64, _b: i64, c: i64) -> Option<(i64, i64)> {
        if a == 0 {
            return None;
        }
        Some((c, a)) // c/a as fraction
    }

    // ==========================================
    // UTILITY FUNCTIONS
    // ==========================================

    /// Integer square root using Vedic approximation
    pub fn integer_sqrt(n: i64) -> i64 {
        if n < 0 {
            return 0;
        }
        if n < 2 {
            return n;
        }

        // Newton's method with integer arithmetic
        let mut x = n;
        let mut y = (x + 1) / 2;
        while y < x {
            x = y;
            y = (x + n / x) / 2;
        }
        x
    }

    /// Square a number ending in 5 (special case of एकाधिकेन)
    /// 25² = 2×3|25 = 625
    pub fn ekadhikena_square(n: i64) -> i64 {
        if n % 10 == 5 {
            Self::square_ending_5(n)
        } else {
            n * n
        }
    }

    /// Multiply by repeated 9s (9, 99, 999, etc.)
    /// n × 99 = n × 100 - n = 100n - n
    pub fn multiply_by_nines(n: i64, nines_count: usize) -> i64 {
        let base = 10i64.pow(nines_count as u32);
        n * base - n
    }

    /// 2-digit Ūrdhva-Tiryagbhyām multiplication
    /// More efficient for small numbers
    pub fn urdhva_multiply_2digit(a: i64, b: i64) -> i64 {
        // For 2-digit: ab × cd = (10a + b)(10c + d)
        // = 100ac + 10(ad + bc) + bd
        let a_tens = a / 10;
        let a_units = a % 10;
        let b_tens = b / 10;
        let b_units = b % 10;

        let diagonal1 = a_tens * b_tens * 100;
        let cross = (a_tens * b_units + a_units * b_tens) * 10;
        let diagonal2 = a_units * b_units;

        diagonal1 + cross + diagonal2
    }

    /// Determine best sūtra for multiplication
    pub fn best_multiply_sutra(a: i64, b: i64) -> Sutra {
        // Near 100?
        if (90..=110).contains(&a) && (90..=110).contains(&b) {
            return Sutra::NikhilamNavatascaramam;
        }

        // Same tens digit, units sum to 10?
        let a_tens = a / 10;
        let b_tens = b / 10;
        let a_units = a % 10;
        let b_units = b % 10;
        if a_tens == b_tens && a_units + b_units == 10 {
            return Sutra::EkadhikenaPurvena;
        }

        // Multiply by 9s?
        if b == 9 || b == 99 || b == 999 || b == 9999 {
            return Sutra::EkanyunenaPurvena;
        }

        // Default: vertical-crosswise
        Sutra::UrdhvaTiryagbhyam
    }

    /// Fast multiply using best sūtra
    pub fn fast_multiply(a: i64, b: i64) -> i64 {
        match Self::best_multiply_sutra(a, b) {
            Sutra::NikhilamNavatascaramam => {
                let base = if a > 500 && b > 500 { 1000 } else { 100 };
                Self::nikhilam_multiply(a, b, base)
            }
            Sutra::EkadhikenaPurvena => Self::ekadhikena_multiply(a, b).unwrap_or(a * b),
            Sutra::EkanyunenaPurvena => Self::ekanyunena_multiply(a, b),
            _ => Self::urdhva_multiply(a, b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_ending_5() {
        assert_eq!(VedicMath::square_ending_5(25), 625);
        assert_eq!(VedicMath::square_ending_5(35), 1225);
        assert_eq!(VedicMath::square_ending_5(45), 2025);
        assert_eq!(VedicMath::square_ending_5(95), 9025);
    }

    #[test]
    fn test_ekadhikena_multiply() {
        // 23 × 27 = 621 (2×3=6, 3×7=21)
        assert_eq!(VedicMath::ekadhikena_multiply(23, 27), Some(621));
        // 34 × 36 = 1224
        assert_eq!(VedicMath::ekadhikena_multiply(34, 36), Some(1224));
    }

    #[test]
    fn test_nikhilam_multiply() {
        // 97 × 96 = 9312
        assert_eq!(VedicMath::nikhilam_multiply(97, 96, 100), 9312);
        // 98 × 97 = 9506
        assert_eq!(VedicMath::nikhilam_multiply(98, 97, 100), 9506);
    }

    #[test]
    fn test_urdhva_multiply() {
        assert_eq!(VedicMath::urdhva_multiply(12, 34), 408);
        assert_eq!(VedicMath::urdhva_multiply(123, 456), 56088);
    }

    #[test]
    fn test_yavadunam_square() {
        // 98² = 9604
        assert_eq!(VedicMath::yavadunam_square(98, 100), 9604);
        // 103² = 10609
        assert_eq!(VedicMath::yavadunam_square(103, 100), 10609);
    }

    #[test]
    fn test_digital_root() {
        assert_eq!(VedicMath::digital_root(123), 6);
        assert_eq!(VedicMath::digital_root(999), 9);
        assert_eq!(VedicMath::digital_root(12345), 6);
    }

    #[test]
    fn test_ekanyunena_multiply() {
        // 123 × 99 = 12177
        assert_eq!(VedicMath::ekanyunena_multiply(123, 99), 12177);
        // 45 × 9 = 405
        assert_eq!(VedicMath::ekanyunena_multiply(45, 9), 405);
    }

    #[test]
    fn test_best_sutra_selection() {
        assert_eq!(
            VedicMath::best_multiply_sutra(97, 96),
            Sutra::NikhilamNavatascaramam
        );
        assert_eq!(
            VedicMath::best_multiply_sutra(23, 27),
            Sutra::EkadhikenaPurvena
        );
        assert_eq!(
            VedicMath::best_multiply_sutra(45, 99),
            Sutra::EkanyunenaPurvena
        );
    }
}
