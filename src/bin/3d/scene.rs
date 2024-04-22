use cgmath::prelude::*;


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
    pub fn new() -> Self
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

        let cube = load_model("cube.obj").unwrap();
        let sphere = load_model("sphere.obj").unwrap();
        let floor = load_model("floor.obj").unwrap();
        let resources = (vec![cube.0, sphere.0, floor.0], vec![cube.1, sphere.1]);
        Self { resources, instances, camera, light, player }
    }
}



    
