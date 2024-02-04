use dnf::{
    auto_remove, check_update, install_packages, list_installed, remove_packages, search_package,
    update_system,
};
use std::env;

mod dnf;

fn get_help() {
    println!(
        "
    Pkg a Fedora package management tool

    Usage: pkg <option>

    Options:

    install - Install a package
    Ex: pkg install git

    remove - Remove a package
    Ex: pkg remove git

    autoremove - Auto remove unused packages
    Ex: pkg autoremove

    check - Checks for available updates
    Ex: pkg check

    update - Updates the system
    Ex: pkg update

    list - List installed packages
    Ex: pkg list

    search - Search for a package
    Ex: pkg search git

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
            "remove" => remove_packages(&args[2..]),
            "autoremove" => auto_remove(),
            "check" => check_update(),
            "update" => update_system(),
            "list" => list_installed(),
            "search" => search_package(args.get(2)),
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
