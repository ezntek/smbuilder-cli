use colored::Colorize;
use smbuilder::prelude::*;

fn main() {
    let mut callbacks = Callbacks::empty()
        .log(|log_type, text| {
            use LogType::*;

            match log_type {
                Error => println!("{}{}", "error: ".bold().red(), text),
                Warn => println!("{}{}", "warn: ".bold().magenta(), text),
                BuildOutput => println!("{}{}", "make: ".bold().cyan(), text),
                Info => println!("{}{}", "info: ".bold().blue(), text),
            };
        })
        .repo_clone_progress(|progress, bytes_transferred| {
            print!(
                "{}{}% ({} kb transferred)\r",
                "clone: ".bold().green(),
                (progress * 100_f64),
                (bytes_transferred * 1024_usize)
            )
        });

    let spec = Spec::from_file_checked("./sample.yaml", &mut callbacks)
        .unwrap_or_else(|e| panic!("failed to create the spec: {}", e));

    let mut builder = Builder::new(spec, "./", callbacks)
        .unwrap_or_else(|e| panic!("failed to create the builder: {}", e));

    builder.build();
}
