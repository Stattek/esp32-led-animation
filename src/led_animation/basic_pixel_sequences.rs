use smart_leds::RGB8;

// predefined pixel sequences
pub const FOURTH_OF_JULY_SEQUENCE: [RGB8; 15] = [
    RGB8::new(255, 0, 0),
    RGB8::new(255, 0, 0),
    RGB8::new(255, 0, 0),
    RGB8::new(255, 0, 0),
    RGB8::new(255, 0, 0),
    RGB8::new(255, 255, 255),
    RGB8::new(255, 255, 255),
    RGB8::new(255, 255, 255),
    RGB8::new(255, 255, 255),
    RGB8::new(255, 255, 255),
    RGB8::new(0, 0, 255),
    RGB8::new(0, 0, 255),
    RGB8::new(0, 0, 255),
    RGB8::new(0, 0, 255),
    RGB8::new(0, 0, 255),
];

pub const SUMMER_SEQUENCE: [RGB8; 10] = [
    RGB8::new(0, 228, 255),
    RGB8::new(0, 228, 255),
    RGB8::new(0, 228, 255),
    RGB8::new(0, 228, 255),
    RGB8::new(0, 228, 255),
    RGB8::new(255, 255, 0),
    RGB8::new(255, 255, 0),
    RGB8::new(255, 255, 0),
    RGB8::new(255, 255, 0),
    RGB8::new(255, 255, 0),
];

pub const PURPLES_SEQUENCE: [RGB8; 10] = [
    RGB8::new(112, 0, 255),
    RGB8::new(112, 0, 255),
    RGB8::new(112, 0, 255),
    RGB8::new(112, 0, 255),
    RGB8::new(112, 0, 255),
    RGB8::new(132, 80, 255),
    RGB8::new(132, 80, 255),
    RGB8::new(132, 80, 255),
    RGB8::new(132, 80, 255),
    RGB8::new(132, 80, 255),
];

pub const OFF_WHITE_SEQUENCE: [RGB8; 11] = [
    RGB8::new(255, 255, 100),
    RGB8::new(255, 255, 90),
    RGB8::new(255, 255, 80),
    RGB8::new(255, 255, 70),
    RGB8::new(255, 255, 60),
    RGB8::new(255, 255, 50),
    RGB8::new(255, 255, 40),
    RGB8::new(255, 255, 30),
    RGB8::new(255, 255, 20),
    RGB8::new(255, 255, 10),
    RGB8::new(255, 255, 0),
];
