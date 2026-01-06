pub mod easing;
pub mod interpolate;

pub use easing::EasingFunction;
pub use interpolate::Interpolate;

/// Animation state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationState {
    /// Animation hasn't started yet
    NotStarted,
    /// Animation is currently running
    Running,
    /// Animation is paused
    Paused,
    /// Animation has completed
    Completed,
}

/// A generic animation that interpolates between two values over time
#[derive(Debug, Clone)]
pub struct Animation<T: Interpolate> {
    /// Starting value
    pub start: T,
    /// Target value
    pub end: T,
    /// Duration of the animation in seconds
    pub duration: f32,
    /// Elapsed time in seconds
    pub elapsed: f32,
    /// Easing function to apply
    pub easing: EasingFunction,
    /// Current state of the animation
    pub state: AnimationState,
}

impl<T: Interpolate> Animation<T> {
    /// Create a new animation from start to end over duration
    pub fn new(start: T, end: T, duration: f32) -> Self {
        Self {
            start,
            end,
            duration,
            elapsed: 0.0,
            easing: EasingFunction::EaseInOut,
            state: AnimationState::NotStarted,
        }
    }

    /// Set the easing function
    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }

    /// Start the animation
    pub fn start(&mut self) {
        self.state = AnimationState::Running;
        self.elapsed = 0.0;
    }

    /// Pause the animation
    pub fn pause(&mut self) {
        if self.state == AnimationState::Running {
            self.state = AnimationState::Paused;
        }
    }

    /// Resume the animation
    pub fn resume(&mut self) {
        if self.state == AnimationState::Paused {
            self.state = AnimationState::Running;
        }
    }

    /// Reset the animation to the beginning
    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.state = AnimationState::NotStarted;
    }

    /// Update the animation with delta time
    pub fn update(&mut self, dt: f32) {
        if self.state != AnimationState::Running {
            return;
        }

        self.elapsed += dt;
        if self.elapsed >= self.duration {
            self.elapsed = self.duration;
            self.state = AnimationState::Completed;
        }
    }

    /// Get the current value of the animation
    pub fn value(&self) -> T {
        let t = if self.duration <= 0.0 {
            1.0
        } else {
            (self.elapsed / self.duration).clamp(0.0, 1.0)
        };

        let eased_t = self.easing.apply(t);
        self.start.interpolate(&self.end, eased_t)
    }

    /// Get the normalized progress (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        if self.duration <= 0.0 {
            1.0
        } else {
            (self.elapsed / self.duration).clamp(0.0, 1.0)
        }
    }

    /// Check if the animation is complete
    pub fn is_complete(&self) -> bool {
        self.state == AnimationState::Completed
    }

    /// Check if the animation is running
    pub fn is_running(&self) -> bool {
        self.state == AnimationState::Running
    }
}

/// Spring-based animation for smooth, physics-based motion
///
/// This uses a damped spring physics model for natural-looking animations.
/// Unlike timed animations, springs don't have a fixed duration - they settle over time.
#[derive(Debug, Clone)]
pub struct SpringAnimation {
    /// Current value
    pub value: f32,
    /// Current velocity
    pub velocity: f32,
    /// Target value
    pub target: f32,
    /// Spring stiffness (higher = faster oscillation, typical: 100-300)
    pub stiffness: f32,
    /// Spring damping (higher = less oscillation, typical: 10-30)
    pub damping: f32,
}

impl SpringAnimation {
    /// Create a new spring animation
    pub fn new(initial: f32, target: f32) -> Self {
        Self {
            value: initial,
            velocity: 0.0,
            target,
            stiffness: 200.0,
            damping: 20.0,
        }
    }

    /// Set spring parameters
    pub fn with_params(mut self, stiffness: f32, damping: f32) -> Self {
        self.stiffness = stiffness;
        self.damping = damping;
        self
    }

    /// Update the spring simulation using semi-implicit Euler integration
    pub fn update(&mut self, dt: f32) {
        // Spring force: F = -k * (x - target)
        let spring_force = -self.stiffness * (self.value - self.target);

        // Damping force: F = -c * v
        let damping_force = -self.damping * self.velocity;

        // Total force and acceleration (assuming mass = 1)
        let acceleration = spring_force + damping_force;

        // Semi-implicit Euler integration (more stable than explicit Euler)
        self.velocity += acceleration * dt;
        self.value += self.velocity * dt;
    }

    /// Set a new target value
    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    /// Check if the spring has approximately settled at the target
    pub fn is_settled(&self, position_threshold: f32, velocity_threshold: f32) -> bool {
        let position_error = (self.value - self.target).abs();
        let velocity_mag = self.velocity.abs();

        position_error < position_threshold && velocity_mag < velocity_threshold
    }

    /// Reset the spring to a new position with zero velocity
    pub fn reset(&mut self, value: f32, target: f32) {
        self.value = value;
        self.target = target;
        self.velocity = 0.0;
    }
}
