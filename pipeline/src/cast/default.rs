use super::Config;

impl Default for Config {
    fn default() -> Self {
        Config {
            file: "pipelight.config.mjs".to_owned(),
            pipelines: None,
        }
    }
}
impl Config {
    pub fn new() -> Self {
        Config::default()
    }
}
