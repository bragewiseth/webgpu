
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use winit::
{
    event_loop::EventLoop,
    window::WindowBuilder,
    window::Window,
};







#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub fn new(title: &str) -> (EventLoop<()>, Window)
{
    cfg_if::cfg_if! 
    {
        if #[cfg(target_arch = "wasm32")] 
        {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } 
        else { env_logger::init(); }
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(title)
        .build(&event_loop)
        .unwrap();

    window.set_inner_size(winit::dpi::LogicalSize::new(1400, 800));


    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::PhysicalSize;

        window.set_inner_size(PhysicalSize::new(1500, 1500));
        

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| 
            {
                let dst = doc.get_element_by_id("f_stop")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            }).expect("Couldn't append canvas to document body.");
    }
    (event_loop, window)
}
