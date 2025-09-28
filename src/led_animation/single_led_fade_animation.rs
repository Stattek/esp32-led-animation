use std::u8;

use smart_leds::RGB8;

use crate::RgbLedAnimation;

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
            // check if we are at the values that we want yet. We are checking them all because
            // floating point arithmetic can be funky and we may be very close but not quite where
            // we wish to be.
            let current_pixel = &mut self.pixels[current_idx_on];
            if self.pixel_color.r != current_pixel.r
                || self.pixel_color.g != current_pixel.g
                || self.pixel_color.b != current_pixel.b
            {
                // we need to keep increasing our value

                // calculate the values to increase by, the maximum we want to increase by is the
                // difference between the maximum u8 value and the current value
                let r_inc = self.fade_step_value.clamp(0, u8::MAX - current_pixel.r);
                let g_inc = self.fade_step_value.clamp(0, u8::MAX - current_pixel.g);
                let b_inc = self.fade_step_value.clamp(0, u8::MAX - current_pixel.b);

                // set the values now
                current_pixel.r += r_inc;
                current_pixel.g += g_inc;
                current_pixel.b += b_inc;
            }
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
                // check that all values are at 0
                let current_pixel = &mut self.pixels[i];
                if current_pixel.r != 0 || current_pixel.g != 0 || current_pixel.b != 0 {
                    // something isn't at 0, perform fade!

                    // calculate the values to decrease by, the max value we want to decrease by is
                    // what the current pixel value is
                    let r_dec = self.fade_step_value.clamp(0, current_pixel.r);
                    let g_dec = self.fade_step_value.clamp(0, current_pixel.g);
                    let b_dec = self.fade_step_value.clamp(0, current_pixel.b);

                    // set the values now
                    current_pixel.r -= r_dec;
                    current_pixel.g -= g_dec;
                    current_pixel.b -= b_dec;
                }
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
}
