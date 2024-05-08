mod window;
use gio::ActionEntry;
use gtk::prelude::*;
use gtk::{gio, glib, Application, ApplicationWindow};
use window::Window;
use gtk4_layer_shell::{self, Layer, LayerShell, KeyboardMode};

const APP_ID: &str = "org.cmarino.perch";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("ui.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);


    
    app.set_accels_for_action("win.close", &["Escape"]);
   
    app.run()

}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    
    window.init_layer_shell();
    window.set_layer(Layer::Top);
    window.set_keyboard_mode(KeyboardMode::OnDemand); 
    window.set_default_size(400, 300);

    let action_close = ActionEntry::builder("close")
        .activate(|window: &Window, _, _| {
            window.close();
        })
        .build();
    window.add_action_entries([action_close]);
    
    window.present();
}
