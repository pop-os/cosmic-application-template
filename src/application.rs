use adw::StyleManager;
use gtk4::gio::{FileMonitorEvent, FileMonitorFlags};
use log::{debug, info};

use glib::clone;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::{gdk, gio, glib};

use crate::config::{APP_ID, PROFILE, VERSION};
use crate::fl;
use crate::window::ExampleApplicationWindow;

mod imp {
    use super::*;
    use glib::WeakRef;
    use gtk4::gio::FileMonitor;
    use once_cell::sync::OnceCell;

    #[derive(Debug, Default)]
    pub struct ExampleApplication {
        pub window: OnceCell<WeakRef<ExampleApplicationWindow>>,
        pub monitor: OnceCell<FileMonitor>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ExampleApplication {
        const NAME: &'static str = "ExampleApplication";
        type Type = super::ExampleApplication;
        type ParentType = gtk4::Application;
    }

    impl ObjectImpl for ExampleApplication {}

    impl ApplicationImpl for ExampleApplication {
        fn activate(&self, app: &Self::Type) {
            debug!("GtkApplication<ExampleApplication>::activate");
            self.parent_activate(app);

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.present();
                return;
            }

            let window = ExampleApplicationWindow::new(app);
            self.window
                .set(window.downgrade())
                .expect("Window already set.");

            app.main_window().present();
        }

        fn startup(&self, app: &Self::Type) {
            debug!("GtkApplication<ExampleApplication>::startup");
            self.parent_startup(app);

            // Set icons for shell
            gtk4::Window::set_default_icon_name(APP_ID);

            app.setup_css();
            app.setup_gactions();
            app.setup_accels();
        }
    }

    impl GtkApplicationImpl for ExampleApplication {}
}

glib::wrapper! {
    pub struct ExampleApplication(ObjectSubclass<imp::ExampleApplication>)
        @extends gio::Application, gtk4::Application,
        @implements gio::ActionMap, gio::ActionGroup;
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

    fn main_window(&self) -> ExampleApplicationWindow {
        self.imp().window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        // Quit
        let action_quit = gio::SimpleAction::new("quit", None);
        action_quit.connect_activate(clone!(@weak self as app => move |_, _| {
            // This is needed to trigger the delete event and saving the window state
            app.main_window().close();
            app.quit();
        }));
        self.add_action(&action_quit);

        // About
        let action_about = gio::SimpleAction::new("about", None);
        action_about.connect_activate(clone!(@weak self as app => move |_, _| {
            app.show_about_dialog();
        }));
        self.add_action(&action_about);
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<Control>q"]);
    }

    fn setup_css(&self) {
        let provider = gtk4::CssProvider::new();
        provider.load_from_resource("/com/system76/CosmicApplicationTemplate/style.css");
        if let Some(display) = gdk::Display::default() {
            gtk4::StyleContext::add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        let user_provider = gtk4::CssProvider::new();
        if let Some(display) = gdk::Display::default() {
            gtk4::StyleContext::add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_USER,
            );
        }

        let path = xdg::BaseDirectories::with_prefix("gtk-4.0")
            .ok()
            .and_then(|xdg_dirs| xdg_dirs.find_config_file("gtk.css"))
            .unwrap_or_else(|| "~/.config/gtk-4.0/gtk.css".into());
        let file = gio::File::for_path(path);
        if let Ok(monitor) = file.monitor(FileMonitorFlags::all(), None::<&gio::Cancellable>) {
            monitor.connect_changed(glib::clone!(@strong user_provider => move |_monitor, file, _other_file, event| {
                match event {
                    FileMonitorEvent::Deleted | FileMonitorEvent::MovedOut | FileMonitorEvent::Renamed => {
                        if adw::is_initialized() {
                            let manager = StyleManager::default();
                            let css = if manager.is_dark() {
                                adw_user_colors_lib::colors::ColorOverrides::dark_default().as_css()
                            } else {
                                adw_user_colors_lib::colors::ColorOverrides::light_default().as_css()
                            };
                            user_provider
                                .load_from_data(css.as_bytes());
                        }
                    },
                    FileMonitorEvent::ChangesDoneHint | FileMonitorEvent::Created | FileMonitorEvent::MovedIn => {
                        user_provider.load_from_file(file);
                    },
                    _ => {} // ignored
                }
            }));
            self.imp().monitor.set(monitor).unwrap();
        }
    }

    fn show_about_dialog(&self) {
        let dialog = gtk4::AboutDialog::builder()
            .logo_icon_name(APP_ID)
            // Insert your license of choice here
            // .license_type(gtk4::License::MitX11)
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
