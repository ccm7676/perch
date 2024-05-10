use crate::window;
use gtk::gdk::Display;
use gio::ActionEntry;
use gtk::prelude::*;
use gtk::{gio, glib, Application, ApplicationWindow, CssProvider};
use window::Window;
use gtk4_layer_shell::{self, Layer, LayerShell, KeyboardMode};
use std::path::Path;


const APP_ID: &str = "org.cmarino.perch";

pub fn start_ui() -> glib::ExitCode {
    gio::resources_register_include!("ui.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    app.connect_startup(|_| load_css());
        
    app.set_accels_for_action("win.close", &["Escape"]);
   
    app.run()

}

fn load_css() {
    let styles = CssProvider::new();
    styles.load_from_path(Path::new("style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("display not found"),
        &styles,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
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
