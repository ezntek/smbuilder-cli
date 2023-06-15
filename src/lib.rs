pub mod builder;
pub mod cli;
pub mod settings;

use settings::*;
use smbuilder::prelude::{Error, RunnableSettings};

pub fn pretty_panic(err: Error, settings: &Settings) {
    let runnable_settings = settings.to_runnable();

    let panic_text = if let Some(e) = &err.cause {
        format!("{}: {}", err.description, *e)
    } else {
        err.description.to_string()
    };

    runnable_settings.error(&panic_text);

    // goodbye, program ;)
    if let CmdoutSettings::Silent = settings.cmdout_settings {
        panic!("{}", panic_text)
    } else {
        panic!("panicked from a pretty-panic: check the error message above.")
    }
}
