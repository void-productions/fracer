extern crate sfml;

use sfml::graphics::RenderWindow;
use sfml::window::{Window, VideoMode, Style};
use std::thread;
use std::time;

fn main() {
    let desktop = VideoMode::desktop_mode();
    let mut window = Window::new(desktop,
                              "SFML window",
                              Style::CLOSE,
                              &Default::default());

    while window.is_open() {
        window.poll_event();
        window.display();
        thread::sleep(time::Duration::from_millis(1));
    }
}
