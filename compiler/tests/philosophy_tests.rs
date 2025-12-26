//! Integration tests for Hindu Philosophy systems in Jagannath compiler
//!
//! Tests all philosophy modules:
//! - Nyāya: 4-pramāṇa type inference
//! - Mīmāṃsā: 6-pramāṇa extended inference
//! - Advaita: Unified memory model
//! - Sāṃkhya: 25-tattva pipeline
//! - Yoga: Ashtanga SDLC, Chakra architecture
//! - Vedic Math: 16-sūtra constant folding
//! - Tantra: SIMD optimization
//! - Āyurveda: System health monitoring

use jagannath_compiler::philosophy::nyaya::*;
use jagannath_compiler::mimamsa::*;
use jagannath_compiler::yoga::ashtanga::*;
use jagannath_compiler::yoga::chakra::*;
use jagannath_compiler::vedic_math::*;
use jagannath_compiler::tantra::*;
use jagannath_compiler::ayurveda::*;
use jagannath_compiler::philosophy::advaita::*;
use jagannath_compiler::philosophy::samkhya::*;

// ═══════════════════════════════════════════════════════════════════
// NYĀYA TESTS — Four Pramāṇa Type Inference
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_nyaya_pratyaksha_inference() {
    // Pratyakṣa: Direct perception = explicit type declaration
    let mut inference = NyayaInference::new();

    let evidence = inference.infer_pratyaksha("x", "i32");

    assert!(evidence.is_some());
    let ev = evidence.unwrap();
    assert_eq!(ev.pramana, Pramana::Pratyaksha);
    assert_eq!(ev.certainty, 1.0); // 100% certain
}

#[test]
fn test_nyaya_anumana_inference() {
    // Anumāna: Inference from context
    let mut inference = NyayaInference::new();

    // Integer literal → i32
    let evidence = inference.infer("x", "42", &[], "");

    assert!(evidence.is_some());
    let ev = evidence.unwrap();
    assert_eq!(ev.pramana, Pramana::Anumana);
    assert!(ev.certainty > 0.8); // High certainty
}

#[test]
fn test_nyaya_upamana_inference() {
    // Upamāna: Inference by analogy
    let mut inference = NyayaInference::new();

    // Similar to known pattern
    let context = vec![
        ("process", "fn(Vec<i32>) -> Vec<i32>"),
        ("data", "Vec<i32>"),
    ];

    let evidence = inference.infer("result", "process(data)", &context, "");

    assert!(evidence.is_some());
    let ev = evidence.unwrap();
    assert_eq!(ev.type_name, "Vec<i32>");
}

#[test]
fn test_nyaya_shabda_inference() {
    // Śabda: Verbal testimony from documentation
    let mut inference = NyayaInference::new();

    let docs = "Returns the count of items as a 64-bit unsigned integer";
    let evidence = inference.infer_shabda("count", docs);

    assert!(evidence.is_some());
    let ev = evidence.unwrap();
    assert_eq!(ev.pramana, Pramana::Shabda);
}

#[test]
fn test_nyaya_panchavayava() {
    // Five-membered syllogism
    let pramana = PramanaValidator::new();

    let pratijna = "n is positive";
    let hetu = "n > 0";
    let udaharana = "all numbers greater than zero are positive";
    let upanaya = "n = 5, which is > 0";
    let nigamana = "therefore n is positive";

    let syllogism = Panchavayava {
        pratijna: pratijna.to_string(),
        hetu: hetu.to_string(),
        udaharana: udaharana.to_string(),
        upanaya: upanaya.to_string(),
        nigamana: nigamana.to_string(),
    };

    assert!(pramana.validate_syllogism(&syllogism).is_valid);
}

// ═══════════════════════════════════════════════════════════════════
// MĪMĀṂSĀ TESTS — Six Pramāṇa Extended Inference
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_mimamsa_6_pramana() {
    // 6 pramāṇas = Nyāya 4 + Arthāpatti + Anupalabdhi
    let mut inference = MimamsaInference::new();

    let evidence = inference.infer_6_pramana(
        "value",
        "compute()",
        &[("compute", "fn() -> i64")],
        "",
        &["has_return_type"],
    );

    assert!(evidence.is_some());
}

