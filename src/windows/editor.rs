use glib::Object;
use gtk::{gio, glib, Application};

mod imp {
    use gtk::subclass::prelude::*;
    use gtk::{glib};

    #[derive(Default)]
    pub struct Editor;

    #[glib::object_subclass]
    impl ObjectSubclass for Editor {
        const NAME: &'static str = "EddyEditor";
        type Type = super::Editor;
        type ParentType = gtk::TextView;
    }

    impl ObjectImpl for Editor {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for Editor {}

    impl TextViewImpl for Editor {}
}

glib::wrapper! {
    pub struct Editor(ObjectSubclass<imp::Editor>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Editor {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }
}
