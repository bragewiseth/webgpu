use cgmath::Matrix4;


pub struct Instance 
{
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub scale: cgmath::Vector3<f32>,
}

pub struct Player
{
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub scale: cgmath::Vector3<f32>,
    pub velocity: cgmath::Vector3<f32>,
    pub acceleration: cgmath::Vector3<f32>,
}

pub struct StaticObject
{}

pub struct KinematicObject
{}


pub struct Light
{
    pub position: cgmath::Point3<f32>,
    pub color: cgmath::Vector3<f32>,
}



impl Instance
{
    pub fn calc_matrix(&self) -> [[f32; 4]; 4]
    {
        (Matrix4::from_translation(self.position) * 
        Matrix4::from(self.rotation) * 
        Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)).into()
    }
}

impl Player
{
    pub fn calc_matrix(&self) -> [[f32; 4]; 4]
    {        
        (Matrix4::from_translation(self.position) *
        Matrix4::from(self.rotation) *
        Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)).into()
    }
}

