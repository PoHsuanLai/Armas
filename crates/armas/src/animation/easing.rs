//! Easing functions for animations
//!
//! These functions transform a linear time value (0.0 to 1.0) into an eased value
//! that creates more natural-looking animations.

/// Easing function type
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum EasingFunction {
    /// No easing, linear interpolation
    Linear,
    /// Accelerating from zero velocity
    EaseIn,
    /// Decelerating to zero velocity
    EaseOut,
    /// Acceleration until halfway, then deceleration
    #[default]
    EaseInOut,
    /// Quadratic ease in
    QuadIn,
    /// Quadratic ease out
    QuadOut,
    /// Quadratic ease in-out
    QuadInOut,
    /// Cubic ease in
    CubicIn,
    /// Cubic ease out
    CubicOut,
    /// Cubic ease in-out
    CubicInOut,
    /// Exponential ease in
    ExpoIn,
    /// Exponential ease out
    ExpoOut,
    /// Exponential ease in-out
    ExpoInOut,
    /// Elastic ease in (spring effect)
    ElasticIn,
    /// Elastic ease out (spring effect)
    ElasticOut,
    /// Bounce ease out
    BounceOut,
    /// Custom cubic bezier curve
    Cubic {
        /// First control point X
        x1: f32,
        /// First control point Y
        y1: f32,
        /// Second control point X
        x2: f32,
        /// Second control point Y
        y2: f32,
    },
}

impl EasingFunction {
    /// Apply the easing function to a time value (0.0 to 1.0)
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);

        match self {
            Self::Linear => t,
            Self::EaseIn => quad_in(t),
            Self::EaseOut => quad_out(t),
            Self::EaseInOut => quad_in_out(t),
            Self::QuadIn => quad_in(t),
            Self::QuadOut => quad_out(t),
            Self::QuadInOut => quad_in_out(t),
            Self::CubicIn => cubic_in(t),
            Self::CubicOut => cubic_out(t),
            Self::CubicInOut => cubic_in_out(t),
            Self::ExpoIn => expo_in(t),
            Self::ExpoOut => expo_out(t),
            Self::ExpoInOut => expo_in_out(t),
            Self::ElasticIn => elastic_in(t),
            Self::ElasticOut => elastic_out(t),
            Self::BounceOut => bounce_out(t),
            Self::Cubic { x1, y1, x2, y2 } => cubic_bezier(t, *x1, *y1, *x2, *y2),
        }
    }
}

// Quadratic easing functions
fn quad_in(t: f32) -> f32 {
    t * t
}

fn quad_out(t: f32) -> f32 {
    t * (2.0 - t)
}

fn quad_in_out(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

// Cubic easing functions
fn cubic_in(t: f32) -> f32 {
    t * t * t
}

fn cubic_out(t: f32) -> f32 {
    let t = t - 1.0;
    t * t * t + 1.0
}

fn cubic_in_out(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let t = 2.0 * t - 2.0;
        1.0 + t * t * t / 2.0
    }
}

// Exponential easing functions
fn expo_in(t: f32) -> f32 {
    if t == 0.0 {
        0.0
    } else {
        2.0f32.powf(10.0 * (t - 1.0))
    }
}

fn expo_out(t: f32) -> f32 {
    if t == 1.0 {
        1.0
    } else {
        1.0 - 2.0f32.powf(-10.0 * t)
    }
}

fn expo_in_out(t: f32) -> f32 {
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }

    if t < 0.5 {
        2.0f32.powf(20.0 * t - 10.0) / 2.0
    } else {
        (2.0 - 2.0f32.powf(-20.0 * t + 10.0)) / 2.0
    }
}

// Elastic easing (spring effect)
fn elastic_in(t: f32) -> f32 {
    if t == 0.0 || t == 1.0 {
        return t;
    }

    let p = 0.3;
    let s = p / 4.0;
    let t = t - 1.0;

    -(2.0f32.powf(10.0 * t) * ((t - s) * (2.0 * std::f32::consts::PI) / p).sin())
}

fn elastic_out(t: f32) -> f32 {
    if t == 0.0 || t == 1.0 {
        return t;
    }

    let p = 0.3;
    let s = p / 4.0;

    2.0f32.powf(-10.0 * t) * ((t - s) * (2.0 * std::f32::consts::PI) / p).sin() + 1.0
}

