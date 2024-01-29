use std::env;
use std::process::Command;

fn install_package(package_name: Option<&String>) {
    match package_name {
        Some(name) => {
            let status = Command::new("sudo")
                .arg("dnf")
                .arg("install")
                .arg(name)
                .status()
                .expect("Failed to execute command");

            if status.success() {
                println!("\nPackage {} installed successfully", name);
            } else {
                println!("\nError installing package {}", name);
            }
        }
        None => println!("Package name not provided for install"),
    }
}

fn update_system() {
    let status = Command::new("sudo")
        .arg("dnf")
        .arg("update")
        .status()
        .expect("Failed to execute command");

    if status.success() {
        println!("\nSystem updated successfully");
    } else {
        println!("\nError updating the system");
    }
}

fn remove_package(package_name: Option<&String>) {
    match package_name {
        Some(name) => {
            let status = Command::new("sudo")
                .arg("dnf")
                .arg("remove")
                .arg(name)
                .status()
                .expect("Failed to execute command");

            if status.success() {
                println!("\nPackage {} installed successfully", name);
            } else {
                println!("\nError installing package {}", name);
            }
        }
        None => println!("Package name not provided for install"),
    }
}

fn list_installed() {
    let status = Command::new("dnf")
        .arg("list")
        .arg("installed")
        .status()
        .expect("Failed to execute command");

    if status.success() {
        println!("\nPackages listed successfully");
    } else {
        println!("\nError listing packages");
    }
}

fn search_package(package_name: Option<&String>) {
    match package_name {
        Some(name) => {
            let status = Command::new("dnf")
                .arg("search")
                .arg(name)
                .status()
                .expect("Failed to execute command");

            if status.success() {
                println!("\nPackage {} searched successfully", name);
            } else {
                println!("\nError searching package {}", name);
            }
        }
        None => println!("Package name not provided for install"),
    }
}

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
            "install" => install_package(args.get(2)),
            "update" => update_system(),
            "remove" => remove_package(args.get(2)),
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
