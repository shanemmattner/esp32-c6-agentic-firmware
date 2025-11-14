//! Statig state machine for color navigation
//!
//! Three-state machine: RedBase, GreenBase, BlueBase
//! - Button cycles through states
//! - IMU tilt adjusts color within each state

use crate::color::{hsv_to_rgb, HsvColor};
use crate::set_led_color;
use log::info;
use statig::prelude::*;

// ============================================================================
// EVENTS
// ============================================================================

#[derive(Debug)]
pub enum Event {
    ButtonPressed,
    ImuUpdate { accel_x: i16, accel_y: i16 },
}

// ============================================================================
// STATE MACHINE
// ============================================================================

#[derive(Default)]
pub struct ColorNavigator;

/// State machine implementation using statig
#[state_machine(
    initial = "State::warm_palette()",
    state(derive(Debug)),
    on_transition = "Self::on_transition"
)]
impl ColorNavigator {
    /// Warm palette state (Red → Orange → Yellow, hue: 0-60°)
    #[state]
    fn warm_palette(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::ButtonPressed => Transition(State::cool_palette()),
            Event::ImuUpdate { accel_x, accel_y } => {
                update_warm_palette(*accel_x, *accel_y);
                Handled
            }
        }
    }

    /// Cool palette state (Cyan → Blue → Purple, hue: 180-300°)
    #[state]
    fn cool_palette(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::ButtonPressed => Transition(State::warm_palette()),
            Event::ImuUpdate { accel_x, accel_y } => {
                update_cool_palette(*accel_x, *accel_y);
                Handled
            }
        }
    }

    /// Called on state transitions
    fn on_transition(&mut self, source: &State, target: &State) {
        info!("🎨 Transition: {:?} → {:?}", source, target);
    }
}

// ============================================================================
// COLOR CALCULATION
// ============================================================================

/// Update LED color for warm palette (Red → Orange → Yellow → Green)
///
/// Uses rotation angle from X and Y axes to smoothly sweep through warm colors
fn update_warm_palette(accel_x: i16, accel_y: i16) {
    // Calculate angle from X and Y tilt for smooth 360° rotation
    // This maps any rotation direction to a hue value
    let angle_deg = calculate_rotation_angle(accel_x, accel_y);

    // Map full rotation (0-360°) to warm hue range (0-120°)
    // Red (0°) → Orange (30°) → Yellow (60°) → Yellow-Green (120°)
    let hue = ((angle_deg * 120) / 360).clamp(0, 120) as u16;

    // Keep brightness constant and low
    let brightness = 35;
    let saturation = 100;

    // Convert HSV to RGB
    let hsv = HsvColor::new(hue, saturation, brightness);
    let (r, g, b) = hsv_to_rgb(hsv);

    // Update shared LED color
    set_led_color(r, g, b);

    // Log color update (throttled in caller)
    info!(
        "💡 LED: HSV({}\u{00b0}, {}%, {}%) → RGB({}, {}, {})",
        hue, saturation, brightness, r, g, b
    );
}

/// Update LED color for cool palette (Cyan → Blue → Purple → Magenta)
///
/// Uses rotation angle from X and Y axes to smoothly sweep through cool colors
fn update_cool_palette(accel_x: i16, accel_y: i16) {
    // Calculate angle from X and Y tilt for smooth 360° rotation
    let angle_deg = calculate_rotation_angle(accel_x, accel_y);

    // Map full rotation (0-360°) to cool hue range (180-300°)
    // Cyan (180°) → Blue (240°) → Purple (270°) → Magenta (300°)
    let hue = 180 + ((angle_deg * 120) / 360).clamp(0, 120) as u16;

    // Keep brightness constant and low
    let brightness = 35;
    let saturation = 100;

    // Convert HSV to RGB
    let hsv = HsvColor::new(hue, saturation, brightness);
    let (r, g, b) = hsv_to_rgb(hsv);

    // Update shared LED color
    set_led_color(r, g, b);

    // Log color update (throttled in caller)
    info!(
        "💡 LED: HSV({}\u{00b0}, {}%, {}%) → RGB({}, {}, {})",
        hue, saturation, brightness, r, g, b
    );
}

/// Calculate rotation angle from X and Y accelerometer values
///
/// Returns angle in degrees (0-360)
fn calculate_rotation_angle(accel_x: i16, accel_y: i16) -> u32 {
    // Convert to i32 for calculation
    let x = accel_x as i32;
    let y = accel_y as i32;

    // Simple approximation of atan2 for embedded (no floating point)
    // This maps the X-Y plane to 0-360 degrees
    // We use a lookup-table-free approximation

    let abs_x = x.abs();
    let abs_y = y.abs();

    // Determine quadrant and calculate angle
    let angle = if abs_x > abs_y {
        // Closer to horizontal
        let ratio = (abs_y * 45) / abs_x.max(1); // Ratio * 45 to get approximate angle
        if x >= 0 {
            if y >= 0 { ratio } else { 360 - ratio }
        } else {
            if y >= 0 { 180 - ratio } else { 180 + ratio }
        }
    } else {
        // Closer to vertical
        let ratio = (abs_x * 45) / abs_y.max(1);
        if y >= 0 {
            if x >= 0 { 90 - ratio } else { 90 + ratio }
        } else {
            if x >= 0 { 270 + ratio } else { 270 - ratio }
        }
    };

    angle.clamp(0, 359) as u32
}
