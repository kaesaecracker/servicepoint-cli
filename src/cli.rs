#[derive(clap::Parser, std::fmt::Debug)]
#[clap(
    version,
    arg_required_else_help = true,
    about = "A command line interface for the ServicePoint display."
)]
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
    pub verbose: bool,
}

#[derive(clap::Parser, std::fmt::Debug)]
pub enum Mode {
    #[command(visible_alias = "r", about = "Reset both pixels and brightness")]
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
    #[command(visible_alias = "s")]
    Stream {
        #[clap(subcommand)]
        stream_command: StreamCommand,
    },
}

#[derive(clap::Parser, std::fmt::Debug)]
#[clap(about = "Commands for manipulating pixels")]
pub enum PixelCommand {
    #[command(
        visible_alias = "r",
        visible_alias = "reset",
        visible_alias = "clear",
        about = "Reset all pixels to the default (off) state"
    )]
    Off,
    #[command(visible_alias = "i", about = "Invert the state of all pixels")]
    Invert,
    #[command(about = "Set all pixels to the on state")]
    On,
}

#[derive(clap::Parser, std::fmt::Debug)]
#[clap(about = "Commands for manipulating the brightness")]
pub enum BrightnessCommand {
    #[command(
        visible_alias = "r",
        visible_alias = "reset",
        about = "Reset brightness to the default (max) level"
    )]
    Max,
    #[command(visible_alias = "s", about = "Set one brightness for the whole screen")]
    Set {
        #[arg()]
        brightness: u8,
    },
    #[command(about = "Set brightness to lowest possible level.")]
    Min,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Protocol {
    Udp,
    WebSocket,
    Fake,
}

#[derive(clap::Parser, std::fmt::Debug)]
#[clap(about = "Continuously send data to the display")]
pub enum StreamCommand {
    #[clap(
        about = "Pipe text to the display, example: `journalctl | servicepoint-cli stream stdin`"
    )]
    Stdin {
        #[arg(
            long,
            short,
            default_value_t = false,
            help = "Wait for a short amount of time before sending the next line"
        )]
        slow: bool,
    },
    #[clap(about = "Stream the default source to the display. \
        On Linux Wayland, this pops up a screen or window chooser, \
        but it also may directly start streaming your main screen.")]
    Screen {
        #[command(flatten)]
        options: StreamScreenOptions,
    },
}

#[derive(clap::Parser, std::fmt::Debug, Clone)]
pub struct StreamScreenOptions {
    #[arg(long, short, default_value_t = false, help = "Disable dithering - improves performance")]
    pub no_dither: bool,

    #[arg(
        long,
        short,
        default_value_t = false,
        help = "Show mouse pointer in video feed"
    )]
    pub pointer: bool,
}
