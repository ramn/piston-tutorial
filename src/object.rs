use piston_window::*;
use gfx_device_gl::{Resources, Output, CommandBuffer};
use gfx_graphics::GfxGraphics;


#[derive(Default)]
pub struct Object {
    x: f64,
    y: f64,
    sprite: Option<Texture<Resources>>
}


type Gfx<'a> = GfxGraphics<'a, Resources, CommandBuffer<Resources>, Output>;


impl Object {
    pub fn new() -> Object {
        Object { ..Default::default() }
    }

    pub fn mov(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }

    pub fn mov_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn render(&self, g: &mut Gfx, view: math::Matrix2d) {
        let square = rectangle::square(0.0, 0.0, 100.0);
        let red = [1.0, 0.0, 0.0, 1.0];
        match self.sprite {
            None => {
                // We translate the rectangle slightly so that it's centered; otherwise
                // only the top left corner would be centered
                rectangle(
                    red,
                    square,
                    view.trans(self.x, self.y).trans(-50.0, -50.0),
                    g);
            }
            Some(ref sprite) => {
                image(sprite, view.trans(self.x, self.y).trans(-50.0, -50.0), g);
            }
        }
    }

    pub fn set_sprite(&mut self, sprite: Texture<Resources>) {
        self.sprite = Some(sprite);
    }
}
