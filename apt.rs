use alloc::vec::Vec;
use alloc::vec;

use spin::Mutex;
use crate::{print, println};

#[derive(Clone)]
pub struct Package {
    pub name: &'static str,
    pub version: &'static str,
    pub section: &'static str,
    pub size_kb: u32,
    pub description: &'static str,
    pub installed: bool,
    pub dependencies: &'static [&'static str],
}

pub struct PackageDatabase {
    pub packages: Vec<Package>,
}

impl PackageDatabase {
    pub fn new() -> Self {
        PackageDatabase {
            packages: vec![
                Package {
                    name: "rustos-shooter",
                    version: "1.2.0-rustos1",
                    section: "games",
                    size_kb: 64,
                    description: "High-octane arcade Space Shooter mini-game for VGA text mode",
                    installed: true,
                    dependencies: &["rust-core"],
                },
                Package {
                    name: "snake-arcade",
                    version: "1.0.0-rustos1",
                    section: "games",
                    size_kb: 48,
                    description: "Retro arcade Snake game with score tracking and speed levels",
                    installed: false,
                    dependencies: &["rust-core"],
                },
                Package {
                    name: "net-tools",
                    version: "2.1.0-1ubuntu1",
                    section: "net",
                    size_kb: 128,
                    description: "Networking utilities: ping, ifconfig, and Realtek RTL8139 probe",
                    installed: true,
                    dependencies: &["rust-core"],
                },
                Package {
                    name: "sys-bench",
                    version: "1.0.4-dev",
                    section: "utils",
                    size_kb: 96,
                    description: "Kernel micro-benchmarks for memory allocation, timer jitter, and CPU ticks",
                    installed: true,
                    dependencies: &["rust-core"],
                },
                Package {
                    name: "sched-sim",
                    version: "2.0.0-paper",
                    section: "admin",
                    size_kb: 112,
                    description: "CPU scheduling simulator (FCFS, SJF, Priority, Round Robin with Gantt charts)",
                    installed: true,
                    dependencies: &["rust-core"],
                },
                Package {
                    name: "snap-core",
                    version: "2.60.4",
                    section: "system",
                    size_kb: 512,
                    description: "Application container and SnapStore runtime interface",
                    installed: true,
                    dependencies: &["rust-core", "net-tools"],
                },
                Package {
                    name: "rust-core",
                    version: "1.99.1-nightly",
                    section: "devel",
                    size_kb: 1024,
                    description: "Core Rust no_std kernel libraries, allocator, and interrupt vectors",
                    installed: true,
                    dependencies: &[],
                },
            ],
        }
    }

    pub fn update(&self) {
        println!("Hit:1 http://archive.rustos.org/debian bookworm InRelease");
        println!("Get:2 http://archive.rustos.org/debian bookworm-updates InRelease [55.4 kB]");
        println!("Get:3 http://security.rustos.org/debian-security bookworm-security InRelease [48.0 kB]");
        println!("Get:4 http://packages.rustos.org/kernel-extras stable Packages [142 kB]");
        println!("Fetched 245 kB in 18ms (13.6 MB/s)");
        println!("Reading package lists... Done");
        println!("Building dependency tree... Done");
        println!("All 7 packages are up to date.");
    }

    pub fn list(&self, installed_only: bool) {
        println!("Listing packages... Done");
        for p in self.packages.iter() {
            if !installed_only || p.installed {
                let status_badge = if p.installed { "[installed]" } else { "[available]" };
                println!("{:<16} {:<16} {:<8} {:<10} {} KB", 
                    p.name, p.version, p.section, status_badge, p.size_kb);
                println!("    {}", p.description);
            }
        }
    }

    pub fn install(&mut self, name: &str) {
        let idx = self.packages.iter().position(|p| p.name == name);
        if let Some(i) = idx {
            if self.packages[i].installed {
                println!("{} is already the newest version ({}).", self.packages[i].name, self.packages[i].version);
                return;
            }

            // Check dependencies
            let deps = self.packages[i].dependencies.to_vec();
            for dep in deps {
                let dep_installed = self.packages.iter().find(|p| p.name == dep).map(|p| p.installed).unwrap_or(false);
                if !dep_installed {
                    println!("Resolving dependency '{}' for '{}'...", dep, self.packages[i].name);
                    if let Some(dep_pkg) = self.packages.iter_mut().find(|p| p.name == dep) {
                        dep_pkg.installed = true;
                        println!("  -> Automatically installed dependency: {} ({})", dep_pkg.name, dep_pkg.version);
                    }
                }
            }

            println!("Reading package lists... Done");
            println!("Building dependency tree... Done");
            println!("The following NEW package will be installed:");
            println!("  {}", self.packages[i].name);
            println!("0 upgraded, 1 newly installed, 0 to remove.");
            println!("Need to get {} KB of archives.", self.packages[i].size_kb);
            println!("Unpacking {} ({}) ...", self.packages[i].name, self.packages[i].version);
            println!("Setting up {} ...", self.packages[i].name);
            println!("Activating command hooks in RustOS shell... OK");
            self.packages[i].installed = true;
            println!("Installation of '{}' completed successfully.", self.packages[i].name);
        } else {
            println!("E: Unable to locate package {}", name);
        }
    }

