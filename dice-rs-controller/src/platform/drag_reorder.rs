use gtk4::DragSource;
use gtk4::DropTarget;
use gtk4::gdk;
use gtk4::gdk::DragAction;
use gtk4::glib;
use gtk4::prelude::*;

/// Sets up drag-and-drop reordering on a container widget.
///
/// The container must be a child of a `gtk4::Box`. Dragging the container
/// and dropping it onto another sibling reorders the children within the
/// parent box. Uses `reorder_child_after` which moves widgets without
/// remove/append (safe for `GLArea`/`Dice3D` widgets).
pub fn setup_drag_reorder(container: &gtk4::Box) {
    // DragSource: provides the container's index within its parent Box.
    let drag_source = DragSource::builder().actions(DragAction::MOVE).build();

    let drag_container = container.clone();
    drag_source.connect_prepare(move |_source, _x, _y| {
        let parent = drag_container.parent()?;
        let parent = parent.downcast::<gtk4::Box>().ok()?;
        let mut index = 0i32;
        let mut child = parent.first_child();
        while let Some(c) = child {
            if c == drag_container {
                let value = glib::Value::from(index);
                return Some(gdk::ContentProvider::for_value(&value));
            }
            index += 1;
            child = c.next_sibling();
        }
        None
    });

    container.add_controller(drag_source);

    // DropTarget: accepts a source index and reorders within the parent Box.
    let drop_target = DropTarget::new(glib::Type::I32, DragAction::MOVE);

    let drop_container = container.clone();
    drop_target.connect_motion(|_target, _x, _y| DragAction::MOVE);

    drop_target.connect_drop(move |_target, value, _x, _y| {
        let Ok(source_index) = value.get::<i32>() else {
            return false;
        };

        let Some(parent) = drop_container.parent().and_then(|p| p.downcast::<gtk4::Box>().ok()) else {
            return false;
        };

        // Find target index by iterating siblings.
        let mut target_index = 0i32;
        let mut child = parent.first_child();
        let mut found = false;
        while let Some(c) = child {
            if c == drop_container {
                found = true;
                break;
            }
            target_index += 1;
            child = c.next_sibling();
        }
        if !found || source_index == target_index {
            return false;
        }

        // Defer reorder to idle callback.
        let parent = parent.clone();
        glib::idle_add_local_once(move || {
            // Collect all children to find source and target widgets.
            let mut children: Vec<gtk4::Widget> = Vec::new();
            let mut child = parent.first_child();
            while let Some(c) = child {
                children.push(c.clone());
                child = c.next_sibling();
            }

            let source = source_index as usize;
            let target = target_index as usize;
            if source >= children.len() || target >= children.len() || source == target {
                return;
            }

            let source_widget = &children[source];
            if source_index < target_index {
                // Moving down: insert after the target widget.
                let sibling = &children[target];
                parent.reorder_child_after(source_widget, Some(sibling));
            } else {
                // Moving up: insert after the widget before target,
                // or prepend if target is 0.
                if target_index == 0 {
                    parent.reorder_child_after(source_widget, None::<&gtk4::Widget>);
                } else {
                    let sibling = &children[target - 1];
                    parent.reorder_child_after(source_widget, Some(sibling));
                }
            }
        });

        true
    });

    container.add_controller(drop_target);
}
