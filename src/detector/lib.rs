use std::process::Command;
use anyhow::{Result, bail};
use super::error::{DetectorError::{ArchDetectionError, OsDetectionError, OsVersionDetectionError, OsNameDetectionError, PackageManagerDetectionError}};

pub struct OsInfo {
  pub name: String,
  pub version: String,
  pub arch: String,
  pub distro: String,
  pub desktop: bool
}

impl OsInfo {
    fn get_os_name() -> Result<String> {
        if cfg!(target_os = "linux") {
            if let Ok(output) = Command::new("lsb_release").arg("-si").output() {
                if let Ok(name) = String::from_utf8(output.stdout) {
                    return Ok(name.trim().to_string());
                }
            }
        }
        bail!(OsNameDetectionError)
    }

    fn get_os_version() -> Result<String> {
        if cfg!(target_os = "linux") {
            if let Ok(output) = Command::new("lsb_release").arg("-sr").output() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    return Ok(version.trim().to_string());
                }
            }
        }
        bail!(OsVersionDetectionError);
    }
    fn get_os_arch() -> Result<String> {
        if cfg!(target_os = "linux") {
            if let Ok(output) = Command::new("uname").arg("-m").output() {
                if let Ok(arch) = String::from_utf8(output.stdout) {
                    return Ok(arch.trim().to_string());
                }
            }
        }
        bail!(ArchDetectionError)
    }
  
    fn get_os_distro() -> Result<String> {
        if cfg!(target_os = "linux") {
            if let Ok(output) = Command::new("lsb_release").arg("-is").output() {
                if let Ok(distro) = String::from_utf8(output.stdout) {
                    return Ok(distro.trim().to_string());
                }
            }
        }
        bail!(OsDetectionError)
    }

    pub fn detect() -> Self {
        let name = Self::get_os_name().unwrap();
        let version = Self::get_os_version().unwrap();
        let arch = Self::get_os_arch().unwrap();
        let distro = Self::get_os_distro().unwrap();

        OsInfo {
            name,
            version,
            arch,
            distro,
            desktop: false,
        }
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_info_detection() {
        let os_info = OsInfo::detect();
        assert!(!os_info.name.is_empty());
        assert!(!os_info.version.is_empty());
        assert!(!os_info.arch.is_empty());
        assert!(!os_info.distro.is_empty());
    }

    #[test]
    fn test_os_name_detection() {
        let name = OsInfo::get_os_name();
        assert!(name.is_ok());
        assert!(!name.unwrap().is_empty());
    }
    #[test]
    fn test_os_name_detect_filed() {
        let name = "";
    }
}