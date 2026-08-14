
use std::str::FromStr;

use clap::{Parser, ValueEnum};
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use raw_cpuid::{CpuId, CpuIdReaderNative};

#[derive(ValueEnum, Clone)]
enum OutputFormat {
    #[value(alias("raw"))]
    Raw,
    #[value(alias("cli"))]
    Cli,
}

impl FromStr for OutputFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> { panic!("STUB: not implemented") }
}

#[derive(Parser)]
#[clap(version = "10.2", author = "Gerd Zellweger <mail@gerdzellweger.com>")]
#[clap(disable_colored_help(true))]
struct Opts {
    
    #[clap(short, long, default_value = "cli")]
    format: OutputFormat,
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
fn main() {
    let opts: Opts = Opts::parse();
    match opts.format {
        OutputFormat::Raw => raw_cpuid::display::raw(CpuIdReaderNative),
        OutputFormat::Cli => {
            let cpuid = CpuId::new();
            raw_cpuid::display::markdown(cpuid);
        }
    };
}
