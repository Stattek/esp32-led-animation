use std::u8;

use smart_leds::RGB8;

use crate::{
    RgbLedAnimation,
    led_helpers::led_fade::{fade_in_led, fade_out_led},
};

/// Struct for setting a single color on an LED strip. All other LEDs will be shut off when one is
/// turned on.
#[derive(Debug)]
pub struct Rgb8SingleLedFadeAnimation {
    pixel_color: RGB8,
    pixels: Vec<RGB8>,
    num_pixels: usize,
    current_idx_on: Option<usize>,
    fade_step_value: u8,
}

impl AsRef<Vec<RGB8>> for Rgb8SingleLedFadeAnimation {
    fn as_ref(&self) -> &Vec<RGB8> {
        &self.pixels
    }
}

impl Rgb8SingleLedFadeAnimation {
    /// Creates a new `Rgb8SingleLedAnimation`, for setting a single LED at a time. The LED will use
    /// the specified color. All other LEDs will be set back to off.
    ///
    /// Useful for situations where an LED specifies a single value, such as an elevator floor
    /// number LED (which is what this was created for).
    pub fn new(num_pixels: usize, pixel_color: RGB8, fade_step_value: u8) -> Self {
        Self {
            pixel_color,
            // the pixels should default to being all off
            pixels: std::iter::repeat(RGB8::new(0, 0, 0))
                .into_iter()
                .cycle()
                .take(num_pixels)
                .collect(),
            num_pixels,
            current_idx_on: None, // since none of the floor lights are on
            // 1.0 is 100% fade on the next frame
            fade_step_value,
        }
    }
}

impl RgbLedAnimation for Rgb8SingleLedFadeAnimation {
    /// Perform the next frame for this animation. This will fade the light in that is on and will
    /// fade out lights that are off.
    fn next_frame(&mut self) {
        // fade in only the LED that we have set currently
        if let Some(current_idx_on) = self.current_idx_on {
            fade_in_led(
                &mut self.pixels[current_idx_on],
                &self.pixel_color,
                self.fade_step_value,
            );
        }

        // fade out all other LEDs that are off
        for i in 0..self.num_pixels {
            let do_fade = match self.current_idx_on {
                Some(current_idx_on) => {
                    // only do the fade out if the index isn't the LED that is currently on
                    i != current_idx_on
                }
                None => {
                    // all LEDs are off, we can check to fade this one
                    true
                }
            };

            if do_fade {
                fade_out_led(&mut self.pixels[i], self.fade_step_value);
            }
        }
    }
}

impl Rgb8SingleLedFadeAnimation {
    /// Sets the specified pixel on based on the input index. All other LEDs will be set to turn off.
    ///
    /// * `idx`: The LED index to light up.
    pub fn set_led_on(&mut self, idx: usize) -> Result<(), ()> {
        if idx >= self.num_pixels {
            // the specified index goes beyond the number of pixels that we have
            Err(())
        } else {
            self.current_idx_on = Some(idx);
            Ok(())
        }

        #[cfg(false)]
        {
            if let Some(current_idx_on) = self.current_idx_on {
                // since only one LED is on, let's set it back to off.
                self.pixels[current_idx_on] = RGB8::new(0, 0, 0);
            }
            self.current_idx_on = Some(idx);

            // now we can set the idx to the color that we want
            self.pixels[idx] = self.pixel_color.clone();
        }
    }

    // TODO: could be worth adding the ability to change the color of the LED.
    // TODO: another good one would be for setting the fade step value (maybe we want to fade
    // faster?)
}
