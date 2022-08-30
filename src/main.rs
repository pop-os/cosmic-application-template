mod components;
#[rustfmt::skip]
mod config;
mod localize;

use config::APP_ID;
use log::info;
use relm4::{adw::{gio, glib, gtk, gdk, builders::ApplicationBuilder}, RelmApp};

use localize::localize;

use crate::{config::{VERSION, PROFILE}, components::example::Example};

fn main() {
    let _monitors = libcosmic::init();

    // Initialize logger
    pretty_env_logger::init();
    info!("Cosmic Application Template ({})", APP_ID);
    info!("Version: {} ({})", VERSION, PROFILE);
    
    let provider = gtk::CssProvider::new();
    provider.load_from_resource("/com/system76/CosmicApplicationTemplate/style.css");
    if let Some(display) = gdk::Display::default() {
        gtk::StyleContext::add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    // Prepare i18n
    localize();

    gio::resources_register_include!("compiled.gresource").unwrap();

    glib::set_application_name(&fl!("app-name"));

    let app = ApplicationBuilder::new()
    .resource_base_path("/com/system76/CosmicApplicationTemplate/")
    .flags(gio::ApplicationFlags::empty())
    .build();
    let relm_app = RelmApp::with_app(app);
    relm_app.run::<Example>(0);
}
