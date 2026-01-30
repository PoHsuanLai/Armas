//! Velocity-based drag control for fine parameter adjustment
//!
//! Provides a drag mode where faster mouse movement creates larger value changes,
//! allowing for both quick coarse adjustments and precise fine-tuning.

use egui::Modifiers;

/// Configuration for velocity-based dragging
#[derive(Debug, Clone)]
pub struct VelocityDragConfig {
    /// Sensitivity multiplier (higher = more responsive). Default: 1.0
    pub sensitivity: f64,
    /// Minimum pixel movement before registering as drag. Default: 1
    pub threshold: i32,
    /// Offset added to velocity calculation (higher = faster minimum speed). Default: 0.0
    pub offset: f64,
    /// Whether user can toggle velocity mode with modifier key. Default: true
    pub allow_modifier_toggle: bool,
    /// Modifier keys that toggle velocity mode. Default: Ctrl/Cmd
    pub toggle_modifiers: Modifiers,
}

impl Default for VelocityDragConfig {
    fn default() -> Self {
        Self {
            sensitivity: 1.0,
            threshold: 1,
            offset: 0.0,
            allow_modifier_toggle: true,
            toggle_modifiers: Modifiers::COMMAND | Modifiers::CTRL,
        }
    }
}

impl VelocityDragConfig {
    /// Create a new velocity drag configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set sensitivity (higher = more responsive to mouse speed)
    pub fn sensitivity(mut self, sensitivity: f64) -> Self {
        self.sensitivity = sensitivity.max(0.1);
        self
    }

    /// Set the minimum pixel threshold for drag detection
    pub fn threshold(mut self, threshold: i32) -> Self {
        self.threshold = threshold.max(1);
        self
    }

    /// Set the velocity offset (minimum speed)
    pub fn offset(mut self, offset: f64) -> Self {
        self.offset = offset.max(0.0);
        self
    }

    /// Enable/disable modifier key toggle for velocity mode
    pub fn allow_modifier_toggle(mut self, allow: bool) -> Self {
        self.allow_modifier_toggle = allow;
        self
    }

    /// Set which modifier keys toggle velocity mode
    pub fn toggle_modifiers(mut self, modifiers: Modifiers) -> Self {
        self.toggle_modifiers = modifiers;
        self
    }
}

/// Drag mode for parameter adjustment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DragMode {
    /// No drag in progress
    #[default]
    None,
    /// Absolute drag - value jumps to mouse position
    Absolute,
    /// Velocity drag - value changes based on mouse speed
    Velocity,
}

/// State for velocity-based dragging
///
/// This helper manages drag state for sliders, knobs, or any parameter control
/// that benefits from velocity-sensitive adjustment.
///
/// # Example
/// ```ignore
/// use armas::animation::{VelocityDrag, VelocityDragConfig, DragMode};
///
/// let mut drag = VelocityDrag::new(VelocityDragConfig::new().sensitivity(1.5));
///
/// // On mouse down:
/// drag.begin(current_value, mouse_y, use_velocity_mode);
///
/// // On mouse drag:
/// let delta = drag.update(mouse_y, value_range);
/// value += delta;
///
/// // On mouse up:
/// drag.end();
/// ```
#[derive(Debug, Clone)]
pub struct VelocityDrag {
    config: VelocityDragConfig,
    mode: DragMode,
    start_value: f64,
    start_pos: f64,
    last_pos: f64,
    accumulated_delta: f64,
}

impl VelocityDrag {
    /// Create a new velocity drag helper with the given configuration
    pub fn new(config: VelocityDragConfig) -> Self {
        Self {
            config,
            mode: DragMode::None,
            start_value: 0.0,
            start_pos: 0.0,
            last_pos: 0.0,
            accumulated_delta: 0.0,
        }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(VelocityDragConfig::default())
    }

    /// Get the current drag mode
    pub fn mode(&self) -> DragMode {
        self.mode
    }

    /// Check if a drag is in progress
    pub fn is_dragging(&self) -> bool {
        self.mode != DragMode::None
    }

    /// Check if velocity mode is active
    pub fn is_velocity_mode(&self) -> bool {
        self.mode == DragMode::Velocity
    }

    /// Check if absolute mode is active
    pub fn is_absolute_mode(&self) -> bool {
        self.mode == DragMode::Absolute
    }

