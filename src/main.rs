mod components;
mod config;
mod localize;

use config::APP_ID;
use log::info;

use localize::localize;

use crate::{components::app, config::VERSION};

fn main() -> cosmic::iced::Result {
    // Initialize logger
    pretty_env_logger::init();
    info!("Cosmic Application Template ({})", APP_ID);
    info!("Version: {}", VERSION);

    // Prepare i18n
    localize();

    app::run()
}
