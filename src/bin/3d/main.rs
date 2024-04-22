extern crate kaos;
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
    let (device, queue, windowsize, surface) = kaos::new_device!(window);
    let scene = scene::Scene::new().await;
    let gpu = renderer::Renderer::new(device, queue, windowsize, surface);
    gpu.load_assets(scene.resources);

    engine::event_loop!(
        window                  => window,
        windowsize              => windowsize,
        key_input_handle        => scene.key_input,
        device_input_handle     => scene.device_input,
        mousewheel_input_handle => scene.mousewheel_input,
        update_handle           => renderer.render,
        scene                   => scene,
    )
}
