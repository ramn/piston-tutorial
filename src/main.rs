extern crate piston_window;
use piston_window::*;


#[derive(Default)]
struct Game {
    rotation: f64,
    x: f64,
    y: f64,
    up_d: bool, down_d: bool, left_d: bool, right_d: bool
}


impl Game {
    fn new() -> Game {
        Game { ..Default::default() }
    }

    fn on_update(&mut self, upd: UpdateArgs) {
        self.rotation += 3.0 * upd.dt;

        if self.up_d {
            self.y += (-50.0) * upd.dt;
        }
        if self.down_d {
            self.y += (50.0) * upd.dt;
        }
        if self.left_d {
            self.x += (-50.0) * upd.dt;
        }
        if self.right_d {
            self.x += (50.0) * upd.dt;
        }
    }

    fn on_render(&mut self, ren: RenderArgs, e: PistonWindow) {
        e.draw_2d(|context, graphics| {
            clear([0.0, 0.0, 0.0, 1.0], graphics);
            let center = context.transform.trans((ren.width / 2) as f64, (ren.height / 2) as f64);
            let square = rectangle::square(0.0, 0.0, 100.0);
            let red = [1.0, 0.0, 0.0, 1.0];

            // We translate the rectangle slightly so that it's centered;
            // otherwise only the top left corner would be centered
            rectangle(
                red,
                square,
                center
                    .trans(self.x, self.y)
                    .rot_rad(self.rotation)
                    .trans(-50.0, -50.0),
                graphics);
        })
    }

    fn on_input(&mut self, inp: Input) {
        match inp {
            Input::Press(but) => {
                match but {
                    Button::Keyboard(Key::Up) => {
                        self.up_d = true;
                    }
                    Button::Keyboard(Key::Down) => {
                        self.down_d = true;
                    }
                    Button::Keyboard(Key::Left) => {
                        self.left_d = true;
                    }
                    Button::Keyboard(Key::Right) => {
                        self.right_d = true;
                    }
                    _ => {}
                }
            }
            Input::Release(but) => {
                match but {
                    Button::Keyboard(Key::Up) => {
                        self.up_d = false;
                    }
                    Button::Keyboard(Key::Down) => {
                        self.down_d = false;
                    }
                    Button::Keyboard(Key::Left) => {
                        self.left_d = false;
                    }
                    Button::Keyboard(Key::Right) => {
                        self.right_d = false;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}


fn main() {
    let window: PistonWindow = WindowSettings::new(
        "piston-tutorial",
        [600, 600]
        )
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new();

    for e in window {
        match e.event {
            Some(Event::Update(upd)) => {
                game.on_update(upd);
            }
            Some(Event::Render(rend)) => {
                game.on_render(rend, e);
            }
            Some(Event::Input(inp)) => {
                game.on_input(inp);
            }
            _ => {}
        };
    }
}
