
mod ppmimage;

type Position = (u32, u32);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Target {
  position: Position,
  colour: u32
}

impl Target {
  fn new(position: Position, colour: u32) -> Self {
    Target { position, colour }
  }
}

fn euclidean_distance(pos_a: Position, pos_b: Position) -> f32 {
  ((pos_a.0 as f32 - pos_b.0 as f32).powf(2.0) + (pos_a.1 as f32 - pos_b.1 as f32).powf(2.0)).sqrt()
}

fn find_closest_target(position: Position, targets: &Vec<Target>) -> Target {
  let mut result: Target = Target::new((0, 0), 0x000000FF);
  let mut closest_distance: f32 = std::f32::MAX;

  targets
    .iter()
    .for_each(|target| {
      let euclidean_distance = euclidean_distance(position, target.position);
      
      if euclidean_distance < closest_distance {
        closest_distance = euclidean_distance;
        result = *target;
      }
    });

  result
}

fn construct_diagram(targets: Vec<Target>) {
  let mut image = ppmimage::PPMImage::new(WIDTH, HEIGHT);

  for i in 0..HEIGHT {
    for j in 0..WIDTH {
      let closest_target_col = find_closest_target((i, j), &targets).colour;
      image.set_data_at_index(i as usize, j as usize, closest_target_col);
    }
  }

  match image.save("test.ppm") {
    Ok(_) => println!("Image saved!"),
    Err(_) => println!("Error saving image!")
  }
}

fn construct_random_targets(n_targets: u32) -> Vec<Target> {
  use rand::Rng;

  let mut rng = rand::thread_rng();
  let mut result = vec![];

  for _ in 0..n_targets {
    let rand_pos = (rng.gen_range(0..HEIGHT), rng.gen_range(0..WIDTH));
    let rand_col = (rng.gen_range(0x000000..=0xFFFFFF) << 8) | 0xFF;
    result.push(Target::new(rand_pos, rand_col));
  }

  result
}

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() {
  let targets = construct_random_targets(12);
  construct_diagram(targets);
}

