use std::f64::consts::PI;

use piston_window::*;
use gfx_device_gl::Resources;


#[derive(Default)]
pub struct Object {
    x: f64,
    y: f64,
    rotation: f64,
    sprite: Option<Texture<Resources>>,
    tur_x: f64,
    tur_y: f64,
    rot_tur: f64,
    turret: Option<Texture<Resources>>
}


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

    pub fn render(&self, g: &mut G2d, view: math::Matrix2d) {
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
                let (spritex, spritey) = sprite.get_size();
                let (ocx, ocy) = (spritex / 2, spritey / 2);
                image(
                    sprite,
                    view
                        .trans(self.x, self.y)
                        .scale(0.75, 0.75)
                        .rot_rad(self.rotation)
                        .trans(-(ocx as f64), -(ocy as f64)),
                    g);
            }
        }

        match self.turret {
            None => {}
            Some(ref sprite) => {
                let (spritex, spritey) = sprite.get_size();
                let (ocx, ocy) = (spritex / 2, spritey / 2);
                image(
                    sprite,
                    view
                        .trans(self.x, self.y)
                        .scale(0.75, 0.75)
                        .rot_rad(self.rot_tur)
                        .trans(-(ocx as f64), -(ocy as f64)),
                    g);
            }
        }
    }

    pub fn set_sprite(&mut self, sprite: Texture<Resources>) {
        self.sprite = Some(sprite);
    }

    pub fn rotate(&mut self, r: f64) {
        self.rotation += r;
    }

    pub fn rotate_to(&mut self, r: f64) {
        self.rotation = r;
    }

    pub fn forward(&mut self, direction: f64) {
        self.x += direction * -self.rotation.sin();
        self.y += direction * self.rotation.cos();
    }

    pub fn set_turret_sprite(&mut self, sprite: Texture<Resources>) {
        self.turret = Some(sprite);
    }

    pub fn point_tur_to(&mut self, x: f64, y: f64) {
        self.tur_x = x;
        self.tur_y = y;
    }

    pub fn update(&mut self, dt: f64) {
        self.calc_tur_pos(dt);
    }

    fn calc_tur_pos(&mut self, dt: f64) {
        let mut new_rot = -(self.tur_x - self.x).atan2(self.tur_y - self.y);
        if new_rot == self.rot_tur {
            return;
        }
        if new_rot < self.rot_tur && self.rot_tur - new_rot > new_rot + 2.0 * PI - self.rot_tur {
            new_rot += 2.0 * PI;
        }
        if new_rot > self.rot_tur && new_rot - self.rot_tur > self.rot_tur + 2.0 * PI - new_rot {
            new_rot -= 2.0 * PI;
        }
        let rot_speed = 1.0;
        if new_rot > self.rot_tur {
            if new_rot - self.rot_tur > rot_speed * dt {
                self.rot_tur += rot_speed * dt;
            } else {
                self.rot_tur = new_rot;
            }
        } else {
            if self.rot_tur - new_rot > rot_speed * dt {
                self.rot_tur -= rot_speed * dt;
            } else {
                self.rot_tur = new_rot;
            }
        }
        if self.rot_tur > 2.0 * PI {
            self.rot_tur -= 2.0 * PI;
        }
        if self.rot_tur < 0.0 {
            self.rot_tur += 2.0 * PI;
        }
    }
}
