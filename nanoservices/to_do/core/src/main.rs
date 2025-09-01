mod api;
mod enums;
mod structs;

use crate::enums::TaskStatus;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    title: String,
    #[arg(short, long)]
    status: String,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let status_enum = TaskStatus::from_string(&args.status)?;
    Ok(())
}
