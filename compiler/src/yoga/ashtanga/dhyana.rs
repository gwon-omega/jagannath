//! Dhyāna - Meditation/Code Review (ध्यान)
//!
//! The seventh limb of Ashtanga Yoga - Deep meditation.
//! In software, this means deep code analysis and review:
//!
//! - Static analysis
//! - Security audit
//! - Performance profiling
//! - Code review process

use std::collections::HashMap;
use std::time::Duration;

/// Review categories (aspects of meditation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReviewAspect {
    /// Correctness - Does it work right?
    Correctness,
    /// Security - Is it safe?
    Security,
    /// Performance - Is it efficient?
    Performance,
    /// Maintainability - Is it clean?
    Maintainability,
    /// Readability - Is it understandable?
    Readability,
    /// Testability - Can it be tested?
    Testability,
}

impl ReviewAspect {
    /// Sanskrit meditation term
    pub fn sanskrit(&self) -> &'static str {
        match self {
            Self::Correctness => "सत्य (Satya) - Truth",
            Self::Security => "रक्षा (Rakṣā) - Protection",
            Self::Performance => "वेग (Vega) - Speed",
            Self::Maintainability => "स्थिरता (Sthiratā) - Stability",
            Self::Readability => "स्पष्टता (Spaṣṭatā) - Clarity",
            Self::Testability => "परीक्षा (Parīkṣā) - Examination",
        }
    }
}

/// Review finding
#[derive(Debug, Clone)]
pub struct Finding {
    pub aspect: ReviewAspect,
    pub severity: FindingSeverity,
    pub location: String,
    pub title: String,
    pub description: String,
    pub suggestion: Option<String>,
    pub references: Vec<String>,
}

/// Finding severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FindingSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// Dhyāna reviewer - deep code analysis
pub struct DhyanaReviewer {
    /// Findings from review
    findings: Vec<Finding>,
    /// Scores per aspect
    scores: HashMap<ReviewAspect, f64>,
    /// Configuration
    config: DhyanaConfig,
    /// Review duration
    duration: Option<Duration>,
}

/// Review configuration
#[derive(Debug, Clone)]
pub struct DhyanaConfig {
    /// Enable security checks
    pub check_security: bool,
    /// Enable performance checks
    pub check_performance: bool,
    /// Enable style checks
    pub check_style: bool,
    /// Minimum acceptable score
    pub min_score: f64,
}

impl Default for DhyanaConfig {
    fn default() -> Self {
        Self {
            check_security: true,
            check_performance: true,
            check_style: true,
            min_score: 70.0,
        }
    }
}

impl DhyanaReviewer {
    pub fn new(config: DhyanaConfig) -> Self {
        Self {
            findings: Vec::new(),
            scores: HashMap::new(),
            config,
            duration: None,
        }
    }

    /// Add a finding
    pub fn add_finding(&mut self, finding: Finding) {
        self.findings.push(finding);
    }

    /// Review correctness
    pub fn review_correctness(&mut self, code: &CodeForReview) {
        // Check for potential null dereferences
        for issue in &code.potential_nulls {
            self.add_finding(Finding {
                aspect: ReviewAspect::Correctness,
                severity: FindingSeverity::High,
                location: issue.location.clone(),
                title: "Potential null dereference".to_string(),
                description: issue.description.clone(),
                suggestion: Some("Add null check or use Option type".to_string()),
                references: vec![],
            });
        }

        // Check for unreachable code
        for unreachable in &code.unreachable_code {
            self.add_finding(Finding {
                aspect: ReviewAspect::Correctness,
                severity: FindingSeverity::Medium,
                location: unreachable.clone(),
                title: "Unreachable code".to_string(),
                description: "Code will never be executed".to_string(),
                suggestion: Some("Remove or fix control flow".to_string()),
                references: vec![],
            });
        }

        // Calculate score
        let issues = self.findings.iter()
            .filter(|f| f.aspect == ReviewAspect::Correctness)
            .count();
        self.scores.insert(ReviewAspect::Correctness, self.calculate_score(issues, code.total_items));
    }

