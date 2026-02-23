//! Hover Card Component (shadcn/ui style)
//!
//! A card that appears on hover with configurable open/close delays.
//! Placeholder — implementation follows.

use crate::components::popover::{Popover, PopoverPosition, PopoverStyle};
use egui::{Id, Ui};

/// Hover Card — appears on hover over a trigger element.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ctx: &egui::Context, trigger: &egui::Response) {
/// use armas_basic::components::HoverCard;
///
/// let mut card = HoverCard::new("user_card");
/// card.show(ctx, trigger, |ui| {
///     ui.label("User details here");
/// });
/// # }
/// ```
pub struct HoverCard {
    id: Id,
    open_delay: f32,
    close_delay: f32,
    position: PopoverPosition,
    width: Option<f32>,
}

/// Response from a hover card.
pub struct HoverCardResponse {
    /// The UI response.
    pub response: egui::Response,
    /// Whether the card is currently open.
    pub is_open: bool,
}

impl HoverCard {
    /// Create a new hover card.
    pub fn new(id: impl Into<Id>) -> Self {
        Self {
            id: id.into(),
            open_delay: 0.7,
            close_delay: 0.3,
            position: PopoverPosition::Bottom,
            width: None,
        }
    }

    /// Set the delay before the card opens on hover (seconds).
    #[must_use]
    pub const fn open_delay(mut self, delay: f32) -> Self {
        self.open_delay = delay;
        self
    }

    /// Set the delay before the card closes after hover leaves (seconds).
    #[must_use]
    pub const fn close_delay(mut self, delay: f32) -> Self {
        self.close_delay = delay;
        self
    }

    /// Set the card position relative to the trigger.
    #[must_use]
    pub const fn position(mut self, pos: PopoverPosition) -> Self {
        self.position = pos;
        self
    }

    /// Set the card width.
    #[must_use]
    pub const fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }

    /// Show the hover card. Opens when `trigger` is hovered.
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        trigger: &egui::Response,
        content: impl FnOnce(&mut Ui),
    ) -> HoverCardResponse {
        let theme = crate::ext::ArmasContextExt::armas_theme(ctx);

        let state_id = self.id.with("hover_card_state");
        let hover_start_id = self.id.with("hover_start");
        let leave_start_id = self.id.with("leave_start");

        let mut is_open = ctx.data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
        let mut hover_start: Option<f64> =
            ctx.data_mut(|d| d.get_temp(hover_start_id).unwrap_or(None));
        let mut leave_start: Option<f64> =
            ctx.data_mut(|d| d.get_temp(leave_start_id).unwrap_or(None));

        let now = ctx.input(|i| i.time);
        let trigger_hovered = trigger.hovered();

        // Show popover (needed to check if card itself is hovered)
        let mut popover = Popover::new(self.id.with("popover"))
            .position(self.position)
            .style(PopoverStyle::Default);
        if let Some(w) = self.width {
            popover = popover.width(w);
        }
        popover.set_open(is_open);

        let popover_response = popover.show(ctx, &theme, trigger.rect, |ui| {
            content(ui);
        });

        let card_hovered = popover_response.response.hovered();
        let any_hovered = trigger_hovered || card_hovered;

        if any_hovered {
            // Something is hovered — cancel any pending close
            leave_start = None;

            if !is_open {
                // Start open timer if not already started
                if hover_start.is_none() {
                    hover_start = Some(now);
                }

                // Check if open delay elapsed
                if let Some(start) = hover_start {
                    if now - start >= f64::from(self.open_delay) {
                        is_open = true;
                        hover_start = None;
                    }
                }
            }
        } else {
            // Nothing hovered — cancel any pending open
            hover_start = None;

            if is_open {
                // Start close timer if not already started
                if leave_start.is_none() {
                    leave_start = Some(now);
                }

                // Check if close delay elapsed
                if let Some(start) = leave_start {
                    if now - start >= f64::from(self.close_delay) {
                        is_open = false;
                        leave_start = None;
                    }
                }
            }
        }

        // Request repaint while timers are active
        if hover_start.is_some() || leave_start.is_some() {
            ctx.request_repaint();
        }

        // Save state
        ctx.data_mut(|d| {
            d.insert_temp(state_id, is_open);
            d.insert_temp(hover_start_id, hover_start);
            d.insert_temp(leave_start_id, leave_start);
        });

        HoverCardResponse {
            response: popover_response.response,
            is_open,
        }
    }
}
