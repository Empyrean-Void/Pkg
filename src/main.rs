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

fn get_help() {
    println!(
        "
    Pkg a Fedora package management tool

    Usage: pkg <option>

    Options:

    install - Install packages
    Ex: pkg install git htop

    finstall - Install a Flatpak

    remove - Remove packages
    Ex: pkg remove git htop

    fremove - Remove a Flatpak

    auto-remove - Auto remove unused packages
    Ex: pkg auto-remove

    check - Checks for available updates
    Ex: pkg check

    update - Updates the system
    Ex: pkg update

    fupdate - Updates Flatpaks

    downgrade - Downgrade packages
    Ex: pkg downgrade git htop

    list - List installed packages
    Ex: pkg list

    search - Search for a package
    Ex: pkg search git

    copr-enable - Enable a COPR repositoy

    copr-disable - Disable a COPR repositoy

    help - Display this message
    Ex: pkg help
    "
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(command) => match command.as_str() {
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
