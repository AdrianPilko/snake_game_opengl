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
}


struct Snake{
    x: i32,
    y: i32,
}
impl Snake {
    fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs){        
          let black: [f32; 4] = [0.0, 0.0, 0.0, 1.0]; 
          let my_square = graphics::rectangle::square(self.x as f64,           
            self.y as f64, 
            50_f64);
          
          gl.draw(arg.viewport(), |_c, gl| {
            let transform = _c.transform;
            
            graphics::rectangle(black, my_square, transform, gl);
            });          
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    
    let mut window: GlutinWindow = WindowSettings::new(
          "Snake Game", 
          [300, 300]
    ).opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake { x: 100, y:100 },
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);            
        }
    }
}

