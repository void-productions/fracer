extern crate sfml;

use sfml::graphics::{RenderWindow, RectangleShape, Color};
use sfml::window::{Window, VideoMode, Style, Event};
use crate::sfml::graphics::RenderTarget;
use crate::sfml::graphics::Shape;
use crate::sfml::graphics::Transformable;
use std::thread;
use std::time;
use sfml::system::Vector2f;

fn main() {
	let desktop = VideoMode::desktop_mode();
	let mut window = RenderWindow::new(
		desktop,
		"SFML window",
		Style::FULLSCREEN,
		&Default::default()
	);

	let mut rectangle = RectangleShape::with_size(Vector2f::new(20.0, 40.0));
	rectangle.set_position(Vector2f::new(50.0, 50.0));
	rectangle.set_fill_color(&Color::WHITE);

	while window.is_open() {
		if let Some(Event::Closed) = window.poll_event() {
			break;
		}

		window.draw(&rectangle);

		window.display();
		thread::sleep(time::Duration::from_millis(1));
    }
}
