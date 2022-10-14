use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "PNGme")]
#[command(author = "Jonathan Pichel Carrera <jonathanpc@protonmail.com>")]
#[command(version = "1.0")]
#[command(about = "Hidden messages in PNG files")]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Args, Debug)]
pub struct EncodeArgs {
    pub path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    pub path: PathBuf,
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    pub path: PathBuf,
    pub chunk_type: String,
    pub output_file: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    pub path: PathBuf,
}