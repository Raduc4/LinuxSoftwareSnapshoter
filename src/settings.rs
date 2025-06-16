  use config::{Config, Environment, File};
use serde::Deserialize;

  #[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
  pub backup_file_name: String

}
impl Settings {

   pub fn new() -> Result<Self, String> {
    let mut cfg_builder = Config::builder().add_source(File::with_name("config.json5"));

    if let Some(runtime_env) = Settings::get_runtime_env() {
      cfg_builder = cfg_builder
        .add_source(File::with_name(&format!("config.{runtime_env}.json5")).required(false));
    }

    cfg_builder = cfg_builder
      .add_source(File::with_name("config.local.json5").required(false));
      // TODO: this isn't exactly right, we might need to make our 'Environment' source in the future.

    let cfg = cfg_builder
      .build()
      .and_then(Config::try_deserialize::<Settings>);

    cfg.map_err(|err| err.to_string())
  }
  fn get_runtime_env() -> Option<String> {
    let env_val = std::env::var("RUNTIME_ENV");
    match env_val {
      Err(..) => None,
      Ok(env_name) => match env_name.as_str() {
        "production" => Some("production".into()),
        "staging" => Some("staging".into()),
        "development" => Some("development".into()),
        val => {
          log::error!("Unknown RUNTIME_ENV value of {}", val);
          None
        }
      },
    }
  }
}