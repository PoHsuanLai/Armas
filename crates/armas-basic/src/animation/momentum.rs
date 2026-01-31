//! Momentum-based animation for drag-and-release interactions
//!
//! Provides physics-based momentum that continues movement after a drag ends,
//! with configurable friction and optional snapping to boundaries.

/// Behavior trait for momentum animations
///
/// Implement this to customize how the position moves after release.
pub trait MomentumBehavior {
    /// Called when the drag is released with a velocity (units per second)
    fn released_with_velocity(&mut self, position: f64, velocity: f64);

    /// Get the next position after elapsed time
    /// This should also update internal state (like decaying velocity)
    fn next_position(&mut self, current_position: f64, elapsed_seconds: f64) -> f64;

    /// Check if the animation has stopped
    fn is_stopped(&self, position: f64) -> bool;
}

/// Continuous momentum with friction-based deceleration
///
/// The position continues moving with the release velocity and gradually
/// slows down due to friction. Good for free-scrolling content.
///
/// # Example
/// ```ignore
/// let mut behavior = ContinuousWithMomentum::new();
/// behavior.set_friction(0.08); // Higher = more friction
/// ```
#[derive(Debug, Clone)]
pub struct ContinuousWithMomentum {
    velocity: f64,
    damping: f64,
    minimum_velocity: f64,
}

impl Default for ContinuousWithMomentum {
    fn default() -> Self {
        Self::new()
    }
}

impl ContinuousWithMomentum {
    /// Create a new continuous momentum behavior
    #[must_use] 
    pub const fn new() -> Self {
        Self {
            velocity: 0.0,
            damping: 0.92,
            minimum_velocity: 0.05,
        }
    }

    /// Set the friction that damps movement
    ///
    /// Typical values are 0.05-0.15. Higher = more friction = stops faster.
    #[must_use] 
    pub fn friction(mut self, friction: f64) -> Self {
        self.damping = 1.0 - friction.clamp(0.0, 0.99);
        self
    }

    /// Set the minimum velocity threshold
    ///
    /// When velocity drops below this, animation stops. Default is 0.05.
    #[must_use] 
    pub const fn minimum_velocity(mut self, min_vel: f64) -> Self {
        self.minimum_velocity = min_vel.abs();
        self
    }

    /// Get the current velocity
    #[must_use] 
    pub const fn velocity(&self) -> f64 {
        self.velocity
    }
}

impl MomentumBehavior for ContinuousWithMomentum {
    fn released_with_velocity(&mut self, _position: f64, velocity: f64) {
        self.velocity = velocity;
    }

    fn next_position(&mut self, current_position: f64, elapsed_seconds: f64) -> f64 {
        let new_pos = current_position + self.velocity * elapsed_seconds;
        // Decay velocity
        self.velocity *= self.damping;
        if self.velocity.abs() < self.minimum_velocity {
            self.velocity = 0.0;
        }
        new_pos
    }

    fn is_stopped(&self, _position: f64) -> bool {
        self.velocity.abs() < self.minimum_velocity
    }
}

/// Snap-to-page momentum behavior
///
/// When released, the position gravitates toward the nearest integer (page) boundary.
/// Useful for paged content or snapping to grid positions.
///
/// # Example
/// ```ignore
/// let mut behavior = SnapToPageBoundaries::new();
/// // Will snap to nearest integer position after release
/// ```
#[derive(Debug, Clone)]
pub struct SnapToPageBoundaries {
    target_position: f64,
    snap_speed: f64,
}

impl Default for SnapToPageBoundaries {
    fn default() -> Self {
        Self::new()
    }
}

impl SnapToPageBoundaries {
    /// Create a new snap-to-page behavior
    #[must_use] 
    pub const fn new() -> Self {
        Self {
            target_position: 0.0,
            snap_speed: 10.0,
        }
    }

    /// Set the speed at which it snaps to the target
    ///
    /// Higher values = faster snapping. Default is 10.0.
    #[must_use] 
    pub const fn snap_speed(mut self, speed: f64) -> Self {
        self.snap_speed = speed.max(1.0);
        self
    }

    /// Get the target snap position
    #[must_use] 
    pub const fn target(&self) -> f64 {
        self.target_position
    }
}

impl MomentumBehavior for SnapToPageBoundaries {
    fn released_with_velocity(&mut self, position: f64, velocity: f64) {
        // Start by snapping to nearest integer
        self.target_position = position.round();

        // If moving fast enough, snap to next/previous page in that direction
        if velocity > 1.0 && self.target_position < position {
            self.target_position += 1.0;
        }
        if velocity < -1.0 && self.target_position > position {
            self.target_position -= 1.0;
        }
    }

