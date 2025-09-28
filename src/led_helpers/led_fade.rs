use smart_leds::RGB8;

/// Fades an LED in on this frame.
///
/// * `pixel`: The pixel (LED) to fade in.
/// * `fade_to_color`: The color that we wish to fade into.
/// * `fade_step_value`: The amount to fade in per frame. This function will not allow going over
/// the desired value.
pub fn fade_in_led(pixel: &mut RGB8, fade_to_color: &RGB8, fade_step_value: u8) {
    // check if we are at all the values that we want
    if pixel.r != fade_to_color.r || pixel.g != fade_to_color.g || pixel.b != fade_to_color.b {
        // we need to keep increasing our value

        // calculate the values to increase by, the maximum we want to increase by is the
        // difference between the maximum u8 value and the current value
        let r_inc = fade_step_value.clamp(0, u8::MAX - pixel.r);
        let g_inc = fade_step_value.clamp(0, u8::MAX - pixel.g);
        let b_inc = fade_step_value.clamp(0, u8::MAX - pixel.b);

        // set the values now
        pixel.r += r_inc;
        pixel.g += g_inc;
        pixel.b += b_inc;
    }
}

/// Fades an LED out on this frame toward being turned off.
///
/// * `pixel`: The pixel (LED) to fade out.
/// * `fade_step_value`: The amount to fade out per frame. This function will not allow going under
/// 0 for each color.
pub fn fade_out_led(pixel: &mut RGB8, fade_step_value: u8) {
    // check that all values are at 0
    if pixel.r != 0 || pixel.g != 0 || pixel.b != 0 {
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