#[test]
fn test_arthapatti_inference() {
    // Arthāpatti: Presumption from circumstance
    let mut engine = ArthapattEngine::new();

    // If we see x + 5 where x: u32, result must be numeric
    let inferred = engine.infer_from_operation("add", "u32");

    assert!(inferred.is_some());
    let inf = inferred.unwrap();
    assert!(inf.type_name.contains("u32") || inf.type_name.contains("numeric"));
}

#[test]
fn test_anupalabdhi_inference() {
    // Anupalabdhi: Inference from absence
    let mut engine = AnupalabdhiEngine::new();

    // No `mut` keyword → immutable
    let features = vec!["typed", "initialized"];
    let conclusion = engine.infer_from_absence(&features);

    assert!(conclusion.is_some());
    // Should conclude immutability from absence of `mut`
}

// ═══════════════════════════════════════════════════════════════════
// ADVAITA TESTS — Unified Memory Model
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_brahman_memory() {
    // Brahman: Unified memory substrate
    let mut brahman = BrahmanMemory::new();

    let handle = brahman.materialize("test_allocation", 1024);

    assert!(handle > 0);
    assert!(brahman.is_manifested(handle));
}

#[test]
fn test_maya_overlay() {
    // Maya: Type overlays on unified memory
    let mut maya = MayaOverlay::new();

    let base = 0x1000usize;
    maya.project::<u32>(base);
    maya.project::<f32>(base);

    // Same memory, different type views
    assert!(maya.has_overlay(base));
}

#[test]
fn test_atman_optimizer() {
    // Ātman: Value identity optimization
    let mut atman = AtmanOptimizer::new();

    let id1 = atman.get_identity(42);
    let id2 = atman.get_identity(42);

    // Same value should have same identity
    assert_eq!(id1, id2);
}

// ═══════════════════════════════════════════════════════════════════
// SĀṂKHYA TESTS — 25-Tattva Pipeline
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_samkhya_pipeline() {
    let mut pipeline = SamkhyaPipeline::new();

    // Should have all 25 tattvas
    assert_eq!(pipeline.tattva_count(), 25);

    // Pipeline flows from subtle to gross
    assert_eq!(pipeline.current_tattva(), Tattva::Purusha);

    pipeline.advance();
    assert_eq!(pipeline.current_tattva(), Tattva::Prakriti);
}

#[test]
fn test_samkhya_tattva_properties() {
    // Each tattva has specific compilation role
    let purusha = Tattva::Purusha;
    assert_eq!(purusha.level(), 25);
    assert!(purusha.is_subtle());

    let prthivi = Tattva::Prthivi;
    assert_eq!(prthivi.level(), 1);
    assert!(prthivi.is_gross());
}

// ═══════════════════════════════════════════════════════════════════
// ASHTANGA YOGA TESTS — 8-Limb SDLC
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_ashtanga_lifecycle() {
    let mut lifecycle = AshtangaLifecycle::new();

    // Start at Yama (code standards)
    assert_eq!(lifecycle.current_anga(), Anga::Yama);

    // Progress through limbs
    assert!(lifecycle.advance().is_ok());
    assert_eq!(lifecycle.current_anga(), Anga::Niyama);
}

#[test]
fn test_yama_code_standards() {
    let analyzer = YamaAnalyzer::new();

    let code = r#"
        fn safe_divide(a: i32, b: i32) -> Option<i32> {
            if b == 0 { None } else { Some(a / b) }
        }
    "#;

    // Ahiṃsā: Non-violence (no harmful patterns)
    let violations = analyzer.check_ahimsa(code);
    assert!(violations.is_empty());
}

#[test]
fn test_niyama_best_practices() {
    let analyzer = NiyamaAnalyzer::new();

    // Śauca: Purity/Clean code
    let code = "fn well_named() -> i32 { 42 }";
    let score = analyzer.check_shauca(code);
    assert!(score > 0.8);
}

