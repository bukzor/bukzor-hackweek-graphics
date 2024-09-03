#![allow(unused_imports)] // FIXME: prototyping
#![allow(dead_code)] // FIXME: prototyping

use std::ops::Deref;
use std::rc::Rc;

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

mod owning_ref {
    use std::marker::PhantomData;
    use std::ops::Deref;

    //pub use stable_deref_trait::CloneStableDeref;
    use stable_deref_trait::StableDeref;

    pub struct OwnedObject<Owner, Owned>
    where
        Owner: StableDeref,
        Owned: Deref,
    {
        pub owned: Owned,
        pub owner: PhantomData<Owner>,
    }

    impl<Owner, Owned> OwnedObject<Owner, Owned>
    where
        Owner: StableDeref,
        Owned: Deref,
    {
        pub fn try_new<F, E>(o: Owner, f: F) -> Result<Self, E>
        where
            F: FnOnce(*const Owner::Target) -> Result<Owned, E>,
        {
            let h: Owned;
            {
                h = f(o.deref() as *const Owner::Target)?;
            }

            Ok(OwnedObject {
                owned: h,
                owner: PhantomData,
            })
        }
    }
}

mod lifetimed {
    use std::marker::PhantomData;
    use std::ops::Deref;

    use stable_deref_trait::StableDeref;

    pub struct ParentLifetime<'parent, Parent>
    where
        Parent: StableDeref,
    {
        pub parent: Parent,
        pub lifetime: PhantomData<&'parent Parent>,
    }

    impl<'parent, Parent> ParentLifetime<'parent, Parent>
    where
        Parent: StableDeref,
    {
        pub fn try_new<F, E>(o: Parent, f: F) -> Result<Self, E>
        where
            F: FnOnce(*const Parent::Target) -> Result<Parent, E>,
        {
            let h: Parent;
            {
                h = f(o.deref() as *const Parent::Target)?;
            }

            Ok(ParentLifetime {
                parent: h,
                lifetime: PhantomData,
            })
        }
    }
}

// Problem statement: App should own a Window and a Surface from that Window
use anyhow::{anyhow, Error, Result};

#[allow(clippy::redundant_allocation)] // OwnedObject::deref has one too many dereferences
fn surface_handle<'window>(ptr: *const Window) -> Result<Rc<Surface<'window>>> {
    let window: &'window Window = unsafe { &*ptr };
    let surface = create_surface(window)?;
    Ok(Rc::new(surface))
}

pub struct App<'window> {
    window: Rc<Window>,
    surface: Rc<Surface<'window>>,
}
impl<'window> App<'window> {
    pub fn new(window: Window) -> Result<Self> {
        let window = Rc::new(window);

        let handle = owning_ref::OwnedObject::try_new(window.clone(), surface_handle)?;
        let surface = handle.owned;

        Ok(Self {
            window,
            surface: surface.clone(),
        })
    }
}

fn demo(non_copy: NonCopy) -> Result<String> {
    let app = App::new(window)?;
    let window = app.window.clone();
    let surface1 = app.surface.clone();
    let surface2 = app.surface.clone();

    if !std::ptr::eq(&*window, &*window) {
        println!("weird!")
    }
    if std::ptr::eq(&*surface1, &*surface2) {
        Ok("The two references to Surface reference the same object".to_string())
    } else {
        Err(anyhow!("Surface references are not the same"))
    }
}

fn main() -> Result<()> {
    let window = Window {};
    resume(window).map(|msg| println!("{}", msg))
}
