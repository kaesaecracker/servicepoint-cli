#[derive(clap::Parser, std::fmt::Debug)]
#[clap(version)]
pub struct Cli {
    #[arg(
        short,
        long,
        help = "ip:port of the servicepoint display",
        default_value = "127.0.0.1:2342"
    )]
    pub destination: String,
    #[arg(
        short,
        long,
        help = "protocol to use for communication with display",
        value_enum,
        default_value = "udp"
    )]
    pub transport: Protocol,
    #[clap(subcommand)]
    pub command: Mode,
    #[clap(short, long, help = "verbose logging")]
    pub verbose: bool
}

#[derive(clap::Parser, std::fmt::Debug)]
pub enum Mode {
    #[command(visible_alias = "r")]
    ResetEverything,
    #[command(visible_alias = "p")]
    Pixels {
        #[clap(subcommand)]
        pixel_command: PixelCommand,
    },
    #[command(visible_alias = "b")]
    Brightness {
        #[clap(subcommand)]
        brightness_command: BrightnessCommand,
    },
}

#[derive(clap::Parser, std::fmt::Debug)]
pub enum PixelCommand {
    #[command(visible_alias = "r")]
    Reset,
}

#[derive(clap::Parser, std::fmt::Debug)]
pub enum BrightnessCommand {
    #[command(visible_alias = "r")]
    Reset,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Protocol {
    Udp,
    WebSocket,
    Fake,
}
