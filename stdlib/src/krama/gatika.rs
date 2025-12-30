//! # Gātika - Dynamic Programming (गतिक)
//!
//! Classic dynamic programming patterns and algorithms.
//!
//! > **"अतीतं भविष्यस्य मार्गदर्शकम्"**
//! > *"The past is the guide to the future"*
//!
//! ## Patterns
//!
//! - Memoization (स्मरण)
//! - Tabulation (सारणी)
//! - Space optimization
//!
//! ## Classic Problems
//!
//! - Fibonacci sequence
//! - Longest common subsequence
//! - Knapsack problem
//! - Edit distance
//! - Matrix chain multiplication

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// FIBONACCI (फिबोनाची)
// ============================================================================

/// Fibonacci with memoization (फिबोनाची स्मरण)
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(n)
#[cfg(feature = "alloc")]
pub fn fibonacci_smaran(n: usize) -> u64 {
    if n <= 1 {
        return n as u64;
    }

    let mut smriti = vec![0u64; n + 1];
    smriti[1] = 1;

    for i in 2..=n {
        smriti[i] = smriti[i - 1] + smriti[i - 2];
    }

    smriti[n]
}

/// Fibonacci with O(1) space (अंतरिक्ष न्यूनीकृत)
pub fn fibonacci_nyunikrit(n: usize) -> u64 {
    if n <= 1 {
        return n as u64;
    }

    let mut purva = 0u64;
    let mut vartamana = 1u64;

    for _ in 2..=n {
        let naya = purva + vartamana;
        purva = vartamana;
        vartamana = naya;
    }

    vartamana
}

/// Fibonacci using matrix exponentiation O(log n)
#[cfg(feature = "alloc")]
pub fn fibonacci_aavyuha(n: usize) -> u64 {
    if n <= 1 {
        return n as u64;
    }

    fn multiply(a: [[u64; 2]; 2], b: [[u64; 2]; 2]) -> [[u64; 2]; 2] {
        [
            [
                a[0][0] * b[0][0] + a[0][1] * b[1][0],
                a[0][0] * b[0][1] + a[0][1] * b[1][1],
            ],
            [
                a[1][0] * b[0][0] + a[1][1] * b[1][0],
                a[1][0] * b[0][1] + a[1][1] * b[1][1],
            ],
        ]
    }

    fn power(mut base: [[u64; 2]; 2], mut exp: usize) -> [[u64; 2]; 2] {
        let mut result = [[1, 0], [0, 1]]; // Identity

        while exp > 0 {
            if exp % 2 == 1 {
                result = multiply(result, base);
            }
            base = multiply(base, base);
            exp /= 2;
        }
        result
    }

    let base = [[1, 1], [1, 0]];
    let result = power(base, n - 1);
    result[0][0]
}

// ============================================================================
// LONGEST COMMON SUBSEQUENCE (दीर्घतम साझा उपक्रम)
// ============================================================================

/// Longest Common Subsequence length (दीर्घतम साझा उपक्रम)
///
/// # Etymology
/// दीर्घतम (dīrghatama) = longest
/// साझा (sājhā) = common
/// उपक्रम (upakrama) = subsequence
///
/// # Complexity
/// - Time: O(m × n)
/// - Space: O(m × n)
#[cfg(feature = "alloc")]
pub fn dirghatama_sajha_upakrama(s1: &str, s2: &str) -> usize {
    let m = s1.len();
    let n = s2.len();
    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();

    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if s1[i - 1] == s2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    dp[m][n]
}

