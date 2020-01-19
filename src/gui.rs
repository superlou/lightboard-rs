use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::graphics::{DrawParam, Rect, set_screen_coordinates};
use ggez::conf::WindowMode;
use ggez::event::{MouseButton, KeyCode, KeyMods};
use ggez::event::{self, EventHandler};
use ggez::nalgebra::Point2;
use crate::imgui_wrapper::ImGuiWrapper;
use crate::installation::Installation;
use std::sync::mpsc;

const INITIAL_WIDTH: f32 = 800.0;
const INITIAL_HEIGHT: f32 = 600.0;

pub fn run_gui(installation: Installation, dmx_send: mpsc::Sender<[u8; 8]>) {
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_gui", "Author")
        .window_mode(WindowMode {
            width: INITIAL_WIDTH,
            height: INITIAL_HEIGHT,
            resizable: true,
            ..WindowMode::default()
        })
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
    view: Rect,
    width: f32,
    height: f32,
}

impl Visualizer {
    pub fn new(ctx: &mut Context, hidpi_factor: f32, installation: Installation,
               dmx_send: mpsc::Sender<[u8; 8]>) -> Visualizer
    {
        let view_rect = Rect::new(0.0, 10.0, 10., -10.);
        set_screen_coordinates(ctx, view_rect).unwrap();

        Visualizer {
            imgui_wrapper: ImGuiWrapper::new(ctx),
            hidpi_factor: hidpi_factor,
            installation: installation,
            dmx_send: dmx_send,
            color: [0.0, 0.0, 0.0, 0.0],
            view: view_rect,
            width: INITIAL_WIDTH,
            height: INITIAL_HEIGHT,
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

        for (_name, fixture) in self.installation.fixtures() {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Point2::new(0.0, 0.0),
                1.0,
                0.01,
                graphics::WHITE,
            ).unwrap();
            let location = fixture.pos;
            // dbg!(location);
            graphics::draw(ctx, &circle, DrawParam::default().dest(location)).unwrap();
        }

        self.imgui_wrapper.render(ctx, self.hidpi_factor, &mut self.color);
        graphics::present(ctx)
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        let (width, height) = (self.width, self.height);
        let _screen_x = x / width * self.view.w + self.view.x;
        let _screen_y = y / height * self.view.h + self.view.y;
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) {
        self.width = width;
        self.height = height;
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
