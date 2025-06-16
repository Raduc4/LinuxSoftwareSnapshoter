use super::error::DetectorError::{
    OsDetectionError, OsNameDetectionError, OsVersionDetectionError, DataIntegrityError
};
use crate::constants::{ARCH, DISTRIBUTIONS};
use anyhow::{bail, Context, Result};
use std::process::Command;
#[derive(Debug, PartialEq, Eq)]
pub struct OsInfo {
    pub name: String,
    pub version: String,
    pub arch: String,
    pub distro: String,
    pub hostname: String,
    pub desktop: bool,
}

impl OsInfo {
    fn new(name: String, version: String, arch: String, distro: String, hostname: String, desktop: bool) -> Self {
        Self {
            name,
            version,
            arch,
            distro,
            hostname,
            desktop,
        }
    }
    fn run(cmd: &str, args: &[&str]) -> Result<String> {
        let output = Command::new(cmd).args(args).output()?;

        if !output.status.success() {
            bail!("`{cmd}` exited with {status}", status = output.status);
        }

        let stdout = String::from_utf8(output.stdout)
            .context("stdout contained invalid UTF-8")?
            .trim()
            .to_string();

        if stdout.is_empty() {
            bail!("`{cmd}` produced empty output");
        }
        Ok(stdout.to_owned())
    }
    fn get_os_name() -> Result<String> {
        if cfg!(target_os = "linux") {
            return Self::run("lsb_release", &["-si"]).context("detecting OS name");
        }
        bail!(OsNameDetectionError)
    }

    fn get_os_version() -> Result<String> {
        if cfg!(target_os = "linux") {
            return Self::run("lsb_release", &["-sr"]).context("detecting OS version");
        }
        bail!(OsVersionDetectionError);
    }
    fn get_os_arch() -> Result<String> {
        Self::run("uname", &["-m"]).context("detecting architecture")
    }

    fn get_os_distro() -> Result<String> {
        if cfg!(target_os = "linux") {
            return Self::run("lsb_release", &["-is"]).context("detecting distro");
        }
        bail!(OsDetectionError)
    }

    fn get_os_hostname() -> Result<String> {
      if cfg!(target_os = "linux") {
            return Self::run("hostname", &[]).context("detecting distro");
        }
        bail!(OsDetectionError)
    }

    fn check(self) -> Result<Self> {
      let checker  = !ARCH.contains(&self.arch.as_str()) || DISTRIBUTIONS.contains(&self.distro.as_str()); 
       if !checker{
        Err(DataIntegrityError.into())
       } else {
       Ok(self) 
       }
    }

    pub fn detect() -> Result<Self> {
        let name = Self::get_os_name()?;
        let version = Self::get_os_version()?;
        let arch = Self::get_os_arch()?;
        let distro = Self::get_os_distro()?;
        let hostname = Self::get_os_hostname()?;

        let output = Self::new(name, version, arch, distro, hostname,true);

        output.check()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_info_detection() {
        match OsInfo::detect() {
            Ok(info) => {
                assert!(!info.name.is_empty());
                assert!(!info.version.is_empty());
                assert!(!info.arch.is_empty());
                assert!(!info.distro.is_empty());
                assert!(!info.hostname.is_empty());
            }
            Err(e) => {
                println!("detection failed as expected: {e}");
            }
        }
    }

    #[test]
    fn test_os_info_check() {
      let os_info = OsInfo::detect();
      assert!(os_info.unwrap().check().is_ok()) 
    }
}
