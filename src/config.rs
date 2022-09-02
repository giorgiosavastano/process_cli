use anyhow::{Context, Result};
use clap::Parser;
use log::debug;
use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

/// Process files.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// The path to the input files to read
    #[clap(long, value_parser)]
    input_path: Option<PathBuf>,

    /// The input files extension
    #[clap(short, long, value_parser, default_value = "nc")]
    input_extension: String,

    /// The path of the outputs
    #[clap(long, value_parser)]
    output_path: Option<PathBuf>,

    /// The path to the temporary directory
    #[clap(long, value_parser, default_value = "default")]
    tmp_dir_path: PathBuf,

    /// The flag for overwrite
    #[clap(short, long, value_parser, default_value = "false")]
    overwrite: bool,

    /// Sets a custom config file
    #[clap(short, long, value_parser, value_name = "FILE")]
    config: Option<PathBuf>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MyConfig {
    pub input_path: Option<PathBuf>,
    pub input_extension: Option<String>,
    pub output_path: Option<PathBuf>,
    pub tmp_dir_path: Option<PathBuf>,
    pub overwrite: Option<bool>,
}

pub fn parse_cli_input(cli: &Cli) -> Result<MyConfig> {
    let mut cfg = MyConfig::default();
    if let Some(input_path) = cli.input_path.as_deref() {
        cfg.input_path = Some(input_path.to_path_buf());
    }
    if let Some(output_path) = cli.output_path.as_deref() {
        cfg.output_path = Some(output_path.to_path_buf());
    }

    cfg.tmp_dir_path = Some(cli.tmp_dir_path.to_path_buf());
    cfg.input_extension = Some(cli.input_extension.to_string());
    cfg.overwrite = Some(cli.overwrite);

    if let Some(config_path) = cli.config.as_deref() {
        let cfg_new: MyConfig = confy::load_path(config_path)
            .with_context(|| format!(" could not load config file {}", config_path.display()))?;

        debug!("Input test {:?}", &cfg_new.input_path);
        debug!("Output test {:?}", &cfg_new.output_path);

        if let Some(input_pat) = cfg_new.input_path {
            cfg.input_path = Some(input_pat)
        }
        if let Some(output_path) = cfg_new.output_path {
            cfg.output_path = Some(output_path)
        }
        if let Some(tmp_dir_path) = cfg_new.tmp_dir_path {
            cfg.tmp_dir_path = Some(tmp_dir_path)
        }
        if let Some(overwrite) = cfg_new.overwrite {
            cfg.overwrite = Some(overwrite)
        }
        if let Some(input_extension) = cfg_new.input_extension {
            cfg.input_extension = Some(input_extension)
        }
    }

    Ok(cfg)
}
