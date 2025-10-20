mod add;
mod list;
mod remove;
mod setup;
mod update;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Setup DDNS
    Setup(setup::SetupCommand),

    /// Check and update DNS Records
    Update(update::UpdateCommand),

    /// Add record(s) to the update list
    Add(add::AddCommand),

    /// Remove a record from the update list
    Remove(remove::RemoveCommand),

    /// List records to be updated
    List(list::ListCommand),
}