    pub fn remove(&mut self, name: &str) {
        let idx = self.packages.iter().position(|p| p.name == name);
        if let Some(i) = idx {
            if !self.packages[i].installed {
                println!("Package '{}' is not installed, so not removed.", self.packages[i].name);
                return;
            }

            if self.packages[i].name == "rust-core" {
                println!("E: Refusing to remove 'rust-core': critical core operating system component.");
                return;
            }

            println!("Reading package lists... Done");
            println!("The following package will be REMOVED:");
            println!("  {}", self.packages[i].name);
            println!("0 upgraded, 0 newly installed, 1 to remove.");
            println!("After this operation, {} KB disk space will be freed.", self.packages[i].size_kb);
            println!("Removing {} ({}) ...", self.packages[i].name, self.packages[i].version);
            self.packages[i].installed = false;
            println!("Removal of '{}' completed successfully.", self.packages[i].name);
        } else {
            println!("E: Unable to locate package {}", name);
        }
    }

    pub fn show(&self, name: &str) {
        if let Some(p) = self.packages.iter().find(|p| p.name == name) {
            println!("Package: {}", p.name);
            println!("Version: {}", p.version);
            println!("Section: {}", p.section);
            println!("Installed-Size: {} KB", p.size_kb);
            println!("Status: {}", if p.installed { "install ok installed" } else { "available" });
            print!("Depends: ");
            if p.dependencies.is_empty() {
                println!("none");
            } else {
                for (i, dep) in p.dependencies.iter().enumerate() {
                    if i > 0 { print!(", "); }
                    print!("{}", dep);
                }
                println!();
            }
            println!("Description: {}", p.description);
        } else {
            println!("E: Package '{}' not found", name);
        }
    }

    pub fn search(&self, term: &str) {
        println!("Sorting... Done");
        println!("Full Text Search... Done");
        let mut count = 0;
        for p in self.packages.iter() {
            if p.name.contains(term) || p.description.contains(term) || p.section.contains(term) {
                let status = if p.installed { "[installed]" } else { "" };
                println!("{} - {} {}", p.name, p.description, status);
                count += 1;
            }
        }
        if count == 0 {
            println!("No packages matched the search term '{}'", term);
        }
    }
}

pub static APT_DB: Mutex<PackageDatabase> = Mutex::new(PackageDatabase { packages: Vec::new() });

pub fn init() {
    *APT_DB.lock() = PackageDatabase::new();
}

pub fn is_installed(pkg_name: &str) -> bool {
    APT_DB.lock().packages.iter().find(|p| p.name == pkg_name).map(|p| p.installed).unwrap_or(false)
}

pub fn handle_apt_command(args: &[&str]) {
    if args.is_empty() {
        println!("RustOS Advanced Package Tool (APT) v1.2 [Debian Compatibility Interface]");
        println!("Usage: apt <command> [options]");
        println!();
        println!("Commands:");
        println!("  update            - Synchronize package index with Debian repository mirrors");
        println!("  list [--installed]- List all or installed kernel packages");
        println!("  install <package> - Install package and resolve dependencies");
        println!("  remove <package>  - Remove an installed package");
        println!("  show <package>    - Display detailed package metadata & dependencies");
        println!("  search <query>    - Search for package by keyword or section");
        println!();
        println!("Educational Note: Simulates Debian package management for embedded OS.");
        println!("Real Linux deb packages are orchestrated via our WSL/Linux supporting bridge.");
        return;
    }

    match args[0] {
        "update" => APT_DB.lock().update(),
        "list" => {
            let installed_only = args.len() > 1 && args[1] == "--installed";
            APT_DB.lock().list(installed_only);
        }
        "install" => {
            if args.len() < 2 {
                println!("E: Please specify a package to install (e.g. 'apt install snake-arcade')");
            } else {
                APT_DB.lock().install(args[1]);
            }
        }
        "remove" => {
            if args.len() < 2 {
                println!("E: Please specify a package to remove (e.g. 'apt remove sys-bench')");
            } else {
                APT_DB.lock().remove(args[1]);
            }
        }
        "show" => {
            if args.len() < 2 {
                println!("E: Please specify a package to inspect");
            } else {
                APT_DB.lock().show(args[1]);
            }
        }
        "search" => {
            if args.len() < 2 {
                println!("E: Please specify a search term");
            } else {
                APT_DB.lock().search(args[1]);
            }
        }
        _ => {
            println!("E: Invalid apt command '{}'. Type 'apt' for usage.", args[0]);
        }
    }
}
