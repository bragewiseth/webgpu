use cgmath::prelude::*;
pub struct NodeId(u32);

struct Node<T>
{
    parent: Option<NodeId>,
    children: Vec<NodeId>,
    siblings: Vec<NodeId>,
    data: T,
}

pub struct Instance 
{
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub scale: cgmath::Vector3<f32>,
}

pub struct Root{}


pub struct StaticObject
{}

pub struct KinematicObject
{}

// match event
// {
//     DeviceEvent::MouseMotion{ delta, } if self.mouse_locked == true => 
//     {
//         self.camera.controller.process_mouse(delta.0, delta.1);
//         true
//     }
//     _ => false,
// }


pub async fn define_scene() -> Node
{
    let nodes = Vec::new();

    let root = Node
    {
        parent: None,
        children: Vec::new(),
        siblings: Vec::new(),
        data: Root{},
    };
    nodes.push(root);

    let camera = Camera::new(
        cgmath::Point3::new(0.0, -10.0, 0.0),
        cgmath::Deg(0.0),
        cgmath::Deg(0.0),
        Projection::new(config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0),
        CameraController::new(10.0, 0.2),
    );

    let camera = Node
    {
        parent: Some(root),
        children: Vec::new(),
        siblings: Vec::new(),
        data: camera,
    };

    let light = Light::new(
        cgmath::Point3::new(0.0, 10.0, 0.0),
        cgmath::Vector3::new(1.0, 1.0, 1.0),
    );
    
    let light = Node
    {
        parent: Some(root),
        children: Vec::new(),
        siblings: Vec::new(),
        data: light,
    };
    

    let floor = StaticObject
    {
        position: cgmath::Vector3::new(0.0, 0.0, 0.0),
        rotation: cgmath::Quaternion::from_angle_x(cgmath::Deg(90.0)),
        scale: cgmath::Vector3::new(10.0, 10.0, 10.0),
    };

    let sphere = KinematicObject
    {
        position: cgmath::Vector3::new(0.0, 0.0, 0.0),
        rotation: cgmath::Quaternion::from_angle_x(cgmath::Deg(90.0)),
        scale: cgmath::Vector3::new(1.0, 1.0, 1.0),
    };


}
