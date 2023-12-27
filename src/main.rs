use bevy::prelude::*;
use clap::Parser;

#[derive(Debug, Default, Parser)]
enum Cli {
    #[default]
    Launch,
    Update,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::parse() {
        Cli::Launch => launch(),
        Cli::Update => update()?,
    }

    Ok(())
}

fn launch() {
    App::new().add_plugins(DefaultPlugins).run();
}

fn update() -> Result<(), Box<dyn std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("washed-up-devs")
        .repo_name(env!("CARGO_PKG_NAME"))
        .bin_name("github")
        .show_download_progress(true)
        .current_version(self_update::cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}
