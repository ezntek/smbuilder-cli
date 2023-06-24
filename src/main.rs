use std::path::PathBuf;

use clap::Parser;
use colored::Colorize;
use smbuilder::prelude::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: PathBuf,
}

fn main() {
    color_eyre::install().unwrap();

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
        .repo_clone_progress(|recv_objs, total_objs, bytes_transferred| {
            print!(
                "{} {}/{} ({}%) objects transferred ({} KiB transferred)\r",
                "clone:".bold().green(),
                recv_objs,
                total_objs,
                (recv_objs * 100) / total_objs,
                (bytes_transferred as f64 / 1024_f64).floor(),
            )
        });

    let spec = Spec::from_file_checked("./sample.yaml", &mut callbacks)
        .unwrap_or_else(|e| panic!("failed to create the spec: {}", e));

    let mut builder = Builder::new(spec, "./", callbacks)
        .unwrap_or_else(|e| panic!("failed to create the builder: {}", e));

    builder.build();
}