/// Get actual LCS string (साझा उपक्रम प्राप्त)
#[cfg(feature = "alloc")]
pub fn sajha_upakrama_prapta(s1: &str, s2: &str) -> String {
    let m = s1.len();
    let n = s2.len();
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if s1_chars[i - 1] == s2_chars[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    // Backtrack to find LCS
    let mut lcs = String::new();
    let mut i = m;
    let mut j = n;

    while i > 0 && j > 0 {
        if s1_chars[i - 1] == s2_chars[j - 1] {
            lcs.push(s1_chars[i - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    lcs.chars().rev().collect()
}

// ============================================================================
// EDIT DISTANCE / LEVENSHTEIN (संपादन दूरी)
// ============================================================================

/// Edit distance / Levenshtein distance (संपादन दूरी)
///
/// Minimum operations to transform s1 into s2.
/// Operations: insert, delete, replace.
///
/// # Etymology
/// संपादन (sampādana) = editing
/// दूरी (dūrī) = distance
///
/// # Complexity
/// - Time: O(m × n)
/// - Space: O(m × n)
#[cfg(feature = "alloc")]
pub fn sampadan_duri(s1: &str, s2: &str) -> usize {
    let m = s1.len();
    let n = s2.len();
    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();

    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    // Base cases
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    // Fill DP table
    for i in 1..=m {
        for j in 1..=n {
            if s1[i - 1] == s2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j - 1] // Replace
                    .min(dp[i - 1][j]) // Delete
                    .min(dp[i][j - 1]); // Insert
            }
        }
    }

    dp[m][n]
}

/// Edit distance with space optimization (अंतरिक्ष न्यूनीकृत)
#[cfg(feature = "alloc")]
pub fn sampadan_duri_nyunikrit(s1: &str, s2: &str) -> usize {
    let m = s1.len();
    let n = s2.len();
    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();

    let mut prev = vec![0usize; n + 1];
    let mut curr = vec![0usize; n + 1];

    // Base case
    for j in 0..=n {
        prev[j] = j;
    }

    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            if s1[i - 1] == s2[j - 1] {
                curr[j] = prev[j - 1];
            } else {
                curr[j] = 1 + prev[j - 1].min(prev[j]).min(curr[j - 1]);
            }
        }
        core::mem::swap(&mut prev, &mut curr);
    }

    prev[n]
}

// ============================================================================
// KNAPSACK (गाठ)
// ============================================================================

/// 0/1 Knapsack Problem (0/1 गाठ समस्या)
///
/// Select items to maximize value within weight capacity.
///
/// # Etymology
/// गाठ (gāṭha) = knapsack, bundle
///
/// # Complexity
/// - Time: O(n × W)
/// - Space: O(n × W)
#[cfg(feature = "alloc")]
pub fn gatha_shunya_eka(
    mulya: &[u64], // Values (मूल्य)
    bhara: &[u64], // Weights (भार)
    kshamata: u64, // Capacity (क्षमता)
) -> u64 {
    let n = mulya.len();
    let w = kshamata as usize;

    let mut dp = vec![vec![0u64; w + 1]; n + 1];

    for i in 1..=n {
        for j in 0..=w {
            // Don't take item i
            dp[i][j] = dp[i - 1][j];

            // Take item i (if possible)
            if bhara[i - 1] as usize <= j {
                let with_item = dp[i - 1][j - bhara[i - 1] as usize] + mulya[i - 1];
                dp[i][j] = dp[i][j].max(with_item);
            }
        }
    }

    dp[n][w]
}

/// 0/1 Knapsack with space optimization
#[cfg(feature = "alloc")]
pub fn gatha_nyunikrit(mulya: &[u64], bhara: &[u64], kshamata: u64) -> u64 {
    let n = mulya.len();
    let w = kshamata as usize;

    let mut dp = vec![0u64; w + 1];

    for i in 0..n {
        // Traverse right to left to avoid using same item twice
        for j in (bhara[i] as usize..=w).rev() {
            dp[j] = dp[j].max(dp[j - bhara[i] as usize] + mulya[i]);
        }
    }

    dp[w]
}

/// Unbounded Knapsack (असीमित गाठ)
///
/// Can use each item multiple times.
#[cfg(feature = "alloc")]
pub fn gatha_asimit(mulya: &[u64], bhara: &[u64], kshamata: u64) -> u64 {
    let w = kshamata as usize;
    let mut dp = vec![0u64; w + 1];

    for j in 1..=w {
        for (i, &b) in bhara.iter().enumerate() {
            if b as usize <= j {
                dp[j] = dp[j].max(dp[j - b as usize] + mulya[i]);
            }
        }
    }

    dp[w]
}

// ============================================================================
// COIN CHANGE (सिक्का परिवर्तन)
// ============================================================================

/// Minimum coins to make amount (न्यूनतम सिक्का)
///
/// # Complexity
/// - Time: O(n × amount)
/// - Space: O(amount)
#[cfg(feature = "alloc")]
pub fn nyunatam_sikka(sikke: &[u64], rashi: u64) -> Option<u64> {
    let n = rashi as usize;
    let mut dp = vec![u64::MAX; n + 1];
    dp[0] = 0;

    for i in 1..=n {
        for &sikka in sikke {
            if sikka as usize <= i && dp[i - sikka as usize] != u64::MAX {
                dp[i] = dp[i].min(dp[i - sikka as usize] + 1);
            }
        }
    }

    if dp[n] == u64::MAX {
        None
    } else {
        Some(dp[n])
    }
}

/// Count ways to make amount (विधि गणना)
#[cfg(feature = "alloc")]
pub fn sikka_vidhi_ganana(sikke: &[u64], rashi: u64) -> u64 {
    let n = rashi as usize;
    let mut dp = vec![0u64; n + 1];
    dp[0] = 1;

    for &sikka in sikke {
        for i in sikka as usize..=n {
            dp[i] += dp[i - sikka as usize];
        }
    }

    dp[n]
}

// ============================================================================
// LONGEST INCREASING SUBSEQUENCE (दीर्घतम वर्धमान उपक्रम)
// ============================================================================

/// Longest Increasing Subsequence length (दीर्घतम वर्धमान उपक्रम)
///
/// # Etymology
/// वर्धमान (vardhamāna) = increasing
///
/// # Complexity
/// - Time: O(n log n) using binary search
/// - Space: O(n)
#[cfg(feature = "alloc")]
pub fn dirghatama_vardhamana_upakrama(arr: &[i64]) -> usize {
    if arr.is_empty() {
        return 0;
    }

    let mut tails: Vec<i64> = Vec::new();

    for &num in arr {
        // Binary search for position
        let pos = tails.partition_point(|&x| x < num);

        if pos == tails.len() {
            tails.push(num);
        } else {
            tails[pos] = num;
        }
    }

    tails.len()
}

/// LIS with O(n²) for getting actual subsequence
#[cfg(feature = "alloc")]
pub fn vardhamana_upakrama_prapta(arr: &[i64]) -> Vec<i64> {
    let n = arr.len();
    if n == 0 {
        return Vec::new();
    }

    let mut dp = vec![1usize; n];
    let mut parent = vec![usize::MAX; n];
    let mut max_len = 1;
    let mut max_idx = 0;

    for i in 1..n {
        for j in 0..i {
            if arr[j] < arr[i] && dp[j] + 1 > dp[i] {
                dp[i] = dp[j] + 1;
                parent[i] = j;
            }
        }
        if dp[i] > max_len {
            max_len = dp[i];
            max_idx = i;
        }
    }

    // Reconstruct
    let mut lis = Vec::new();
    let mut idx = max_idx;
    while idx != usize::MAX {
        lis.push(arr[idx]);
        idx = parent[idx];
    }
    lis.reverse();
    lis
}

// ============================================================================
// MATRIX CHAIN MULTIPLICATION (आव्यूह श्रृंखला गुणन)
// ============================================================================

/// Matrix Chain Multiplication minimum operations (आव्यूह श्रृंखला)
///
/// Find optimal parenthesization to minimize scalar multiplications.
///
/// # Parameters
/// `dims[i]` is the row dimension of matrix i, `dims[i+1]` is column dimension.
///
/// # Complexity
/// - Time: O(n³)
/// - Space: O(n²)
#[cfg(feature = "alloc")]
pub fn aavyuha_shrinkhala_gunana(dims: &[u64]) -> u64 {
    let n = dims.len() - 1; // Number of matrices
    if n <= 1 {
        return 0;
    }

    // dp[i][j] = min cost to multiply matrices i..j
    let mut dp = vec![vec![u64::MAX; n]; n];

    // Single matrix has no cost
    for i in 0..n {
        dp[i][i] = 0;
    }

    // Chain length from 2 to n
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;

            // Try all split points
            for k in i..j {
                let cost = dp[i][k] + dp[k + 1][j] + dims[i] * dims[k + 1] * dims[j + 1];
                dp[i][j] = dp[i][j].min(cost);
            }
        }
    }

    dp[0][n - 1]
}

