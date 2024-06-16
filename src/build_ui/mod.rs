use gtk::*;
use gtk::prelude::*;
use adw::*;
use adw::prelude::*;
use crate::{content, PRETTY_NAME};

pub fn build_ui(app: &adw::Application) {
    let window = adw::ApplicationWindow::new(app);

    load_icon_theme(&window);

    window.connect_close_request(move |window| {
        if let Some(application) = window.application() {
            application.remove_window(window);
        }
        glib::Propagation::Proceed
    });

    let window_headerbar = adw::HeaderBar::builder()
        .title_widget(&adw::WindowTitle::builder().title(PRETTY_NAME).build())
        .build();

    let window_bottombar = gtk::Box::builder()
        .hexpand(true)
        .homogeneous(true)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .margin_start(15)
        .build();

    let apply_button = gtk::Button::builder()
        .halign(Align::End)
        .label("Apply Changes")
        .build();

    apply_button.add_css_class("pill");
    apply_button.add_css_class("destructive-action");

    let cancel_button = gtk::Button::builder()
        .halign(Align::Start)
        .label("Cancel Changes")
        .build();

    cancel_button.add_css_class("pill");

    window_bottombar.append(&cancel_button);
    window_bottombar.append(&apply_button);

    let window_toolbar = adw::ToolbarView::builder()
        .content(&content::content())
        .build();

    window_toolbar.add_top_bar(&window_headerbar);
    window_toolbar.add_bottom_bar(&window_bottombar);

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .content(&window_toolbar)
        .width_request(600)
        .height_request(600)
        .resizable(false)
        .build();

    window.present();
}

fn load_icon_theme(window: &adw::ApplicationWindow) {
    let icon_theme = gtk::IconTheme::for_display(&WidgetExt::display(window));

    icon_theme.add_resource_path("/com/github/cosmicfusion/fedora-kernel-manager/icons/");
    icon_theme.add_resource_path("/com/github/cosmicfusion/fedora-kernel-manager/icons/scalable/actions/");
}