    /// Review security
    pub fn review_security(&mut self, code: &CodeForReview) {
        if !self.config.check_security {
            return;
        }

        // Check for hardcoded secrets
        for secret in &code.hardcoded_secrets {
            self.add_finding(Finding {
                aspect: ReviewAspect::Security,
                severity: FindingSeverity::Critical,
                location: secret.location.clone(),
                title: "Hardcoded secret".to_string(),
                description: format!("Potential secret: {}", secret.pattern),
                suggestion: Some("Use environment variables or secret manager".to_string()),
                references: vec!["CWE-798".to_string()],
            });
        }

        // Check for SQL injection
        for injection in &code.sql_injections {
            self.add_finding(Finding {
                aspect: ReviewAspect::Security,
                severity: FindingSeverity::Critical,
                location: injection.clone(),
                title: "Potential SQL injection".to_string(),
                description: "User input in SQL query without sanitization".to_string(),
                suggestion: Some("Use parameterized queries".to_string()),
                references: vec!["CWE-89".to_string()],
            });
        }

        let issues = self.findings.iter()
            .filter(|f| f.aspect == ReviewAspect::Security)
            .count();
        self.scores.insert(ReviewAspect::Security, self.calculate_score(issues, code.total_items));
    }

    /// Review performance
    pub fn review_performance(&mut self, code: &CodeForReview) {
        if !self.config.check_performance {
            return;
        }

        // Check for O(n²) loops
        for quadratic in &code.quadratic_loops {
            self.add_finding(Finding {
                aspect: ReviewAspect::Performance,
                severity: FindingSeverity::Medium,
                location: quadratic.clone(),
                title: "Potential O(n²) complexity".to_string(),
                description: "Nested loops may cause performance issues".to_string(),
                suggestion: Some("Consider using HashMap or sorting".to_string()),
                references: vec![],
            });
        }

        // Check for unnecessary allocations
        for alloc in &code.unnecessary_allocations {
            self.add_finding(Finding {
                aspect: ReviewAspect::Performance,
                severity: FindingSeverity::Low,
                location: alloc.clone(),
                title: "Unnecessary allocation".to_string(),
                description: "Allocation inside loop or hot path".to_string(),
                suggestion: Some("Move allocation outside loop".to_string()),
                references: vec![],
            });
        }

        let issues = self.findings.iter()
            .filter(|f| f.aspect == ReviewAspect::Performance)
            .count();
        self.scores.insert(ReviewAspect::Performance, self.calculate_score(issues, code.total_items));
    }

    /// Review maintainability
    pub fn review_maintainability(&mut self, code: &CodeForReview) {
        // Check for code duplication
        for dup in &code.duplications {
            self.add_finding(Finding {
                aspect: ReviewAspect::Maintainability,
                severity: FindingSeverity::Medium,
                location: dup.locations.join(", "),
                title: "Code duplication".to_string(),
                description: format!("{} lines duplicated in {} places", dup.lines, dup.locations.len()),
                suggestion: Some("Extract common code to shared function".to_string()),
                references: vec![],
            });
        }

        let issues = self.findings.iter()
            .filter(|f| f.aspect == ReviewAspect::Maintainability)
            .count();
        self.scores.insert(ReviewAspect::Maintainability, self.calculate_score(issues, code.total_items));
    }

    /// Review readability
    pub fn review_readability(&mut self, code: &CodeForReview) {
        if !self.config.check_style {
            return;
        }

        // Check for magic numbers
        for magic in &code.magic_numbers {
            self.add_finding(Finding {
                aspect: ReviewAspect::Readability,
                severity: FindingSeverity::Low,
                location: magic.location.clone(),
                title: "Magic number".to_string(),
                description: format!("Unexplained literal: {}", magic.value),
                suggestion: Some("Extract to named constant".to_string()),
                references: vec![],
            });
        }

        let issues = self.findings.iter()
            .filter(|f| f.aspect == ReviewAspect::Readability)
            .count();
        self.scores.insert(ReviewAspect::Readability, self.calculate_score(issues, code.total_items));
    }

    /// Run full review
    pub fn review(&mut self, code: &CodeForReview) {
        let start = std::time::Instant::now();

        self.review_correctness(code);
        self.review_security(code);
        self.review_performance(code);
        self.review_maintainability(code);
        self.review_readability(code);

        self.duration = Some(start.elapsed());
    }

    /// Calculate score (100 - penalty per issue)
    fn calculate_score(&self, issues: usize, total_items: usize) -> f64 {
        if total_items == 0 {
            return 100.0;
        }
        let penalty = (issues as f64 / total_items as f64) * 100.0;
        (100.0 - penalty).max(0.0)
    }

