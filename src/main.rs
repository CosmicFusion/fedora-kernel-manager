mod build_ui;
mod content;
mod kernel_package_row;
mod kernel_pkg;
mod sched_ext;

use std::fs;
use rust_i18n::Backend;
use std::collections::HashMap;
use std::env;
use adw::prelude::*;
use gtk::{gio,gdk,CssProvider};

use crate::gdk::Display;

pub struct I18nBackend {
    trs: HashMap<String, HashMap<String, String>>,
}
impl I18nBackend {
    fn new() -> Self {
        let mut trs = HashMap::new();
        let locales_dir = fs::read_dir("/usr/lib/fedora-kernel-manager/locales").expect("No translation files found");
        for locale_file in locales_dir {
            let locale_file_path = locale_file.expect("couldn't change dir entry to path").path();
            let locale = String::from(locale_file_path.file_name().unwrap().to_str().unwrap().trim_end_matches(".json"));
            let locale_data = fs::read_to_string(locale_file_path).expect(format!("invalid json for {}", locale).as_str());
            let locale_json = serde_json::from_str::<HashMap<String, String>>(&locale_data).unwrap();
            trs.insert(locale.to_string(), locale_json);
        }

        return Self {
            trs
        };
    }
}

impl Backend for I18nBackend {
    fn available_locales(&self) -> Vec<&str> {
        return self.trs.keys().map(|k| k.as_str()).collect();
    }

    fn translate(&self, locale: &str, key: &str) -> Option<&str> {
        return self.trs.get(locale)?.get(key).map(|k| k.as_str());
    }
}

#[macro_use]
extern crate rust_i18n;
i18n!(fallback = "en_US", backend = I18nBackend::new());

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

#[allow(dead_code)]
#[derive(Clone)]
struct KernelBranch {
    name: String,
    db_url: String,
    db: String,
    init_script: String,
}

fn main() -> glib::ExitCode {
    let current_locale = match env::var_os("LANG") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$LANG is not set"),
    };
    rust_i18n::set_locale(current_locale.strip_suffix(".UTF-8").unwrap());

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
