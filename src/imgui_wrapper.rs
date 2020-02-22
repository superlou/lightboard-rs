use std::time::Instant;
use imgui::{im_str, ImString, StyleVar, StyleColor};
use imgui_gfx_renderer::{Shaders, Renderer};
use gfx_core::memory::Typed;
use gfx_core::handle::RenderTargetView;
use gfx_device_gl;
use ggez::{graphics, Context};
use crate::effect::EffectPool;
use crate::gui::DmxStatus;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
  pos: (i32, i32),
  pressed: (bool, bool, bool),
  wheel: f32,
}

pub struct ImGuiWrapper {
    pub imgui: imgui::Context,
    pub renderer: Renderer<gfx_core::format::Rgba8, gfx_device_gl::Resources>,
    last_frame: Instant,
    mouse_state: MouseState,
    show_popup: bool,
}

const ROW_KEY_MAP: [&str; 6] = ["A", "B", "C", "D", "E", "F"];

fn effect_pool_ui(ui: &imgui::Ui, effect_pool: &mut EffectPool) {
    let num_columns = 5;
    let num_rows = 2;

    for row in 0..num_rows {
        ui.columns(num_columns, im_str!("test"), true);

        for col in 0..num_columns {
            let key = format!("{}{}", ROW_KEY_MAP[row], col + 1);
            if let Some(effect) = effect_pool.get_effect_by_key(&key) {
                let id = im_str!("##{},{}", row, col);

                let border_token = ui.push_style_var(StyleVar::FrameBorderSize(1.0));

                let strength = effect.strength();
                let strength_color = [strength, strength, strength, strength];
                let color_token = ui.push_style_color(StyleColor::FrameBg, strength_color);
                let hover_token = ui.push_style_color(StyleColor::FrameBgHovered, strength_color);
                let active_token = ui.push_style_color(StyleColor::FrameBgActive, strength_color);

                imgui::VerticalSlider::new(&id, [12.0, 80.0], 0.0..=1.0)
                    .display_format(im_str!(""))
                    .build(&ui, effect.strength_mut());

                color_token.pop(&ui);
                border_token.pop(&ui);
                hover_token.pop(&ui);
                active_token.pop(&ui);

                ui.same_line(26.0);

                let token = ui.begin_group();
                ui.text(&ImString::new(effect.name().clone()));
                let key = im_str!("{}{}", ROW_KEY_MAP[row as usize], col + 1);
                ui.text(key);
                token.end(&ui);
            }

            ui.next_column();
        }
        ui.columns(1, im_str!("test"), true);
        ui.separator();
    }
}

impl ImGuiWrapper {
    pub fn new(ctx: &mut Context) -> Self {
      let mut imgui = imgui::Context::create();
      let (factory, gfx_device, _, _, _) = graphics::gfx_objects(ctx);

      let shaders = {
        let version = gfx_device.get_info().shading_language;
        if version.is_embedded {
          if version.major >= 3 {
            Shaders::GlSlEs300
          } else {
            Shaders::GlSlEs100
          }
        } else if version.major >= 4 {
          Shaders::GlSl400
        } else if version.major >= 3 {
          Shaders::GlSl130
        } else {
          Shaders::GlSl110
        }
      };

      let renderer = Renderer::init(&mut imgui, &mut *factory, shaders).unwrap();

      Self {
        imgui: imgui,
        renderer: renderer,
        last_frame: Instant::now(),
        mouse_state: MouseState::default(),
        show_popup: false,
      }
    }

    pub fn want_capture_mouse(&self) -> bool {
         self.imgui.io().want_capture_mouse
    }

    pub fn render(&mut self, ctx: &mut Context, hidpi_factor: f32,
                  effect_pool: &mut EffectPool, dmx_status: &DmxStatus, dmx_chain: &Vec<u8>)
    {
        self.update_mouse();

        let now = Instant::now();
        let delta = now - self.last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1.0e9;
        self.last_frame = now;

        let (draw_width, draw_height) = graphics::drawable_size(ctx);
        self.imgui.io_mut().display_size = [draw_width, draw_height];
        self.imgui.io_mut().display_framebuffer_scale = [hidpi_factor, hidpi_factor];
        self.imgui.io_mut().delta_time = delta_s;

        let ui = self.imgui.frame();
        let window_rounding = ui.push_style_var(StyleVar::WindowRounding(0.0));

        imgui::Window::new(im_str!("Effect Pool"))
          .size([300.0, 300.0], imgui::Condition::FirstUseEver)
          .position([100.0, 100.0], imgui::Condition::FirstUseEver)
          .build(&ui, || { effect_pool_ui(&ui, effect_pool) });

        imgui::Window::new(im_str!("DMX Channels"))
          .size([300.0, 300.0], imgui::Condition::FirstUseEver)
          .position([100.0, 300.0], imgui::Condition::FirstUseEver)
          .build(&ui, || {
            if dmx_status != &DmxStatus::Ok {
                ui.text(im_str!("Sending failed!"));
            }

            for (i, channel) in dmx_chain.iter().enumerate() {
                ui.text(im_str!("{}: {}", i + 1, channel));
            }
        });

        window_rounding.pop(&ui);

        let (factory, _, encoder, _, render_target) = graphics::gfx_objects(ctx);
        let draw_data = ui.render();
        self.renderer.render(
            &mut *factory,
            encoder,
            &mut RenderTargetView::new(render_target.clone()),
            draw_data
        ).unwrap();

    }

    fn update_mouse(&mut self) {
        self.imgui.io_mut().mouse_pos = [
            self.mouse_state.pos.0 as f32,
            self.mouse_state.pos.1 as f32
        ];

        self.imgui.io_mut().mouse_down = [
            self.mouse_state.pressed.0,
            self.mouse_state.pressed.1,
            self.mouse_state.pressed.2,
            false,
            false
        ];

        self.imgui.io_mut().mouse_wheel = self.mouse_state.wheel;
        self.mouse_state.wheel = 0.0;
    }

    pub fn update_mouse_pos(&mut self, x: f32, y: f32) {
      self.mouse_state.pos = (x as i32, y as i32);
    }

    pub fn update_mouse_down(&mut self, pressed: (bool, bool, bool)) {
      self.mouse_state.pressed = pressed;

      if pressed.0 {
        self.show_popup = false;
      }
    }

    pub fn open_popup(&mut self) {
      self.show_popup = true;
    }
}
