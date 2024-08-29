#![allow(unused_variables)] // FIXME: only for prototyping

// START(external stuff) -- copied out of wgpu crate:
pub fn create_surface<'window>(
    target: impl Into<Surface<'window>>,
) -> Result<Surface<'window>, Error> {
    // Simple implementation: just convert the target into a Surface
    Ok(target.into())
}

pub struct Window {}
impl Window {}

pub struct Surface<'window> {
    #[allow(dead_code, clippy::redundant_allocation)]
    window: Box<&'window Window>,
}
impl<'window> From<&'window Window> for Surface<'window> {
    fn from(window: &'window Window) -> Self {
        Self {
            window: Box::new(window),
        }
    }
}
// END(external stuff)

// Problem statement: App should own a Window and a Surface from that Window
use anyhow::{anyhow, Error, Result};

// impl<'a> std::ops::Deref for Surface<'a> {
//     type Target = Box<&'a Surface<'a>>;
//
//     fn deref(self: &Surface<'a>) -> &Self::Target {
//         &Box::new(self)
//     }
// }

pub struct App<'window> {
    _refs: owning_ref::OwningHandle<Box<WindowRef<'window>>, Box<Surface<'window>>>,
}

use std::marker::PhantomData;
use std::ops::Deref;

// An owned Window, with a borrow-lifetime attached.
struct WindowRef<'window> {
    oref: owning_ref::OwningRef<Box<Window>, Window>,
    _marker: PhantomData<&'window Window>,
}

impl<'window> WindowRef<'window> {
    fn new(window: Window) -> Self {
        let oref: owning_ref::OwningRef<Box<Window>, Window> =
            owning_ref::OwningRef::new(Box::new(window));

        Self {
            oref,
            _marker: PhantomData,
        }
    }
}

impl<'window> From<&'window WindowRef<'window>> for Surface<'window> {
    fn from(val: &'window WindowRef<'window>) -> Self {
        let this = val.oref.as_owner().as_ref();
        create_surface(this).expect("unable to create surface")
    }
}

fn surface_handle<'window>(ptr: *const WindowRef) -> Box<Surface<'window>> {
    let owner = unsafe { &*ptr };
    let window: &'window Window = &owner.oref;
    let surface = create_surface(window).expect("failed to create surface");
    Box::new(surface)
}

// Implement ToHandle<Surface> for Window
impl<'window> owning_ref::ToHandle for WindowRef<'window> {
    type Handle = Box<Surface<'window>>;

    unsafe fn to_handle(ptr: *const Self) -> Self::Handle {
        let window = unsafe { &*ptr };
        let surface = create_surface(window).expect("failed to create surface");
        Box::new(surface)
    }
}

impl<'window> App<'window> {
    pub fn new(window: Window) -> Result<App<'window>> {
        let window = WindowRef::new(window);
        let window_and_surface =
            owning_ref::OwningHandle::new_with_fn(Box::new(window), surface_handle);

        Ok(Self {
            _refs: window_and_surface,
        })
    }

    pub fn window(&self) -> &Window {
        *self._refs.window
    }
    pub fn surface(&self) -> &Surface {
        self._refs.deref()
    }
}

fn resume(window: Window) -> Result<String> {
    let app = App::new(window)?;
    let window: &Window = app.window();
    let surface1 = &*app._refs;
    let surface2 = &*app._refs;

    if std::ptr::eq(surface1, surface2) {
        Ok("The two references to Surface reference the same object".to_string())
    } else {
        Err(anyhow!("Surface references are not the same"))
    }
}

fn main() -> Result<()> {
    let window = Window {};
    resume(window).map(|msg| println!("{}", msg))
}