    /// Get overall score
    pub fn overall_score(&self) -> f64 {
        if self.scores.is_empty() {
            return 100.0;
        }
        self.scores.values().sum::<f64>() / self.scores.len() as f64
    }

    /// Get findings by severity
    pub fn findings_by_severity(&self, severity: FindingSeverity) -> Vec<&Finding> {
        self.findings.iter().filter(|f| f.severity == severity).collect()
    }

    /// Get findings by aspect
    pub fn findings_by_aspect(&self, aspect: ReviewAspect) -> Vec<&Finding> {
        self.findings.iter().filter(|f| f.aspect == aspect).collect()
    }

    /// Check if review passes
    pub fn passes(&self) -> bool {
        let overall = self.overall_score();
        let critical = self.findings_by_severity(FindingSeverity::Critical);

        overall >= self.config.min_score && critical.is_empty()
    }

    /// Generate review report
    pub fn report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Dhyāna Review Report (Deep Analysis) ===\n\n");
        report.push_str(&format!("Overall Score: {:.1}%\n", self.overall_score()));
        report.push_str(&format!("Status: {}\n\n", if self.passes() { "PASSED ✓" } else { "FAILED ✗" }));

        report.push_str("Scores by Aspect:\n");
        for (aspect, score) in &self.scores {
            report.push_str(&format!("  {} ({}): {:.1}%\n",
                format!("{:?}", aspect), aspect.sanskrit(), score));
        }
        report.push('\n');

        // Critical findings first
        let critical = self.findings_by_severity(FindingSeverity::Critical);
        if !critical.is_empty() {
            report.push_str("CRITICAL FINDINGS:\n");
            for f in critical {
                report.push_str(&format!("  🔴 [{}] {}\n", f.location, f.title));
                report.push_str(&format!("     {}\n", f.description));
            }
            report.push('\n');
        }

        // High findings
        let high = self.findings_by_severity(FindingSeverity::High);
        if !high.is_empty() {
            report.push_str("HIGH SEVERITY:\n");
            for f in high {
                report.push_str(&format!("  🟠 [{}] {}\n", f.location, f.title));
            }
            report.push('\n');
        }

        if let Some(duration) = self.duration {
            report.push_str(&format!("Review completed in {:?}\n", duration));
        }

        report
    }

    /// Clear for reuse
    pub fn clear(&mut self) {
        self.findings.clear();
        self.scores.clear();
        self.duration = None;
    }
}

/// Code prepared for review
#[derive(Debug, Default)]
pub struct CodeForReview {
    pub total_items: usize,
    pub potential_nulls: Vec<NullIssue>,
    pub unreachable_code: Vec<String>,
    pub hardcoded_secrets: Vec<SecretIssue>,
    pub sql_injections: Vec<String>,
    pub quadratic_loops: Vec<String>,
    pub unnecessary_allocations: Vec<String>,
    pub duplications: Vec<Duplication>,
    pub magic_numbers: Vec<MagicNumber>,
}

#[derive(Debug)]
pub struct NullIssue {
    pub location: String,
    pub description: String,
}

#[derive(Debug)]
pub struct SecretIssue {
    pub location: String,
    pub pattern: String,
}

#[derive(Debug)]
pub struct Duplication {
    pub locations: Vec<String>,
    pub lines: usize,
}

#[derive(Debug)]
pub struct MagicNumber {
    pub location: String,
    pub value: String,
}

impl Default for DhyanaReviewer {
    fn default() -> Self {
        Self::new(DhyanaConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_findings() {
        let mut reviewer = DhyanaReviewer::default();

        let code = CodeForReview {
            total_items: 100,
            hardcoded_secrets: vec![SecretIssue {
                location: "config.rs:10".to_string(),
                pattern: "API_KEY=...".to_string(),
            }],
            ..Default::default()
        };

        reviewer.review_security(&code);

        let security_findings = reviewer.findings_by_aspect(ReviewAspect::Security);
        assert!(!security_findings.is_empty());
        assert_eq!(security_findings[0].severity, FindingSeverity::Critical);
    }

    #[test]
    fn test_overall_score() {
        let mut reviewer = DhyanaReviewer::default();

        let code = CodeForReview {
            total_items: 100,
            ..Default::default()
        };

        reviewer.review(&code);

        // No issues = perfect score
        assert!(reviewer.overall_score() >= 99.0);
        assert!(reviewer.passes());
    }
}
