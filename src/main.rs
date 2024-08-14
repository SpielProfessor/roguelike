/***[ R O G U E L I K E . R S ]***\
 * a roguelike test written in   *
 * rust. Still very much in beta *
 * * * * * * * * * * * * * * * * *
 * Copyright (c) 2024 MrVollbart *
 *  - spielprofessor.github.io - *
 * Licensed under the GNU GPL 3.0*
 * License found in the LICENSE  *
 * file.                         *
 * THIS PROGRAM COMES WITH NO    *
 * WARRANTY OF ANY KIND.         *
 * * * * * * * * * * * * * * * * *
 * Depends on: NCurses, Rust,    *
 * Rust STD, rand library (rust) *
 * The rust NCurses library is   *
 * licensed under the MIT license*
\*********************************/

// define modules;
mod gen;
mod objects;
mod utils;

// load libraries & stuff from modules;
use ncurses::*;
use objects::*;
use rand::{thread_rng, Rng};
use std::sync::Arc;
use utils::*;

// the main game
#[derive(Clone)]
struct Game {
  // player x/y
  x: i32,
  y: i32,
  running: bool,
  map: Vec<Vec<i32>>,
  // sets tile IDs and textures/wether they've got collision
  tile_index: Vec<Tile>,
  discovered: DiscoveredMap,
}
impl Game {
  // initialize the game
  fn init(&mut self) {
    self.running = true;
    initscr();
    keypad(stdscr(), true);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    self.generate_level();
    self.discovered.discover_around(Vec2 {
      x: self.x,
      y: self.y,
    });
  }
  // main loop. Blocking
  fn game_loop(&mut self) {
    while self.running {
      self.draw();
      self.update();
    }
    endwin();
  }

  // draw game map
  fn draw_map(&mut self) {
    for y in 0..self.map.len() {
      for x in 0..self.map[y].len() {
        if self.discovered.content[y][x] == true {
          let tile = &self.get_map_tile(x as i32, y as i32).char;
          let _ = mvprintw(y as i32, x as i32, tile);
        }
      }
    }
  }

  // get tile at x/y in the game map. Returns tile at tile_index 0 if it doesn't exist
  fn get_map_tile(&self, x: i32, y: i32) -> &Tile {
    return &self
      .tile_index
      .get(self.get_map_tile_id(x, y) as usize)
      .unwrap();
  }
  // gets the tile's ID at x/y in the game map. Returns 0 if tile doesn't exist
  fn get_map_tile_id(&self, x: i32, y: i32) -> i32 {
    return *self
      .map
      .get(y as usize)
      .unwrap_or(&vec![0])
      .get(x as usize)
      .unwrap_or(&0);
  }

  // Well, a draw function that draws everything in the game
  fn draw(&mut self) {
    clear();
    let _ = mvprintw(0, 0, "x");
    let _ = mvprintw(0, SCREEN_W, "x");
    let _ = mvprintw(SCREEN_H, 0, "x");
    let _ = mvprintw(SCREEN_H, SCREEN_W, "x");
    self.draw_map();

    let _ = mvprintw(self.y, self.x, "@");
  }
  // update variables
  fn update(&mut self) {
    self.discovered.discover_around(Vec2 {
      x: self.x,
      y: self.y,
    });
    let key = getch();
    self.match_keys(key);
  }
  // get keyboard input and player movement
  fn match_keys(&mut self, key: i32) {
    let mut shade_x = self.x;
    let mut shade_y = self.y;
    match key {
      KEY_LEFT | 104 => shade_x -= 1,
      KEY_RIGHT | 108 => shade_x += 1,
      KEY_UP | 107 => shade_y -= 1,
      KEY_DOWN | 106 => shade_y += 1,
      // close on press of 'q', 'Q' or 'ESC'
      27 => {
        self.running = false;
      }
      113 | 81 => {
        self.running = false;
      }
      32 => {
        self
          .get_map_tile(self.x, self.y)
          .clone()
          .interact(self, self.x, self.y);
      }
      //a
      97 => {
        let _ = mvprintw(0, 0, "[INTERACT]: Which direction (<^v> HJKL .)?");
        let key = getch();
        let mut x = self.x;
        let mut y = self.y;
        match key {
          KEY_LEFT | 104 => {
            x -= 1;
          }
          KEY_RIGHT | 108 => {
            x += 1;
          }
          KEY_UP | 107 => {
            y -= 1;
          }
          KEY_DOWN | 106 => {
            y += 1;
          }
          46 => {}
          _ => {
            let _ = mvprintw(0, 0, "Invalid keypress (GAME HALTED)");
            getch();
          }
        }
        self.get_map_tile(x, y).clone().interact(self, x, y);
      }
      _ => (),
    }
    // collision detection
    if shade_x <= SCREEN_W && shade_x >= 0 && shade_y >= 0 && shade_y <= SCREEN_H {
      if self.get_map_tile(shade_x, shade_y).collidable == false {
        self.x = shade_x;
        self.y = shade_y;
      } else {
        // run-into action
        let act = self.get_map_tile(shade_x, shade_y).clone();
        act.interact(self, shade_x, shade_y);
      }
    }
  }
}

// Main function
fn main() {
  // initialize game
  let mut game = Game {
    discovered: DiscoveredMap::new(SCREEN_W, SCREEN_H),
    x: 2,
    y: 2,
    running: true,
    map: vec![
      vec![4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 4, 4, 4, 4, 4],
      vec![3, 1, 1, 1, 1, 1, 1, 3, 0, 0, 0, 3, 1, 1, 1, 3],
      vec![3, 1, 1, 1, 1, 1, 1, 6, 2, 2, 2, 5, 1, 1, 1, 3],
      vec![3, 1, 1, 1, 1, 1, 1, 3, 0, 0, 0, 3, 1, 1, 7, 3],
      vec![4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 4, 4, 4, 4, 4],
    ],
    tile_index: vec![
      // 0: empty tile
      Tile {
        collidable: true,
        char: " ".to_string(),
        ..Default::default()
      },
      // 1: floor tile
      Tile::default(),
      // 2: corridor tile
      Tile {
        collidable: false,
        char: "#".to_string(),
        ..Default::default()
      },
      // 3: vertical wall tile
      Tile {
        collidable: true,
        char: "|".to_string(),
        ..Default::default()
      },
      // 4: horizontal wall tile
      Tile {
        collidable: true,
        char: "-".to_string(),
        ..Default::default()
      },
      // 5: closed door tile
      Tile {
        collidable: true,
        char: "+".to_string(),
        ri_action: Arc::new(|game, x, y| {
          game.map[y as usize][x as usize] = 6;
        }),
      },
      // 6: open door tile
      Tile {
        collidable: false,
        char: "/".to_string(),
        ri_action: Arc::new(|game, x, y| {
          game.map[y as usize][x as usize] = 5;
        }),
      },
      Tile {
        collidable: false,
        char: ">".to_string(),
        ri_action: Arc::new(|game, _, _| {
          game.generate_level();
          game.discovered.discover_around(Vec2 {
            x: game.x,
            y: game.y,
          });
        }),
      },
    ],
  };
  // initialize game and run game loop
  game.init();
  game.game_loop();
}
