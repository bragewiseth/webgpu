use crate::core::renderer::Resource;


use cgmath::{*, Matrix};
use wgpu::util::DeviceExt;
use winit::event::*;
use winit::dpi::PhysicalPosition;
use instant::Duration;
use std::f32::consts::FRAC_PI_2;





#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);


#[derive(Debug)]
pub struct CameraState
{
    pub position: Point3<f32>,
    pub rotation: Quaternion<f32>,
    pub velocity: Vector3<f32>,
}

#[derive(Debug)]
pub struct Camera {
    pub state: CameraState,
    pub bind_group: wgpu::BindGroup,
    pub buffer : wgpu::Buffer,
    pub uniform: CameraUniform,
    pub controller: CameraController,
    pub projection: Projection,
}


impl Camera {
    pub fn new<
        V: Into<Point3<f32>>,
        Y: Into<Rad<f32>>,
        P: Into<Rad<f32>>,
    >(
        position: V,
        yaw: Y,
        pitch: P,
        projection: Projection,
        controller: CameraController,
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self
    {
        
        let camera_uniform = CameraUniform::new();

        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );


        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0, resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });


        let rotation = Quaternion::from(Euler::new(pitch.into() - Rad(FRAC_PI_2), yaw.into(), Rad(0.0)));
        let state = CameraState 
        {
            position: position.into(),
            rotation,
            velocity: Vector3::zero(),
        };


        let mut cam = Self {
            state,
            bind_group: camera_bind_group,
            buffer: camera_buffer,
            uniform: camera_uniform,
            controller,
            projection,
        };
        cam.update_view_proj();
        cam

    }

    pub fn update_view_proj(&mut self) 
    {
        self.uniform.view_position = self.state.position.to_homogeneous().into();
        self.uniform.proj = self.projection.calc_matrix().into();
        self.uniform.view = self.calc_matrix().into();
    }


    pub fn update_fps(&mut self, dt: Duration) 
    {
        let c = &mut self.controller;
        let v = &mut self.state.velocity;
        *v += (c.impulse * c.speed * 0.06 ) / (v.magnitude() + 1.0);
        let dt = dt.as_secs_f32();
        let forward = self.state.rotation * Vector3::unit_z();
        let right = self.state.rotation * Vector3::unit_x();
        let yaw =  Quaternion::from_angle_z(Rad(-c.rotate_horizontal) * c.sensitivity * dt);  // world z
        let pitch =  Quaternion::from_angle_x(Rad(-c.rotate_vertical) * c.sensitivity * dt);  // world x

        self.state.position += forward * v.z * c.speed * dt;
        self.state.position += right * v.x * c.speed * dt;
        self.state.position += forward * c.scroll * c.speed * c.sensitivity * dt;
        let mut new_fovy = self.projection.fovy + Rad(1.0) * c.scroll * c.speed * c.sensitivity * dt * 0.1;
        if new_fovy < Rad(0.1) { new_fovy = Rad(0.1); }
        if new_fovy > Rad(1.5) { new_fovy = Rad(1.5); }
        self.projection.set_fovy(new_fovy);
        self.state.position.z += v.y * c.speed * dt;
        if self.state.position.z < 0.4 { self.state.position.z = 0.4; }

        c.scroll = 0.0;
        c.rotate_horizontal = 0.0;
        c.rotate_vertical = 0.0;
        *v *= 0.9;
        self.state.rotation = yaw * self.state.rotation * pitch; 

    }

    pub fn update_orbit(&mut self, dt: Duration) 
    {
        let c = &mut self.controller;
        let v = &mut self.state.velocity;
        *v += (c.impulse * c.speed * 0.06 ) / (v.magnitude() + 1.0);
        let dt = dt.as_secs_f32();
        let forward = self.state.rotation * Vector3::unit_z();
        let yaw =  Quaternion::from_angle_z(Rad(-c.rotate_horizontal) * c.sensitivity * dt);                                        // world z
        let pitch =  Quaternion::from_axis_angle(self.state.rotation * Vector3::unit_x(), Rad(-c.rotate_vertical) * c.sensitivity * dt); // current x
        let rotation = yaw * pitch;
        let point_as_quat = Quaternion::new(0.0, self.state.position.x, self.state.position.y, self.state.position.z);
        let rotated_quat = rotation * point_as_quat * rotation.invert();
        let rotated_point = Point3::new(rotated_quat.v.x, rotated_quat.v.y, rotated_quat.v.z);

        self.state.position = rotated_point;
        self.state.position += forward * v.z * c.speed * dt;
        self.state.position.z += v.y * c.speed * dt;
        self.state.position.x += v.x * c.speed * dt;
        self.state.position.y += c.scroll * c.speed * c.sensitivity * dt;

        c.scroll *= 0.0;
        c.rotate_horizontal = 0.0;
        c.rotate_vertical = 0.0;
        *v *= 0.9;
        self.state.rotation = rotation * self.state.rotation;

    }

    pub fn update_2d(&mut self, dt: Duration) 
    {
    }


    pub fn calc_matrix(&self) -> Matrix4<f32> 
    {        
        // world is z-up, camera is y-up z-forward
        Matrix4::look_to_rh
        (
            self.state.position,
            -self.state.rotation * Vector3::unit_z(),
            Vector3::unit_z(),
        )
    }

}



#[derive(Debug)]
pub struct Projection 
{
    aspect: f32,
    fovy: Rad<f32>,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new<F: Into<Rad<f32>>>(
        width: u32,
        height: u32,
        fovy: F,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) 
    {
        self.aspect = width as f32 / height as f32;
    }

    pub fn set_fovy(&mut self, fovy: Rad<f32>) 
    {
        self.fovy = fovy;
    }


    pub fn calc_matrix(&self) -> Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }
}



#[derive(Debug)]
pub struct CameraController 
{
    impulse: Vector3<f32>,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
}
 
impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            impulse: Vector3::zero(),
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
        }
    }

    pub fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool{
        let amount = match state {
            ElementState::Pressed => 1.0,
            ElementState::Released => 0.0,
        };
        match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.impulse.z = amount;
                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.impulse.z = -amount;
                true
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.impulse.x = -amount;
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.impulse.x = amount;
                true
            }
            VirtualKeyCode::Space => {
                self.impulse.y = amount;
                true
            }
            VirtualKeyCode::LShift => {
                self.impulse.y = -amount;
                true
            }
            _ => false,
        }
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scroll = -match delta {
            // I'm assuming a line is about 100 pixels
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition {
                y: scroll,
                ..
            }) => *scroll as f32,
        };
    }
}




#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view: [[f32; 4]; 4],
    proj: [[f32; 4]; 4],
    view_position: [f32; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view: cgmath::Matrix4::identity().into(),
            proj: cgmath::Matrix4::identity().into(),
            view_position: [0.0; 4],
        }
    }
}

