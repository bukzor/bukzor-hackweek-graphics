use anyhow::Result;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Default)]
struct WinitApplication {
    window_id: Option<WindowId>,
    window: Option<Window>,
}

impl ApplicationHandler for WinitApplication {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        log::debug!("Resumed");
        let window = event_loop
            .create_window(Default::default())
            .expect("failed to create initial window");
        self.window_id = Some(window.id());
        self.window = Some(window);
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if Some(window_id) != self.window_id {
            return;
        }
        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => event_loop.exit(),
            _ => {}
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() -> Result<()> {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new()?;

    //#[cfg(target_arch = "wasm32")]
    //{
    //    // Winit prevents sizing with CSS, so we have to set
    //    // the size manually when on web.
    //    use winit::dpi::PhysicalSize;
    //    let _ = window.request_inner_size(PhysicalSize::new(450, 400));

    //    use winit::platform::web::WindowExtWebSys;
    //    web_sys::window()
    //        .and_then(|win| win.document())
    //        .and_then(|doc| {
    //            let dst = doc.get_element_by_id("wasm-example")?;
    //            let canvas = web_sys::Element::from(window.canvas()?);
    //            dst.append_child(&canvas).ok()?;
    //            Some(())
    //        })
    //        .expect("Couldn't append canvas to document body.");
    //}

    let mut state = WinitApplication::default();
    return Ok(event_loop.run_app(&mut state)?);
}
