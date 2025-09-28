#![allow(unexpected_cfgs)] // TODO: add tests so this can be removed

pub mod led_animation;
pub mod led_helpers;

/// Trait for structs that can handle performing an RGB LED animation.
pub trait RgbLedAnimation {
    /// Calculates next frame of animation.
    fn next_frame(&mut self);
}

#[derive(Clone, Copy)]
pub enum PixelColor {
    Red,
    Blue,
    Green,
}

/// Direction for the animation to move in.
#[derive(Clone, Copy)]
pub enum Direction {
    Forward,
    Backward,
}

// TODO: add tests
#[cfg(target_arch = "none")]
#[cfg(test)]
mod tests {
    use super::*;
}