// Bounce easing
fn bounce_out(t: f32) -> f32 {
    if t < 1.0 / 2.75 {
        7.5625 * t * t
    } else if t < 2.0 / 2.75 {
        let t = t - 1.5 / 2.75;
        7.5625 * t * t + 0.75
    } else if t < 2.5 / 2.75 {
        let t = t - 2.25 / 2.75;
        7.5625 * t * t + 0.9375
    } else {
        let t = t - 2.625 / 2.75;
        7.5625 * t * t + 0.984375
    }
}

// Cubic bezier using Newton-Raphson to solve for t given x
fn cubic_bezier(t: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    // First, we need to solve for the parameter value that gives us our input t
    // in the x-dimension, then use that to calculate the y value

    // For standard easing curves, x1 and x2 should be in [0, 1]
    let x1 = x1.clamp(0.0, 1.0);
    let x2 = x2.clamp(0.0, 1.0);

    // If the curve is linear in x, we can directly compute y
    if x1 == 0.0 && x2 == 1.0 {
        return t;
    }

    // Use Newton-Raphson to find the t value for the given x
    let mut guess = t;
    for _ in 0..8 {
        // Calculate x for current guess using cubic bezier formula
        let guess_x = cubic_bezier_x(guess, x1, x2);
        let error = guess_x - t;

        if error.abs() < 0.001 {
            break;
        }

        // Calculate derivative
        let slope = cubic_bezier_x_derivative(guess, x1, x2);
        if slope.abs() < 0.000001 {
            break;
        }

        // Newton-Raphson step
        guess -= error / slope;
        guess = guess.clamp(0.0, 1.0);
    }

    // Now calculate y using the found t value
    cubic_bezier_y(guess, y1, y2)
}

// Calculate x coordinate of cubic bezier at parameter t
// B(t) = (1-t)³P₀ + 3(1-t)²tP₁ + 3(1-t)t²P₂ + t³P₃
// Where P₀ = (0,0) and P₃ = (1,1)
fn cubic_bezier_x(t: f32, x1: f32, x2: f32) -> f32 {
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;

    3.0 * mt2 * t * x1 + 3.0 * mt * t2 * x2 + t3
}

// Calculate y coordinate of cubic bezier at parameter t
fn cubic_bezier_y(t: f32, y1: f32, y2: f32) -> f32 {
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;

    3.0 * mt2 * t * y1 + 3.0 * mt * t2 * y2 + t3
}

// Derivative of cubic bezier x with respect to t
fn cubic_bezier_x_derivative(t: f32, x1: f32, x2: f32) -> f32 {
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let t2 = t * t;

    3.0 * mt2 * x1 + 6.0 * mt * t * (x2 - x1) + 3.0 * t2 * (1.0 - x2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear() {
        assert_eq!(EasingFunction::Linear.apply(0.0), 0.0);
        assert_eq!(EasingFunction::Linear.apply(0.5), 0.5);
        assert_eq!(EasingFunction::Linear.apply(1.0), 1.0);
    }

    #[test]
    fn test_ease_in_out_bounds() {
        let result = EasingFunction::EaseInOut.apply(0.0);
        assert!((0.0..=1.0).contains(&result));

        let result = EasingFunction::EaseInOut.apply(1.0);
        assert!((0.0..=1.0).contains(&result));
    }

    #[test]
    fn test_all_easing_functions() {
        let functions = [
            EasingFunction::Linear,
            EasingFunction::EaseIn,
            EasingFunction::EaseOut,
            EasingFunction::EaseInOut,
            EasingFunction::QuadIn,
            EasingFunction::QuadOut,
            EasingFunction::QuadInOut,
            EasingFunction::CubicIn,
            EasingFunction::CubicOut,
            EasingFunction::CubicInOut,
        ];

        for func in &functions {
            // All functions should start at 0 and end at 1
            assert!((func.apply(0.0) - 0.0).abs() < 0.001);
            assert!((func.apply(1.0) - 1.0).abs() < 0.001);

            // All results should be in valid range
            for i in 0..=10 {
                let t = i as f32 / 10.0;
                let result = func.apply(t);
                assert!(
                    (-0.1..=1.1).contains(&result),
                    "Easing {:?} at t={} gave {}",
                    func,
                    t,
                    result
                );
            }
        }
    }
}
