use glib_build_tools::compile_resources;

fn main() {
    // Compile the GTK4 UI template into a GResource file
    glib_build_tools::compile_resources(
        &["./ui"],
        "./ui/resources.gresource.xml",
        "ui.gresource",
    );
}


