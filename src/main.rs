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
    // Enhanced check for RPM-based systems
    let output = Command::new("rpm")
        .arg("-q")
        .arg("wazuh-agent")
        .output()
        .map_err(|e| format!("Failed to query package: {}", e))?;

    if output.status.success() {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn install_wazuh_agent() -> Result<(), String> {
    let package_manager = determine_package_manager()?;
    let repo_config_path = determine_repo_config_path(&package_manager)?;

    let repo_exists = Path::new(&repo_config_path).exists();

    if repo_exists {
        let install_status = Command::new("sh")
            .arg("-c")
            .arg(format!("{} install -y wazuh-agent", package_manager))
            .status()
            .map_err(|_| "Failed to execute package manager.".to_string())?;
        if !install_status.success() {
            return Err("Failed to install Wazuh agent package.".to_string());
        }
    } else {
        let install_cmd = setup_repository_and_install(&package_manager)?;
        let install_status = Command::new("sh")
            .arg("-c")
            .arg(install_cmd)
            .status()
            .map_err(|_| "Failed to setup repository or install Wazuh agent.".to_string())?;
        if !install_status.success() {
            return Err("Failed to setup repository or install Wazuh agent.".to_string());
        }
    }

    Ok(())
}

fn determine_package_manager() -> Result<String, String> {
    if cfg!(target_os = "linux") {
        if Path::new("/etc/debian_version").exists() {
            Ok("apt-get".to_string())
        } else if Path::new("/etc/redhat-release").exists() {
            Ok("yum".to_string())  // Use yum for RHEL/CentOS and older Fedora.
        } else if Path::new("/etc/fedora-release").exists() {
            Ok("dnf".to_string()) // Use dnf for Fedora.
        } else {
            Err("Unsupported operating system.".to_string())
        }
    } else {
        Err("Unsupported operating system.".to_string())
    }
}

fn determine_repo_config_path(package_manager: &str) -> Result<String, String> {
    match package_manager {
        "apt-get" => Ok("/etc/apt/sources.list.d/wazuh.list".to_string()),
        "yum" => Ok("/etc/yum.repos.d/wazuh.repo".to_string()),
        _ => Err("Unsupported package manager.".to_string()),
    }
}

fn setup_repository_and_install(package_manager: &str) -> Result<String, String> {
    match package_manager {
        "apt-get" => Ok(r#"curl -s https://packages.wazuh.com/key/GPG-KEY-WAZUH | gpg --no-default-keyring --keyring gnupg-ring:/usr/share/keyrings/wazuh.gpg --import &&
            chmod 644 /usr/share/keyrings/wazuh.gpg &&
            echo "deb [signed-by=/usr/share/keyrings/wazuh.gpg] https://packages.wazuh.com/4.x/apt/ stable main" | tee -a /etc/apt/sources.list.d/wazuh.list &&
            apt-get update && apt-get install -y wazuh-agent"#.to_string()),
        "yum" => Ok(r#"rpm --import https://packages.wazuh.com/key/GPG-KEY-WAZUH &&
            cat > /etc/yum.repos.d/wazuh.repo << EOF
[wazuh]
gpgcheck=1
gpgkey=https://packages.wazuh.com/key/GPG-KEY-WAZUH
enabled=1
name=EL-\$releasever - Wazuh
baseurl=https://packages.wazuh.com/4.x/yum/
protect=1
EOF
            yum makecache && yum install -y wazuh-agent"#.to_string()),
        _ => Err("Unsupported package manager.".to_string()),
    }
}
