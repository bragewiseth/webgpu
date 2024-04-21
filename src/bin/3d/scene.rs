use kaos::model::{Model, Instances, Instance, Mesh, Material  };
use kaos::assets;

use cgmath::prelude::*;
use wgpu::util::DeviceExt;



struct Node
{
    model: Model,
    children: Vec<Node>,
}



pub async fn new_scene(device: &wgpu::Device, queue: &wgpu::Queue, layouts : &BindGroupLayouts  ) -> Self
{
    let (sphere_mesh, sphere_mat) = assets::load_model("sphere.obj").await.unwrap();
    let (floor_mesh, _) = assets::load_model("floor.obj").await.unwrap();


    let camera = Camera::new(
        cgmath::Point3::new(0.0, -10.0, 0.0),
        cgmath::Deg(0.0),
        cgmath::Deg(0.0),
        Projection::new(config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0),
        CameraController::new(10.0, 0.2),
    );
}




