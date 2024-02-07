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
        // Ok(exit_status) if exit_status.success() => {
        // println!("\nChecked for updates successfully");
        // }
        Ok(exit_status) if exit_status.code() == Some(1) => {
            println!("\nOperation canceled by the user.");
        }
        Ok(_) => {
            println!("\nChecked for updates successfully");
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

pub fn downgrade_packages(package_names: &[String]) {
    let status = Command::new("sudo").arg("dnf").arg("downgrade").status();

    if package_names.is_empty() {
        println!("No packages provided for downgrade.")
    }

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("\nPackages downgraded successfully");
        }
        Ok(exit_status) if exit_status.code() == Some(1) => {
            println!("\nOperation canceled by the user.");
        }
        Ok(_) => {
            println!("\nError downgrading packages");
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

pub fn search_package(package_name: &String) {
    let status = Command::new("dnf").arg("search").arg(package_name).status();

    if package_name.is_empty() {
        println!("No package provided for search");
    }

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("\nPackages searched successfully");
        }
        Ok(exit_status) if exit_status.code() == Some(1) => {
            println!("\nOperation canceled by the user.");
        }
        Ok(_) => {
            println!("\nError searching packages");
        }
        Err(e) => {
            println!("\nFailed to execute command: {}", e);
        }
    }
}
