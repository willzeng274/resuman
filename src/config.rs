use serde_derive::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Clone)]
pub struct Data {
    // all under [main] for now, I guess we can make subcategories later
    pub main: Config,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub root_dir: PathBuf,
    pub template_dir: Option<PathBuf>,
    pub db_path: Option<PathBuf>,

    // company must be specified, but group, template, and position are optional
    // if they are not specified, the default values should be used
    // the default for group should be "default"
    // the default for template should be "default"
    // the default for position should be "swe"
    pub default_group: Option<String>,
    pub default_template: Option<String>,
    pub default_position: Option<String>,
    
    pub default_length: Option<u16>, // default is 16 weeks or 4 months
    pub default_status: Option<String>, // default is "applying"

    pub folder_pattern: Option<String>, // default is "{company}_{position}_{date}"
    pub file_pattern: Option<String>,   // default is "resume_{company}_{position}_{date}"
    pub date_format: Option<String>,    // default is "%Y_%m_%d"
}
