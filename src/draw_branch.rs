use crate::config::Config;
use raqote::{DrawOptions, DrawTarget, PathBuilder, Point, SolidSource, Source, StrokeStyle};

pub fn draw_branch(dt: &mut DrawTarget, config: Config) {
  if config.depth == 0 {
    return;
  }

  let end = Point::new(
    config.start_point.x + config.length * config.angle.cos(),
    config.start_point.y + config.length * config.angle.sin(),
  );

  let mut pb = PathBuilder::new();
  pb.move_to(config.start_point.x, config.start_point.y);
  pb.line_to(end.x, end.y);
  let path = pb.finish();
  dt.stroke(
    &path,
    &Source::Solid(SolidSource::from_unpremultiplied_argb(
      config.color.0,
      config.color.1,
      config.color.2,
      config.color.3,
    )),
    &StrokeStyle::default(),
    &DrawOptions::new(),
  );

  let angle1 = config.angle + config.angle_delta;
  let angle2 = config.angle - config.angle_delta;

  draw_branch(
    dt,
    Config {
      start_point: end,
      length: config.length * config.length_delta,
      length_delta: config.length_delta,
      angle: angle1,
      angle_delta: config.angle_delta,
      depth: config.depth - 1,
      color: config.color,
    },
  );
  draw_branch(
    dt,
    Config {
      start_point: end,
      length: config.length * config.length_delta,
      length_delta: config.length_delta,
      angle: angle2,
      angle_delta: config.angle_delta,
      depth: config.depth - 1,
      color: config.color,
    },
  );
}
