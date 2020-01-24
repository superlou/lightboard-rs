use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::graphics::{DrawParam, Rect, set_screen_coordinates};
use ggez::conf::WindowMode;
use ggez::event::{MouseButton, KeyCode, KeyMods};
use ggez::event::{self, EventHandler};
use ggez::nalgebra::Point2;
use std::sync::mpsc;
use crate::imgui_wrapper::ImGuiWrapper;
use crate::installation::Installation;
use crate::installation::Fixture;
use crate::scene::SceneManager;

const INITIAL_WIDTH: f32 = 800.0;
const INITIAL_HEIGHT: f32 = 600.0;

pub fn run_gui(installation: Installation, scene_manager: SceneManager, dmx_send: mpsc::Sender<[u8; 8]>) {
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_gui", "Author")
        .window_mode(WindowMode {
            width: INITIAL_WIDTH,
            height: INITIAL_HEIGHT,
            resizable: true,
            ..WindowMode::default()
        })
        .build()
        .expect("Could not create ggez context!");

    let mut gui = Visualizer::new(&mut ctx, 1.0, installation, scene_manager,
                                  dmx_send);

    match event::run(&mut ctx, &mut event_loop, &mut gui) {
        Ok(_) => println!("Exited GUI"),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct Visualizer {
    imgui_wrapper: ImGuiWrapper,
    hidpi_factor: f32,
    installation: Installation,
    scene_manager: SceneManager,
    dmx_send: mpsc::Sender<[u8; 8]>,
    color: [f32; 4],
    view: Rect,
    width: f32,
    height: f32,
    selected: Vec<String>,
}

fn make_view_rect(installation: (f32, f32), window: (f32, f32)) -> Rect {
    let install_aspect_ratio = &installation.1 / &installation.0;
    let window_aspect_ratio = window.1 / window.0;

    let width;
    let height;

    if window_aspect_ratio < install_aspect_ratio {
        height = installation.1;
        width = height / window_aspect_ratio;
    } else {
        width = installation.0;
        height = width * window_aspect_ratio;
    }

    Rect::new(0.0, height, width, -height)
}

impl Visualizer {
    pub fn new(ctx: &mut Context, hidpi_factor: f32,
               installation: Installation, scene_manager: SceneManager,
               dmx_send: mpsc::Sender<[u8; 8]>) -> Visualizer
    {
        let view_rect = make_view_rect((installation.size().0, installation.size().1),
                                       (INITIAL_WIDTH, INITIAL_HEIGHT));
        set_screen_coordinates(ctx, view_rect).unwrap();

        Visualizer {
            imgui_wrapper: ImGuiWrapper::new(ctx),
            hidpi_factor: hidpi_factor,
            installation: installation,
            scene_manager: scene_manager,
            dmx_send: dmx_send,
            color: [0.0, 0.0, 0.0, 0.0],
            view: view_rect,
            width: INITIAL_WIDTH,
            height: INITIAL_HEIGHT,
            selected: vec![],
        }
    }
}

lazy_static! {
    static ref COLOR_FIXTURE_BG: graphics::Color = graphics::Color::new(0.1, 0.1, 0.1, 1.0);
    static ref COLOR_FIXTURE_OUTLINE: graphics::Color = graphics::Color::new(0.4, 0.4, 0.4, 1.0);
    static ref COLOR_FIXTURE_OUTLINE_SELECTED: graphics::Color = graphics::Color::new(1.0, 1.0, 1.0, 1.0);
}

fn draw_fixture(fixture: &Fixture, ctx: &mut Context, is_selected: bool) {
    let location = fixture.pos;

    let background = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        Rect::new(0.0, 0.0, fixture.elements.len() as f32 * 1.0, 1.0),
        *COLOR_FIXTURE_BG,
    ).unwrap();
    graphics::draw(ctx, &background, DrawParam::default().dest(location)).unwrap();

    for (i, (_name, element)) in fixture.elements.iter().enumerate() {
        let color = graphics::Color::new(element.color().0,
                                         element.color().1,
                                         element.color().2,
                                         1.0);

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2::new(i as f32 * 1.0 + 0.5, 0.5),
            0.5,
            0.001,
            color,
        ).unwrap();
        graphics::draw(ctx, &circle, DrawParam::default().dest(location)).unwrap();
    }

    let outline = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::stroke(0.04),
        Rect::new(0.0, 0.0, fixture.elements.len() as f32 * 1.0, 1.0),
        *COLOR_FIXTURE_OUTLINE,
    ).unwrap();
    graphics::draw(ctx, &outline, DrawParam::default().dest(location)).unwrap();

    if is_selected {
        let outline = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(0.04),
            Rect::new(0.0, 0.0, fixture.elements.len() as f32 * 1.0, 1.0),
            *COLOR_FIXTURE_OUTLINE_SELECTED,
        ).unwrap();
        graphics::draw(ctx, &outline, DrawParam::default().dest(location)).unwrap();
    }
}

impl Visualizer {
    fn screen_coordinates_for(&self, x: f32, y: f32) -> (f32, f32) {
        let (width, height) = (self.width, self.height);
        let screen_x = x / width * self.view.w + self.view.x;
        let screen_y = y / height * self.view.h + self.view.y;
        (screen_x, screen_y)
    }
}

impl EventHandler for Visualizer {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !ggez::timer::check_update_time(ctx, 30) {
            return Ok(())
        }

        self.scene_manager.apply_to(&mut self.installation);

        let red = (self.color[0] * 255.0) as u8;
        let green = (self.color[1] * 255.0) as u8;
        let blue = (self.color[2] * 255.0) as u8;
        self.dmx_send.send([0, 0, 0, 255, red, green, blue, 0]).unwrap();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        for (name, fixture) in self.installation.fixtures() {
            let is_selected = self.selected.as_slice().contains(name);
            draw_fixture(fixture, ctx, is_selected);
        }

        self.imgui_wrapper.render(ctx, self.hidpi_factor, &mut self.scene_manager);
        graphics::present(ctx)
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let view_rect = make_view_rect((self.installation.size().0, self.installation.size().1),
                                       (width, height));
        set_screen_coordinates(ctx, view_rect).unwrap();
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.imgui_wrapper.update_mouse_down((
            button == MouseButton::Left,
            button == MouseButton::Right,
            button == MouseButton::Middle,
        ));

        if self.imgui_wrapper.want_capture_mouse() {
            return;
        }

        // todo This is really gross. There needs to be a rendered installation
        // that knows about rendering info.
        let (screen_x, screen_y) = self.screen_coordinates_for(x, y);

        self.selected = self.installation.fixtures().iter().filter_map(|(name, fixture)| {
            let bounds = Rect::new(fixture.pos.x, fixture.pos.y,
                                   fixture.elements.len() as f32 * 1.0, 1.0);
            if bounds.contains(Point2::new(screen_x, screen_y)) {
                Some(name.clone())
            } else {
                None
            }
        }).collect();
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
