use anyhow::{Context, Result, bail};
use std::path::PathBuf;

#[derive(Debug)]
pub enum RunType {
    Compress,
    Decompress
}
#[derive(Debug)]
pub struct CliArgs {
    pub runtype: RunType,
    pub input: PathBuf,
    pub output: PathBuf,
}

impl Default for CliArgs {
    fn default() -> Self {
        CliArgs {
            runtype: RunType::Compress,
            input: PathBuf::from("."),
            output: PathBuf::from("."),
        }
    }
}

pub fn parse_args(args: &[String]) -> Result<CliArgs> {
    let mut cli = CliArgs::default();
    let mut output: Option<PathBuf> = None;
    let mut input: Option<PathBuf> = None;
    let mut runtype: Option<RunType> = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }

            "--input" => {
                i += 1;
                let value = args.get(i).context("--input requires a value")?;
                input = Some(PathBuf::from(value));
            }

            "--output" => {
                i += 1;
                let value = args.get(i).context("--output requires a value")?;
                output = Some(PathBuf::from(value));
            }

            flag if flag.starts_with('-') => bail!("unknown option '{flag}'"),
            rt => {
                if runtype.is_some() {
                    bail!("unexpected argument '{rt}'");
                }
                if rt == "compress" || rt == "c" {
                    runtype = Some(RunType::Compress)
                } else if rt == "decompress" || rt == "d" {
                    runtype = Some(RunType::Decompress)
                } else {
                    bail!("invalid runtype argument '{rt}'");
                }
            }
        }

        i += 1;
    }

    cli.input = input.expect("No path given to input file");
    cli.output = output.expect("No path given to output file");
    cli.runtype = runtype.expect("No runtype specified");

    Ok(cli)
}

pub fn print_usage() {
    let program_path =
        std::fs::canonicalize(std::env::args().next().unwrap_or_default()).unwrap_or_default();
    let program = program_path
        .strip_prefix(std::env::current_dir().unwrap_or_default())
        .unwrap_or(std::path::Path::new(""))
        .to_str()
        .unwrap_or_default();

    print!(
        "\
ironpress - a file compressor  and decompressor

USAGE:
    {program} <runtype> [OPTIONS]

ARGS:
    <runtype>    compress / decompress

OPTIONS:
    --input <dir>     Path to input file
    --output <dir>    Path to output file
    -h, --help        Print this help
"
    );
}