#![recursion_limit = "256"]

#[cfg(test)]
mod tests;

pub mod core;

pub mod prelude {
    pub use crate::core::prelude::*;
}
use prelude::*;

#[cfg(target_arch = "wasm32")]
#[macro_use]
pub mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub mod server;

pub fn run<Comp, Msg>(_root: Comp)
where
    Comp: Component<Msg>,
    Msg: Clone + 'static,
{
    #[cfg(target_arch = "wasm32")]
    App::new(Rc::new(RefCell::new(Box::new(_root)))).run();
}
