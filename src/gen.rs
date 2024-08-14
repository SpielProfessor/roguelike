use crate::{thread_rng, DiscoveredMap, Game, Rect, Rng, Vec2, RECS_PER_LEVEL, SCREEN_H, SCREEN_W};
/***[ L E V E L  G E N E R A T I O N ]***
 * Concept:                             *
 * 0.The map gets reset to tile 0s only *
 * 1.generate random rectangles at      *
 *   random positions on the map. These *
 *   are stored in an array             *
 *   They get carved out of the map     *
 * 2.A random route (randomly chosen    *
 *   wether to go horizontal/vertical)  *
 *   gets created between the rooms,    *
 *   starting at the array's 0th index. *
 *   It always connects a room to the   *
 *   next one.                          *
 * 3.The player is moved to the first   *
 *   room.                              *
 * 4.A staircase is placed in the last  *
 *   room                               *
 ****************************************/
// WARN: the random generation is still unfinished, so it probably won't work yet
// TODO: Improve random generation

impl Game {
  pub fn generate_level(&mut self) {
    //reset
    self.discovered = DiscoveredMap::new(SCREEN_W, SCREEN_H);
    self.map.clear();
    for y in 0..SCREEN_H {
      self.map.push(vec![]);
      for _ in 0..SCREEN_W {
        self.map[y as usize].push(0);
      }
    }
    // generate recs
    let min_w = 4;
    let min_h = 2;
    let max_w = 10;
    let max_h = 10;
    let mut rects = vec![];
    for _ in 0..RECS_PER_LEVEL {
      rects.push(Rect::random(min_w, min_h, max_w, max_h));
    }
    self.x = rects[0].x + 1;
    self.y = rects[0].y + 1;
    for rect in 0..rects.len() {
      self.carve_walls(&rects[rect]);
      self.carve(1, &rects[rect]);
      if rect < rects.len() - 1 {
        self.carve_tunnels(
          Vec2 {
            x: rects[rect].x,
            y: rects[rect].y,
          },
          Vec2 {
            x: rects[rect + 1].x,
            y: rects[rect + 1].y,
          },
          2,
          true,
        );
      }
    }
    // corridor from first to last room
    self.carve_tunnels(
      Vec2 {
        x: rects[0].x,
        y: rects[0].y,
      },
      Vec2 {
        x: rects[rects.len() - 1].x,
        y: rects[rects.len() - 1].y,
      },
      2,
      true,
    );

    // staircase down
    self.map[rects[rects.len() - 1].y as usize + 1][rects[rects.len() - 1].x as usize + 1] = 7;
  }
  // carve out a rectangle
  // WARNING: If rectangle is out of bounds, little crash protection is included.
  // WARNING: MAKE SURE THAT THE MAP IS AS BIG AS SCREEN_W/SCREEN_H!
  pub fn carve(&mut self, tile_id: i32, rect: &Rect) {
    for y in rect.y..(rect.y + rect.h) {
      for x in rect.x..(rect.x + rect.w) {
        if x < SCREEN_W && y < SCREEN_H {
          self.map[y as usize][x as usize] = tile_id;
        }
      }
    }
  }
  pub fn carve_walls(&mut self, rect: &Rect) {
    for x in rect.x - 1..=(rect.x + rect.w) {
      if self.map[rect.y as usize - 1][x as usize] == 0 {
        self.map[rect.y as usize - 1][x as usize] = 4;
      }
    }
    for y in rect.y..(rect.y + rect.h) {
      if self.map[y as usize][rect.x as usize - 1] == 0 {
        self.map[y as usize][rect.x as usize - 1] = 3;
      }
    }
    for y in rect.y..(rect.y + rect.h) {
      if self.map[y as usize][(rect.x + rect.w) as usize] == 0 {
        self.map[y as usize][(rect.x + rect.w) as usize] = 3;
      }
    }
    for x in rect.x - 1..=(rect.x + rect.w) {
      if self.map[(rect.y + rect.h) as usize][x as usize] == 0 {
        self.map[(rect.y + rect.h) as usize][x as usize] = 4;
      }
    }
  }
  // Carve tunnels in the map
  // TODO: Carve without making a lot of doors
  pub fn carve_tunnels(&mut self, start: Vec2, end: Vec2, tile_id: i32, replace_empty_only: bool) {
    let mut finished = false;
    let mut current = start;
    // 0: move horizontally | 1: move vertically | 3: move like before
    let mut rng_val = 0;
    let mut rng = thread_rng();
    // begin by moving left
    if !replace_empty_only || self.get_map_tile_id(current.x, current.y) == 0 {
      self.map[current.y as usize][current.x as usize] = tile_id;
    }
    if self.get_map_tile_id(current.x, current.y) == 3
      || self.get_map_tile_id(current.x, current.y) == 4
    {
      self.map[current.y as usize][current.x as usize] = 5;
    }
    current.x += 1;
    while !finished {
      // horizontal movement
      if rng_val == 0 {
        if current.x < end.x {
          if !replace_empty_only || self.get_map_tile_id(current.x, current.y) == 0 {
            self.map[current.y as usize][current.x as usize] = tile_id;
          }
          if self.get_map_tile_id(current.x, current.y) == 3
            || self.get_map_tile_id(current.x, current.y) == 4
          {
            self.map[current.y as usize][current.x as usize] = 5;
          }
          current.x += 1;
        } else if current.x > end.x {
          if !replace_empty_only || self.get_map_tile_id(current.x, current.y) == 0 {
            self.map[current.y as usize][current.x as usize] = tile_id;
          }

          if self.get_map_tile_id(current.x, current.y) == 3
            || self.get_map_tile_id(current.x, current.y) == 4
          {
            self.map[current.y as usize][current.x as usize] = 5;
          }
          current.x -= 1;
        }
      }

      // vertical movement
      if rng_val == 1 {
        if current.y < end.y {
          if !replace_empty_only || self.get_map_tile_id(current.x, current.y) == 0 {
            self.map[current.y as usize][current.x as usize] = tile_id;
          }
          if self.get_map_tile_id(current.x, current.y) == 3
            || self.get_map_tile_id(current.x, current.y) == 4
          {
            self.map[current.y as usize][current.x as usize] = 5;
          }
          current.y += 1;
        } else if current.y > end.y {
          if !replace_empty_only || self.get_map_tile_id(current.x, current.y) == 0 {
            self.map[current.y as usize][current.x as usize] = tile_id;
          }

          if self.get_map_tile_id(current.x, current.y) == 3
            || self.get_map_tile_id(current.x, current.y) == 4
          {
            self.map[current.y as usize][current.x as usize] = 5;
          }
          current.y -= 1;
        }
      }

      if current == end {
        finished = true;
      } else
      // generate new number
      if current.y == end.y {
        rng_val = 0;
      } else if current.x == end.x {
        rng_val = 1;
      } else {
        let random_value = rng.gen_range(0..6);
        if random_value < 2 {
          rng_val = random_value;
        }
      }
    }
  }
}
