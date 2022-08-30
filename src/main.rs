mod _components;
mod components;
#[rustfmt::skip]
mod config;
mod localize;

use relm4::{adw::{gio, glib}, RelmApp};

use _components::{ExampleApplication, app_model::AppModel};
use localize::localize;

fn main() {
    let _monitors = libcosmic::init();

    // Initialize logger
    pretty_env_logger::init();

    // Prepare i18n
    localize();

    gio::resources_register_include!("compiled.gresource").unwrap();

    glib::set_application_name(&fl!("app-name"));

    let app = ExampleApplication::new();
    let relm_app = RelmApp::with_app(app.clone());
    relm_app.run::<AppModel>(());
    // app.run();
}
