use rand::rngs::ThreadRng;
use std::{thread, time::Duration};
use embedded_graphics::pixelcolor::Bgr565;
use embedded_graphics::prelude::*;
use embedded_snake::*;
use lcd_1in44_hat::gamepad::{Gamepad, Key};
use lcd_1in44_hat::display::Display;

fn main() {
    let gamepad = Gamepad::new();
    let mut display = Display::new();
    display.setup();

    let mut game = SnakeGame::<100, Bgr565, ThreadRng>::new(
        128,
        128,
        3,
        3,
        rand::thread_rng(),
        Bgr565::RED,
        Bgr565::YELLOW,
        50,
    );

    'running: loop {
        if gamepad.is_pressed(Key::BUTTON1) {
            break 'running;
        }
        let direction = if gamepad.is_pressed(Key::UP) {
            Direction::Up
        } else if gamepad.is_pressed(Key::DOWN) {
            Direction::Down
        } else if gamepad.is_pressed(Key::LEFT) {
            Direction::Left
        } else if gamepad.is_pressed(Key::RIGHT) {
            Direction::Right
        } else {
            Direction::None
        };

        game.set_direction(direction);

        {
            let mut fbuf = display.get_buffer();
            fbuf.clear(Bgr565::BLACK).unwrap();
            game.draw(&mut fbuf);
        }

        display.draw_buffer();
        thread::sleep(Duration::from_millis(50));
    }
}
