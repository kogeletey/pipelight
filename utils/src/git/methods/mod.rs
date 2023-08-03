use crate::git::{Flag, Git, Hook, Special};
use git2::Repository;
use std::env;
// Enum workaround
use std::string::ToString;
use strum::{EnumIter, IntoEnumIterator};

// Error Handling
use miette::{Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};

// File systeme crates
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

impl Git {
    pub fn new() -> Git {
        let root = env::current_dir().unwrap();
        Git {
            repo: Repository::discover(root).ok(),
        }
    }
    pub fn teleport(&mut self) {
        if self.exists() {
            let wd = self
                .repo
                .as_mut()
                .unwrap()
                .workdir()
                .unwrap()
                .display()
                .to_string();
            env::set_current_dir(wd).unwrap();
        }
    }
    ///  Detect if inside a git repo
    pub fn exists(&mut self) -> bool {
        self.repo.is_some()
    }
    /// Return actual attached branch
    pub fn get_branch(&self) -> Result<Option<String>> {
        // Only tested on attached HEAD
        // No edge case when head is a commit or else...
        let repo = self.repo.as_ref().unwrap();
        let head = repo.head().into_diagnostic()?;
        let name = Some(head.shorthand().unwrap().to_owned());
        Ok(name)
    }
    /// Return tag if its is latest commit
    pub fn get_tag(&self) -> Result<Option<String>> {
        let repo = self.repo.as_ref().unwrap();
        let head = repo.head().into_diagnostic()?;
        let tag = if head.is_tag() {
            Some(head.name().unwrap().to_string())
        } else {
            None
        };
        Ok(tag)
    }
}

impl Hook {
    /// Detect name of the hook that triggers script
    pub fn origin() -> Result<Flag> {
        let root = env::current_dir().into_diagnostic()?;
        let path_string = root.display().to_string();
        if path_string.contains("/.git/hooks/") {
            // Get hook name from folder name
            let name = root.file_stem().unwrap().to_str().unwrap().to_owned();
            let hook = Flag::Hook(Hook::from(&name));
            Ok(hook)
        } else {
            Ok(Flag::Special(Special::Manual))
            // let message = "Can't trigger hook outside of repository hook folder";
            // Err(Box::from(message))
        }
    }
    /// Ensure .git/hook folder
    pub fn new() -> Result<()> {
        let root = ".git/hooks";
        let extension = ".d";
        let bin = "pipelight";

        for hook in Hook::iter() {
            let caller = format!("{}/{}", root, String::from(&hook));
            let caller_path = Path::new(&caller);

            let dot_d_dir = format!("{}/{}{}", root, String::from(&hook), extension);
            let dot_d_dir_path = Path::new(&dot_d_dir);

            let script = format!("{}/{}", dot_d_dir, bin);
            let script_path = Path::new(&script);

            if Git::new().repo.is_some() {
                Hook::create_script(dot_d_dir_path, script_path)?;
                Hook::create_subscripts_caller(caller_path, &hook)?;
            }
        }
        Ok(())
    }
    /// Create a hook that will call scrpts from a hook.d subfolder
    fn create_subscripts_caller(path: &Path, hook: &Hook) -> Result<()> {
        let git = Git::new();
        let action = String::from(hook);
        let root = git.repo.unwrap().path().display().to_string();
        let mut file = fs::File::create(path).into_diagnostic()?;
        let s = format!(
            "#!/bin/sh \n\
            dir=\"{root}hooks/{action}.d\" \n\
            for file in \"$dir/*\"; do \n\
              cd $dir
              $file \n\
            done",
            root = root,
            action = action
        );
        let b = s.as_bytes();
        file.write_all(b).into_diagnostic()?;

        // Set permissions
        let metadata = file.metadata().into_diagnostic()?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms).into_diagnostic()?;

        Ok(())
    }
    fn create_script(directory_path: &Path, file_path: &Path) -> Result<()> {
        fs::create_dir_all(directory_path).into_diagnostic()?;
        let mut file = fs::File::create(file_path).into_diagnostic()?;
        #[cfg(debug_assertions)]
        let s = "#!/bin/sh \n\
            cargo run --bin pipelight trigger \
            "
        .to_owned();
        #[cfg(not(debug_assertions))]
        let s = "#!/bin/sh \n\
            pipelight trigger \
            "
        .to_owned();
        let b = s.as_bytes();
        file.write_all(b).into_diagnostic()?;

        // Set permissions
        let metadata = file.metadata().into_diagnostic()?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(file_path, perms).into_diagnostic()?;

        Ok(())
    }
}