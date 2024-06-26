use cgmath::prelude::*;
use tobj;
use wgpu::SurfaceConfiguration;
use crate::kaos::scene::assets::load_model;
use crate::kaos::scene::camera::*;
use crate::kaos::scene::objects::*;


pub struct Scene
{
    pub resources: (Vec<tobj::Model>, Vec<tobj::Material>),
    pub instances: Vec<Instance>,
    pub camera: Camera,
    pub light: Light,
    pub player: Player,
}



impl Scene
{
    pub async fn new(config: &SurfaceConfiguration) -> Self
    {
        let camera = Camera::new(
            cgmath::Point3::new(0.0, -10.0, 0.0),
            cgmath::Deg(0.0),
            cgmath::Deg(0.0),
            Projection::new(config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0),
            CameraController::new(10.0, 0.2),
        );

        let light = Light
        {
            position: cgmath::Point3::new(0.0, 10.0, 0.0),
            color: cgmath::Vector3::new(1.0, 1.0, 1.0),
        };

        let player = Player
        {
            position: cgmath::Vector3::new(0.0, 0.0, 0.0),
            rotation: cgmath::Quaternion::from_angle_y(cgmath::Deg(0.0)),
            scale: cgmath::Vector3::new(1.0, 1.0, 1.0),
            velocity: cgmath::Vector3::new(0.0, 0.0, 0.0),
            acceleration: cgmath::Vector3::new(0.0, 0.0, 0.0),
        };
        
        let instances = vec![
            Instance
            {
                position: cgmath::Vector3::new(0.0, 0.0, 0.0),
                rotation: cgmath::Quaternion::from_angle_y(cgmath::Deg(0.0)),
                scale: cgmath::Vector3::new(1.0, 1.0, 1.0),
            },
        ];

        let cube = load_model("cube.obj").await.unwrap();
        let mut sphere = load_model("sphere.obj").await.unwrap();
        let mut floor = load_model("floor.obj").await.unwrap();
        let mut meshes = cube.0;
        Vec::append(&mut meshes, &mut sphere.0);
        Vec::append(&mut meshes, &mut floor.0);
        let mut materials = cube.1;
        Vec::append(&mut materials, &mut sphere.1);
        Vec::append(&mut materials, &mut floor.1);
        let resources = (meshes, materials);
        Self { resources, instances, camera, light, player }
    }

    pub fn update(&mut self, delta_time: f32)
    {

    }

    pub fn key_input(&mut self, )
    {

    }

    pub fn mousewheel_input(&mut self, delta: winit::event::MouseScrollDelta)
    {
    }

    pub fn resize(&mut self, width: u32, height: u32)
    {

    }
    
    pub fn device_input(&mut self, device: &winit::event::DeviceId, event: winit::event::DeviceEvent)
    {

    }


}



    