#[test]
fn test_pratyahara_encapsulation() {
    let analyzer = PratyaharaAnalyzer::new();

    // Check for proper encapsulation
    let symbols = vec![
        Symbol::new("internal_helper", Visibility::Private),
        Symbol::new("public_api", Visibility::Public),
    ];

    let report = analyzer.analyze(&symbols);
    assert!(report.encapsulation_score() > 0.7);
}

#[test]
fn test_dharana_srp() {
    let analyzer = DharanaAnalyzer::new();

    // Single Responsibility Principle
    let component = Component::new("Calculator")
        .with_method("add")
        .with_method("subtract");

    let violations = analyzer.check_srp(&component);
    assert!(violations.is_empty());
}

#[test]
fn test_dhyana_code_review() {
    let reviewer = DhyanaReviewer::new();

    let code = "fn example() { let x = 42; }";
    let findings = reviewer.review(code);

    // Should complete without panic
    assert!(findings.quality_score() > 0.0);
}

#[test]
fn test_samadhi_deployment() {
    let mut deployment = SamadhiDeployment::new();

    // Check deployment readiness
    assert_eq!(deployment.current_stage(), DeploymentStage::Building);

    // Add passing criteria
    deployment.add_criterion(ReadinessCriterion::AllTestsPass);
    deployment.mark_criterion_met(ReadinessCriterion::AllTestsPass);

    // Can advance stage
    assert!(deployment.can_advance());
}

// ═══════════════════════════════════════════════════════════════════
// CHAKRA TESTS — 7-Layer Architecture
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_chakra_architecture() {
    let mut arch = ChakraArchitecture::new();

    // Assign components to chakras
    arch.assign("driver.rs".to_string(), Chakra::Muladhara);
    arch.assign("business_logic.rs".to_string(), Chakra::Anahata);
    arch.assign("api_handler.rs".to_string(), Chakra::Vishuddha);

    // Validate dependencies
    let violations = arch.validate_dependencies();
    assert!(violations.is_empty());
}

