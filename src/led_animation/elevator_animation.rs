use std::isize;

use crate::{
    Direction, PixelColor, RgbLedAnimation,
    led_helpers::led_fade::{fade_in_led, fade_out_led},
};
use smart_leds::RGB8;

/// Represents the direction the elevator is moving in.
#[derive(Clone, Copy)]
pub enum ElevatorDirection {
    Up,
    Down,
}

/// Keeps track of where the elevator is located when the animation starts, in resepect to the
/// `num_pixels` used to represent the elevator.
pub enum ElevatorStartingPosition {
    Above,
    Below,
    Center,
}

/// Struct to handle performing an elevator animation. This animation will be a sliding
/// bar, which fades in the body of the elevator and fades out everywhere else. When changing
/// directions, the head and tail indices switch.
pub struct Rgb8ElevatorAnimation {
    elevator_color: RGB8,
    pixels: Vec<RGB8>,
    // we want to go above or below, so we need to be able to go negative (so we're using isize)
    head_idx: isize,
    tail_idx: isize,
    elevator_length: usize,
    elevator_direction: ElevatorDirection,
    elevator_stopped: bool,
    /// Amount to fade in and out as the elevator moves
    fade_step_value: u8,
    elevator_speed: usize,
}

impl AsRef<Vec<RGB8>> for Rgb8ElevatorAnimation {
    fn as_ref(&self) -> &Vec<RGB8> {
        &self.pixels
    }
}

impl Rgb8ElevatorAnimation {
    /// Creates a new animation, with all lights off at the start.
    pub fn new(
        elevator_color: RGB8,
        num_pixels: usize,
        elevator_length: usize,
        fade_step_value: u8,
        elevator_stopped: bool,
        elevator_direction: ElevatorDirection,
        elevator_speed: usize,
        elevator_starting_position: ElevatorStartingPosition,
    ) -> Self {
        // let's calculate the starting position of the head and tail indices
        let (top_idx, bottom_idx): (isize, isize) = match elevator_starting_position {
            ElevatorStartingPosition::Above => (
                (num_pixels + (elevator_length - 1)) as isize,
                num_pixels as isize,
            ),
            ElevatorStartingPosition::Center => ((elevator_length - 1) as isize, 0),
            ElevatorStartingPosition::Below => (-1, -(elevator_length as isize)),
        };

        // the head is always facing the direction that the elevator is moving in
        let (head_idx, tail_idx) = match elevator_direction {
            ElevatorDirection::Up => (top_idx, bottom_idx),
            ElevatorDirection::Down => (bottom_idx, top_idx),
        };

        Self {
            elevator_color,
            pixels: std::iter::repeat_n(RGB8::new(0, 0, 0), num_pixels).collect(),
            head_idx,
            tail_idx,
            elevator_length,
            elevator_stopped,
            elevator_direction,
            fade_step_value,
            elevator_speed,
        }
    }
}

impl RgbLedAnimation for Rgb8ElevatorAnimation {
    fn next_frame(&mut self) {
        if !self.elevator_stopped {
            // calculate the displacement based on speed and direction
            let displacement: isize = match self.elevator_direction {
                ElevatorDirection::Up => self.elevator_speed as isize,
                ElevatorDirection::Down => -(self.elevator_speed as isize),
            };
            // both move in the same direction, obviously
            self.head_idx += displacement;
            self.tail_idx += displacement;
        }

        for i in 0..self.pixels.len() {
            // fade in body
            if (i as isize <= self.head_idx && i as isize >= self.tail_idx)
                || (i as isize >= self.head_idx && i as isize <= self.tail_idx)
            {
                fade_in_led(
                    &mut self.pixels[i],
                    &self.elevator_color,
                    self.fade_step_value,
                );
            } else {
                // fade out rest
                fade_out_led(&mut self.pixels[i], self.fade_step_value);
            }
        }
    }
}

impl Rgb8ElevatorAnimation {
    /// Changes the direction of the elevator.
    pub fn change_direction(&mut self) {
        std::mem::swap(&mut self.head_idx, &mut self.tail_idx);
        self.elevator_direction = match self.elevator_direction {
            ElevatorDirection::Up => ElevatorDirection::Down,
            ElevatorDirection::Down => ElevatorDirection::Up,
        };
    }

    /// Sets the speed of the elevator.
    ///
    /// * `elevator_speed`: The new speed.
    pub fn set_speed(&mut self, elevator_speed: usize) {
        self.elevator_speed = elevator_speed;
    }

    /// Sets the color of the Elevator.
    ///
    /// * `elevator_color`: The new color of the elevator.
    pub fn set_color(&mut self, elevator_color: RGB8) {
        self.elevator_color = elevator_color;
    }
}
