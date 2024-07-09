use gtk::prelude::*;
use gtk::{glib, Application};

mod imp {
    use gtk::glib;
    use tk::subclass::prelude::*;

    #[derive(Default)]
    pub struct Infobar;

    #[glib::object_subclass]
    impl ObjectSubclass for Infobar {
        const NAME: &'static str = "EddiInfobar";
        type Type = super::Infobar;
        type ParentType = gtk::Label;
    }

    impl ObjectImpl for Infobar {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
}

glib::wrapper! {
    pub struct Infobar(ObjectSubclass<imp::Infobar>)
        @extends gtk::Window, gtk::Label;
}

impl Infobar {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }
}
