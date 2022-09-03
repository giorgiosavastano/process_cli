use anyhow::{Context, Ok, Result};
use clap::Parser;
use log::info;

use crate::process::{Item, ProcessingCore};

pub mod config;
pub mod process;

fn main() -> Result<()> {
    env_logger::init();
    info!(" Starting up");

    let cli = config::Cli::parse();

    let cfg = config::parse_cli_input(&cli)
        .with_context(|| format!("could not parse cli `{:?}`", &cli))?;

    let mut proc = process::Process {
        name: String::from("Test"),
        inputs_dir_path: cfg.input_path.unwrap(),
        inputs_extenion: cfg.input_extension.unwrap(),
        outputs_dir_path: cfg.output_path.unwrap(),
        tmp_dir_path: cfg.tmp_dir_path.unwrap(),
        overwrite: cfg.overwrite.unwrap(),
        items: Vec::new(),
    };

    proc.set_items()?;

    if proc.check_all_inputs_exist()? {
        info!("All good!");
    }

    if proc.tmp_dir_path.to_str() != Some("default") {
        proc.create_tmp_directory()?;
    }

    if proc.process_items(_process_item)? {
        info!(" Daje!")
    }

    Ok(())
}

fn _process_item(item: &Item) -> Result<bool> {
    // define how to process a single item
    info!(
        "Processing {} {:?} -> {:?}",
        item.name, item.input_item_path, item.output_item_path
    );
    // ...

    Ok(true)
}
