use std::process::Command;
use std::fs;

fn main() {
    match check_wazuh_installed() {
        Ok(installed) => {
            if installed {
                println!("Wazuh agent is already installed.");
            } else {
                println!("Wazuh agent is not installed. Installing...");
                match install_wazuh_agent() {
                    Ok(_) => println!("Wazuh agent installed successfully."),
                    Err(e) => println!("Failed to install Wazuh agent: {}", e),
                }
            }
        }
        Err(e) => println!("Error checking Wazuh agent installation: {}", e),
    }
}

fn check_wazuh_installed() -> Result<bool, String> {
    let wazuhctl_result = Command::new("which").arg("wazuhctl").output();

    if wazuhctl_result.is_err() {
        return Ok(false);
    }

    let wazuhctl_installed = wazuhctl_result.unwrap().status.success();

    Ok(wazuhctl_installed)
}

fn install_wazuh_agent() -> Result<(), String> {
    let _version = "4.7.3"; // Unused variable warning suppressed

    // Hard-coded package URLs
    let package_urls = [
        "https://packages.wazuh.com/4.x/alpine/v3.12/main/x86/wazuh-agent-4.7.3-r1.apk",
        "https://packages.wazuh.com/4.x/alpine/v3.12/main/x86_64/wazuh-agent-4.7.3-r1.apk",
        "https://packages.wazuh.com/4.x/alpine/v3.12/main/aarch64/wazuh-agent-4.7.3-r1.apk",
        "https://packages.wazuh.com/4.x/alpine/v3.12/main/armv7/wazuh-agent-4.7.3-r1.apk",
        "https://packages.wazuh.com/4.x/alpine/v3.12/main/armhf/wazuh-agent-4.7.3-r1.apk",
        "https://packages.wazuh.com/4.x/alpine/v3.12/main/ppc64le/wazuh-agent-4.7.3-r1.apk",
        "https://packages.wazuh.com/4.x/yum/wazuh-agent-4.7.3-1.ppc64le.rpm",
        "https://packages.wazuh.com/4.x/yum/wazuh-agent-4.7.3-1.i386.rpm",
        "https://packages.wazuh.com/4.x/yum/wazuh-agent-4.7.3-1.x86_64.rpm",
        "https://packages.wazuh.com/4.x/yum/wazuh-agent-4.7.3-1.aarch64.rpm",
        "https://packages.wazuh.com/4.x/yum/wazuh-agent-4.7.3-1.armv7hl.rpm",
        "https://packages.wazuh.com/4.x/apt/pool/main/w/wazuh-agent/wazuh-agent_4.7.3-1_ppc64el.deb",
        "https://packages.wazuh.com/4.x/apt/pool/main/w/wazuh-agent/wazuh-agent_4.7.3-1_i386.deb",
        "https://packages.wazuh.com/4.x/apt/pool/main/w/wazuh-agent/wazuh-agent_4.7.3-1_amd64.deb",
        "https://packages.wazuh.com/4.x/apt/pool/main/w/wazuh-agent/wazuh-agent_4.7.3-1_arm64.deb",
        "https://packages.wazuh.com/4.x/apt/pool/main/w/wazuh-agent/wazuh-agent_4.7.3-1_armhf.deb",
    ];

    let package_path = "/tmp/wazuh-agent.deb"; // Assuming Debian-based system for simplicity

    // Check for curl
    if Command::new("curl").output().is_err() {
        return Err("Curl is not installed.".to_string());
    }

    // Download package
    let download_result = Command::new("curl")
        .args(&["-L", &package_urls[14], "-o", package_path])
        .status();

    if download_result.is_err() || !download_result.unwrap().success() {
        return Err("Failed to download the Wazuh agent package.".to_string());
    }

    // Sudo privileges check
    let sudo_check = Command::new("sudo").arg("-v").output();
    if sudo_check.is_err() || !sudo_check.unwrap().status.success() {
        return Err("Sudo privileges are required for installation.".to_string());
    }

    // Installation
    let package_manager = "apt"; // Assuming Debian-based system for simplicity
    let install_status = Command::new("sudo")
        .args(&[package_manager, "install", "-y", package_path])
        .status();

    if install_status.is_err() || !install_status.unwrap().success() {
        return Err(format!("Failed to install Wazuh agent package using {}.", package_manager));
    }

    // Attempt to clean up the downloaded package regardless of installation success
    let _ = fs::remove_file(package_path);

    Ok(())
}

