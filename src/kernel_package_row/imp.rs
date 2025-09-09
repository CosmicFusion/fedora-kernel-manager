use std::{cell::RefCell, sync::OnceLock};

use adw::*;
use adw::{prelude::*, subclass::prelude::*};
use glib::{subclass::Signal, Properties};
use gtk::glib;

// ANCHOR: custom_button
// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::KernelPackageRow)]
pub struct KernelPackageRow {
    #[property(get, set)]
    package: RefCell<String>,
}
// ANCHOR_END: custom_button

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for KernelPackageRow {
    const NAME: &'static str = "KernelPackageRow";
    type Type = super::KernelPackageRow;
    type ParentType = adw::ExpanderRow;
}

// ANCHOR: object_impl
// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for KernelPackageRow {
    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| vec![Signal::builder("row-deleted").build()])
    }
    fn constructed(&self) {
        self.parent_constructed();

        // Bind label to number
        // `SYNC_CREATE` ensures that the label will be immediately set
        let obj = self.obj();

        let basic_expander_row_package_label = gtk::Label::builder().build();

        obj.add_suffix(&basic_expander_row_package_label);

        // Bind label to number
        // `SYNC_CREATE` ensures that the label will be immediately set
        let obj = self.obj();
        obj.bind_property("package", &basic_expander_row_package_label, "label")
            .sync_create()
            .bidirectional()
            .build();
    }
}
// Trait shared by all widgets
impl WidgetImpl for KernelPackageRow {}

// Trait shared by all buttons
// Trait shared by all buttons

impl ListBoxRowImpl for KernelPackageRow {}
impl PreferencesRowImpl for KernelPackageRow {}
impl ExpanderRowImpl for KernelPackageRow {}
