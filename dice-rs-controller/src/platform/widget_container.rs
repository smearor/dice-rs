use gtk4::prelude::*;

/// Trait for composite widgets that expose a root GTK widget.
///
/// Implementors wrap a GTK widget and provide access to it for packing
/// and visibility control. This unifies the interface across all
/// composite widget types in the application.
pub trait WidgetContainer {
    /// Returns the root widget for packing.
    fn widget(&self) -> &gtk4::Widget;

    /// Sets the visibility of the underlying widget.
    fn set_visible(&self, visible: bool) {
        self.widget().set_visible(visible);
    }

    /// Packs this widget into a GTK box container.
    fn pack_into(&self, container: &gtk4::Box) {
        container.append(self.widget());
    }
}
