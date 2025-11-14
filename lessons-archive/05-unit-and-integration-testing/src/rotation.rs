//! Rotation angle calculation from accelerometer data
//!
//! Pure function suitable for both host-based unit testing and embedded use.
//! Demonstrates integer-only atan2 approximation.

/// Calculate rotation angle from X and Y accelerometer values
///
/// This is a pure function with no hardware dependencies,
/// making it ideal for host-based unit testing.
///
/// Returns angle in degrees (0-360) using integer-only atan2 approximation
///
/// Algorithm:
/// - Determines quadrant based on X/Y signs
/// - Uses ratio approximation instead of trigonometry
/// - No floating point operations
pub fn calculate_rotation_angle(accel_x: i16, accel_y: i16) -> u32 {
    // Convert to i32 for calculation
    let x = accel_x as i32;
    let y = accel_y as i32;

    // Simple approximation of atan2 for embedded (no floating point)
    // This maps the X-Y plane to 0-360 degrees

    let abs_x = x.abs();
    let abs_y = y.abs();

    // Determine quadrant and calculate angle
    let angle = if abs_x > abs_y {
        // Closer to horizontal
        let ratio = (abs_y * 45) / abs_x.max(1); // Ratio * 45 to get approximate angle
        if x >= 0 {
            if y >= 0 {
                ratio
            } else {
                360 - ratio
            }
        } else {
            if y >= 0 {
                180 - ratio
            } else {
                180 + ratio
            }
        }
    } else {
        // Closer to vertical
        let ratio = (abs_x * 45) / abs_y.max(1);
        if y >= 0 {
            if x >= 0 {
                90 - ratio
            } else {
                90 + ratio
            }
        } else {
            if x >= 0 {
                270 + ratio
            } else {
                270 - ratio
            }
        }
    };

    angle.clamp(0, 359) as u32
}

// ============================================================================
// HOST-BASED UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_degrees() {
        // Pointing right (positive X)
        let angle = calculate_rotation_angle(1000, 0);
        assert!(angle < 5, "Should be close to 0°, got {}°", angle);
    }

    #[test]
    fn test_90_degrees() {
        // Pointing up (positive Y)
        let angle = calculate_rotation_angle(0, 1000);
        assert!(
            (85..=95).contains(&angle),
            "Should be close to 90°, got {}°",
            angle
        );
    }

    #[test]
    fn test_180_degrees() {
        // Pointing left (negative X)
        let angle = calculate_rotation_angle(-1000, 0);
        assert!(
            (175..=185).contains(&angle),
            "Should be close to 180°, got {}°",
            angle
        );
    }

    #[test]
    fn test_270_degrees() {
        // Pointing down (negative Y)
        let angle = calculate_rotation_angle(0, -1000);
        assert!(
            (265..=275).contains(&angle),
            "Should be close to 270°, got {}°",
            angle
        );
    }

    #[test]
    fn test_quadrant_1() {
        // Positive X, positive Y (0-90°)
        let angle = calculate_rotation_angle(1000, 1000);
        assert!(
            (40..=50).contains(&angle),
            "45° should be in range 40-50°, got {}°",
            angle
        );
    }

    #[test]
    fn test_quadrant_2() {
        // Negative X, positive Y (90-180°)
        let angle = calculate_rotation_angle(-1000, 1000);
        assert!(
            (130..=140).contains(&angle),
            "135° should be in range 130-140°, got {}°",
            angle
        );
    }

    #[test]
    fn test_quadrant_3() {
        // Negative X, negative Y (180-270°)
        let angle = calculate_rotation_angle(-1000, -1000);
        assert!(
            (220..=230).contains(&angle),
            "225° should be in range 220-230°, got {}°",
            angle
        );
    }

    #[test]
    fn test_quadrant_4() {
        // Positive X, negative Y (270-360°)
        let angle = calculate_rotation_angle(1000, -1000);
        assert!(
            (310..=320).contains(&angle),
            "315° should be in range 310-320°, got {}°",
            angle
        );
    }

    #[test]
    fn test_small_values() {
        // Should handle small accelerometer values
        let angle = calculate_rotation_angle(10, 5);
        assert!(angle < 360, "Angle should be valid");
    }

    #[test]
    fn test_large_values() {
        // Should handle large accelerometer values (real MPU9250 range: ±16384)
        let angle = calculate_rotation_angle(16000, 8000);
        assert!(angle < 360, "Angle should be valid");
    }

    #[test]
    fn test_zero_input() {
        // Edge case: both values zero
        let angle = calculate_rotation_angle(0, 0);
        assert!(angle < 360, "Should handle zero input gracefully");
    }

    #[test]
    fn test_angle_range() {
        // All angles should be 0-359
        for x in [-16000, -8000, 0, 8000, 16000].iter() {
            for y in [-16000, -8000, 0, 8000, 16000].iter() {
                let angle = calculate_rotation_angle(*x, *y);
                assert!(
                    angle < 360,
                    "Angle should be 0-359, got {} for ({}, {})",
                    angle,
                    x,
                    y
                );
            }
        }
    }
}
