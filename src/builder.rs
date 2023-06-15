use duct::cmd;

use std::io::{BufRead, BufReader};
use std::path::Path;

use smbuilder::builder::types::SmbuilderSetupStage;
use smbuilder::prelude::*;

use crate::{pretty_panic, settings::*};

pub struct CliBuilder {
    settings: Settings,
}

impl CliBuilder {
    pub fn new(settings: Settings) -> CliBuilder {
        CliBuilder { settings }
    }
}

impl Smbuilder for CliBuilder {
    fn setup_build(&self, wrapper: &SmbuilderWrapper) {
        use SmbuilderSetupStage::*;

        let needed_targets = get_needed_setup_tasks(
            &wrapper.spec,
            &wrapper.base_dir,
            Box::new(self.settings.to_runnable()),
        );

        // define some closures for less indents
        let handle_write_spec = || {
            if let Err(e) = wrapper.write_spec() {
                pretty_panic(e, &self.settings)
            }
        };

        let handle_clone_repo = || {
            if let Err(e) = wrapper.clone_repo() {
                pretty_panic(e, &self.settings)
            }
        };

        let handle_copy_rom = |repo_dir: &Path| {
            if let Err(e) = wrapper.copy_rom(repo_dir) {
                pretty_panic(e, &self.settings)
            }
        };

        let handle_create_build_script = |repo_dir: &Path| {
            if let Err(e) = wrapper.create_build_script(repo_dir) {
                pretty_panic(e, &self.settings)
            }
        };

        for target in needed_targets {
            match target {
                WriteSpec => handle_write_spec(),
                CloneRepo => handle_clone_repo(),
                CopyRom => handle_copy_rom(&wrapper.base_dir.join(&wrapper.spec.repo.name)),
                CreateBuildScript => {
                    handle_create_build_script(&wrapper.base_dir.join(&wrapper.spec.repo.name))
                }
            }
        }
    }

    fn build(&self, wrapper: &SmbuilderWrapper) -> Result<(), smbuilder::error::Error> {
        let build_cmdout = cmd!(wrapper.base_dir.join("build.sh")).stderr_to_stdout();

        let output = build_cmdout.reader().unwrap(); // FIXME: unwrap
        let reader = BufReader::new(output);

        for line in reader.lines() {
            let ln = match line {
                Ok(line) => line,
                Err(e) => {
                    return Err(smbuilder::error::Error::new(
                        Some(Box::new(e)),
                        "the build command failed to run",
                    ))
                } // exit when there is no more output
            };

            (*wrapper.runnable_settings).show_build_output(&ln);
        }

        Ok(())
    }
}
