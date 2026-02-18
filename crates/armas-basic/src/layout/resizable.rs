//! Resizable Panel Component (shadcn/ui style)
//!
//! Resizable panel groups with draggable handles between panels.
//!
//! ```rust,no_run
//! # use egui::Ui;
//! # fn example(ui: &mut Ui) {
//! use armas_basic::prelude::*;
//!
//! let panels = vec![
//!     ResizablePanel::new(0.25),
//!     ResizablePanel::new(0.75),
//! ];
//! let mut resizable = Resizable::new("split", ResizableDirection::Horizontal);
//! resizable.show(ui, &panels, |ui, index| {
//!     ui.label(format!("Panel {}", index + 1));
//! });
//! # }
//! ```

use egui::{vec2, CursorIcon, Id, Pos2, Rect, Sense, Ui};

// Constants
const HANDLE_SIZE: f32 = 4.0;
const HANDLE_HIT_SIZE: f32 = 8.0; // Larger hit area for easier grabbing
const GRIP_DOT_SIZE: f32 = 2.0;
const GRIP_DOT_GAP: f32 = 3.0;

/// Direction of the resizable panel group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizableDirection {
    /// Panels are arranged left to right, handle is vertical.
    Horizontal,
    /// Panels are arranged top to bottom, handle is horizontal.
    Vertical,
}

/// Configuration for a single panel in a resizable group.
#[derive(Debug, Clone, Copy)]
pub struct ResizablePanel {
    /// Default size as a fraction (0.0..1.0).
    pub default_size: f32,
    /// Minimum size fraction.
    pub min_size: Option<f32>,
    /// Maximum size fraction.
    pub max_size: Option<f32>,
}

impl ResizablePanel {
    /// Create a new panel with a default size fraction.
    #[must_use]
    pub const fn new(default_size: f32) -> Self {
        Self {
            default_size,
            min_size: None,
            max_size: None,
        }
    }

    /// Set the minimum size fraction.
    #[must_use]
    pub const fn min_size(mut self, min: f32) -> Self {
        self.min_size = Some(min);
        self
    }

    /// Set the maximum size fraction.
    #[must_use]
    pub const fn max_size(mut self, max: f32) -> Self {
        self.max_size = Some(max);
        self
    }
}

/// Resizable panel group — panels separated by draggable handles.
pub struct Resizable {
    id: Id,
    direction: ResizableDirection,
}

/// Response from a resizable panel group.
pub struct ResizableResponse {
    /// The UI response.
    pub response: egui::Response,
    /// Current panel sizes as fractions.
    pub sizes: Vec<f32>,
    /// Whether any handle was dragged this frame.
    pub changed: bool,
}

impl Resizable {
    /// Create a new resizable panel group.
    pub fn new(id: impl Into<Id>, direction: ResizableDirection) -> Self {
        Self {
            id: id.into(),
            direction,
        }
    }

