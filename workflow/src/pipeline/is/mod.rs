// Test
mod test;
// Unix process manipulation
use rustix::process::test_kill_process;
// Structs
use crate::types::{Logs, Pipeline, Trigger};
use utils::git::{Flag, Special};
// Traits
use exec::Status;
// Error Handling
use miette::{Error, Result};

/**
The following methods returns informations about pipeline states.
They question and sanitize the logs according to the unix kernel answers.

Reasons:
To avoid duplicates,
pipelight use its autogenerated logs as a lock file to keep the state of the executing pipelines.
But this method is much more error prone than a lock file as logs are frequently manipulated.

That is why pipelight chose to distruss the generated log files and concider the unix kernel
a much older piece of software as the uniq source of truth.

Those methods are to be used everytime logs are loaded.
*/

impl Pipeline {
    /**
    Check if a triggered pipeline has an already running instance.
    Should be combined with .is_ok() and .is_err() to generate a boolean.

    It cascade checks the following conditions:
    - if running homologous(same name) in logs exists.
    - if homologous pid exists on the unix process registry.
    - if corresponding program is a "pipelight" instance.

    If those conditions are met we assume the pipeline has an already running instance.
    */
    pub fn has_homologous_already_running(&self) -> Result<()> {
        let mut pipelines = Logs::get_many_by_name(&self.name)?;
        pipelines.reverse();
        for pipeline in pipelines {
            if pipeline.is_running()? {
                return Ok(());
            }
        }
        let message = "pipeline has no homologous already running";
        Err(Error::msg(message))
    }
    /**
    Check if the pipeline instance(loaded from logs) is running.

    It cascade checks the following conditions:
    - if pipeline pid exists on the unix process registry.
    - if corresponding program is a "pipelight" instance.

    If those conditions are met we assume the pipeline is running.
    */
    pub fn is_running(&self) -> Result<bool> {
        if let Some(event) = self.event.clone() {
            let pid = rustix::process::Pid::from_raw(event.pid.unwrap());
            Ok(test_kill_process(pid.unwrap()).is_ok())
        } else {
            Ok(false)
        }
    }
    /**
    Check if the pipeline can be triggered in the actual environment
    */
    pub fn is_triggerable_strict(&self) -> Result<bool> {
        let env = Trigger::get()?;
        // If pipeline has defined triggers
        if let Some(triggers) = self.triggers.clone() {
            Ok(env.has_match_strict(triggers)?)
        } else {
            Ok(false)
        }
    }
    /**
    Check if the pipeline can be triggered in the actual environment
    */
    pub fn is_triggerable(&self) -> Result<bool> {
        let env = Trigger::get()?;
        // If pipeline has defined triggers
        if let Some(triggers) = self.triggers.clone() {
            Ok(env.has_match(triggers)?)
        } else {
            Ok(true)
        }
    }
    /**
    Check if the pipeline has a trigger that contains a "watch" flag
    */
    pub fn is_watchable(&self) -> Result<bool> {
        if let Some(triggers) = self.triggers.clone() {
            let is = triggers
                .iter()
                .any(|e| e.get_action().unwrap() == Some(Flag::Special(Special::Watch)));
            Ok(is)
        } else {
            Ok(false)
        }
    }
    /**
    Tells if the pipeline execution has been aborted.

    Compares if log_pid is in system pid list.
    If not, the program has been aborted
    */
    pub fn is_aborted(&mut self) -> bool {
        if self.event.is_some() {
            if self.status == Some(Status::Aborted) {
                return true;
            }
            if self.status == Some(Status::Running) {
                let pid = rustix::process::Pid::from_raw(self.event.clone().unwrap().pid.unwrap());
                test_kill_process(pid.unwrap()).is_err()
            } else {
                false
            }
        } else {
            false
        }
    }
    /**
     Report if pipeline has options
    */
    pub fn has_options(&self) -> Result<bool> {
        Ok(self.options.is_some())
    }
    /**
     Report if pipeline has options
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
    pub fn has_loglevel_option(&self) -> Result<bool> {
        if let Some(options) = &self.options {
            Ok(options.log_level.is_some())
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
}
