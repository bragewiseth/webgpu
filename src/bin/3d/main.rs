mod scene;
mod renderer;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;






fn main() 
{
    pollster::block_on(run());
}



#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
async fn run()
{
    let (event_loop, window) = kaos::window::new("floating"); // name it floating since i3wm makes windows with that title float
    let (device, queue, size, surface) = kaos::new_device!(window);
    let scene = scene::Scene::new(device, queue, size).await;
    let gpu = renderer::Renderer::new(device, queue, size, surface, &scene).await;
    gpu.load_assets().await; // creates buffers, bind groups, etc. from the scene
    engine::event_loop!() // this expands to the event loop code found in core::engine
}
