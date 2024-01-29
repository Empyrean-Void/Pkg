use std::env;
use std::process::Command;

fn install_packages(package_names: &[String]) {
    if package_names.is_empty() {
        println!("No package names provided for install");
        return;
    }

    let status = Command::new("sudo")
        .arg("dnf")
        .arg("install")
        .args(package_names)
        .status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("\nPackages installed successfully");
        }
        Ok(exit_status) if exit_status.code() == Some(1) => {
            println!("\nOperation canceled by the user.");
        }
        Ok(_) => {
            println!("\nError installing packages");
        }
        Err(e) => {
            println!("\nFailed to execute command: {}", e);
        }
    }
}

fn remove_packages(package_names: &[String]) {
    if package_names.is_empty() {
        println!("No package names provided for removal");
        return;
    }

    let status = Command::new("sudo")
        .arg("dnf")
        .arg("remove")
        .args(package_names)
        .status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("\nPackages removed successfully");
        }
        Ok(exit_status) if exit_status.code() == Some(1) => {
            println!("\nOperation canceled by the user.");
        }
        Ok(_) => {
            println!("\nError removing packages");
        }
        Err(e) => {
            println!("\nFailed to execute command: {}", e);
        }
    }
}

fn auto_remove() {
    let status = Command::new("sudo").arg("dnf").arg("autoremove").status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("\nPackages auto removed successfully");
        }
        Ok(exit_status) if exit_status.code() == Some(1) => {
            println!("\nOperation canceled by the user.");
        }
        Ok(_) => {
            println!("\nError auto removing packages");
        }
        Err(e) => {
            println!("\nFailed to execute command: {}", e);
        }
    }
}

fn update_system() {
    let status = Command::new("sudo").arg("dnf").arg("update").status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("\nSystem updated successfully");
        }
        Ok(exit_status) if exit_status.code() == Some(1) => {
            println!("\nOperation canceled by the user.");
        }
        Ok(_) => {
            println!("\nError updating the system");
        }
        Err(e) => {
            println!("\nFailed to execute command: {}", e);
        }
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
                println!("\nPackage searched successfully");
            } else {
                println!("\nError searching package");
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

    autoremove - Auto remove unused packages
    Ex: pkg autoremove

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
