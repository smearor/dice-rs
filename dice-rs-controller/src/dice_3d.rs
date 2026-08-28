use std::cell::RefCell;
use std::rc::Rc;

use std::time::Duration;

use dice_rs::model::acceleration::Acceleration;
use dice_rs::model::dice::DiceColor;
use glam::Quat;
use glam::Vec3;
use glow::HasContext;
use gtk4::glib;
use gtk4::prelude::*;
use tracing::error;

use crate::models::model_for_type;
use crate::dice_renderer::DiceRenderer;
use crate::orientation_state::OrientationState;
use dice_rs::model::dice::DiceType;

/// GTK4 widget that renders a 3D dice model using OpenGL (glow + glam).
///
/// Wraps a `gtk4::GLArea` without subclassing - connects to signals directly.
#[derive(Clone)]
pub struct Dice3D {
    gl_area: gtk4::GLArea,
    orientation: Rc<RefCell<OrientationState>>,
    renderer: Rc<RefCell<Option<DiceRenderer>>>,
    dice_type: Rc<RefCell<DiceType>>,
    needs_rebuild: Rc<RefCell<bool>>,
}

impl Dice3D {
    /// Create a new 3D dice widget.
    pub fn new() -> Self {
        let gl_area = gtk4::GLArea::builder().has_depth_buffer(true).auto_render(true).build();

        let widget = Self {
            gl_area,
            orientation: Rc::new(RefCell::new(OrientationState::default())),
            renderer: Rc::new(RefCell::new(None)),
            dice_type: Rc::new(RefCell::new(DiceType::D6)),
            needs_rebuild: Rc::new(RefCell::new(false)),
        };

        widget.connect_signals();
        widget
    }

    /// Set the target orientation from accelerometer data.
    pub fn set_orientation(&self, acceleration: Acceleration) {
        let gravity = Vec3::new(acceleration.x as f32, acceleration.y as f32, acceleration.z as f32);

        if gravity.length() < 0.1 {
            return;
        }

        let gravity = gravity.normalize();
        let target = Quat::from_rotation_arc(gravity, Vec3::Y);

        self.orientation.borrow_mut().target = target;
        self.gl_area.queue_render();
    }

    /// Set the target orientation directly from a quaternion.
    #[allow(dead_code)]
    pub fn set_orientation_quat(&self, quat: Quat) {
        self.orientation.borrow_mut().target = quat;
        self.gl_area.queue_render();
    }

    /// Set the dice color for rendering.
    pub fn set_color(&self, color: DiceColor) {
        let rgb = match color {
            DiceColor::Black => [0.2, 0.2, 0.2],
            DiceColor::Red => [0.8, 0.15, 0.15],
            DiceColor::Green => [0.15, 0.7, 0.2],
            DiceColor::Blue => [0.15, 0.3, 0.85],
            DiceColor::Yellow => [0.9, 0.8, 0.1],
            DiceColor::Orange => [0.9, 0.5, 0.1],
        };
        self.orientation.borrow_mut().color = rgb;
        self.gl_area.queue_render();
    }

    /// Returns the underlying GLArea widget for packing.
    pub fn widget(&self) -> &gtk4::GLArea {
        &self.gl_area
    }

    /// Set the dice type for 3D model selection.
    ///
    /// Defers renderer rebuild to the render callback where the GL context
    /// is current. Dropping OpenGL resources outside the render callback
    /// causes a segfault because the GL context is not current.
    pub fn set_dice_type(&self, dice_type: DiceType) {
        *self.dice_type.borrow_mut() = dice_type;
        *self.needs_rebuild.borrow_mut() = true;
        self.gl_area.queue_render();
    }

    fn connect_signals(&self) {
        let orientation = self.orientation.clone();
        let renderer = self.renderer.clone();
        let dice_type_cell = self.dice_type.clone();
        let needs_rebuild = self.needs_rebuild.clone();
        let gl_area = self.gl_area.clone();

        self.gl_area.connect_render(move |area, _context| {
            // Rebuild renderer if dice type changed (GL context is current here).
            if *needs_rebuild.borrow() {
                *needs_rebuild.borrow_mut() = false;
                *renderer.borrow_mut() = None;
            }

            // Initialize renderer on first render or after dice type change.
            if renderer.borrow().is_none() {
                area.make_current();

                if area.error().is_some() {
                    return gtk4::glib::Propagation::Proceed;
                }

                gl_loader::init_gl();
                let loader = |sym: &str| -> *const std::ffi::c_void { gl_loader::get_proc_address(sym) as *const std::ffi::c_void };

                let glow_ctx = unsafe { glow::Context::from_loader_function(loader) };
                let dt = *dice_type_cell.borrow();
                let (model_impl, is_d10x) = model_for_type(dt);
                let mut model = model_impl.model();
                model.is_d10x = is_d10x;
                match DiceRenderer::new(Rc::new(glow_ctx), &model) {
                    Ok(r) => *renderer.borrow_mut() = Some(r),
                    Err(e) => {
                        error!(error = %miette::Report::new(e), "failed to initialize dice renderer");
                        return gtk4::glib::Propagation::Proceed;
                    }
                }
            }

            let r = renderer.borrow();
            let Some(renderer) = r.as_ref() else {
                return gtk4::glib::Propagation::Proceed;
            };

            let width = area.width() as f32;
            let height = area.height() as f32;
            let aspect = if height > 0.0 { width / height } else { 1.0 };

            // Snap to target orientation, then apply continuous Y-axis spin.
            let mut state = orientation.borrow_mut();
            state.spin_angle += 0.02;
            let spin = Quat::from_rotation_y(state.spin_angle);
            let render_orientation = spin * state.target;
            state.orientation = render_orientation;
            drop(state);

            unsafe {
                renderer.gl.clear_color(0.1, 0.1, 0.12, 1.0);
                renderer.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            }

            renderer
                .render(render_orientation, aspect, [0.95, 0.95, 0.95], orientation.borrow().color)
                .unwrap_or_else(|e| {
                    error!(error = %miette::Report::new(e), "dice render failed");
                });

            gtk4::glib::Propagation::Proceed
        });

        // Drive continuous rendering with a timer for smooth animation.
        let timer_area = gl_area.clone();
        glib::timeout_add_local(Duration::from_millis(16), move || {
            timer_area.queue_render();
            glib::ControlFlow::Continue
        });
    }
}

impl Default for Dice3D {
    fn default() -> Self {
        Self::new()
    }
}
