extern crate kaos;
mod engine;
mod scene;


#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use winit::
{
    event::*,
    event_loop::ControlFlow,
};

fn main() 
{
    pollster::block_on(run());
}



#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
async fn run()
{
    let mut engine = engine::Engine::new().await;
    engine.run();
}
