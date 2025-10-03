#![allow(dead_code)]

use std::{env, error::Error, process::Command};

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{e}");
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("build_cli") => build_cli()?,
        Some("build_wasm") => build_wasm()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "tasks: 
build_cli           build release binary
build_wasm          build and bundle WASM target"
    )
}

fn build_cli() -> Result<(), Box<dyn Error>> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let status = Command::new(cargo)
        .args(["build", "--release", "--bin=nlbt"])
        .status()?;

    if !status.success() {
        Err("cargo build failed")?;
    }

    Ok(())
}

fn build_wasm() -> Result<(), Box<dyn Error>> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let status = Command::new(cargo)
        .args([
            "build",
            "--release",
            "--lib",
            "--features=wasm",
            "--no-default-features",
            "--target=wasm32-unknown-unknown",
        ])
        .status()?;

    if !status.success() {
        Err("cargo build failed")?;
    }

    let outdir = env::args().nth(2).unwrap_or("./pkg/".into());

    let status = Command::new("wasm-bindgen")
        .args([
            "--out-dir",
            &outdir,
            "./target/wasm32-unknown-unknown/release/nlbl.wasm",
        ])
        .status()?;

    if !status.success() {
        Err("wasm bundle failed")?;
    }

    println!("\nwasm bundled into: ");
    Command::new("realpath").args([outdir]).status()?;

    Ok(())
}
