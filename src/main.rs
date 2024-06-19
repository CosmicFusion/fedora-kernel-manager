mod build_ui;
mod content;
mod kernel_package_row;
mod kernel_pkg;
mod sched_ext;

use adw::prelude::*;
use gtk::*;

use crate::gdk::Display;

const APP_ID: &str = "com.github.cosmicfusion.fedora-kernel-manager";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_ICON: &str = "com.github.cosmicfusion.fedora-kernel-manager";
pub const APP_GITHUB: &str = "https://github.com/CosmicFusion/fedora-kernel-manager";

#[derive(Clone)]
struct RunningKernelInfo {
    kernel: String,
    version: String,
    sched: String,
}

#[derive(Clone)]
struct KernelBranch {
    name: String,
    db_url: String,
    db: String,
    init_script: String
}

fn main() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_startup(|app| {
        load_gresource();
        load_css();
        app.connect_activate(build_ui::build_ui);
    });

    // Run the application
    app.run()
}

fn load_gresource() {
    gio::resources_register_include!("data.gresource").expect("Failed to register resources.");
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_resource("/com/github/cosmicfusion/fedora-kernel-manager/css/style.css");

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
