mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct KernelPackageRow(ObjectSubclass<imp::KernelPackageRow>)
        @extends adw::ExpanderRow, gtk::Widget, gtk::ListBoxRow, adw::PreferencesRow,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl KernelPackageRow {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
// ANCHOR_END: mod

impl Default for KernelPackageRow {
    fn default() -> Self {
        Self::new()
    }
}
