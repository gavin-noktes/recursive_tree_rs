use raqote::Point;

pub struct Config {
  pub start_point: Point,
  pub length: f32,
  pub length_delta: f32,
  pub angle: f32,
  pub angle_delta: f32,
  pub depth: u16,
  pub color: (u8, u8, u8, u8),
}
