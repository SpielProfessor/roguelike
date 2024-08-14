// UTILS.RS - DEFINITION OF SOME MISC. FEATURES OF THE PROGRAM
use std::sync::Arc;

use rand::{thread_rng, Rng};

use crate::Game;

// map/screen dimensions
pub const SCREEN_W: i32 = 50;
pub const SCREEN_H: i32 = 30;

// Make the player only see stuff on the map when it was in a certain radius of him.
// TODO: "raycasting"
#[derive(Clone, Debug, Default)]
pub struct DiscoveredMap {
  pub content: Vec<Vec<bool>>,
}
impl DiscoveredMap {
  pub fn discover_around(&mut self, pos: Vec2) {
    let radius = 2;
    for y in pos.y - radius..=pos.y + radius {
      for x in pos.x - radius..=pos.x + radius {
        if y < self.content.len() as i32 && y >= 0 {
          if x < self.content[y as usize].len() as i32 && x >= 0 {
            self.content[y as usize][x as usize] = true;
          }
        }
      }
    }
  }
  pub fn new(w: i32, h: i32) -> Self {
    let mut map = DiscoveredMap::default();
    for y in 0..h {
      map.content.push(vec![]);
      for _ in 0..w {
        map.content[y as usize].push(false);
      }
    }
    return map;
  }
}

// a rectangle
#[derive(Clone, Copy, Debug)]
pub struct Rect {
  pub x: i32,
  pub y: i32,
  pub w: i32,
  pub h: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
  pub x: i32,
  pub y: i32,
}
impl Rect {
  pub fn random(w_min: i32, h_min: i32, w_max: i32, h_max: i32) -> Self {
    let mut rng = thread_rng();

    let mut w = rng.gen_range(w_min..w_max);
    let mut h = rng.gen_range(h_min..h_max);
    while w > SCREEN_W - 1 {
      w = rng.gen_range(w_min..w_max);
    }

    while h > SCREEN_H - 1 {
      h = rng.gen_range(h_min..h_max);
    }
    let x = rng.gen_range(1..SCREEN_W - w);
    let y = rng.gen_range(1..SCREEN_H - h);

    return Rect { x, y, w, h };
  }
}
pub const RECS_PER_LEVEL: i32 = 5;
// a map tile struct
#[derive(Clone)]
pub struct Tile {
  pub collidable: bool,
  pub char: String,
  // Run-into action: What happens when you run into it?
  pub ri_action: Arc<dyn Fn(&mut Game, i32, i32)>,
}
impl Default for Tile {
  fn default() -> Self {
    Self {
      collidable: false,
      char: ".".to_string(),
      ri_action: Arc::new(|_, _, _| {}),
    }
  }
}
impl Tile {
  pub fn interact(&self, game: &mut Game, x: i32, y: i32) {
    (self.ri_action)(game, x, y);
  }
}
