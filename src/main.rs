use std::process::Command;
use std::path::Path;

fn main() {
    match check_wazuh_agent_installed() {
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

fn check_wazuh_agent_installed() -> Result<bool, String> {
    // Simplified check, for illustration
    Ok(false)
}

fn install_wazuh_agent() -> Result<(), String> {
    let package_manager = if cfg!(target_os = "linux") {
        if Path::new("/etc/debian_version").exists() {
            "apt-get"
        } else if Path::new("/etc/redhat-release").exists() {
            "yum"
        } else {
            return Err("Unsupported operating system.".to_string());
        }
    } else {
        return Err("Unsupported operating system.".to_string());
    };

    // Check if repository configurations exist
    let repo_exists = if cfg!(target_os = "linux") {
        if Path::new("/etc/debian_version").exists() {
            Path::new("/etc/apt/sources.list.d/wazuh.list").exists()
        } else if Path::new("/etc/redhat-release").exists() {
            Path::new("/etc/yum.repos.d/wazuh.repo").exists()
        } else {
            false
        }
    } else {
        false
    };

    // If repository configurations exist, directly install the Wazuh agent
    if repo_exists {
        let install_status = Command::new("sh")
            .arg("-c")
            .arg(format!("{} install -y wazuh-agent", package_manager))
            .status()
            .map_err(|_| "Failed to install Wazuh agent package.".to_string())?;
        if !install_status.success() {
            return Err("Failed to install Wazuh agent package.".to_string());
        }
    } else {
        // If repository configurations don't exist, follow the steps to add them
        let install_cmd = if cfg!(target_os = "linux") {
            if Path::new("/etc/debian_version").exists() {
                r#"curl -s https://packages.wazuh.com/key/GPG-KEY-WAZUH | gpg --no-default-keyring --keyring gnupg-ring:/usr/share/keyrings/wazuh.gpg --import && chmod 644 /usr/share/keyrings/wazuh.gpg &&
                echo "deb [signed-by=/usr/share/keyrings/wazuh.gpg] https://packages.wazuh.com/4.x/apt/ stable main" | tee -a /etc/apt/sources.list.d/wazuh.list &&
                apt-get update && apt-get install -y wazuh-agent"#
            } else if Path::new("/etc/redhat-release").exists() {
                r#"rpm --import https://packages.wazuh.com/key/GPG-KEY-WAZUH &&
                cat > /etc/yum.repos.d/wazuh.repo << EOF
[wazuh]
gpgcheck=1
gpgkey=https://packages.wazuh.com/key/GPG-KEY-WAZUH
enabled=1
name=EL-\$releasever - Wazuh
baseurl=https://packages.wazuh.com/4.x/yum/
protect=1
EOF
                yum makecache"#
            } else {
                return Err("Unsupported operating system.".to_string());
            }
        } else {
            return Err("Unsupported operating system.".to_string());
        };

        // Install Wazuh agent
        let install_status = Command::new("sh")
            .arg("-c")
            .arg(install_cmd)
            .status()
            .map_err(|_| "Failed to install Wazuh agent package.".to_string())?;
        if !install_status.success() {
            return Err("Failed to install Wazuh agent package.".to_string());
        }
    }

    Ok(())
}
