extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

/// game written following (loosely) the tutorial from
/// YouCodeThings
/// https://www.youtube.com/watch?v=HCwMb0KslX8
///

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::vec::Vec;
use std::convert::From;

#[derive(Clone, PartialEq)]
enum Direction {
    Left, Right, Up, Down
}

struct Game { 
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
               
        let green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(green, gl);
        });
        
        self.snake.render(&mut self.gl , arg);
    }
  fn update (&mut self) {
    self.snake.update();
    }
  
  fn button_pressed(&mut self, button: &Button) 
  {
    let last_direction = self.snake.direction.clone();
    
    self.snake.direction = match button {
      &Button::Keyboard(Key::Up) 
        if last_direction != Direction::Down => Direction::Up,
      &Button::Keyboard(Key::Down) 
        if last_direction != Direction::Up => Direction::Down,
      &Button::Keyboard(Key::Left) 
        if last_direction != Direction::Right => Direction::Left,
      &Button::Keyboard(Key::Right)  
        if last_direction != Direction::Left => Direction::Right,
      _ => last_direction
    };
  }
}

pub struct SnakeBodyElement
{
  x: i32,
  y: i32,
}
impl From<(i32, i32)> for SnakeBodyElement {
  fn from(t: (i32, i32)) -> SnakeBodyElement {
    SnakeBodyElement { x: t.0, y: t.1}
  }
}

struct Snake{
    body: Vec<SnakeBodyElement>,
    direction: Direction,
}
impl Snake {
    fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs){        
          let black: [f32; 4] = [0.0, 0.0, 0.0, 1.0]; 
          let snake_block_size: f64 = 10.0;
          let my_square = graphics::rectangle::square(
            (self.body[0].x * snake_block_size as i32) as f64,           
            (self.body[0].y * snake_block_size as i32) as f64, 
            snake_block_size);
          
          gl.draw(arg.viewport(), |_c, gl| {
            let transform = _c.transform;
            
            graphics::rectangle(black, my_square, transform, gl);
            });          
    } 
    fn update(&mut self) {
        match self.direction{
            Direction::Left => self.body[0].x = self.body[0].x - 1,
            Direction::Right => self.body[0].x = self.body[0].x + 1,
            Direction::Up => self.body[0].y = self.body[0].y - 1,
            Direction::Down => self.body[0].y = self.body[0].y + 1,
        }
        if self.body[0].x <= 0 { 
          self.body[0].x = 0; 
        }
        if self.body[0].x >= 39 { 
          self.body[0].x = 39; 
        }
        if self.body[0].y <= 0 { 
          self.body[0].y = 0; 
        }
        if self.body[0].y >= 39 { 
          self.body[0].y = 39; 
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    
    let play_area_size_x = 400;
    let play_area_size_y = 400;
    let snake_initial_pos_x = 10;
    let snake_initial_pos_y = 10;
    
    let mut window: GlutinWindow = WindowSettings::new(
          "Snake Game", 
          [play_area_size_x, play_area_size_y]
    ).opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake { 
            body: vec![(1, 1).into(),(0, 1).into()],
            direction: Direction::Down}
    };

    let mut events = Events::new(EventSettings::new().ups(30));

    while let Some(e) = events.next(&mut window) {
      
        if let Some(r) = e.render_args() {
            game.render(&r);            
        }
        
        if let Some(u) = e.update_args(){          
            game.update();
        }
        
        if let Some(k) = e.button_args(){          
            if k.state == ButtonState::Press {
            game.button_pressed(&k.button);
            }
        }
        
    }
}

