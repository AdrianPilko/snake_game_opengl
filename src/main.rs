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

#[derive(Clone, PartialEq)]
enum Direction {
    left, right, up, down
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
        if last_direction != Direction::down => Direction::up,
      &Button::Keyboard(Key::Down) 
        if last_direction != Direction::up => Direction::down,
      &Button::Keyboard(Key::Left) 
        if last_direction != Direction::right => Direction::left,
      &Button::Keyboard(Key::Right)  
        if last_direction != Direction::left => Direction::right,
      _ => last_direction
    };
  }
}


struct Snake{
    x: i32,
    y: i32,
    direction: Direction,
}
impl Snake {
    fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs){        
          let black: [f32; 4] = [0.0, 0.0, 0.0, 1.0]; 
          let snake_block_size: f64 = 10.0;
          let my_square = graphics::rectangle::square(
            (self.x * snake_block_size as i32) as f64,           
            (self.y * snake_block_size as i32) as f64, 
//            (self.x *) as f64,           
  //          (self.y ) as f64, 
            snake_block_size);
          
          gl.draw(arg.viewport(), |_c, gl| {
            let transform = _c.transform;
            
            graphics::rectangle(black, my_square, transform, gl);
            });          
    }
    fn update(&mut self) {
        match self.direction{
            Direction::left => self.x = self.x - 1,
            Direction::right => self.x = self.x + 1,
            Direction::up => self.y = self.y - 1,
            Direction::down => self.y = self.y + 1,
        }
        if self.x <= 0 { 
          self.x = 0; 
        }
        if self.x >= 39 { 
          self.x = 39; 
        }
        if self.y <= 0 { 
          self.y = 0; 
        }
        if self.y >= 39 { 
          self.y = 39; 
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
        snake: Snake { x: snake_initial_pos_x, y:snake_initial_pos_y, direction: Direction::down},
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

