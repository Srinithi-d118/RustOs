use alloc::vec::Vec;
use alloc::vec;
use spin::Mutex;
use crate::println;

#[derive(Clone)]
pub struct SnapApp {
    pub id: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub version: &'static str,
    pub publisher: &'static str,
    pub confinement: &'static str,
    pub description: &'static str,
    pub rating: &'static str,
    pub installed: bool,
}

pub struct SnapStore {
    pub apps: Vec<SnapApp>,
}

impl SnapStore {
    pub fn new() -> Self {
        SnapStore {
            apps: vec![
                SnapApp {
                    id: "rustos-shooter",
                    name: "RustOS Space Shooter",
                    category: "Games",
                    version: "v1.2.0",
                    publisher: "RustOS Core Team",
                    confinement: "strict",
                    description: "Fast-paced retro arcade shooter running in bare-metal VGA text mode",
                    rating: "4.9/5 (1,280 votes)",
                    installed: true,
                },
                SnapApp {
                    id: "snake-arcade",
                    name: "Arcade Snake Classic",
                    category: "Games",
                    version: "v1.0.0",
                    publisher: "Retro Games Lab",
                    confinement: "strict",
                    description: "Classic grid-based snake game with score tracking and speed levels",
                    rating: "4.8/5 (950 votes)",
                    installed: false,
                },
                SnapApp {
                    id: "sys-bench",
                    name: "RustOS System Benchmarks",
                    category: "System Tools",
                    version: "v2.0.1",
                    publisher: "Performance Lab",
                    confinement: "classic",
                    description: "High-precision CPU cycle, page allocation, and interrupt latency profiler",
                    rating: "4.7/5 (840 votes)",
                    installed: true,
                },
                SnapApp {
                    id: "sched-sim",
                    name: "Process Scheduler Sim",
                    category: "Education",
                    version: "v2.0.0",
                    publisher: "Systems Engineering",
                    confinement: "strict",
                    description: "Comparative visual evaluation of FCFS, SJF, Priority, and Round Robin",
                    rating: "5.0/5 (1,120 votes)",
                    installed: true,
                },
                SnapApp {
                    id: "netmon",
                    name: "RustOS NetMonitor",
                    category: "Networking",
                    version: "v1.4.2",
                    publisher: "NetOps",
                    confinement: "strict",
                    description: "Packet monitor and PCI Ethernet/Wi-Fi adapter traffic analyzer",
                    rating: "4.8/5 (920 votes)",
                    installed: true,
                },
            ],
        }
    }

    pub fn display_store(&self) {
        println!("================================================================================");
        println!("                         RustOS SnapStore (App Center)                          ");
        println!("   [Educational Prototype - Simulating Snapd Isolated Container Architecture]   ");
        println!("================================================================================");
        println!("Featured Categories: [Games]  [System Tools]  [Networking]  [Education]");
        println!("--------------------------------------------------------------------------------");
        println!("{:<16} {:<14} {:<10} {:<12} {:<18}", "APPLICATION", "CATEGORY", "VERSION", "STATUS", "PUBLISHER");
        println!("--------------------------------------------------------------------------------");
        for app in self.apps.iter() {
            let status = if app.installed { "[Installed]" } else { "[Available]" };
            println!("{:<16} {:<14} {:<10} {:<12} {:<18}",
                app.name, app.category, app.version, status, app.publisher);
            println!("  -> Description: {} (Rating: {})", app.description, app.rating);
            println!("     Confinement: {} | Tracking: latest/stable", app.confinement);
            println!();
        }
        println!("Use 'snap install <id>' or 'snap info <id>' to manage applications.");
        println!("================================================================================");
    }

    pub fn list(&self) {
        println!("{:<16} {:<10} {:<6} {:<14} {:<18} {:<10}", 
            "Name", "Version", "Rev", "Tracking", "Publisher", "Notes");
        println!("--------------------------------------------------------------------------------");
        for (i, app) in self.apps.iter().enumerate() {
            if app.installed {
                println!("{:<16} {:<10} {:<6} {:<14} {:<18} {:<10}",
                    app.id, app.version, (i + 1) * 14, "latest/stable", app.publisher, app.confinement);
            }
        }
    }

