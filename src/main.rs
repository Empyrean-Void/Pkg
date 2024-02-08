use copr::{copr_disable, copr_enable};
use dnf::{
    auto_remove, check_update, downgrade_packages, install_packages, list_installed,
    remove_packages, search_package, update_system,
};
use flatpak::{flat_install, flat_remove, flat_update};
use std::env;

mod copr;
mod dnf;
mod flatpak;

fn pkg_update() {
    println!("I'm not ready yet!");
}

fn get_help() {
    println!("\nUsage: pkg [arg [...]]");
    println!("\nArguments:\n");
    println!("  help           - Show this message\n");
    println!("  install        - Install a package or packages");
    println!("  remove         - Remove a package or packages");
    println!("  auto-remove    - Remove unused packages");
    println!("  check          - Check for updates");
    println!("  update         - Update the system");
    println!("  downgrade      - Downgrade a package or packages");
    println!("  list           - List installed packages");
    println!("  search         - Search for a package\n");
    println!("  finstall       - Install a Flatpak");
    println!("  fremove        - Remove a Flatpak");
    println!("  fupdate        - Update a Flatpak\n");
    println!("  copr-enable    - Enable a COPR repository");
    println!("  copr-disable   - Disable a COPR repository\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(command) => match command.as_str() {
            "pkg-update" => pkg_update(),
            "install" => install_packages(&args[2..]),
            "finstall" => flat_install(&args[2], &args[3]),
            "remove" => remove_packages(&args[2..]),
            "fremove" => flat_remove(&args[2]),
            "auto-remove" => auto_remove(),
            "check" => check_update(),
            "update" => update_system(),
            "fupdate" => flat_update(),
            "downgrade" => downgrade_packages(&args[2..]),
            "list" => list_installed(),
            "search" => search_package(&args[2]),
            "copr-enable" => copr_enable(&args[2]),
            "copr-disable" => copr_disable(&args[2]),
            "help" => get_help(),
            _ => {
                println!("Unknown command: {}", command);
                get_help();
            }
        },
        None => {
            get_help();
        }
    }
}
