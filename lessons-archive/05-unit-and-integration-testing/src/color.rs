//! HSV to RGB color conversion
//!
//! Pure function suitable for both host-based unit testing and embedded use.
//! This module demonstrates testable embedded code.

/// HSV color representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HsvColor {
    pub hue: u16,        // 0-360 degrees
    pub saturation: u8,  // 0-100 percent
    pub value: u8,       // 0-100 percent (brightness)
}

impl HsvColor {
    /// Create a new HSV color with automatic clamping
    pub fn new(hue: u16, saturation: u8, value: u8) -> Self {
        Self {
            hue: hue % 360,
            saturation: saturation.min(100),
            value: value.min(100),
        }
    }
}

/// Convert HSV color to RGB (0-255 range)
///
/// This is a pure function with no hardware dependencies,
/// making it ideal for host-based unit testing.
///
/// Algorithm: Standard HSV→RGB conversion using sector-based calculation
/// with integer-only math (no floating point).
pub fn hsv_to_rgb(hsv: HsvColor) -> (u8, u8, u8) {
    // Shortcut for grayscale (no saturation)
    if hsv.saturation == 0 {
        let v = (hsv.value as u32 * 255 / 100) as u8;
        return (v, v, v);
    }

    // Convert to fixed-point math (avoid f32)
    let h = hsv.hue % 360;
    let s = hsv.saturation as u32;
    let v = hsv.value as u32;

    // Determine sector (0-5)
    let sector = h / 60;
    let remainder = (h % 60) as u32;

    // Calculate intermediate values (scaled by 100 for percentage)
    let p = (v * (100 - s)) / 100;
    let q = (v * (100 - (s * remainder) / 60)) / 100;
    let t = (v * (100 - (s * (60 - remainder)) / 60)) / 100;

    // Scale to 0-255
    let v_scaled = (v * 255 / 100) as u8;
    let p_scaled = (p * 255 / 100) as u8;
    let q_scaled = (q * 255 / 100) as u8;
    let t_scaled = (t * 255 / 100) as u8;

    // Select RGB based on sector
    match sector {
        0 => (v_scaled, t_scaled, p_scaled), // Red → Yellow
        1 => (q_scaled, v_scaled, p_scaled), // Yellow → Green
        2 => (p_scaled, v_scaled, t_scaled), // Green → Cyan
        3 => (p_scaled, q_scaled, v_scaled), // Cyan → Blue
        4 => (t_scaled, p_scaled, v_scaled), // Blue → Magenta
        _ => (v_scaled, p_scaled, q_scaled), // Magenta → Red
    }
}

// ============================================================================
// HOST-BASED UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_red() {
        let hsv = HsvColor::new(0, 100, 100);
        let rgb = hsv_to_rgb(hsv);
        assert_eq!(rgb, (255, 0, 0), "Pure red should be RGB(255, 0, 0)");
    }

    #[test]
    fn test_pure_green() {
        let hsv = HsvColor::new(120, 100, 100);
        let rgb = hsv_to_rgb(hsv);
        assert_eq!(rgb, (0, 255, 0), "Pure green should be RGB(0, 255, 0)");
    }

    #[test]
    fn test_pure_blue() {
        let hsv = HsvColor::new(240, 100, 100);
        let rgb = hsv_to_rgb(hsv);
        assert_eq!(rgb, (0, 0, 255), "Pure blue should be RGB(0, 0, 255)");
    }

    #[test]
    fn test_cyan() {
        let hsv = HsvColor::new(180, 100, 100);
        let rgb = hsv_to_rgb(hsv);
        assert_eq!(rgb, (0, 255, 255), "Cyan should be RGB(0, 255, 255)");
    }

    #[test]
    fn test_magenta() {
        let hsv = HsvColor::new(300, 100, 100);
        let rgb = hsv_to_rgb(hsv);
        assert_eq!(rgb, (255, 0, 255), "Magenta should be RGB(255, 0, 255)");
    }

    #[test]
    fn test_yellow() {
        let hsv = HsvColor::new(60, 100, 100);
        let rgb = hsv_to_rgb(hsv);
        assert_eq!(rgb, (255, 255, 0), "Yellow should be RGB(255, 255, 0)");
    }

    #[test]
    fn test_half_brightness_red() {
        let hsv = HsvColor::new(0, 100, 50);
        let rgb = hsv_to_rgb(hsv);
        assert_eq!(rgb, (127, 0, 0), "Half brightness red should be RGB(127, 0, 0)");
    }

    #[test]
    fn test_white() {
        let hsv = HsvColor::new(0, 0, 100);
        let rgb = hsv_to_rgb(hsv);
        assert_eq!(rgb, (255, 255, 255), "White (no saturation) should be RGB(255, 255, 255)");
    }

    #[test]
    fn test_black() {
        let hsv = HsvColor::new(0, 0, 0);
        let rgb = hsv_to_rgb(hsv);
        assert_eq!(rgb, (0, 0, 0), "Black (no value) should be RGB(0, 0, 0)");
    }

    #[test]
    fn test_gray() {
        let hsv = HsvColor::new(0, 0, 50);
        let rgb = hsv_to_rgb(hsv);
        assert_eq!(rgb, (127, 127, 127), "50% gray should be RGB(127, 127, 127)");
    }

    #[test]
    fn test_hue_wrapping() {
        let hsv1 = HsvColor::new(0, 100, 100);
        let hsv2 = HsvColor::new(360, 100, 100);
        assert_eq!(
            hsv_to_rgb(hsv1),
            hsv_to_rgb(hsv2),
            "Hue should wrap at 360 degrees"
        );
    }

    #[test]
    fn test_saturation_clamping() {
        let hsv = HsvColor::new(0, 150, 100); // Saturation > 100
        assert_eq!(hsv.saturation, 100, "Saturation should be clamped to 100");
    }
}