    fn next_position(&mut self, current_position: f64, elapsed_seconds: f64) -> f64 {
        if self.is_stopped(current_position) {
            return self.target_position;
        }

        let velocity = (self.target_position - current_position) * self.snap_speed;
        let new_pos = current_position + velocity * elapsed_seconds;

        // If we've overshot, clamp to target
        if (current_position < self.target_position && new_pos > self.target_position)
            || (current_position > self.target_position && new_pos < self.target_position)
        {
            self.target_position
        } else {
            new_pos
        }
    }

    fn is_stopped(&self, position: f64) -> bool {
        (self.target_position - position).abs() < 0.001
    }
}

/// Animated position with momentum physics
///
/// Models a 1D position that can be dragged and released with momentum.
/// The position continues moving after release based on the configured behavior.
///
/// # Example
/// ```ignore
/// use armas_basic::animation::{MomentumPosition, ContinuousWithMomentum};
///
/// let mut pos = MomentumPosition::new(ContinuousWithMomentum::new().friction(0.08));
/// pos.set_limits(0.0, 100.0);
///
/// // During drag:
/// pos.begin_drag();
/// pos.drag(delta_from_start);
/// pos.end_drag();
///
/// // Each frame:
/// pos.update(dt);
/// let current = pos.position();
/// ```
#[derive(Debug, Clone)]
pub struct MomentumPosition<B: MomentumBehavior> {
    position: f64,
    grabbed_position: f64,
    release_velocity: f64,
    limits: (f64, f64),
    last_drag_time: f64,
    last_drag_position: f64,
    is_dragging: bool,
    is_animating: bool,
    /// The behavior that controls momentum physics
    pub behavior: B,
}

impl<B: MomentumBehavior> MomentumPosition<B> {
    /// Create a new momentum position with the given behavior
    pub const fn new(behavior: B) -> Self {
        Self {
            position: 0.0,
            grabbed_position: 0.0,
            release_velocity: 0.0,
            limits: (f64::MIN, f64::MAX),
            last_drag_time: 0.0,
            last_drag_position: 0.0,
            is_dragging: false,
            is_animating: false,
            behavior,
        }
    }

    /// Set the position limits
    pub const fn set_limits(&mut self, min: f64, max: f64) {
        self.limits = (min, max);
        self.position = self.position.clamp(min, max);
    }

    /// Get current position
    pub const fn position(&self) -> f64 {
        self.position
    }

    /// Set position directly (stops any animation)
    pub const fn set_position(&mut self, position: f64) {
        self.position = position.clamp(self.limits.0, self.limits.1);
        self.is_animating = false;
        self.is_dragging = false;
    }

    /// Check if currently being dragged
    pub const fn is_dragging(&self) -> bool {
        self.is_dragging
    }

    /// Check if momentum animation is active
    pub const fn is_animating(&self) -> bool {
        self.is_animating
    }

    /// Begin a drag operation
    pub const fn begin_drag(&mut self) {
        self.grabbed_position = self.position;
        self.release_velocity = 0.0;
        self.last_drag_time = 0.0;
        self.last_drag_position = self.position;
        self.is_dragging = true;
        self.is_animating = false;
    }

    /// Update position during drag
    ///
    /// `delta` is the total offset from where the drag started
    /// `elapsed_since_last` is time since last `drag()` call (for velocity calculation)
    pub fn drag(&mut self, delta: f64, elapsed_since_last: f64) {
        let new_position = (self.grabbed_position + delta).clamp(self.limits.0, self.limits.1);

        // Calculate velocity for momentum
        if elapsed_since_last > 0.005 {
            let velocity = (new_position - self.last_drag_position) / elapsed_since_last;
            // Only update if significant movement
            if velocity.abs() > 0.2 {
                self.release_velocity = velocity;
            }
            self.last_drag_position = new_position;
            self.last_drag_time = elapsed_since_last;
        }

        self.position = new_position;
    }

    /// End the drag and start momentum animation
    pub fn end_drag(&mut self) {
        if self.is_dragging {
            self.is_dragging = false;
            self.behavior
                .released_with_velocity(self.position, self.release_velocity);
            self.is_animating = true;
        }
    }

    /// Apply a nudge (like from mouse wheel)
    pub fn nudge(&mut self, delta: f64) {
        self.position = (self.position + delta).clamp(self.limits.0, self.limits.1);
        self.behavior.released_with_velocity(self.position, 0.0);
        self.is_animating = true;
    }

    /// Update the animation (call each frame)
    ///
    /// Returns true if the position changed
    pub fn update(&mut self, dt: f64) -> bool {
        if self.is_dragging || !self.is_animating {
            return false;
        }

        let new_position = self.behavior.next_position(self.position, dt);
        let clamped = new_position.clamp(self.limits.0, self.limits.1);

        if self.behavior.is_stopped(clamped) {
            self.is_animating = false;
            if (self.position - clamped).abs() > 0.0001 {
                self.position = clamped;
                return true;
            }
            return false;
        }

        if (self.position - clamped).abs() > 0.0001 {
            self.position = clamped;
            true
        } else {
            false
        }
    }
}
