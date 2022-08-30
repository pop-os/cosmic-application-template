use log::{debug, info};

use relm4::{adw::{gdk, gio, glib, gtk, prelude::*, subclass::prelude::*}};
use crate::config::{APP_ID, PROFILE, VERSION};
use crate::fl;
use crate::_components::ExampleApplicationWindow;

mod imp {
    use super::*;
    use glib::WeakRef;
    use once_cell::sync::OnceCell;

    #[derive(Debug, Default)]
    pub struct ExampleApplication {
        pub window: OnceCell<WeakRef<ExampleApplicationWindow>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ExampleApplication {
        const NAME: &'static str = "ExampleApplication";
        type Type = super::ExampleApplication;
        type ParentType = gtk::Application;
    }

    impl ObjectImpl for ExampleApplication {}

    impl ApplicationImpl for ExampleApplication {
        fn activate(&self, app: &Self::Type) {
            debug!("GtkApplication<ExampleApplication>::activate");
            self.parent_activate(app);

            if self.window.get().is_none() {
                let window = ExampleApplicationWindow::new(app);
                self.window
                    .set(window.downgrade())
                    .expect("Window already set.");
            }

            app.main_window().present();
        }

        fn startup(&self, app: &Self::Type) {
            debug!("GtkApplication<ExampleApplication>::startup");
            self.parent_startup(app);

            // Set icons for shell
            gtk::Window::set_default_icon_name(APP_ID);

            app.setup_css();
            app.setup_gactions();
            app.setup_accels();
        }
    }

    impl GtkApplicationImpl for ExampleApplication {}
}

glib::wrapper! {
    pub struct ExampleApplication(ObjectSubclass<imp::ExampleApplication>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Default for ExampleApplication {
    fn default() -> Self {
        Self::new()
    }
}

impl ExampleApplication {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &Some(APP_ID)),
            ("flags", &gio::ApplicationFlags::empty()),
            (
                "resource-base-path",
                &Some("/com/system76/CosmicApplicationTemplate/"),
            ),
        ])
        .expect("Application initialization failed...")
    }

    pub fn main_window(&self) -> ExampleApplicationWindow {
        if let Some(w) = self.imp().window.get() {
            w.upgrade().unwrap()
        } else {
            let w = ExampleApplicationWindow::new(self);
            self.imp().window.set(w.downgrade()).unwrap();
            w
        }
    }

    fn setup_gactions(&self) {
        // Quit
        let action_quit = gio::SimpleAction::new("quit", None);
        action_quit.connect_activate(glib::clone!(@weak self as app => move |_, _| {
            // This is needed to trigger the delete event and saving the window state
            app.main_window().close();
            app.quit();
        }));
        self.add_action(&action_quit);

        // About
        let action_about = gio::SimpleAction::new("about", None);
        action_about.connect_activate(glib::clone!(@weak self as app => move |_, _| {
            app.show_about_dialog();
        }));
        self.add_action(&action_about);
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<Control>q"]);
    }

    fn setup_css(&self) {
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/com/system76/CosmicApplicationTemplate/style.css");
        if let Some(display) = gdk::Display::default() {
            gtk::StyleContext::add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    fn show_about_dialog(&self) {
        let dialog = gtk::AboutDialog::builder()
            .logo_icon_name(APP_ID)
            // Insert your license of choice here
            // .license_type(gtk::License::MitX11)
            // Insert your website here
            // .website("https://gitlab.gnome.org/bilelmoussaoui/cosmic-application-template/")
            .version(VERSION)
            .transient_for(&self.main_window())
            .translator_credits(&fl!("translator-credits"))
            .modal(true)
            .authors(vec!["Ashley Wulber".into()])
            .artists(vec!["Ashley Wulber".into()])
            .build();

        dialog.present();
    }

    pub fn run(&self) {
        info!("Cosmic Application Template ({})", APP_ID);
        info!("Version: {} ({})", VERSION, PROFILE);

        ApplicationExtManual::run(self);
    }
}