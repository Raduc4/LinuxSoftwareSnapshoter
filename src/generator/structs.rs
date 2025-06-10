use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Backup {
    system: SystemInfo,
    packages: PackageInfo,
    services: ServiceInfo,
    config_files: Vec<ConfigFile>,
    network: NetworkInfo,
    notes: String,
}

#[derive(Serialize, Deserialize)]
pub struct SystemInfo {
    pub distro: String,
    pub kernel: String,
    pub architecture: String,
    pub hostname: String,
}

#[derive(Serialize, Deserialize)]
struct PackageInfo {
    manager: String,
    list: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct ServiceInfo {
    enabled: Vec<String>,
    disabled: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct ConfigFile {
    source: String,
    destination: String,
}

#[derive(Serialize, Deserialize)]
struct NetworkInfo {
    hostname: String,
    interfaces: Vec<NetworkInterface>,
}

#[derive(Serialize, Deserialize)]
struct NetworkInterface {
    name: String,
    dhcp: bool,
}

#[derive(Serialize, Deserialize)]
struct UserInfo {
    name: String,
    shell: String,
    groups: Vec<String>,
}
