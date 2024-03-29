use gio::SimpleAction;
use glib::{clone, Object, PropertySet};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, FileDialog};

use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::rc::Rc;

use crate::editor::editor::Editor;

mod imp {
    use glib::subclass::InitializingObject;
    use gtk::subclass::prelude::*;
    use gtk::{glib, CompositeTemplate};

    use std::cell::RefCell;

    use super::*;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/cleyl/eddy/window.ui")]
    pub struct Window {
        #[template_child]
        pub editor: TemplateChild<Editor>,

        pub file: RefCell<Option<gio::File>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "EddyWindow";
        type Type = super::Window;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Window {
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().setup_actions();
        }
    }

    impl WidgetImpl for Window {}

    impl WindowImpl for Window {}

    impl ApplicationWindowImpl for Window {}
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    pub fn setup_actions(&self) {
        let action_open = SimpleAction::new("open", None);
        action_open.connect_activate(clone!(@weak self as app => move |_, _| {
            app.open();
        }));
        self.add_action(&action_open);

        let action_save = SimpleAction::new("save", None);
        action_save.connect_activate(clone!(@weak self as app => move |_, _| {
            app.save(false);
        }));
        self.add_action(&action_save);

        let action_save_as = SimpleAction::new("save_as", None);
        action_save_as.connect_activate(clone!(@weak self as app => move |_, _| {
            app.save(true);
        }));
        self.add_action(&action_save_as);
    }

    //TODO Move to editor.rs
    fn open(&self) {
        let editor = self.imp().editor.get();
        //TODO: show unsave dialog if current file is not saved.
        let open_dialog = FileDialog::builder()
            .accept_label("Open")
            .title("Open")
            .modal(true)
            .build();

        open_dialog.open(Some(self), gio::Cancellable::NONE, move |file| {
            //let file_core = Rc::new(file.expect("Couldn't get file"));
            let file_core = file.unwrap();

            self.set_file(Some(file_core.clone()));
            self.set_title(Some(format!("{} - Eddy", file_core.clone().path().unwrap().to_str().unwrap()).as_str()));

            let filename = file_core.path().expect("Couldn't get path");
            let raw_file = File::open(&filename).expect("Couldn't open file");

            let mut reader = BufReader::new(raw_file);
            let mut contents = String::new();
            reader.read_to_string(&mut contents).unwrap();

            editor.write_to(&contents);
        });
    }

    //TODO: Move to editor.rs
    fn save(&self, is_as: bool) {
        let file = self.imp().file.borrow().clone();
        let mut need_save_as = is_as;
        if file == None {
            need_save_as = true;
        }

        let editor = self.imp().editor.get();
        let buffer = editor.buffer();
        let (start, end) = buffer.bounds();
        let text = buffer.text(&start, &end, true).into_bytes_with_nul();

        if need_save_as {
            let save_dialog = FileDialog::builder()
                .accept_label("Save")
                .title("Save")
                .modal(true)
                .build();

            save_dialog.open(Some(self), gio::Cancellable::NONE, move |file| {
                let file_core = file.unwrap();

                self.set_file(Some(file_core.clone()));
                self.set_title(Some(format!("{} - Eddy", file_core.clone().path().unwrap().to_str().unwrap()).as_str()));

                let mut file = File::create(file_core.path().unwrap()).unwrap();
                file.write_all(&text).unwrap();
                file.flush().unwrap();
            });
        } else {
            let mut file = File::create(file.unwrap().path().unwrap()).unwrap();
            file.write_all(&text).unwrap();
            file.flush().unwrap();
        }
    }

    fn set_file(&self, file: Option<gio::File>) {
        self.imp().file.set(file);
    }}
