

pub struct World
{       
        camera: u32,
}



impl World 
{
    pub fn new(config: &wgpu::SurfaceConfiguration, device: &wgpu::Device, queue: &wgpu::Queue) -> Self
    {

        let camera = 1;
        let cube = 2;
        let floor = 3;

        let obj_model =
            resources::load_model("shpere.obj", &device, &queue, &world.entities[0].material.texture_bind_group_layout)
                .await
                .unwrap();

        const SPACE_BETWEEN: f32 = 3.0;
        let instances = (0..1).flat_map(|z| {
            (0..1).map(move |x| {
                let x = 2.0 * (x as f32 - 1.0 as f32 / 2.0);
                let z = 2.0 * (z as f32 - 1.0 as f32 / 2.0);

                let position = cgmath::Vector3 { x, y: 0.0, z };

                let rotation = if position.is_zero() {
                    cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
                } else {
                    cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                };

                Instance {
                    position, rotation,
                }
            })
        }).collect::<Vec<_>>();
        Self
        {
            camera

        }


    }
}
