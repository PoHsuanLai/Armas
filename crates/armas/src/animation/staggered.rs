//! Staggered animations for choreographed entrance effects
//!
//! Provides tools for creating aceternity-style staggered animations where
//! multiple elements animate with sequential delays.

use super::{Animation, AnimationState, EasingFunction, Interpolate};

/// Staggered animation system for multiple items
///
/// Creates a choreographed animation where items animate in sequence
/// with configurable delays between them.
#[derive(Debug, Clone)]
pub struct StaggeredAnimation<T: Interpolate> {
    /// Base delay before first item starts (seconds)
    pub base_delay: f32,
    /// Delay between each item (seconds)
    pub stagger_delay: f32,
    /// Duration of each item's animation (seconds)
    pub duration: f32,
    /// Easing function for each item
    pub easing: EasingFunction,
    /// Total elapsed time
    elapsed: f32,
    /// Number of items
    item_count: usize,
    /// Start and end values
    start: T,
    end: T,
}

impl<T: Interpolate> StaggeredAnimation<T> {
    /// Create a new staggered animation
    pub fn new(start: T, end: T, item_count: usize, stagger_delay: f32, duration: f32) -> Self {
        Self {
            base_delay: 0.0,
            stagger_delay,
            duration,
            easing: EasingFunction::EaseOut,
            elapsed: 0.0,
            item_count,
            start,
            end,
        }
    }

    /// Set the base delay before first item
    pub fn with_base_delay(mut self, delay: f32) -> Self {
        self.base_delay = delay;
        self
    }

    /// Set the easing function
    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }

    /// Update the animation
    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;
    }

    /// Reset the animation
    pub fn reset(&mut self) {
        self.elapsed = 0.0;
    }

    /// Get the value for a specific item index
    pub fn value(&self, index: usize) -> T {
        let item_start_time = self.base_delay + (index as f32 * self.stagger_delay);
        let item_end_time = item_start_time + self.duration;

        if self.elapsed < item_start_time {
            // Not started yet
            return self.start.clone();
        }

        if self.elapsed >= item_end_time {
            // Completed
            return self.end.clone();
        }

        // In progress
        let item_elapsed = self.elapsed - item_start_time;
        let t = (item_elapsed / self.duration).clamp(0.0, 1.0);
        let eased_t = self.easing.apply(t);

        self.start.interpolate(&self.end, eased_t)
    }

    /// Get the progress (0.0 to 1.0) for a specific item
    pub fn progress(&self, index: usize) -> f32 {
        let item_start_time = self.base_delay + (index as f32 * self.stagger_delay);
        let item_end_time = item_start_time + self.duration;

        if self.elapsed < item_start_time {
            0.0
        } else if self.elapsed >= item_end_time {
            1.0
        } else {
            ((self.elapsed - item_start_time) / self.duration).clamp(0.0, 1.0)
        }
    }

    /// Check if all items have completed
    pub fn is_complete(&self) -> bool {
        let last_item_end =
            self.base_delay + ((self.item_count - 1) as f32 * self.stagger_delay) + self.duration;
        self.elapsed >= last_item_end
    }

    /// Get opacity for item (useful for fade-in effects)
    pub fn opacity(&self, index: usize) -> f32 {
        self.progress(index)
    }

    /// Get scale for item (useful for scale-in effects)
    pub fn scale(&self, index: usize) -> f32 {
        let t = self.progress(index);
        0.8 + 0.2 * t // Scale from 0.8 to 1.0
    }

    /// Get Y offset for item (useful for slide-in effects)
    pub fn y_offset(&self, index: usize, distance: f32) -> f32 {
        let t = self.progress(index);
        distance * (1.0 - t)
    }
}

/// Animation sequence that runs animations one after another
#[derive(Debug)]
pub struct AnimationSequence<T: Interpolate> {
    animations: Vec<SequenceStep<T>>,
    current_step: usize,
    elapsed: f32,
}

#[derive(Debug)]
struct SequenceStep<T: Interpolate> {
    delay: f32,
    animation: Animation<T>,
}

