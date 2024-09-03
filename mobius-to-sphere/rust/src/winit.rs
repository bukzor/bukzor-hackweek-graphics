use anyhow::Result;
use log::info;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

#[derive(Default)]
pub enum RenderState<'s> {
    #[default]
    Suspended,

    #[allow(dead_code)] // I'll want this. Pretty sure.
    Active(crate::wgpu::WgpuApplication<'s>),
}

#[derive(Default)]
struct WinitApplication<'window> {
    //pub app: Option<crate::wgpu::WgpuApplication<'window>>,

    // The fields MUST be in this order, so that the surface is dropped before the window
    // Window is cached even when suspended so that it can be reused when the app is resumed after being suspended
    pub render_state: RenderState<'window>,
    pub window: Option<Arc<Window>>,
}

impl<'window> WinitApplication<'window> {
    #[allow(dead_code)] // I'll want this. Pretty sure.
    fn app(&self) -> &crate::wgpu::WgpuApplication {
        if let RenderState::Active(state) = &self.render_state {
            state
        } else {
            panic!("app not initialized")
        }
    }
    fn window(&self) -> &Window {
        self.window.as_ref().expect("window not found")
    }
}

fn name<T: std::fmt::Debug>(this: T) -> String {
    let debug = format!("{:#?}", this);
    debug
        .split(" ")
        .next()
        .unwrap_or_else(|| panic!("failed to find enum name: {}", debug))
        .to_string()
}

use futures::executor::block_on;

impl<'window> ApplicationHandler for WinitApplication<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Resumed (v5)");
        let window = Arc::new(
            event_loop
                .create_window(Default::default())
                .expect("failed to create initial window"),
        );

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            let _ = window.request_inner_size(PhysicalSize::new(450, 400));
            // On wasm, append the canvas to the document body
            use winit::platform::web::WindowExtWebSys;
            let canvas = window.canvas().expect("failed to create a canvas");
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.body())
                .and_then(|body| body.append_child(&web_sys::Element::from(canvas)).ok())
                .expect("couldn't append canvas to document body");
        };

        let app = block_on(crate::wgpu::WgpuApplication::new(window.clone()))
            .unwrap_or_else(|err| panic!("{:#?}", err));

        *self = WinitApplication {
            render_state: RenderState::Active(app),
            window: Some(window),
        };

        info!("Resume complete.");
    }

    fn suspended(&mut self, _: &ActiveEventLoop) {
        self.render_state = RenderState::Suspended;
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        info!("Window event: {:#?}", event);
        if window_id != self.window().id() {
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
            } => {
                info!("Event loop exiting.");
                event_loop.exit()
            }
            _ => {}
        }
        info!("Window event complete: {}", name(event));
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;
    use wasm_bindgen::prelude::JsValue;

    #[wasm_bindgen(start)]
    pub fn run_wasm() -> Result<(), JsValue> {
        let result = crate::run();
        match result {
            Err(error) => {
                let msg = format!(
                    "[{}:{}:{}] {} = {:#?}",
                    file!(),
                    line!(),
                    column!(),
                    stringify!(error),
                    error
                );
                Err(JsValue::from_str(&msg))
            }

            Ok(ok) => Ok(ok),
        }
    }
}

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
    let mut state = WinitApplication::default();
    Ok(event_loop.run_app(&mut state)?)
}
