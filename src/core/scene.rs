
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


pub struct StaticObject
{}

pub struct KinematicObject
{}



impl Instance
{
    pub fn to_buffer(&self) -> [[f32; 4]; 4]
    {
        (
            cgmath::Matrix4::from_translation(self.position) * 
            cgmath::Matrix4::from(self.rotation) * 
            cgmath::Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
        ).into()
    }
}

