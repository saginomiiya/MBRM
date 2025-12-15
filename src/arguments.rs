use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "MBRM",
    version = "1.0",
    author = "quetsyl",
    about = "a simple mbr management utility in rust"
)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub input: String,

    #[arg(short, long, default_value_t = 32, help = "Set amount of bytes in one chunk")]
    pub chunk_bytes: usize,

    #[arg(short, long, help = "Print blake3 checksum at the end of output")]
    pub blake3: bool,

    #[arg(short, long, help = "Write MBR data to given path")]
    pub output: Option<String>,

    #[arg(short, long, help = "Remove color from MBR hex dump")]
    pub no_color: bool
}

