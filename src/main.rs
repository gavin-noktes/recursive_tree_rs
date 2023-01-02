mod config;
mod draw_branch;

use config::Config;
use draw_branch::draw_branch;
use minifb::{Window, WindowOptions};
use raqote::{DrawTarget, Point};

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;
const BRANCH_LENGTH: f32 = 100.0;
const BRANCH_LENGTH_DELTA: f32 = 0.8;
const INITIAL_ANGLE: f32 = -std::f32::consts::PI / 2.0;
const ANGLE_DELTA: f32 = 0.4;
const DEPTH: u16 = 8;
const COLOR: (u8, u8, u8, u8) = (0xff, 0, 0xff, 0);

fn main() {
  let mut window = Window::new(
    "Gavin's Recursive Trees",
    WIDTH,
    HEIGHT,
    WindowOptions {
      ..WindowOptions::default()
    },
  )
  .unwrap();

  let size = window.get_size();
  let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);
  let center_x = size.0 as f32 / 2.0;
  let bottom_y = size.1 as f32;

  loop {
    draw_branch(
      &mut dt,
      Config {
        start_point: Point::new(center_x, bottom_y),
        length: BRANCH_LENGTH,
        length_delta: BRANCH_LENGTH_DELTA,
        angle: INITIAL_ANGLE,
        angle_delta: ANGLE_DELTA,
        depth: DEPTH,
        color: COLOR,
      },
    );

    window
      .update_with_buffer(dt.get_data(), size.0, size.1)
      .unwrap();
  }
}
