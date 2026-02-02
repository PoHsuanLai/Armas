//! Generic Icon System for egui
//!
//! Provides a generic SVG-based icon system with:
//! - Static icon data via [`IconData`]
//! - Runtime SVG parsing via the `runtime` feature
//! - Rendering with dynamic colors and sizes
//!
//! # Architecture
//!
//! - [`IconData`] - Pre-tessellated icon geometry (static references)
//! - [`OwnedIconData`] - Runtime-parsed icon geometry (owned buffers)
//! - [`Icon`] - Generic icon widget (works with both)
//! - [`render_icon`] / [`render_icon_data`] - Low-level rendering
//!
//! # Example
//!
//! ```rust,no_run
//! use armas_icon::{Icon, IconData};
//! use egui::{Color32, Ui};
//!
//! static MY_ICON: IconData = IconData {
//!     name: "my_icon",
//!     vertices: &[(0.0, 0.0), (24.0, 12.0), (0.0, 24.0)],
//!     indices: &[0, 1, 2],
//!     viewbox_width: 24.0,
//!     viewbox_height: 24.0,
//! };
//!
//! fn show_icon(ui: &mut Ui) {
//!     Icon::new(&MY_ICON)
//!         .size(24.0)
//!         .color(Color32::WHITE)
//!         .show(ui);
//! }
//! ```

#[cfg(feature = "runtime")]
mod tessellate;

#[cfg(feature = "runtime")]
pub mod runtime;

use egui::{epaint::Vertex, Color32, Mesh, Painter, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Pre-tessellated icon data
///
/// Contains the geometry data for rendering an icon. This is typically
/// generated at compile time from SVG files using the build script.
#[derive(Debug, Clone)]
pub struct IconData {
    /// Icon name (for debugging)
    pub name: &'static str,
    /// Vertex positions as (x, y) tuples
    pub vertices: &'static [(f32, f32)],
    /// Triangle indices
    pub indices: &'static [u32],
    /// Original viewbox width
    pub viewbox_width: f32,
    /// Original viewbox height
    pub viewbox_height: f32,
}

/// Icon data that owns its buffers.
///
/// Runtime counterpart to [`IconData`]. Use this when icon geometry is
/// produced at runtime (e.g. from [`runtime::parse_svg`]).
#[derive(Debug, Clone)]
pub struct OwnedIconData {
    /// Icon name
    pub name: String,
    /// Vertex positions as (x, y) tuples
    pub vertices: Vec<(f32, f32)>,
    /// Triangle indices
    pub indices: Vec<u32>,
    /// Original viewbox width
    pub viewbox_width: f32,
    /// Original viewbox height
    pub viewbox_height: f32,
}

/// Render icon geometry to an egui painter.
///
/// Low-level function that takes raw vertex/index slices. Both
/// [`render_icon`] and [`OwnedIconData::render`] delegate to this.
pub fn render_icon_data(
    painter: &Painter,
    rect: Rect,
    vertices: &[(f32, f32)],
    indices: &[u32],
    viewbox_width: f32,
    viewbox_height: f32,
    color: Color32,
) {
    let scale_x = rect.width() / viewbox_width;
    let scale_y = rect.height() / viewbox_height;
    let scale = scale_x.min(scale_y);

    let offset_x = rect.left() + viewbox_width.mul_add(-scale, rect.width()) / 2.0;
    let offset_y = rect.top() + viewbox_height.mul_add(-scale, rect.height()) / 2.0;

    let mut mesh = Mesh::default();

    for &(x, y) in vertices {
        let pos = Pos2::new(offset_x + x * scale, offset_y + y * scale);
        mesh.vertices.push(Vertex {
            pos,
            uv: Pos2::ZERO,
            color,
        });
    }

    mesh.indices.extend_from_slice(indices);

    painter.add(mesh);
}

/// Render icon data to an egui painter.
///
/// Transforms and renders the pre-tessellated icon geometry to fit
/// within the given rectangle, maintaining aspect ratio and centering.
pub fn render_icon(painter: &Painter, rect: Rect, icon_data: &IconData, color: Color32) {
    render_icon_data(
        painter,
        rect,
        icon_data.vertices,
        icon_data.indices,
        icon_data.viewbox_width,
        icon_data.viewbox_height,
        color,
    );
}

impl OwnedIconData {
    /// Render this icon to an egui painter.
    pub fn render(&self, painter: &Painter, rect: Rect, color: Color32) {
        render_icon_data(
            painter,
            rect,
            &self.vertices,
            &self.indices,
            self.viewbox_width,
            self.viewbox_height,
            color,
        );
    }
}

/// Generic icon widget
///
/// Renders any [`IconData`] with configurable size and color.
///
/// # Example
///
/// ```rust,no_run
/// # use armas_icon::{Icon, IconData};
/// # use egui::{Color32, Ui};
/// # static MY_ICON: IconData = IconData {
/// #     name: "test", vertices: &[], indices: &[],
/// #     viewbox_width: 24.0, viewbox_height: 24.0,
/// # };
/// # fn example(ui: &mut Ui) {
/// Icon::new(&MY_ICON)
///     .size(32.0)
///     .color(Color32::RED)
///     .show(ui);
/// # }
/// ```
pub struct Icon<'a> {
    vertices: &'a [(f32, f32)],
    indices: &'a [u32],
    viewbox_width: f32,
    viewbox_height: f32,
    size: f32,
    color: Color32,
}

impl<'a> Icon<'a> {
    /// Create a new icon widget from static [`IconData`].
    #[must_use]
    pub const fn new(icon_data: &'a IconData) -> Self {
        Self {
            vertices: icon_data.vertices,
            indices: icon_data.indices,
            viewbox_width: icon_data.viewbox_width,
            viewbox_height: icon_data.viewbox_height,
            size: 24.0,
            color: Color32::WHITE,
        }
    }

    /// Create a new icon widget from [`OwnedIconData`].
    #[must_use]
    pub fn from_owned(data: &'a OwnedIconData) -> Self {
        Self {
            vertices: &data.vertices,
            indices: &data.indices,
            viewbox_width: data.viewbox_width,
            viewbox_height: data.viewbox_height,
            size: 24.0,
            color: Color32::WHITE,
        }
    }

    /// Set the icon size (width and height will be equal)
    #[must_use]
    pub const fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the icon color
    #[must_use]
    pub const fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    /// Show the icon
    pub fn show(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(self.size), Sense::click());

        if ui.is_rect_visible(rect) {
            if self.vertices.is_empty() {
                ui.painter().rect_filled(rect, 2.0, Color32::from_gray(100));
            } else {
                render_icon_data(
                    ui.painter(),
                    rect,
                    self.vertices,
                    self.indices,
                    self.viewbox_width,
                    self.viewbox_height,
                    self.color,
                );
            }
        }

        response
    }
}
