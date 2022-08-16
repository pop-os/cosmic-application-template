use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::{gio, glib};

use crate::application::ExampleApplication;
use crate::config::{APP_ID, PROFILE};

mod imp {
    use gtk4::gio::SettingsSchemaSource;

    use super::*;

    pub struct ExampleApplicationWindow {
        pub settings: Option<gio::Settings>,
    }

    impl Default for ExampleApplicationWindow {
        fn default() -> Self {
            Self {
                settings: SettingsSchemaSource::default()
                    .and_then(|s| s.lookup(APP_ID, true))
                    .map(|_| gio::Settings::new(APP_ID)),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ExampleApplicationWindow {
        const NAME: &'static str = "ExampleApplicationWindow";
        type Type = super::ExampleApplicationWindow;
        type ParentType = gtk4::ApplicationWindow;

        fn class_init(_: &mut Self::Class) {}

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(_: &glib::subclass::InitializingObject<Self>) {}
    }

    impl ObjectImpl for ExampleApplicationWindow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            // Devel Profile
            if PROFILE == "Devel" {
                obj.add_css_class("devel");
            }

            // Load latest window state
            obj.load_window_size();
        }
    }

    impl WidgetImpl for ExampleApplicationWindow {}
    impl WindowImpl for ExampleApplicationWindow {
        // Save window state on delete event
        fn close_request(&self, window: &Self::Type) -> gtk4::Inhibit {
            if let Err(err) = window.save_window_size() {
                log::warn!("Failed to save window state, {}", &err);
            }

            // Pass close request on to the parent
            self.parent_close_request(window)
        }
    }

    impl ApplicationWindowImpl for ExampleApplicationWindow {}
}

glib::wrapper! {
    pub struct ExampleApplicationWindow(ObjectSubclass<imp::ExampleApplicationWindow>)
        @extends gtk4::Widget, gtk4::Window, gtk4::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup, gtk4::Root;
}

impl ExampleApplicationWindow {
    pub fn new(app: &ExampleApplication) -> Self {
        glib::Object::new(&[("application", app)])
            .expect("Failed to create ExampleApplicationWindow")
    }

    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let imp = self.imp();

        let (width, height) = self.default_size();

        if let Some(settings) = imp.settings.as_ref() {
            settings.set_int("window-width", width)?;
            settings.set_int("window-height", height)?;

            settings.set_boolean("is-maximized", self.is_maximized())?;
        }

        Ok(())
    }

    fn load_window_size(&self) {
        let imp = self.imp();

        if let Some(settings) = imp.settings.as_ref() {
            let width = settings.int("window-width");
            let height = settings.int("window-height");
            let is_maximized = settings.boolean("is-maximized");

            self.set_default_size(width, height);

            if is_maximized {
                self.maximize();
            }
        }
    }
}
