use clap::Parser;
use smbuilder::{prelude::SmbuilderWrapper, types::Spec};
use smbuilder_cli::{builder::*, cli::*, pretty_panic, settings::*};

fn main() {
    let args = Args::parse();

    let Command::Build(build_args) = args.cmd;

    let log_level_setting = if build_args.verbose {
        CmdoutSettings::LogProgress { log_level: 3 }
    } else {
        if let Some(log_level) = build_args.log_level {
            CmdoutSettings::LogProgress { log_level }
        } else {
            CmdoutSettings::LogProgress { log_level: 2 }
        }
    };

    let settings = Settings {
        cmdout_settings: log_level_setting,
    };

    let spec = match Spec::from_file(build_args.filename) {
        Ok(s) => s,
        Err(e) => {
            pretty_panic(e, &settings);
            panic!(); // dummy code
        }
    };

    let builder = match SmbuilderWrapper::new(
        spec,
        "./",
        Box::new(settings.to_runnable()),
        Box::new(CliBuilder::new(settings)),
    ) {
        Ok(b) => b,
        Err(e) => {
            pretty_panic(e, &settings);
            panic!() // dummy code for the compiler
        }
    };
    match builder.build() {
        Ok(_) => (),
        Err(e) => pretty_panic(e, &settings),
    };
}
