// Test
mod test;
// Globals
use crate::globals::CONFIG;
// Error Handling
use miette::Result;
// Struct
use crate::pipeline::Filters;
use crate::types::Config;
use utils::git::Flag;

impl Config {
    pub fn get() -> Result<Self> {
        let config = CONFIG.lock().unwrap().clone();
        Ok(config)
    }
    /**
    Check if any of the pipelines have a trigger with "watch" flag.
    */
    pub fn has_watchable(&self) -> Result<bool> {
        if let Some(pipelines) = self.pipelines.clone() {
            let mut pipelines = Filters::to_hashmap(pipelines);
            pipelines.retain(|_, pipeline| pipeline.is_watchable().unwrap());
            return Ok(!pipelines.is_empty());
        }
        Ok(false)
    }
    /**
    Check if any of the pipelines have a trigger with a git hook flag.
    */
    pub fn has_git_flag(&self) -> Result<bool> {
        if let Some(pipelines) = self.pipelines.clone() {
            let mut pipelines = Filters::to_hashmap(pipelines);
            pipelines.retain(|_, pipeline| {
                if let Some(triggers) = pipeline.triggers.clone() {
                    triggers.iter().any(|e| {
                        if let Some(action) = e.get_action().unwrap().clone() {
                            match action {
                                Flag::Hook(_) => true,
                                _ => false,
                            }
                        } else {
                            false
                        }
                    })
                } else {
                    false
                }
            });
            return Ok(!pipelines.is_empty());
        }
        Ok(false)
    }
    /**
     Report if config has a global options.attach property
    */
    pub fn has_attach_option(&self) -> Result<bool> {
        if let Some(options) = &self.options {
            Ok(options.attach.is_some())
        } else {
            Ok(false)
        }
    }
    /**
     Report if pipeline has options
    */
    pub fn should_detach(&self) -> Result<bool> {
        if let Some(options) = &self.options {
            if let Some(attach) = options.attach {
                Ok(!attach)
            } else {
                Ok(true)
            }
        } else {
            Ok(true)
        }
    }
    /**
     Report if pipeline has options
    */
    pub fn has_loglevel_option(&self) -> Result<bool> {
        if let Some(options) = &self.options {
            Ok(options.log_level.is_some())
        } else {
            Ok(false)
        }
    }
}
