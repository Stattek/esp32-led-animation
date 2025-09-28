use crate::{
    Direction, PixelColor, RgbLedAnimation,
    led_helpers::led_fade::{fade_in_led, fade_out_led},
};
use smart_leds::RGB8;

/// Struct to handle performing a fading snake window animation. This animation will be a sliding
/// bar, which fades in the body of the snake and fades out everywhere else. When changing
/// directions, the head and tail indices switch, making it more like a sliding window,
/// but it's a snake window because it's cooler.
pub struct Rgb8FadingSnakeWindowAnimation {
    snake_color: RGB8,
    pixels: Vec<RGB8>,
    // we want to go above or below, so we need to be able to go negative (so we're using isize)
    head_idx: isize,
    tail_idx: isize,
    snake_length: usize,
    snake_direction: Direction, // direction snake is moving in
    fade_step_value: u8,
}

impl AsRef<Vec<RGB8>> for Rgb8FadingSnakeWindowAnimation {
    fn as_ref(&self) -> &Vec<RGB8> {
        &self.pixels
    }
}

impl Rgb8FadingSnakeWindowAnimation {
    /// Creates a new animation, with all lights off at the start.
    pub fn new(
        snake_color: RGB8,
        num_pixels: usize,
        snake_length: usize,
        fade_step_value: u8,
    ) -> Self {
        // let's start off with all off
        let head_idx: isize = -1; // off screen
        let tail_idx: isize = (head_idx + 1) - snake_length as isize;
        Self {
            snake_color,
            pixels: std::iter::repeat_n(RGB8::new(0, 0, 0), num_pixels).collect(),
            head_idx,
            tail_idx, // FIXME: idk
            snake_length,
            snake_direction: Direction::Forward,
            fade_step_value,
        }
    }
}

impl RgbLedAnimation for Rgb8FadingSnakeWindowAnimation {
    fn next_frame(&mut self) {
        for i in 0..self.pixels.len() {
            // fade in body
            if i as isize <= self.head_idx && i as isize >= self.tail_idx {
                fade_in_led(&mut self.pixels[i], &self.snake_color, self.fade_step_value);
            } else {
                // fade out rest
                fade_out_led(&mut self.pixels[i], self.fade_step_value);
            }
        }
    }
}
