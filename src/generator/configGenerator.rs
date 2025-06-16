use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

pub fn write_json<'a, T: Serialize + Deserialize<'a>>(backup: T) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(&backup).unwrap();
    let mut file = File::create("backup.json")?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::generator::{configGenerator::write_json, structs::SystemInfo};

    #[test]
    fn write_json_minimal_config() {
        let x = write_json(SystemInfo {
            distro: "Ubuntu".into(),
            kernel: "29".into(),
            architecture: "x86_64".into(),
            hostname: "alu".into(),
        });
        assert!(x.is_ok())
    }
}
