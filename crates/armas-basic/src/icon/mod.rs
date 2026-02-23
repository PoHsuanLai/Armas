//! Icon system for Armas
//!
//! Re-exports the generic icon infrastructure from [`armas_icon`] and provides
//! procedural drawing functions for the small set of icons used by built-in
//! components (close, chevrons, info, error).
//!
//! For custom icons, use [`armas_icon::Icon`] with your own [`armas_icon::IconData`]
//! or parse SVGs at runtime with the `runtime` feature of `armas_icon`.

// Re-export the generic icon infrastructure
pub use armas_icon::{render_icon, render_icon_data, Icon, IconData, OwnedIconData};

use egui::{Color32, Painter, Pos2, Rect, Stroke};

/// Draw a close icon (X) within the given rect.
pub fn draw_close(painter: &Painter, rect: Rect, color: Color32) {
    let stroke = Stroke::new(1.5, color);
    let m = rect.shrink(rect.width() * 0.25);
    painter.line_segment([m.left_top(), m.right_bottom()], stroke);
    painter.line_segment([m.right_top(), m.left_bottom()], stroke);
}

/// Draw a chevron-left icon (<) within the given rect.
pub fn draw_chevron_left(painter: &Painter, rect: Rect, color: Color32) {
    let stroke = Stroke::new(1.5, color);
    let cx = rect.center().x;
    let cy = rect.center().y;
    let half = rect.width() * 0.2;
    painter.line_segment(
        [
            Pos2::new(cx + half, cy - half * 2.0),
            Pos2::new(cx - half, cy),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(cx - half, cy),
            Pos2::new(cx + half, cy + half * 2.0),
        ],
        stroke,
    );
}

/// Draw a chevron-right icon (>) within the given rect.
pub fn draw_chevron_right(painter: &Painter, rect: Rect, color: Color32) {
    let stroke = Stroke::new(1.5, color);
    let cx = rect.center().x;
    let cy = rect.center().y;
    let half = rect.width() * 0.2;
    painter.line_segment(
        [
            Pos2::new(cx - half, cy - half * 2.0),
            Pos2::new(cx + half, cy),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(cx + half, cy),
            Pos2::new(cx - half, cy + half * 2.0),
        ],
        stroke,
    );
}

/// Draw a chevron-down icon (v) within the given rect.
pub fn draw_chevron_down(painter: &Painter, rect: Rect, color: Color32) {
    let stroke = Stroke::new(1.5, color);
    let cx = rect.center().x;
    let cy = rect.center().y;
    let half = rect.height() * 0.2;
    painter.line_segment(
        [
            Pos2::new(cx - half * 2.0, cy - half),
            Pos2::new(cx, cy + half),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(cx, cy + half),
            Pos2::new(cx + half * 2.0, cy - half),
        ],
        stroke,
    );
}

/// Draw an info icon (circle with "i") within the given rect.
pub fn draw_info(painter: &Painter, rect: Rect, color: Color32) {
    let center = rect.center();
    let r = rect.width().min(rect.height()) * 0.45;
    let stroke = Stroke::new(1.5, color);

    // Circle
    painter.circle_stroke(center, r, stroke);

    // Dot above the line
    let dot_y = center.y - r * 0.3;
    painter.circle_filled(Pos2::new(center.x, dot_y), 1.2, color);

    // Vertical line of "i"
    let line_top = center.y;
    let line_bottom = center.y + r * 0.45;
    painter.line_segment(
        [
            Pos2::new(center.x, line_top),
            Pos2::new(center.x, line_bottom),
        ],
        stroke,
    );
}

/// Draw an error icon (circle with "!") within the given rect.
pub fn draw_error(painter: &Painter, rect: Rect, color: Color32) {
    let center = rect.center();
    let r = rect.width().min(rect.height()) * 0.45;
    let stroke = Stroke::new(1.5, color);

    // Circle
    painter.circle_stroke(center, r, stroke);

    // Vertical line of "!"
    let line_top = center.y - r * 0.45;
    let line_bottom = center.y + r * 0.1;
    painter.line_segment(
        [
            Pos2::new(center.x, line_top),
            Pos2::new(center.x, line_bottom),
        ],
        stroke,
    );

    // Dot below the line
    let dot_y = center.y + r * 0.4;
    painter.circle_filled(Pos2::new(center.x, dot_y), 1.2, color);
}