    /// Show the resizable panel group.
    pub fn show(
        &mut self,
        ui: &mut Ui,
        panels: &[ResizablePanel],
        mut content: impl FnMut(&mut Ui, usize),
    ) -> ResizableResponse {
        let theme = crate::ext::ArmasContextExt::armas_theme(ui.ctx());

        if panels.is_empty() {
            let (_, response) = ui.allocate_exact_size(vec2(0.0, 0.0), Sense::hover());
            return ResizableResponse {
                response,
                sizes: vec![],
                changed: false,
            };
        }

        let is_horizontal = self.direction == ResizableDirection::Horizontal;
        let available = ui.available_rect_before_wrap();

        // Load persisted sizes
        let sizes_id = self.id.with("sizes");
        let mut sizes: Vec<f32> =
            ui.ctx()
                .data_mut(|d| d.get_temp(sizes_id))
                .unwrap_or_else(|| {
                    let defaults: Vec<f32> = panels.iter().map(|p| p.default_size).collect();
                    normalize_sizes(&defaults)
                });

        // Ensure sizes array matches panel count
        if sizes.len() != panels.len() {
            sizes = normalize_sizes(&panels.iter().map(|p| p.default_size).collect::<Vec<_>>());
        }

        let total_main = if is_horizontal {
            available.width()
        } else {
            available.height()
        };
        let handle_count = panels.len().saturating_sub(1);
        let usable_main = total_main - (handle_count as f32 * HANDLE_SIZE);

        // Allocate the full rect
        let (full_rect, full_response) = ui.allocate_exact_size(available.size(), Sense::hover());

        let mut changed = false;

        // Draw panels and handles
        let mut offset = 0.0;
        for i in 0..panels.len() {
            let panel_extent = sizes[i] * usable_main;

            // Panel rect
            let panel_rect = if is_horizontal {
                Rect::from_min_size(
                    Pos2::new(full_rect.left() + offset, full_rect.top()),
                    vec2(panel_extent, full_rect.height()),
                )
            } else {
                Rect::from_min_size(
                    Pos2::new(full_rect.left(), full_rect.top() + offset),
                    vec2(full_rect.width(), panel_extent),
                )
            };

            // Create child UI for panel content
            let mut child_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(panel_rect)
                    .layout(egui::Layout::top_down(egui::Align::LEFT)),
            );
            child_ui.set_clip_rect(panel_rect);
            content(&mut child_ui, i);

            offset += panel_extent;

            // Draw handle between panels (not after the last one)
            if i < panels.len() - 1 {
                let handle_rect = if is_horizontal {
                    Rect::from_min_size(
                        Pos2::new(full_rect.left() + offset, full_rect.top()),
                        vec2(HANDLE_SIZE, full_rect.height()),
                    )
                } else {
                    Rect::from_min_size(
                        Pos2::new(full_rect.left(), full_rect.top() + offset),
                        vec2(full_rect.width(), HANDLE_SIZE),
                    )
                };

                // Larger hit area
                let hit_rect = if is_horizontal {
                    handle_rect.expand2(vec2((HANDLE_HIT_SIZE - HANDLE_SIZE) / 2.0, 0.0))
                } else {
                    handle_rect.expand2(vec2(0.0, (HANDLE_HIT_SIZE - HANDLE_SIZE) / 2.0))
                };

                let handle_id = self.id.with(("handle", i));
                let handle_response = ui.interact(hit_rect, handle_id, Sense::drag());

                // Cursor change
                if handle_response.hovered() || handle_response.dragged() {
                    ui.ctx().set_cursor_icon(if is_horizontal {
                        CursorIcon::ResizeHorizontal
                    } else {
                        CursorIcon::ResizeVertical
                    });
                }

                // Handle visual
                let handle_color = if handle_response.dragged() || handle_response.hovered() {
                    theme.ring()
                } else {
                    theme.border()
                };

                ui.painter().rect_filled(handle_rect, 0.0, handle_color);

                // Grip dots in center
                let center = handle_rect.center();
                if is_horizontal {
                    for dy in [-GRIP_DOT_GAP, 0.0, GRIP_DOT_GAP] {
                        ui.painter().circle_filled(
                            Pos2::new(center.x, center.y + dy),
                            GRIP_DOT_SIZE / 2.0,
                            theme.muted_foreground(),
                        );
                    }
                } else {
                    for dx in [-GRIP_DOT_GAP, 0.0, GRIP_DOT_GAP] {
                        ui.painter().circle_filled(
                            Pos2::new(center.x + dx, center.y),
                            GRIP_DOT_SIZE / 2.0,
                            theme.muted_foreground(),
                        );
                    }
                }

                // Handle drag
                if handle_response.dragged() {
                    let delta = if is_horizontal {
                        handle_response.drag_delta().x
                    } else {
                        handle_response.drag_delta().y
                    };

                    let delta_frac = delta / usable_main;

                    // Redistribute between adjacent panels
                    let new_left = sizes[i] + delta_frac;
                    let new_right = sizes[i + 1] - delta_frac;

                    // Apply min/max constraints
                    let min_left = panels[i].min_size.unwrap_or(0.05);
                    let max_left = panels[i].max_size.unwrap_or(0.95);
                    let min_right = panels[i + 1].min_size.unwrap_or(0.05);
                    let max_right = panels[i + 1].max_size.unwrap_or(0.95);

                    if new_left >= min_left
                        && new_left <= max_left
                        && new_right >= min_right
                        && new_right <= max_right
                    {
                        sizes[i] = new_left;
                        sizes[i + 1] = new_right;
                        changed = true;
                    }
                }

                offset += HANDLE_SIZE;
            }
        }

        // Save state
        let sizes_clone = sizes.clone();
        ui.ctx().data_mut(|d| d.insert_temp(sizes_id, sizes_clone));

        ResizableResponse {
            response: full_response,
            sizes,
            changed,
        }
    }
}

/// Normalize sizes so they sum to 1.0.
fn normalize_sizes(sizes: &[f32]) -> Vec<f32> {
    let sum: f32 = sizes.iter().sum();
    if sum <= 0.0 {
        let n = sizes.len();
        return vec![1.0 / n as f32; n];
    }
    sizes.iter().map(|s| s / sum).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_sizes() {
        let sizes = normalize_sizes(&[0.25, 0.75]);
        assert!((sizes[0] - 0.25).abs() < f32::EPSILON);
        assert!((sizes[1] - 0.75).abs() < f32::EPSILON);
    }

    #[test]
    fn test_normalize_sizes_unequal() {
        let sizes = normalize_sizes(&[1.0, 2.0, 1.0]);
        assert!((sizes[0] - 0.25).abs() < f32::EPSILON);
        assert!((sizes[1] - 0.5).abs() < f32::EPSILON);
        assert!((sizes[2] - 0.25).abs() < f32::EPSILON);
    }

    #[test]
    fn test_resizable_panel_builder() {
        let panel = ResizablePanel::new(0.5).min_size(0.2).max_size(0.8);
        assert_eq!(panel.default_size, 0.5);
        assert_eq!(panel.min_size, Some(0.2));
        assert_eq!(panel.max_size, Some(0.8));
    }
}
