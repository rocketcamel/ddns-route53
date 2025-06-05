use clap::builder::styling::{self, AnsiColor};

pub fn get_styles() -> clap::builder::styling::Styles {
    styling::Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Green.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Green.on_default())
}