// ============================================================================
// SUBSET SUM (उपसमूह योग)
// ============================================================================

/// Subset sum problem (उपसमूह योग)
///
/// Can subset of array sum to target?
#[cfg(feature = "alloc")]
pub fn upasamuha_yoga(arr: &[u64], lakshya: u64) -> bool {
    let n = arr.len();
    let t = lakshya as usize;

    let mut dp = vec![false; t + 1];
    dp[0] = true;

    for &num in arr {
        let num = num as usize;
        for j in (num..=t).rev() {
            dp[j] = dp[j] || dp[j - num];
        }
    }

    dp[t]
}

/// Count subsets with given sum (उपसमूह गणना)
#[cfg(feature = "alloc")]
pub fn upasamuha_ganana(arr: &[u64], lakshya: u64) -> u64 {
    let t = lakshya as usize;
    let mut dp = vec![0u64; t + 1];
    dp[0] = 1;

    for &num in arr {
        let num = num as usize;
        for j in (num..=t).rev() {
            dp[j] += dp[j - num];
        }
    }

    dp[t]
}

// ============================================================================
// PARTITION PROBLEM (विभाजन)
// ============================================================================

/// Equal sum partition (समान विभाजन)
///
/// Can array be partitioned into two subsets with equal sum?
#[cfg(feature = "alloc")]
pub fn samana_vibhajana(arr: &[u64]) -> bool {
    let total: u64 = arr.iter().sum();

    // Can't partition odd sum equally
    if total % 2 != 0 {
        return false;
    }

    upasamuha_yoga(arr, total / 2)
}

