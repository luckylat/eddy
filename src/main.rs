mod editor;

use gtk::prelude::*;
use gtk::{gio, Application};

use editor::Window;

const APP_ID: &str = "com.cleyl.eddy";

fn main() {
    gio::resources_register_include!("eddy.gresource").expect("Failed to register resources");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(setup_shortcuts);
    app.connect_activate(build_ui);

    app.run();
}

fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.open", &["<Ctrl>o"]);
    app.set_accels_for_action("win.save", &["<Ctrl>s"]);
    app.set_accels_for_action("win.save_as", &["<Ctrl><Shift>s"]);
}

fn build_ui(app: &Application) {
    //TODO: Widget

    //TODO: Tab
    let window = Window::new(app);
    // let window = windows::window::build();

    //let window = ApplicationWindow::builder()
    //    .application(app)
    //    .title("Eddy")
    //    .child(&editor)
    //    .build();

    window.present();
}
