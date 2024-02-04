use std::process::Command;

pub fn install_packages(package_names: &[String]) {
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

pub fn remove_packages(package_names: &[String]) {
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

pub fn auto_remove() {
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

pub fn check_update() {
    let status = Command::new("dnf").arg("check-update").status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("\nChecked for updates successfully");
        }
        Ok(exit_status) if exit_status.code() == Some(1) => {
            println!("\nOperation canceled by the user.");
        }
        Ok(_) => {
            println!("\nError checking for updates.");
        }
        Err(e) => {
            println!("\nFailed to execute command: {}", e);
        }
    }
}

pub fn update_system() {
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

pub fn list_installed() {
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

pub fn search_package(package_name: Option<&String>) {
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