impl<T: Interpolate> AnimationSequence<T> {
    /// Create a new empty sequence
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
            current_step: 0,
            elapsed: 0.0,
        }
    }

    /// Add an animation step with optional delay
    pub fn then(mut self, animation: Animation<T>, delay: f32) -> Self {
        self.animations.push(SequenceStep { delay, animation });
        self
    }

    /// Update the sequence
    pub fn update(&mut self, dt: f32) {
        if self.current_step >= self.animations.len() {
            return;
        }

        self.elapsed += dt;
        let step = &mut self.animations[self.current_step];

        // Wait for delay
        if self.elapsed < step.delay {
            return;
        }

        // Update animation
        let animation_dt = self.elapsed - step.delay;
        step.animation.elapsed = animation_dt;

        if animation_dt >= step.animation.duration {
            // Move to next step
            self.current_step += 1;
            self.elapsed = 0.0;
        }
    }

    /// Get the current value
    pub fn value(&self) -> T {
        if self.current_step >= self.animations.len() {
            // Return the last animation's end value
            if let Some(last) = self.animations.last() {
                return last.animation.end.clone();
            }
        }

        if let Some(step) = self.animations.get(self.current_step) {
            step.animation.value()
        } else {
            self.animations[0].animation.start.clone()
        }
    }

    /// Check if the entire sequence is complete
    pub fn is_complete(&self) -> bool {
        self.current_step >= self.animations.len()
    }

    /// Reset the sequence
    pub fn reset(&mut self) {
        self.current_step = 0;
        self.elapsed = 0.0;
        for step in &mut self.animations {
            step.animation.reset();
        }
    }
}

impl<T: Interpolate> Default for AnimationSequence<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Loop mode for repeating animations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopMode {
    /// Play once and stop
    Once,
    /// Loop infinitely
    Loop,
    /// Ping-pong (forward then backward)
    PingPong,
}

/// Animation with loop support
#[derive(Debug, Clone)]
pub struct LoopingAnimation<T: Interpolate> {
    animation: Animation<T>,
    mode: LoopMode,
    forward: bool,
}

impl<T: Interpolate> LoopingAnimation<T> {
    /// Create a new looping animation
    pub fn new(start: T, end: T, duration: f32, mode: LoopMode) -> Self {
        Self {
            animation: Animation::new(start, end, duration),
            mode,
            forward: true,
        }
    }

    /// Set easing function
    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.animation.easing = easing;
        self
    }

    /// Update the animation
    pub fn update(&mut self, dt: f32) {
        self.animation.update(dt);

        if self.animation.is_complete() {
            match self.mode {
                LoopMode::Once => {
                    // Stay at end
                }
                LoopMode::Loop => {
                    // Restart from beginning
                    self.animation.reset();
                    self.animation.start();
                }
                LoopMode::PingPong => {
                    // Reverse direction
                    self.forward = !self.forward;
                    std::mem::swap(&mut self.animation.start, &mut self.animation.end);
                    self.animation.reset();
                    self.animation.start();
                }
            }
        } else if self.animation.state == AnimationState::NotStarted {
            self.animation.start();
        }
    }

    /// Get current value
    pub fn value(&self) -> T {
        self.animation.value()
    }

    /// Get progress
    pub fn progress(&self) -> f32 {
        self.animation.progress()
    }

    /// Reset the animation
    pub fn reset(&mut self) {
        self.animation.reset();
        self.forward = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staggered_animation() {
        let mut anim = StaggeredAnimation::new(0.0_f32, 1.0, 3, 0.1, 0.3);

        // At start, all items should be at 0
        assert_eq!(anim.value(0), 0.0);
        assert_eq!(anim.value(1), 0.0);
        assert_eq!(anim.value(2), 0.0);

        // After 0.15s, first item should be halfway, second just started
        anim.elapsed = 0.15;
        assert!(anim.progress(0) > 0.0);
        assert!(anim.progress(1) < 0.5);
        assert_eq!(anim.progress(2), 0.0);
    }

    #[test]
    fn test_looping_animation() {
        let mut anim = LoopingAnimation::new(0.0_f32, 1.0, 1.0, LoopMode::Loop);

        // First update starts the animation
        anim.update(0.0);
        anim.update(0.5);
        assert!(anim.progress() > 0.0 && anim.progress() < 1.0);

        anim.update(0.6); // Should loop
        assert!(anim.progress() < 0.5);
    }
}
