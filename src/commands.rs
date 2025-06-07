mod setup;
mod update;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Setup DDNS
    Setup(setup::SetupCommand),

    /// Check and update DNS Records
    Update(update::UpdateCommand),
}
