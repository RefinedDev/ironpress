mod cli;

use anyhow::{Context, Result};
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cli = match cli::parse_args(&args) {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("error: {e:#}\n");
            cli::print_usage();
            std::process::exit(1);
        }
    };

    let input_path = Path::new(&cli.input);
    let output_path = Path::new(&cli.output);

    let input = std::fs::read(input_path)
        .with_context(|| format!("failed to read {}", input_path.display()))?;

    let output = match cli.runtype {
        cli::RunType::Compress => ironpress::compress(&input),
        cli::RunType::Decompress => ironpress::decompress(&input)
            .with_context(|| format!("failed to decompress {}", input_path.display()))?,
    };

    if let Some(parent) = output_path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }

    std::fs::write(output_path, &output)
        .with_context(|| format!("failed to write {}", output_path.display()))?;

    let percent = if input.is_empty() {
        0.0
    } else {
        output.len() as f64 / input.len() as f64 * 100.0
    };
    println!("{} -> {} bytes ({:.1}%)", input.len(), output.len(), percent);

    Ok(())
}