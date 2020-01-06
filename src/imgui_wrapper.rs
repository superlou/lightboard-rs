use std::time::Instant;
use imgui::{TextureId, im_str};
use imgui_gfx_renderer::{Shaders, Renderer};
use gfx_core::{format, texture};
use gfx_core::factory::Factory;
use gfx_core::memory::Typed;
use gfx_core::handle::RenderTargetView;
use gfx_device_gl;
use ggez::{graphics, Context};

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

      // Create instace
      Self {
        imgui: imgui,
        renderer: renderer,
        last_frame: Instant::now(),
        mouse_state: MouseState::default(),
        show_popup: false,
      }
    }

    pub fn render(&mut self, ctx: &mut Context, hidpi_factor: f32, color: &mut [f32; 4]) {
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

        ui.window(im_str!("Hello world"))
          .size([300.0, 300.0], imgui::Condition::FirstUseEver)
          .position([100.0, 100.0], imgui::Condition::FirstUseEver)
          .build(&ui, || {
            ui.text(im_str!("Hello world!"));
            ui.text(im_str!("こんにちは世界！"));
            ui.text(im_str!("This...is...imgui-rs!"));
            ui.separator();
            let mouse_pos = ui.io().mouse_pos;
            ui.text(im_str!(
              "Mouse Position: ({:.1},{:.1})",
              mouse_pos[0],
              mouse_pos[1]
            ));

            if ui.small_button(im_str!("small button")) {
              println!("Small button clicked");
            }

            imgui::ColorPicker::new(im_str!("My Color"), color).build(&ui);
          });

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
