use std::process::Command;

pub fn copr_enable(repo: &String) {
    if repo.is_empty() {
        println!("No repository provided.")
    }

    let status = Command::new("sudo")
        .arg("dnf")
        .arg("copr")
        .arg("enable")
        .arg(repo)
        .status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("\nRepository enabled successfully");
        }
        Ok(exit_status) if exit_status.code() == Some(1) => {
            println!("\nOperation canceled by the user");
        }
        Ok(_) => {
            println!("\nError enabling repository: {}", repo);
        }
        Err(e) => {
            println!("\nFailed to execute command: {}", e);
        }
    }
}

pub fn copr_disable(repo: &String) {
    if repo.is_empty() {
        println!("No repository provided.")
    }

    let status = Command::new("sudo")
        .arg("dnf")
        .arg("copr")
        .arg("disable")
        .arg(repo)
        .status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("\nRepository disabled successfully");
        }
        Ok(exit_status) if exit_status.code() == Some(1) => {
            println!("\nOperation canceled by the user");
        }
        Ok(_) => {
            println!("\nError disabling repository: {}", repo);
        }
        Err(e) => {
            println!("\nFailed to execute command: {}", e);
        }
    }
}