#[test]
fn test_chakra_optimizer() {
    let mut optimizer = ChakraOptimizer::default();

    // Add optimization passes per chakra
    optimizer.add_pass(OptimizationPass::new(Chakra::Muladhara, "register_alloc"));
    optimizer.add_pass(OptimizationPass::new(Chakra::Manipura, "inline"));

    // Run at optimization level 2
    let result = optimizer.optimize(2);
    assert!(result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════
// VEDIC MATH TESTS — 16-Sūtra Constant Folding
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_vedic_ekadhikena() {
    // Sūtra 1: Ekādhikena Pūrveṇa
    let vm = VedicMath::new();

    // Multiply by number ending in 5
    let result = vm.ekadhikena_purvena(25, 25);
    assert_eq!(result, 625);
}

#[test]
fn test_vedic_nikhilam() {
    // Sūtra 2: Nikhilam Navataścaramam Daśataḥ
    let vm = VedicMath::new();

    // Near-base multiplication
    let result = vm.nikhilam(98, 97, 100);
    assert_eq!(result, 9506);
}

#[test]
fn test_vedic_urdhva_tiryak() {
    // Sūtra 3: Ūrdhva-Tiryagbhyām (crosswise multiplication)
    let vm = VedicMath::new();

    let result = vm.urdhva_tiryak(12, 13);
    assert_eq!(result, 156);
}

#[test]
fn test_vedic_constant_folder() {
    let mut folder = VedicConstantFolder::new();

    // Fold constant expressions
    let expr = Expr::BinOp(
        Box::new(Expr::Const(10)),
        BinOp::Mul,
        Box::new(Expr::Const(10)),
    );

    let folded = folder.fold(expr);

    if let Expr::Const(v) = folded {
        assert_eq!(v, 100);
    } else {
        panic!("Expected constant");
    }
}

#[test]
fn test_vedic_best_sutra_selection() {
    let vm = VedicMath::new();

    // Numbers near 100 → use Nikhilam
    let sutra = vm.best_multiply_sutra(98, 97);
    assert_eq!(sutra, VedicSutra::Nikhilam);

    // Numbers ending in 5 → use Ekādhikena
    let sutra = vm.best_multiply_sutra(35, 35);
    assert_eq!(sutra, VedicSutra::EkadhikenaPurvena);
}

// ═══════════════════════════════════════════════════════════════════
// TANTRA TESTS — SIMD Optimization
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_sri_yantra_tiling() {
    let yantra = ShriYantra::new();

    // Get optimal tiling for matrix multiply
    let tiling = yantra.optimal_tiling(1024, 1024, 1024);

    assert!(tiling.tile_m > 0);
    assert!(tiling.tile_n > 0);
    assert!(tiling.tile_k > 0);
}

#[test]
fn test_kundalini_cache_flow() {
    let flow = KundaliniFlow::default();

    // Analyze data structure for cache efficiency
    let structure = DataStructure::new("Matrix", 8, 1024 * 1024);
    let recommendation = flow.recommend_alignment(&structure);

    assert!(recommendation.alignment >= 64); // At least cache line
}

#[test]
fn test_mandala_scheduler() {
    let mut scheduler = MandalaScheduler::default();

    // Schedule tasks in concentric priority rings
    scheduler.add_task(Task::new("critical", RingPriority::Bindu));
    scheduler.add_task(Task::new("normal", RingPriority::Padma));
    scheduler.add_task(Task::new("background", RingPriority::Bhupura));

    // Bindu (center) tasks first
    let next = scheduler.next_task();
    assert!(next.is_some());
    assert_eq!(next.unwrap().priority, RingPriority::Bindu);
}

// ═══════════════════════════════════════════════════════════════════
// ĀYURVEDA TESTS — System Health Monitoring
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_ayurveda_dosha_balance() {
    let mut monitor = AyurvedaMonitor::new();

    // Update system metrics
    monitor.update_vata(50.0);   // CPU
    monitor.update_pitta(40.0);  // Memory
    monitor.update_kapha(30.0);  // Disk

    let balance = monitor.dosha_balance();
    assert!(balance.is_healthy());
}

#[test]
fn test_ayurveda_recommendations() {
    let mut monitor = AyurvedaMonitor::new();

    // Simulate high CPU (Vāta imbalance)
    monitor.update_vata(95.0);

    let recs = monitor.recommendations();
    assert!(!recs.is_empty());
    assert!(recs.iter().any(|r| r.contains("CPU") || r.contains("Vāta")));
}

#[test]
fn test_ayurveda_prakriti() {
    // System constitution analysis
    let monitor = AyurvedaMonitor::new();

    let prakriti = monitor.analyze_prakriti();

    // Should identify dominant dosha
    assert!(matches!(
        prakriti.dominant,
        Dosha::Vata | Dosha::Pitta | Dosha::Kapha
    ));
}

// ═══════════════════════════════════════════════════════════════════
// INTEGRATION TESTS — Philosophy Engine
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_philosophy_engine_creation() {
    let engine = PhilosophyEngine::new();

    // Should not be deployment-ready initially
    assert!(!engine.is_samadhi_ready());
}

#[test]
fn test_philosophy_engine_inference() {
    let mut engine = PhilosophyEngine::new();

    let evidence = engine.infer_type(
        "result",
        "a + b",
        &[("a", "i32"), ("b", "i32")],
        "",
        &[],
    );

    assert!(evidence.is_some());
}

#[test]
fn test_philosophy_engine_health() {
    let mut engine = PhilosophyEngine::new();

    engine.update_health(80.0, 60.0, 40.0);
    let recs = engine.health_recommendations();

    // High CPU should trigger recommendation
    assert!(!recs.is_empty());
}

#[test]
fn test_philosophy_engine_report() {
    let engine = PhilosophyEngine::new();

    let report = engine.report();

    assert!(report.contains("Jagannath"));
    assert!(report.contains("Ashtanga"));
    assert!(report.contains("Āyurveda"));
}
