use std::u8;

use smart_leds::RGB8;

/// Fades toward a particular color value.
///
/// * `color`: The current color value.
/// * `desired_color`: The desired color value.
/// * `fade_step_value`: The amount to fade toward per frame.
/// * `over_fade_value`: The amount to over fade in per frame, for a flickering effect.
/// NOTE: this value will only affect pixels that haven't yet reached their desired color.
/// If it can never reach its desired color, it will remain in a back-and-forth between
/// two colors forever.
fn handle_fade(cur_color: &mut u8, desired_color: u8, fade_step_value: u8, over_fade_value: u8) {
    if *cur_color < desired_color {
        // calculate the value to increase by, the maximum we want to increase by is the
        // difference between the maximum u8 value and the current value
        let increase_by = fade_step_value.clamp(0, desired_color - *cur_color);

        // set the values now
        *cur_color += increase_by;

        // calculate and perform over fade
        let remaining_til_overflow = u8::MAX - *cur_color;
        let over_increase = over_fade_value.clamp(0, remaining_til_overflow);
        *cur_color += over_increase;
    } else if *cur_color > desired_color {
        // calculate value to decrease by. The maximum we want to decrease by is the amount that we
        // have now.
        let decrease_by = fade_step_value.clamp(0, *cur_color);

        // set the value
        *cur_color -= decrease_by;
    }
}

/// Fades an LED in on this frame.
///
/// * `pixel`: The pixel (LED) to fade in.
/// * `fade_to_color`: The color that we wish to fade into.
/// * `fade_step_value`: The amount to fade in per frame. This function will not allow going over
/// the desired value.
/// * `over_fade_value`: The amount to over fade in per frame, for a flickering effect.
pub fn fade_in_led(
    pixel: &mut RGB8,
    fade_to_color: &RGB8,
    fade_step_value: u8,
    over_fade_value: u8,
) {
    // check if we are at all the values that we want and increase them
    handle_fade(
        &mut pixel.r,
        fade_to_color.r,
        fade_step_value,
        over_fade_value,
    );
    handle_fade(
        &mut pixel.g,
        fade_to_color.g,
        fade_step_value,
        over_fade_value,
    );
    handle_fade(
        &mut pixel.b,
        fade_to_color.b,
        fade_step_value,
        over_fade_value,
    );
}

/// Checks that an LED is faded in already.
///
/// * `pixel`: The pixel (LED) to fade in.
/// * `fade_to_color`: The color that we wish to fade into.
pub fn is_led_faded_in(pixel: &RGB8, fade_to_color: &RGB8) -> bool {
    pixel.r == fade_to_color.r && pixel.g == fade_to_color.g && pixel.b == fade_to_color.b
}

/// Checks that an LED is faded out already.
///
/// * `pixel`: The pixel (LED) to fade in.
/// * `fade_to_color`: The color that we wish to fade into.
pub fn is_led_faded_out(pixel: &RGB8) -> bool {
    pixel.r == 0 && pixel.g == 0 && pixel.b == 0
}

/// Fades an LED out on this frame toward being turned off.
///
/// * `pixel`: The pixel (LED) to fade out.
/// * `fade_step_value`: The amount to fade out per frame. This function will not allow going under
/// 0 for each color.
pub fn fade_out_led(pixel: &mut RGB8, fade_step_value: u8) {
    // check that all values are at 0
    if !is_led_faded_out(&pixel) {
        // something isn't at 0, perform fade!

        // calculate the values to decrease by, the max value we want to decrease by is
        // what the current pixel value is
        let r_dec = fade_step_value.clamp(0, pixel.r);
        let g_dec = fade_step_value.clamp(0, pixel.g);
        let b_dec = fade_step_value.clamp(0, pixel.b);

        // set the values now
        pixel.r -= r_dec;
        pixel.g -= g_dec;
        pixel.b -= b_dec;
    }
}
