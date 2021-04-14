use clap::Clap;

#[derive(Default, Clap, Debug)]
#[clap(version = "0.1.8")]
pub struct Args {
    #[clap(long)]
    pub gui: bool,

    #[clap(short, long, default_value = "20")]
    pub row: usize,

    #[clap(short, long, default_value = "20")]
    pub col: usize,
}