    /// Begin a drag operation
    ///
    /// - `current_value`: The current parameter value
    /// - `mouse_pos`: Current mouse position (typically Y for vertical, X for horizontal)
    /// - `use_velocity_mode`: Whether to use velocity mode (can be toggled by modifier)
    pub fn begin(&mut self, current_value: f64, mouse_pos: f64, use_velocity_mode: bool) {
        self.start_value = current_value;
        self.start_pos = mouse_pos;
        self.last_pos = mouse_pos;
        self.accumulated_delta = 0.0;
        self.mode = if use_velocity_mode {
            DragMode::Velocity
        } else {
            DragMode::Absolute
        };
    }

    /// Begin drag, automatically choosing mode based on modifier keys
    ///
    /// If `default_velocity_mode` is true, velocity mode is the default and
    /// modifier keys switch to absolute. Otherwise, absolute is default.
    pub fn begin_auto(
        &mut self,
        current_value: f64,
        mouse_pos: f64,
        modifiers: &Modifiers,
        default_velocity_mode: bool,
    ) {
        let modifier_pressed = self.config.allow_modifier_toggle
            && modifiers.matches_logically(self.config.toggle_modifiers);

        let use_velocity = if default_velocity_mode {
            !modifier_pressed
        } else {
            modifier_pressed
        };

        self.begin(current_value, mouse_pos, use_velocity);
    }

    /// Update during drag and return the value delta
    ///
    /// - `mouse_pos`: Current mouse position
    /// - `value_range`: Total range of the parameter (max - min)
    /// - `drag_pixels`: Number of pixels for full range in absolute mode
    ///
    /// Returns the delta to add to the current value
    pub fn update(&mut self, mouse_pos: f64, value_range: f64, drag_pixels: f64) -> f64 {
        if self.mode == DragMode::None {
            return 0.0;
        }

        let pixel_delta = mouse_pos - self.last_pos;
        self.last_pos = mouse_pos;

        // Check threshold
        if pixel_delta.abs() < self.config.threshold as f64 {
            return 0.0;
        }

        match self.mode {
            DragMode::Absolute => {
                // Linear mapping: pixels -> value
                let total_delta = mouse_pos - self.start_pos;
                let value_per_pixel = value_range / drag_pixels;
                let target_value = self.start_value + total_delta * value_per_pixel;
                target_value - (self.start_value + self.accumulated_delta)
            }
            DragMode::Velocity => {
                // Velocity-based: faster movement = larger change
                let speed = pixel_delta.abs();
                let sign = pixel_delta.signum();

                // Apply sensitivity and offset
                let velocity = (speed * self.config.sensitivity + self.config.offset) * sign;

                // Scale to value range (assume ~200px for full range as baseline)
                let value_per_unit = value_range / 200.0;
                velocity * value_per_unit
            }
            DragMode::None => 0.0,
        }
    }

    /// Update and track accumulated delta
    ///
    /// Same as `update()` but also tracks the total change for absolute mode
    pub fn update_tracked(&mut self, mouse_pos: f64, value_range: f64, drag_pixels: f64) -> f64 {
        let delta = self.update(mouse_pos, value_range, drag_pixels);
        self.accumulated_delta += delta;
        delta
    }

    /// End the drag operation
    pub fn end(&mut self) {
        self.mode = DragMode::None;
    }

    /// Get the start value when drag began
    pub fn start_value(&self) -> f64 {
        self.start_value
    }

    /// Get the total accumulated delta since drag began
    pub fn accumulated_delta(&self) -> f64 {
        self.accumulated_delta
    }
}

/// Double-click to reset functionality
#[derive(Debug, Clone)]
pub struct DoubleClickReset {
    /// Whether double-click reset is enabled
    pub enabled: bool,
    /// The default value to reset to
    pub default_value: f64,
    /// Time window for double-click detection (seconds)
    pub double_click_time: f64,
    /// Last click time
    last_click: f64,
}

impl DoubleClickReset {
    /// Create a new double-click reset helper
    pub fn new(default_value: f64) -> Self {
        Self {
            enabled: true,
            default_value,
            double_click_time: 0.3,
            last_click: 0.0,
        }
    }

    /// Enable or disable the feature
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set the default value to reset to
    pub fn default_value(mut self, value: f64) -> Self {
        self.default_value = value;
        self
    }

    /// Set the double-click time window
    pub fn double_click_time(mut self, seconds: f64) -> Self {
        self.double_click_time = seconds.max(0.1);
        self
    }

    /// Handle a click and return true if it was a double-click
    ///
    /// `current_time` should be the current time in seconds
    pub fn on_click(&mut self, current_time: f64) -> bool {
        if !self.enabled {
            return false;
        }

        let is_double_click = (current_time - self.last_click) < self.double_click_time;
        self.last_click = current_time;
        is_double_click
    }

    /// Get the value to reset to
    pub fn reset_value(&self) -> f64 {
        self.default_value
    }
}
