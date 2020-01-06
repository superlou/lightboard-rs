use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{MouseButton, KeyCode, KeyMods};
use ggez::event::{self, EventHandler};
use crate::imgui_wrapper::ImGuiWrapper;
use crate::installation::Installation;
use std::sync::mpsc;

pub fn run_gui(installation: Installation, dmx_send: mpsc::Sender<[u8; 8]>) {
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_gui", "Author")
        .build()
        .expect("Could not create ggez context!");

    let mut gui = Visualizer::new(&mut ctx, 1.0, installation, dmx_send);

    match event::run(&mut ctx, &mut event_loop, &mut gui) {
        Ok(_) => println!("Exited GUI"),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct Visualizer {
    imgui_wrapper: ImGuiWrapper,
    hidpi_factor: f32,
    installation: Installation,
    dmx_send: mpsc::Sender<[u8; 8]>,
    color: [f32; 4],
}

impl Visualizer {
    pub fn new(ctx: &mut Context, hidpi_factor: f32, installation: Installation,
               dmx_send: mpsc::Sender<[u8; 8]>) -> Visualizer
    {
        Visualizer {
            imgui_wrapper: ImGuiWrapper::new(ctx),
            hidpi_factor: hidpi_factor,
            installation: installation,
            dmx_send: dmx_send,
            color: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

impl EventHandler for Visualizer {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !ggez::timer::check_update_time(ctx, 30) {
            return Ok(())
        }

        let red = (self.color[0] * 255.0) as u8;
        let green = (self.color[1] * 255.0) as u8;
        let blue = (self.color[2] * 255.0) as u8;
        self.dmx_send.send([0, 0, 0, 255, red, green, blue, 0]).unwrap();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        self.imgui_wrapper.render(ctx, self.hidpi_factor, &mut self.color);
        graphics::present(ctx)
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        self.imgui_wrapper.update_mouse_down((
            button == MouseButton::Left,
            button == MouseButton::Right,
            button == MouseButton::Middle,
        ));
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        self.imgui_wrapper.update_mouse_down((false, false, false));
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::P => {
                self.imgui_wrapper.open_popup();
            }
            _ => (),
        }
    }
}
