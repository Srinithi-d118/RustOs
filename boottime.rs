use spin::Mutex;
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::println;

#[derive(Clone, Copy)]
pub struct BootMilestone {
    pub name: &'static str,
    pub cycle: u64,
    pub timestamp_ms: u64,
}

const MAX_MILESTONES: usize = 32;

pub static BOOT_MILESTONES: Mutex<[BootMilestone; MAX_MILESTONES]> = Mutex::new([BootMilestone {
    name: "",
    cycle: 0,
    timestamp_ms: 0,
}; MAX_MILESTONES]);

pub static MILESTONE_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn record_milestone(name: &'static str) {
    let cycle = unsafe { core::arch::x86_64::_rdtsc() };
    let timestamp_ms = crate::interrupts::uptime_ms();
    let idx = MILESTONE_COUNT.fetch_add(1, Ordering::SeqCst);
    if idx < MAX_MILESTONES {
        let mut list = BOOT_MILESTONES.lock();
        list[idx] = BootMilestone {
            name,
            cycle,
            timestamp_ms,
        };
    }
}

pub fn print_boot_report() {
    let count = MILESTONE_COUNT.load(Ordering::SeqCst).min(MAX_MILESTONES);
    if count == 0 {
        println!("No boot milestones recorded.");
        return;
    }

    let milestones = BOOT_MILESTONES.lock();
    let first_cycle = milestones[0].cycle;
    let last_cycle = milestones[count - 1].cycle;
    let total_cycles = last_cycle.saturating_sub(first_cycle).max(1);
    let total_ms = milestones[count - 1].timestamp_ms;

    println!("================================================================================");
    println!("                   RustOS Kernel Boot-Time Telemetry Report                    ");
    println!("================================================================================");
    println!("{:<32} {:<14} {:<14} {:<12}", "BOOT COMPONENT / MILESTONE", "TSC CYCLES", "DELTA (K-CYC)", "% OVERHEAD");
    println!("--------------------------------------------------------------------------------");

    let mut prev_cycle = first_cycle;
    for i in 0..count {
        let m = &milestones[i];
        let delta_cycles = m.cycle.saturating_sub(prev_cycle);
        let pct_x10 = (delta_cycles as u128 * 1000 / total_cycles as u128) as u64;
        println!("{:<32} {:<14} {:<14} {:>3}.{}%",
            m.name,
            m.cycle.saturating_sub(first_cycle),
            delta_cycles / 1000,
            pct_x10 / 10,
            pct_x10 % 10
        );
        prev_cycle = m.cycle;
    }

    println!("--------------------------------------------------------------------------------");
    println!("Total Initialization Cycles: {} cycles", total_cycles);
    println!("Active Timer Boot Elapsed:  {} ms (at ~100 Hz PIT calibration)", total_ms);
    println!("Hardware TSC Frequency est:  ~2.00 GHz (Intel Tiger Lake i3-1125G4 baseline)");
    println!();
    println!("--- Component Boot Overhead Breakdown (Visual Graph) ---");
    prev_cycle = first_cycle;
    let bar_chars = "====================================";
    for i in 1..count {
        let m = &milestones[i];
        let delta_cycles = m.cycle.saturating_sub(prev_cycle);
        let pct_x10 = (delta_cycles as u128 * 1000 / total_cycles as u128) as u64;
        let bar_len = ((delta_cycles as u128 * 35 / total_cycles as u128) as usize).min(bar_chars.len()).max(1);
        println!("{:<24} [{:<35}] {:>3}.{}%", 
            m.name, 
            &bar_chars[..bar_len], 
            pct_x10 / 10,
            pct_x10 % 10
        );
        prev_cycle = m.cycle;
    }
    println!("================================================================================");
}