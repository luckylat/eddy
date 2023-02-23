use glib::{Object, clone};
use gio::{
    SimpleAction
};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{
    gio,
    glib,

    Application,
    FileChooserAction,
    FileChooserDialog,
    ResponseType,
};

use std::io::{
    BufReader,
    Read
};
use std::fs::File;

use crate::{
    editor::{
        editor::Editor,
    }
};

mod imp {
    use glib::subclass::InitializingObject;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{glib, CompositeTemplate};
    
    use super::*;

    #[derive(CompositeTemplate, Default)]
    #[template(resource="/com/cleyl/eddy/window.ui")]
    pub struct Window {
        #[template_child]
        pub editor: TemplateChild<Editor>,
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
            app.save();
        }));
        self.add_action(&action_save);
    }
    
    //editor.rs
    fn open(&self) {
        let editor = self.imp().editor.get();
        // Open FileDialog
        // Deprecated since v4_10
        let open_dialog = FileChooserDialog::builder()
            .action(FileChooserAction::Open)
            .title("Open")
            .transient_for(self)
            .modal(true)
            .build();
        open_dialog.add_button("_Cancel", ResponseType::Cancel);
        open_dialog.add_button("_Open", ResponseType::Accept);

        open_dialog.connect_response(move |dialog: &FileChooserDialog, response: ResponseType| {
            if response == ResponseType::Accept {
                let file = dialog.file().expect("Couldn't get file");

                let filename = file.path().expect("Couldn't get path");
                let file = File::open(filename).expect("Couldn't oepn file");

                let mut reader = BufReader::new(file);
                let mut contents = String::new();
                reader.read_to_string(&mut contents);

                editor.write_to(&contents);

                dialog.close();
            }
        });

        open_dialog.show();
    }
    
    fn save(&self) {
        let editor = self.imp().editor.get();
        let buffer = editor.buffer();
        let (start, end) = buffer.bounds();
        let text = buffer.text(&start, &end, true);

        println!("{}", text);

        // Open FileDialog
        // Deprecated since v4_10
        let save_dialog = FileChooserDialog::builder()
            .action(FileChooserAction::Save)
            .title("Save")
            .transient_for(self)
            .modal(true)
            .build();
        save_dialog.show();
    }
}