    pub fn find(&self, query: &str) {
        println!("{:<16} {:<10} {:<16} {:<10} {}", 
            "Name", "Version", "Publisher", "Status", "Summary");
        for app in self.apps.iter() {
            if query.is_empty() || app.id.contains(query) || app.name.contains(query) || app.category.contains(query) {
                let status = if app.installed { "installed" } else { "available" };
                println!("{:<16} {:<10} {:<16} {:<10} {}",
                    app.id, app.version, app.publisher, status, app.description);
            }
        }
    }

    pub fn install(&mut self, id: &str) {
        if let Some(app) = self.apps.iter_mut().find(|a| a.id == id) {
            if app.installed {
                println!("snap \"{}\" is already installed, see 'snap info {}'", app.id, app.id);
                return;
            }
            println!("Download snap \"{}\" (rev 104) from channel \"latest/stable\"", app.id);
            println!("Fetching and mounting squashfs sandbox layer...");
            println!("Mounting snap file system into /snap/{} ... OK", app.id);
            println!("Setting up security profile (AppArmor confinement: {}) ... OK", app.confinement);
            app.installed = true;
            // Also sync with APT database
            if let Some(pkg) = crate::apt::APT_DB.lock().packages.iter_mut().find(|p| p.name == id) {
                pkg.installed = true;
            }
            println!("{} {} from {} installed successfully!", app.name, app.version, app.publisher);
        } else {
            println!("error: snap \"{}\" not found in SnapStore catalog", id);
        }
    }

    pub fn remove(&mut self, id: &str) {
        if let Some(app) = self.apps.iter_mut().find(|a| a.id == id) {
            if !app.installed {
                println!("snap \"{}\" is not installed", app.id);
                return;
            }
            println!("Unmounting snap \"{}\" (rev 104) ...", app.id);
            println!("Removing sandbox isolation namespace...");
            app.installed = false;
            // Also sync with APT database
            if let Some(pkg) = crate::apt::APT_DB.lock().packages.iter_mut().find(|p| p.name == id) {
                pkg.installed = false;
            }
            println!("{} removed successfully.", app.name);
        } else {
            println!("error: snap \"{}\" not found", id);
        }
    }

    pub fn info(&self, id: &str) {
        if let Some(app) = self.apps.iter().find(|a| a.id == id) {
            println!("name:        {}", app.id);
            println!("title:       {}", app.name);
            println!("summary:     {}", app.description);
            println!("publisher:   {}", app.publisher);
            println!("license:     MIT / Apache-2.0");
            println!("confinement: {}", app.confinement);
            println!("installed:   {} (rev 104) status: {}",
                app.version, if app.installed { "active" } else { "not-installed" });
            println!("channels:");
            println!("  latest/stable:    {} (104) [confinement: {}]", app.version, app.confinement);
            println!("  latest/candidate: {} (105) [confinement: {}]", app.version, app.confinement);
        } else {
            println!("error: no snap found matching \"{}\"", id);
        }
    }
}

pub static SNAP_STORE: Mutex<SnapStore> = Mutex::new(SnapStore { apps: Vec::new() });

pub fn init() {
    *SNAP_STORE.lock() = SnapStore::new();
}

pub fn handle_snap_command(args: &[&str]) {
    if args.is_empty() {
        SNAP_STORE.lock().display_store();
        return;
    }

    match args[0] {
        "list" => SNAP_STORE.lock().list(),
        "find" => {
            if args.len() < 2 {
                SNAP_STORE.lock().find("");
            } else {
                SNAP_STORE.lock().find(args[1]);
            }
        }
        "install" => {
            if args.len() < 2 {
                println!("error: specify snap to install (e.g. 'snap install snake-arcade')");
            } else {
                SNAP_STORE.lock().install(args[1]);
            }
        }
        "remove" => {
            if args.len() < 2 {
                println!("error: specify snap to remove (e.g. 'snap remove sys-bench')");
            } else {
                SNAP_STORE.lock().remove(args[1]);
            }
        }
        "info" => {
            if args.len() < 2 {
                println!("error: specify snap to query info for");
            } else {
                SNAP_STORE.lock().info(args[1]);
            }
        }
        _ => {
            println!("unknown command \"{}\", see 'snap --help' or type 'snapstore'", args[0]);
        }
    }
}