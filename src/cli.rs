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
    #[command(visible_alias = "t")]
    Text {
        #[clap(subcommand)]
        text_command: TextCommand,
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
    #[command(visible_alias = "f", about = "Invert the state of all pixels")]
    Flip,
    #[command(about = "Set all pixels to the on state")]
    On,
    #[command(
        visible_alias = "i",
        about = "Send an image file (e.g. jpeg or png) to the display."
    )]
    Image {
        #[command(flatten)]
        send_image_options: SendImageOptions,
        #[command(flatten)]
        image_processing_options: ImageProcessingOptions,
    },
    #[command(
        visible_alias = "s",
        about = "Stream the default screen capture source to the display. \
        On Linux Wayland, this pops up a screen or window chooser, \
        but it also may directly start streaming your main screen."
    )]
    Screen {
        #[command(flatten)]
        stream_options: StreamScreenOptions,
        #[command(flatten)]
        image_processing: ImageProcessingOptions,
    },
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
#[clap(about = "Commands for sending text to the screen")]
pub enum TextCommand {
    #[command(
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
}

#[derive(clap::Parser, std::fmt::Debug, Clone)]
pub struct StreamScreenOptions {
    #[arg(
        long,
        short,
        default_value_t = false,
        help = "Show mouse pointer in video feed"
    )]
    pub pointer: bool,
}

#[derive(clap::Parser, std::fmt::Debug, Clone)]
pub struct ImageProcessingOptions {
    #[arg(long, help = "Disable histogram correction")]
    pub no_hist: bool,

    #[arg(long, help = "Disable blur")]
    pub no_blur: bool,

    #[arg(long, help = "Disable sharpening")]
    pub no_sharp: bool,

    #[arg(
        long,
        help = "Disable dithering. Brightness will be adjusted so that around half of the pixels are on."
    )]
    pub no_dither: bool,

    #[arg(long, help = "Do not remove the spacers from the image.")]
    pub no_spacers: bool,

    #[arg(long, help = "Do not keep aspect ratio when resizing.")]
    pub no_aspect: bool,
}

#[derive(clap::Parser, std::fmt::Debug, Clone)]
pub struct SendImageOptions {
    #[arg()]
    pub file_name: String,
}
