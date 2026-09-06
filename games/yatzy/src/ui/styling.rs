use gtk4::CssProvider;

/// Load and apply the application CSS to the default display.
///
/// Should be called once during application startup (e.g. from the
/// `startup` signal handler).
pub fn load_css() {
    let css = include_str!("../../resources/style.css");
    let provider = CssProvider::new();
    provider.load_from_data(css);
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
}
