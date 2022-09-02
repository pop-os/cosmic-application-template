mod components;
#[rustfmt::skip]
mod config;
mod localize;

use config::APP_ID;
use log::info;

use localize::localize;

use crate::{config::{VERSION, PROFILE}, components::example};

fn main() -> iced::Result {
    // Initialize logger
    pretty_env_logger::init();
    info!("Cosmic Application Template ({})", APP_ID);
    info!("Version: {} ({})", VERSION, PROFILE);

    // Prepare i18n
    localize();

    example::run()
}
