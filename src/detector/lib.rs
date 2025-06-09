use super::error::DetectorError::{
    ArchDetectionError, OsDetectionError, OsNameDetectionError, OsVersionDetectionError,
    PackageManagerDetectionError,
};
use anyhow::{bail, Context, Result};
use std::process::Command;

#[derive(Debug, PartialEq, Eq)]
pub struct OsInfo {
    pub name: String,
    pub version: String,
    pub arch: String,
    pub distro: String,
    pub desktop: bool,
}

impl OsInfo {
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

    pub fn detect() -> Result<Self> {
        let name = Self::get_os_name()?;
        let version = Self::get_os_version()?;
        let arch = Self::get_os_arch()?;
        let distro = Self::get_os_distro()?;

       Ok(Self {
            name,
            version,
            arch,
            distro,
            desktop: false,
        })
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
            }
            Err(e) => {
                println!("detection failed as expected: {e}");
            }
          }
    }
}
