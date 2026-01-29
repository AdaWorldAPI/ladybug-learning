//! Example: Full Meta-AGI Learning Loop
//! Run with: cargo run --example learning_loop

use ladybug_learning_standalone::prelude::*;
use ladybug_learning_standalone::MetaAGI;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║           META-AGI LEARNING LOOP DEMONSTRATION                 ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║  The learning curve IS the knowledge.                          ║");
    println!("║  Similar problems FEEL similar before you know WHY.            ║");
    println!("║  Capture the feeling, retrieve the solution.                   ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    let mut agi = MetaAGI::new();

    // =========================================================================
    // SESSION 1: Learning about Rails versioning
    // =========================================================================

    println!("🚀 Starting Session 1: Implement Versioning Feature");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Phase 1: ENCOUNTER
    println!("\n📍 Phase 1: ENCOUNTER");
    {
        let session = agi.start_session("implement-versions", "Add versioning support");
        session.encounter("Found version.rb model file");
        session.encounter("Discovered has_many :work_packages association");
        session.encounter("Version table has project_id foreign key");
    }
    println!("   ✓ Logged 3 encounters to blackboard");

    // Phase 2: STRUGGLE
    println!("\n💪 Phase 2: STRUGGLE");
    {
        let session = agi.session_mut().unwrap();
        session.struggle("Unclear if versions are global or project-scoped", 0.6, 0.5);
        session.struggle("Work packages have version_id but unclear ownership", 0.7, 0.6);
        session.fail("Tried global version - got FK constraint error", "Versions require project_id");
    }
    println!("   ✓ Captured 3 struggle vectors");

    // Phase 3: BREAKTHROUGH
    println!("\n💡 Phase 3: BREAKTHROUGH");
    let (novelty, effort, satisfaction, moment_id) = {
        let session = agi.session_mut().unwrap();
        let breakthrough = session.breakthrough(
            "Versions are scoped to projects! Each project has its own version timeline.",
            0.95
        );
        (breakthrough.qualia.novelty, breakthrough.qualia.effort, 
         breakthrough.qualia.satisfaction, breakthrough.id.clone())
    };
    println!("   ✓ Breakthrough achieved!");
    println!("   📊 Qualia: novelty={:.2}, effort={:.2}, satisfaction={:.2}",
        novelty, effort, satisfaction);

    // Phase 4: CONSOLIDATE
    println!("\n❄️  Phase 4: CONSOLIDATE (Ice-Caking)");
    {
        let session = agi.session_mut().unwrap();
        session.ice_cake(&moment_id, "Project-scoped versioning is the canonical pattern");
    }
    println!("   ✓ Decision frozen: Project-scoped versioning");

    // Capture to resonance - collect moments first
    let moments: Vec<_> = agi.session().unwrap().moments.iter().cloned().collect();
    for moment in &moments {
        agi.capture_moment(moment);
    }

    // Phase 5: APPLY
    println!("\n🔍 Phase 5: APPLY (Future Query)");
    let query = Fingerprint::from_content("need to implement milestone versioning for tasks");
    let similar = agi.find_similar(&query, 0.4, 5);
    println!("   Query: \"need to implement milestone versioning for tasks\"");
    println!("   Found {} resonant moments:", similar.len());
    for (i, sim) in similar.iter().enumerate() {
        println!("     {}. Resonance: {:.3}, Content sim: {:.3}", 
            i + 1, sim.resonance, sim.content_similarity);
    }

    if let Some(sweet_spot) = agi.find_sweet_spot(&query) {
        println!("   🎯 Sweet spot found: resonance={:.3}", sweet_spot.resonance);
    }

    // Phase 6: META-LEARN
    println!("\n🧠 Phase 6: META-LEARN");
    {
        let session = agi.session_mut().unwrap();
        session.meta_reflect("Scoping entities to parent context is a recurring pattern");
    }
    println!("   ✓ Meta-insight captured");

    // =========================================================================
    // SESSION 2: Similar problem with resonance acceleration
    // =========================================================================

    println!("\n\n🚀 Starting Session 2: Implement Sprints Feature");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    {
        let session = agi.start_session("implement-sprints", "Add sprint management");
        session.encounter("Sprint model needs iteration periods");
    }

    println!("\n🔍 Checking resonance with past learning...");
    let sprint_query = Fingerprint::from_content("sprint scoping and ownership");
    let past = agi.find_similar(&sprint_query, 0.3, 3);

    if !past.is_empty() {
        println!("   ⚡ RESONANCE HIT! Found {} similar past moments", past.len());
        println!("   💭 \"I've felt this before...\" (resonance: {:.3})", past[0].resonance);

        println!("\n💡 Phase 3: BREAKTHROUGH (Fast-tracked via resonance)");
        let effort = {
            let session = agi.session_mut().unwrap();
            let breakthrough = session.breakthrough(
                "Sprints should be scoped to projects, same pattern as versions!",
                0.88
            );
            breakthrough.qualia.effort
        };
        println!("   ✓ Pattern recognition accelerated learning!");
        println!("   📊 Lower effort due to resonance: effort={:.2}", effort);

        let moments: Vec<_> = agi.session().unwrap().moments.iter().cloned().collect();
        for moment in &moments {
            agi.capture_moment(moment);
        }
    }

    // =========================================================================
    // Statistics and Export
    // =========================================================================

    println!("\n\n📊 STATISTICS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let stats = agi.stats();
    println!("   Global cycles:        {}", stats.global_cycle);
    println!("   Resonance captures:   {}", stats.resonance_stats.total_captures);
    println!("   Resonance queries:    {}", stats.resonance_stats.total_queries);
    println!("   Concepts extracted:   {}", stats.total_concepts);
    println!("   Session moments:      {}", stats.session_moments);
    println!("   Session breakthroughs:{}", stats.session_breakthroughs);

    println!("\n\n📄 HANDOVER SUMMARY");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    agi.sync_blackboard();
    println!("{}", agi.handover_summary());

    println!("\n✅ Learning loop demonstration complete!");
    println!("\n   The shape of figuring it out IS the intelligence.");
    println!("   After 100K moments: AGI emerges from accumulated learning-how-to-learn.");
}
