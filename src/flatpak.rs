use std::process::Command;

pub fn flat_install(remote: &String, flatpak: &String) {
    if remote.is_empty() {
        println!("Remote not provided.");
    }

    if flatpak.is_empty() {
        println!("Flatpak not provided.");
    }

    let status = Command::new("sudo")
        .arg("flatpak")
        .arg("install")
        .arg(remote)
        .arg(flatpak)
        .status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("\nFlatpak installed successfully");
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

pub fn flat_remove(flatpak: &String) {
    if flatpak.is_empty() {
        println!("Flatpak not provided.");
    }

    let status = Command::new("sudo")
        .arg("flatpak")
        .arg("uninstall")
        .arg(flatpak)
        .status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("\nFlatpak removed successfully");
        }
        Ok(exit_status) if exit_status.code() == Some(1) => {
            println!("\nOperation canceled by the user.");
        }
        Ok(_) => {
            println!("\nError removing Flatpak");
        }
        Err(e) => {
            println!("\nFailed to execute command: {}", e);
        }
    }
}

pub fn flat_update() {
    let status = Command::new("sudo").arg("flatpak").arg("update").status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("\nFlatpaks updated successfully");
        }
        Ok(exit_status) if exit_status.code() == Some(1) => {
            println!("\nOperation canceled by the user.");
        }
        Ok(_) => {
            println!("\nError updating Flatpaks");
        }
        Err(e) => {
            println!("\nFailed to execute command: {}", e);
        }
    }
}
