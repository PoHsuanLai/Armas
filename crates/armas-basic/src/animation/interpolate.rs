use egui::{Color32, Pos2, Rect, Vec2};

/// Trait for types that can be interpolated between two values
pub trait Interpolate: Clone {
    /// Interpolate between self and other by factor t (0.0 to 1.0)
    #[must_use]
    fn interpolate(&self, other: &Self, t: f32) -> Self;
}

// Implementation for f32
impl Interpolate for f32 {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        self + (other - self) * t
    }
}

// Implementation for f64
impl Interpolate for f64 {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        self + (other - self) * Self::from(t)
    }
}

// Implementation for Vec2
impl Interpolate for Vec2 {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        Self::new(
            self.x.interpolate(&other.x, t),
            self.y.interpolate(&other.y, t),
        )
    }
}

// Implementation for Pos2
impl Interpolate for Pos2 {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        Self::new(
            self.x.interpolate(&other.x, t),
            self.y.interpolate(&other.y, t),
        )
    }
}

// Implementation for Rect
impl Interpolate for Rect {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        Self {
            min: self.min.interpolate(&other.min, t),
            max: self.max.interpolate(&other.max, t),
        }
    }
}

// Implementation for Color32
impl Interpolate for Color32 {
    #[allow(clippy::many_single_char_names)]
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        let r = interpolate_u8(self.r(), other.r(), t);
        let g = interpolate_u8(self.g(), other.g(), t);
        let b = interpolate_u8(self.b(), other.b(), t);
        let a = interpolate_u8(self.a(), other.a(), t);
        Self::from_rgba_premultiplied(r, g, b, a)
    }
}

// Helper function to interpolate u8 values
fn interpolate_u8(a: u8, b: u8, t: f32) -> u8 {
    let a = f32::from(a);
    let b = f32::from(b);
    (a + (b - a) * t).round().clamp(0.0, 255.0) as u8
}

// Implementation for tuples (useful for multiple values)
impl<A: Interpolate, B: Interpolate> Interpolate for (A, B) {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        (
            self.0.interpolate(&other.0, t),
            self.1.interpolate(&other.1, t),
        )
    }
}

impl<A: Interpolate, B: Interpolate, C: Interpolate> Interpolate for (A, B, C) {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        (
            self.0.interpolate(&other.0, t),
            self.1.interpolate(&other.1, t),
            self.2.interpolate(&other.2, t),
        )
    }
}

// Implementation for Option (interpolate if both are Some)
impl<T: Interpolate> Interpolate for Option<T> {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        match (self, other) {
            (Some(a), Some(b)) => Some(a.interpolate(b, t)),
            (Some(_), None) if t < 0.5 => self.clone(),
            (None, Some(_)) if t >= 0.5 => other.clone(),
            _ => None,
        }
    }
}

/// Helper function for smooth interpolation with custom easing
pub fn lerp<T: Interpolate>(a: &T, b: &T, t: f32) -> T {
    a.interpolate(b, t)
}

/// Smooth step interpolation (S-curve)
#[must_use] 
pub fn smooth_step(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Smoother step interpolation (smoother S-curve)
#[must_use] 
pub fn smoother_step(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f32_interpolate() {
        assert_eq!(0.0f32.interpolate(&10.0, 0.0), 0.0);
        assert_eq!(0.0f32.interpolate(&10.0, 0.5), 5.0);
        assert_eq!(0.0f32.interpolate(&10.0, 1.0), 10.0);
    }

    #[test]
    fn test_vec2_interpolate() {
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(10.0, 20.0);

        let mid = a.interpolate(&b, 0.5);
        assert_eq!(mid.x, 5.0);
        assert_eq!(mid.y, 10.0);
    }

    #[test]
    fn test_color_interpolate() {
        let black = Color32::BLACK;
        let white = Color32::WHITE;

        let gray = black.interpolate(&white, 0.5);
        assert!(gray.r() > 100 && gray.r() < 155);
    }

    #[test]
    fn test_smooth_step() {
        assert_eq!(smooth_step(0.0), 0.0);
        assert_eq!(smooth_step(1.0), 1.0);
        assert!(smooth_step(0.5) > 0.4 && smooth_step(0.5) < 0.6);
    }

    #[test]
    fn test_tuple_interpolate() {
        let a = (0.0f32, 0.0f32);
        let b = (10.0f32, 20.0f32);

        let mid = a.interpolate(&b, 0.5);
        assert_eq!(mid, (5.0, 10.0));
    }
}
