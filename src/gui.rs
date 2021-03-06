use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::graphics::{DrawParam, Rect, Text, set_screen_coordinates};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{MouseButton, KeyCode, KeyMods};
use ggez::event::{self, EventHandler};
use ggez::nalgebra::{Point2, Vector2};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::path::Path;
use std::thread;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use crate::imgui_wrapper::ImGuiWrapper;
use crate::installation::Installation;
use crate::fixture::{Fixture, ElementKind};
use crate::effect::EffectPool;
use crate::hitbox::HitboxManager;
use crate::ggez_util::mutate_from_key;
use crate::command_input_parser;
use crate::cue::CueList;
use crate::effect::Command;
use crate::command_input_parser::Chunk;

const INITIAL_WIDTH: f32 = 1000.0;
const INITIAL_HEIGHT: f32 = 600.0;

pub fn run_gui(installation: Installation, effect_pool: EffectPool,
               cue_list: CueList, dmx_send: mpsc::Sender<Vec<u8>>)
{
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_gui", "Author")
        .window_mode(WindowMode {
            width: INITIAL_WIDTH,
            height: INITIAL_HEIGHT,
            resizable: true,
            ..WindowMode::default()
        })
        .window_setup(WindowSetup {
            title: "Lightboard".to_owned(),
            ..WindowSetup::default()
        })
        .build()
        .expect("Could not create ggez context!");

    let mut gui = Visualizer::new(&mut ctx, 1.0, installation, effect_pool,
                                  cue_list, dmx_send);

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

type NotifyRx = Receiver<Result<notify::event::Event, notify::Error>>;

struct Visualizer {
    imgui_wrapper: ImGuiWrapper,
    hidpi_factor: f32,
    installation: Installation,
    effect_pool: EffectPool,
    cue_list: CueList,
    dmx_send: mpsc::Sender<Vec<u8>>,
    dmx_chain: Vec<u8>,
    selected: Vec<String>,
    hitbox_manager: HitboxManager,
    installation_view_origin: Point2<f32>,
    installation_view_scale: f32,
    dmx_status: DmxStatus,
    command_input_buffer: String,
    watcher: Option<RecommendedWatcher>,
    watcher_recv: Option<NotifyRx>,
}

impl Visualizer {
    pub fn new(ctx: &mut Context, hidpi_factor: f32,
               installation: Installation, effect_pool: EffectPool, cue_list: CueList,
               dmx_send: mpsc::Sender<Vec<u8>>) -> Self
    {
        let mut visualizer = Self {
            imgui_wrapper: ImGuiWrapper::new(ctx),
            hidpi_factor,
            installation,
            effect_pool,
            cue_list,
            dmx_send,
            dmx_chain: vec![],
            selected: vec![],
            hitbox_manager: HitboxManager::new(),
            installation_view_origin: Point2::new(10.0, 10.0),
            installation_view_scale: 40.0,
            dmx_status: DmxStatus::Unknown,
            command_input_buffer: String::new(),
            watcher: None,
            watcher_recv: None,
        };

        visualizer.watch("patterns");
        visualizer.update_hitboxes();

        visualizer
    }

    fn update_hitboxes(&mut self) {
        let origin = self.installation_view_origin;
        let scale = self.installation_view_scale;
        self.hitbox_manager.clear();

        for (name, fixture) in self.installation.fixtures() {
            let location = fixture.pos();
            let rect = Rect::new(location.x * scale + origin.coords.x,
                                 location.y * scale + origin.coords.y,
                                 fixture.elements().len() as f32 * 1.0 * scale,
                                 1.0 * scale);
            self.hitbox_manager.add(rect, name);
        }
    }

    fn watch<P: AsRef<Path>>(&mut self, path: P) {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher: RecommendedWatcher = Watcher::new_immediate(move |res| tx.send(res).unwrap()).unwrap();
        watcher.watch(path, RecursiveMode::Recursive).unwrap();
        self.watcher = Some(watcher);
        self.watcher_recv = Some(rx);
    }
}

lazy_static! {
    static ref COLOR_FIXTURE_BG: graphics::Color = graphics::Color::new(0.1, 0.1, 0.1, 1.0);
    static ref COLOR_FIXTURE_OUTLINE: graphics::Color = graphics::Color::new(0.4, 0.4, 0.4, 1.0);
    static ref COLOR_FIXTURE_OUTLINE_SELECTED: graphics::Color = graphics::Color::new(1.0, 1.0, 1.0, 1.0);
}

fn render_installation(ctx: &mut Context, installation: &Installation, selected: &[String],
                       origin: Point2<f32>, scale: f32)
{
    for (name, fixture) in installation.fixtures() {
        let is_selected = selected.contains(name);
        draw_fixture(ctx, fixture, name, is_selected, origin, scale);
    }
}

fn draw_fixture(ctx: &mut Context, fixture: &Fixture, name: &str, is_selected: bool,
                origin: Point2<f32>, scale: f32)
{
    let location = fixture.pos();
    let origin = origin.coords;

    let background = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        Rect::new(0.0, 0.0, fixture.elements().len() as f32 * 1.0 * scale, 1.0 * scale),
        *COLOR_FIXTURE_BG,
    ).unwrap();
    graphics::draw(ctx, &background, DrawParam::default().dest(location * scale + origin)).unwrap();

    for (_name, element) in fixture.elements().iter() {
        let i = element.pos().0;

        match element.kind() {
            ElementKind::Intensity(intensity) => {
                let color = graphics::Color::new(*intensity, *intensity, *intensity, 1.0);
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
            ElementKind::Rgbiu{rgb: color, ..} => {
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
        Rect::new(0.0, 0.0, fixture.elements().len() as f32 * 1.0 * scale, 1.0 * scale),
        *COLOR_FIXTURE_OUTLINE,
    ).unwrap();
    graphics::draw(ctx, &outline, DrawParam::default().dest(location * scale + origin)).unwrap();

    if is_selected {
        let outline = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            Rect::new(0.0, 0.0, fixture.elements().len() as f32 * 1.0 * scale, 1.0 * scale),
            *COLOR_FIXTURE_OUTLINE_SELECTED,
        ).unwrap();
        graphics::draw(ctx, &outline, DrawParam::default().dest(location * scale + origin)).unwrap();

        for (name, element) in fixture.elements().iter() {
            let i = element.pos().0;
            let label = Text::new(name.clone());
            let dim = label.dimensions(ctx);
            let draw_param = DrawParam::default()
                .dest(location * scale + origin + Vector2::new(i as f32 * 1.0 * scale, 1.0 * scale - dim.1 as f32))
                .color(graphics::WHITE);
            graphics::draw(ctx, &label, draw_param).unwrap();
        }

        let channel_text = Text::new(format!("CH {}", fixture.channel()));
        let draw_param = DrawParam::default()
            .dest(location * scale + origin + Vector2::new(0.0, 1.0 * scale))
            .color(graphics::WHITE)
            .scale(Vector2::new(0.8, 0.8));
        graphics::draw(ctx, &channel_text, draw_param).unwrap();
    }

    let label = Text::new(name);
    graphics::draw(ctx, &label, (location * scale + origin, graphics::WHITE)).unwrap();
}

impl EventHandler for Visualizer {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !ggez::timer::check_update_time(ctx, 30) {
            return Ok(())
        }

        if let Some(rx) = &self.watcher_recv {
            // Drain all events, then reload patterns, since many events can
            // be received at once.
            // todo Consider a debounce.
            let mut changed = false;

            while rx.try_recv().is_ok() {
                changed = true;
            }

            if changed {
                self.effect_pool.reload_patterns();
            }
        }

        self.effect_pool.run_commands();
        self.effect_pool.apply_to(&mut self.installation);

        let chain = self.installation.build_dmx_chain();
        self.dmx_chain = chain.clone();
        self.dmx_status = match self.dmx_send.send(chain) {
            Ok(_) => DmxStatus::Ok,
            Err(_) => DmxStatus::Error,
        };
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        render_installation(ctx, &self.installation, &self.selected,
                            self.installation_view_origin, self.installation_view_scale);
        self.imgui_wrapper.render(ctx, self.hidpi_factor, &mut self.effect_pool,
                                  &self.cue_list,
                                  &self.dmx_status, &self.dmx_chain,
                                  &self.command_input_buffer);
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
        mutate_from_key(&mut self.command_input_buffer, keycode);

        match keycode {
            KeyCode::Return | KeyCode::NumpadEnter => {
                let chunks = command_input_parser::parse(&self.command_input_buffer);
                let commands = expand_cues(chunks, &self.cue_list);
                self.effect_pool.add_commands(commands);
                self.command_input_buffer.clear();
            },
            _ => {},
        }
    }
}

pub fn expand_cues(chunks: Vec<Chunk>, cue_list: &CueList) -> Vec<Command> {
    let mut commands = vec![];

    for chunk in chunks {
        match chunk {
            Chunk::Effect(e) => commands.push(e),
            Chunk::CueNum(i) => {
                let command = cue_list.cue_command(i - 1);
                let chunks = command_input_parser::parse(&command.unwrap());
                let mut result = expand_cues(chunks, cue_list);
                commands.append(&mut result);
            }
        }
    }

    commands
}
