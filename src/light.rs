#[derive(Debug, Clone)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

pub type Intensity = f32;

fn clamp(x: f32) -> f32 {
    if x.is_nan() {
        0.0
    } else if x < 0.0 {
        0.0
    } else if x > 1.0 {
        1.0
    } else {
        x
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: clamp(r),
            g: clamp(g),
            b: clamp(b),
        }
    }

    pub fn black() -> Self {
        Self {r: 0.0, g: 0.0, b: 0.0}
    }

    pub fn r(&self) -> f32 {
        self.r
    }

    pub fn g(&self) -> f32 {
        self.g
    }

    pub fn b(&self) -> f32 {
        self.b
    }

    pub fn scale(&mut self, strength: f32) {
        self.r = clamp(self.r * strength);
        self.g = clamp(self.g * strength);
        self.b = clamp(self.b * strength);
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Color::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl From<i32> for Color {
    fn from(x: i32) -> Self {
        let r = ((x >> 16) & 0xff) as f32 / 255.0;
        let g = ((x >>  8) & 0xff) as f32 / 255.0;
        let b = ((x >>  0) & 0xff) as f32 / 255.0;
        Color::new(r, g, b)
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from(x: (f32, f32, f32)) -> Self {
        Color::new(clamp(x.0), clamp(x.1), clamp(x.2))
    }
}

#[cfg(test)]
mod tests {
    use std::f32::NAN;
    use super::*;

    #[test]
    fn test_color_creation_limits_values() {
        let c = Color::new(-0.5, 1.5, NAN);
        assert_eq!(0.0, c.r);
        assert_eq!(1.0, c.g);
        assert_eq!(0.0, c.b);
    }

    #[test]
    fn test_create_color_from_u32() {
        let c: Color = 0x00FF80.into();
        assert_eq!(0.0, c.r);
        assert_eq!(1.0, c.g);
        assert_eq!(0.5019608, c.b);
    }

    #[test]
    fn test_adding_colors() {
        let c0 = Color::new(0.1, 0.2, 0.3);
        let c1 = Color::new(0.2, 0.3, 0.4);
        let c2 = c0 + c1;
        assert!((0.3 - c2.r).abs() < 0.0001);
        assert!((0.5 - c2.g).abs() < 0.0001);
        assert!((0.7 - c2.b).abs() < 0.0001);
    }
}
