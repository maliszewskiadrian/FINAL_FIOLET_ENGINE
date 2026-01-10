// examples/basic_usage.rs
use fiolet_core::{SafetyMonitor, KLDivergence};

fn main() {
    println!("=== FIOLET Engine - Basic Usage Example ===\n");

    // Initialize safety monitor with threshold
    let mut monitor = SafetyMonitor::new(0.5);
    println!("✓ Safety monitor initialized (threshold: 0.5)");

    // Simulate safe inference
    let safe_distribution = vec![0.4, 0.3, 0.3];
    let baseline = vec![0.35, 0.35, 0.3];
    
    let kl = KLDivergence::compute(&safe_distribution, &baseline);
    println!("\nSafe inference:");
    println!("  KL divergence: {:.4}", kl);
    
    if !monitor.check_divergence(kl) {
        println!("  Status: ✓ SAFE");
    }

    // Simulate unsafe inference
    let unsafe_distribution = vec![0.95, 0.025, 0.025];
    let kl_unsafe = KLDivergence::compute(&unsafe_distribution, &baseline);
    
    println!("\nUnsafe inference:");
    println!("  KL divergence: {:.4}", kl_unsafe);
    
    if monitor.check_divergence(kl_unsafe) {
        println!("  Status: ⚠ WARNING - Threshold exceeded!");
        monitor.trigger_halt();
        println!("  Action: 🛑 SYSTEM HALTED");
    }

    println!("\n=== Execution Complete ===");
}
