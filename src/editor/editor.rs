use glib::Object;
use gtk::prelude::*;
use gtk::{glib, Application};

mod imp {
    use gtk::glib;
    use gtk::subclass::prelude::*;

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
        @extends gtk::Window, gtk::TextView, gtk::Widget;
}

impl Editor {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    pub fn write_to(&self, contents: &str) {
        let buffer = self.buffer();
        buffer.set_text(contents);
    }
}
