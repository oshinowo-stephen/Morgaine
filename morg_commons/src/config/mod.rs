use dirs::ProjectDirs;
use std::path::PathBuf;
use conf::{self, Config, File};

pub mod settings;

pub fn load() -> Result<settings::MainConfig, conf::ConfigError> {
	let mut s = Config::default();

	s.merge(File::with_name("config/default.yml"))?;

	if let Some(mut morg_conf_dir) = get_path() {
		morg_conf_dir.push(".morgaine.yml");
		let morg_conf = morg_conf_dir.to_str().unwrap();

		s.merge(File::with_name(morg_conf).required(false))?;

		Ok(s.try_into().unwrap())
	} else {
		Err(conf::ConfigError::NotFound(
			"unable to locate config path.".to_owned()
		))
	}
}

fn get_path() -> Option<PathBuf> {
    if let Some(morg) = ProjectDirs::from("com", "Settings", "Morgaine") {
        Some(morg.config_dir().to_owned())
    } else {
        None
    }
}