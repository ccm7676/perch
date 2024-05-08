mod window;
use gtk::prelude::*;
use gtk::{gio, glib, Application};
use window::Window;
use gtk4_layer_shell::{self, Layer, LayerShell,Edge};

const APP_ID: &str = "org.cmarino.perch";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("ui.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
 
    window.init_layer_shell();
    window.set_layer(Layer::Top);
    
    window.set_default_size(400, 300);
    window.present();
}
