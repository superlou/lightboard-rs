use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::graphics::{DrawParam, Rect, Text, set_screen_coordinates};
use ggez::conf::WindowMode;
use ggez::event::{MouseButton, KeyCode, KeyMods};
use ggez::event::{self, EventHandler};
use ggez::nalgebra::{Point2, Vector2};
use std::sync::mpsc;
use crate::imgui_wrapper::ImGuiWrapper;
use crate::installation::Installation;
use crate::fixture::{Fixture, ElementKind};
use crate::scene::SceneManager;
use crate::hitbox::HitboxManager;

const INITIAL_WIDTH: f32 = 800.0;
const INITIAL_HEIGHT: f32 = 600.0;

pub fn run_gui(installation: Installation, scene_manager: SceneManager,
               dmx_send: mpsc::Sender<Vec<u8>>)
{
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_gui", "Author")
        .window_mode(WindowMode {
            width: INITIAL_WIDTH,
            height: INITIAL_HEIGHT,
            resizable: true,
            ..WindowMode::default()
        })
        .build()
        .expect("Could not create ggez context!");

    let mut gui = Visualizer::new(&mut ctx, 1.0, installation, scene_manager, dmx_send);

    match event::run(&mut ctx, &mut event_loop, &mut gui) {
        Ok(_) => println!("Exited GUI"),
        Err(e) => println!("Error occured: {}", e),
    }
}

#[derive(PartialEq)]
pub enum DmxStatus {
    Ok,
    Error,
    Unknown,
}

struct Visualizer {
    imgui_wrapper: ImGuiWrapper,
    hidpi_factor: f32,
    installation: Installation,
    scene_manager: SceneManager,
    dmx_send: mpsc::Sender<Vec<u8>>,
    selected: Vec<String>,
    hitbox_manager: HitboxManager,
    installation_view_origin: Point2<f32>,
    installation_view_scale: f32,
    dmx_status: DmxStatus,
}

impl Visualizer {
    pub fn new(ctx: &mut Context, hidpi_factor: f32,
               installation: Installation, scene_manager: SceneManager,
               dmx_send: mpsc::Sender<Vec<u8>>) -> Self
    {
        let mut visualizer = Self {
            imgui_wrapper: ImGuiWrapper::new(ctx),
            hidpi_factor: hidpi_factor,
            installation: installation,
            scene_manager: scene_manager,
            dmx_send: dmx_send,
            selected: vec![],
            hitbox_manager: HitboxManager::new(),
            installation_view_origin: Point2::new(10.0, 10.0),
            installation_view_scale: 40.0,
            dmx_status: DmxStatus::Unknown,
        };

        visualizer.update_hitboxes();

        visualizer
    }

    fn update_hitboxes(&mut self) {
        let origin = self.installation_view_origin;
        let scale = self.installation_view_scale;
        self.hitbox_manager.clear();

        for (name, fixture) in self.installation.fixtures() {
            let location = fixture.pos;
            let rect = Rect::new(location.x * scale + origin.coords.x,
                                 location.y * scale + origin.coords.y,
                                 fixture.elements.len() as f32 * 1.0 * scale,
                                 1.0 * scale);
            self.hitbox_manager.add(rect, name);
        }
    }
}

lazy_static! {
    static ref COLOR_FIXTURE_BG: graphics::Color = graphics::Color::new(0.1, 0.1, 0.1, 1.0);
    static ref COLOR_FIXTURE_OUTLINE: graphics::Color = graphics::Color::new(0.4, 0.4, 0.4, 1.0);
    static ref COLOR_FIXTURE_OUTLINE_SELECTED: graphics::Color = graphics::Color::new(1.0, 1.0, 1.0, 1.0);
}

fn render_installation(ctx: &mut Context, installation: &Installation, selected: &Vec<String>,
                       origin: &Point2<f32>, scale: f32)
{
    for (name, fixture) in installation.fixtures() {
        let is_selected = selected.as_slice().contains(name);
        draw_fixture(ctx, fixture, name, is_selected, &origin, scale);
    }
}

fn draw_fixture(ctx: &mut Context, fixture: &Fixture, name: &str, is_selected: bool,
                origin: &Point2<f32>, scale: f32)
{
    let location = fixture.pos;
    let origin = origin.coords;

    let background = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        Rect::new(0.0, 0.0, fixture.elements.len() as f32 * 1.0 * scale, 1.0 * scale),
        *COLOR_FIXTURE_BG,
    ).unwrap();
    graphics::draw(ctx, &background, DrawParam::default().dest(location * scale + origin)).unwrap();

    for (i, (_name, element)) in fixture.elements.iter().enumerate() {
        match element.kind() {
            ElementKind::Rgbi(color) => {
                let color = graphics::Color::new(color.r(), color.g(), color.b(), 1.0);
                let circle = graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    Point2::new(i as f32 * 1.0 + 0.5, 0.5) * scale,
                    0.5 * scale,
                    0.001,
                    color,
                ).unwrap();
                graphics::draw(ctx, &circle, DrawParam::default().dest(location * scale + origin)).unwrap();
            },
            ElementKind::Rgbiu{rgb: color, uv: _} => {
                let color = graphics::Color::new(color.r(), color.g(), color.b(), 1.0);
                let circle = graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    Point2::new(i as f32 * 1.0 + 0.5, 0.5) * scale,
                    0.5 * scale,
                    0.001,
                    color,
                ).unwrap();
                graphics::draw(ctx, &circle, DrawParam::default().dest(location * scale + origin)).unwrap();
            }
            _ => {},
        }

    }

    let outline = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::stroke(1.0),
        Rect::new(0.0, 0.0, fixture.elements.len() as f32 * 1.0 * scale, 1.0 * scale),
        *COLOR_FIXTURE_OUTLINE,
    ).unwrap();
    graphics::draw(ctx, &outline, DrawParam::default().dest(location * scale + origin)).unwrap();

    if is_selected {
        let outline = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            Rect::new(0.0, 0.0, fixture.elements.len() as f32 * 1.0 * scale, 1.0 * scale),
            *COLOR_FIXTURE_OUTLINE_SELECTED,
        ).unwrap();
        graphics::draw(ctx, &outline, DrawParam::default().dest(location * scale + origin)).unwrap();

        for (i, (name, _element)) in fixture.elements.iter().enumerate() {
            let label = Text::new(name.clone());
            let dim = label.dimensions(ctx);
            graphics::draw(ctx, &label, (location * scale + origin + Vector2::new(i as f32 * 1.0 * scale, 1.0 * scale - dim.1 as f32), graphics::WHITE)).unwrap();
        }
    }

    let label = Text::new(name);
    graphics::draw(ctx, &label, (location * scale + origin, graphics::WHITE)).unwrap();
}

impl EventHandler for Visualizer {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !ggez::timer::check_update_time(ctx, 30) {
            return Ok(())
        }

        self.scene_manager.apply_to(&mut self.installation);

        let chain = self.installation.build_dmx_chain();
        self.dmx_status = match self.dmx_send.send(chain) {
            Ok(_) => DmxStatus::Ok,
            Err(_) => DmxStatus::Error,
        };
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        render_installation(ctx, &self.installation, &self.selected,
                            &self.installation_view_origin, self.installation_view_scale);
        self.imgui_wrapper.render(ctx, self.hidpi_factor, &mut self.scene_manager, &self.dmx_status);
        graphics::present(ctx)
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let view_rect = Rect::new(0.0, 0.0, width, height);
        self.update_hitboxes();
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

        let point = Point2::new(x, y);
        self.selected = self.hitbox_manager.targets_at(point);
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