// ============================================================================
// CLIMBING STAIRS (सीढ़ी चढ़ना)
// ============================================================================

/// Ways to climb n stairs, 1 or 2 steps at a time (सीढ़ी विधि)
#[cfg(feature = "alloc")]
pub fn sidhi_vidhi(n: usize) -> u64 {
    if n <= 2 {
        return n as u64;
    }
    fibonacci_nyunikrit(n + 1)
}

/// Ways to climb with variable step sizes (चर सोपान)
#[cfg(feature = "alloc")]
pub fn char_sopan(n: usize, steps: &[usize]) -> u64 {
    let mut dp = vec![0u64; n + 1];
    dp[0] = 1;

    for i in 1..=n {
        for &step in steps {
            if step <= i {
                dp[i] += dp[i - step];
            }
        }
    }

    dp[n]
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_fibonacci() {
        assert_eq!(fibonacci_smaran(0), 0);
        assert_eq!(fibonacci_smaran(1), 1);
        assert_eq!(fibonacci_smaran(10), 55);
        assert_eq!(fibonacci_smaran(20), 6765);

        assert_eq!(fibonacci_nyunikrit(10), 55);
        assert_eq!(fibonacci_aavyuha(10), 55);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_lcs() {
        assert_eq!(dirghatama_sajha_upakrama("ABCDGH", "AEDFHR"), 3);
        assert_eq!(dirghatama_sajha_upakrama("AGGTAB", "GXTXAYB"), 4);

        let lcs = sajha_upakrama_prapta("AGGTAB", "GXTXAYB");
        assert_eq!(lcs.len(), 4);
        assert_eq!(lcs, "GTAB");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_edit_distance() {
        assert_eq!(sampadan_duri("sunday", "saturday"), 3);
        assert_eq!(sampadan_duri("kitten", "sitting"), 3);
        assert_eq!(sampadan_duri("", "abc"), 3);
        assert_eq!(sampadan_duri("abc", "abc"), 0);

        assert_eq!(sampadan_duri_nyunikrit("sunday", "saturday"), 3);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_knapsack() {
        let mulya = vec![60, 100, 120];
        let bhara = vec![10, 20, 30];

        assert_eq!(gatha_shunya_eka(&mulya, &bhara, 50), 220);
        assert_eq!(gatha_nyunikrit(&mulya, &bhara, 50), 220);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_unbounded_knapsack() {
        let mulya = vec![10, 30, 20];
        let bhara = vec![5, 10, 15];

        // Capacity 30: can take 3 items of weight 10 = 90 value
        assert_eq!(gatha_asimit(&mulya, &bhara, 30), 90);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_coin_change() {
        let coins = vec![1, 2, 5];
        assert_eq!(nyunatam_sikka(&coins, 11), Some(3)); // 5+5+1
        assert_eq!(nyunatam_sikka(&coins, 0), Some(0));

        let coins2 = vec![2];
        assert_eq!(nyunatam_sikka(&coins2, 3), None); // Impossible

        assert_eq!(sikka_vidhi_ganana(&coins, 5), 4); // 1+1+1+1+1, 1+1+1+2, 1+2+2, 5
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_lis() {
        let arr = vec![10, 22, 9, 33, 21, 50, 41, 60, 80];
        assert_eq!(dirghatama_vardhamana_upakrama(&arr), 6);

        let lis = vardhamana_upakrama_prapta(&arr);
        assert_eq!(lis.len(), 6);

        // Verify it's increasing
        for i in 1..lis.len() {
            assert!(lis[i] > lis[i - 1]);
        }
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_matrix_chain() {
        let dims = vec![10, 30, 5, 60];
        // Matrices: 10×30, 30×5, 5×60
        // Optimal: (A×B)×C = 10×30×5 + 10×5×60 = 1500 + 3000 = 4500
        assert_eq!(aavyuha_shrinkhala_gunana(&dims), 4500);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_subset_sum() {
        let arr = vec![3, 34, 4, 12, 5, 2];
        assert!(upasamuha_yoga(&arr, 9)); // 4+5 or 3+4+2
        assert!(!upasamuha_yoga(&arr, 30));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_partition() {
        let arr1 = vec![1, 5, 11, 5];
        assert!(samana_vibhajana(&arr1)); // {1,5,5} and {11}

        let arr2 = vec![1, 2, 3, 5];
        assert!(!samana_vibhajana(&arr2)); // Sum 11 is odd
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_stairs() {
        assert_eq!(sidhi_vidhi(2), 2);
        assert_eq!(sidhi_vidhi(3), 3);
        assert_eq!(sidhi_vidhi(4), 5);

        // With steps 1, 2, 3
        assert_eq!(char_sopan(4, &[1, 2, 3]), 7);
    }
}
