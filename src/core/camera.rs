use crate::core::renderer::Resource;


use cgmath::*;
use wgpu::util::DeviceExt;
use winit::event::*;
use winit::dpi::PhysicalPosition;
use instant::Duration;
use std::f32::consts::FRAC_PI_2;




const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

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


        let rotation = Quaternion::from(Euler::new(yaw.into(), pitch.into(), Rad(0.0)));
        let state = CameraState 
        {
            position: position.into(),
            rotation,
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


    pub fn update_camera(&mut self , dt: Duration) 
    {
        self.controller.update_fps(&mut self.state ,dt);
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> 
    {
        let rotation = Euler::from(self.state.rotation);
        let (sin_pitch, cos_pitch) = rotation.x.sin_cos();
        let (sin_yaw, cos_yaw) = rotation.y.sin_cos();
        Matrix4::look_to_rh(
            self.state.position,
            Vector3::new(
                cos_pitch * sin_yaw,
                cos_pitch * cos_yaw,
                sin_pitch,
            ).normalize(),
            Vector3::unit_z(),
        )

        // Matrix4::from(rotation).inverse_transform().unwrap() * Matrix4::from_translation(-self.state.position.to_vec())
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

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }
}



#[derive(Debug)]
pub struct CameraController 
{
    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
        }
    }

    pub fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool{
        let amount = if state == ElementState::Pressed { 1.0 } else { 0.0 };
        match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.amount_forward = amount;
                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.amount_backward = amount;
                true
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.amount_left = amount;
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.amount_right = amount;
                true
            }
            VirtualKeyCode::Space => {
                self.amount_up = amount;
                true
            }
            VirtualKeyCode::LShift => {
                self.amount_down = amount;
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

    pub fn update_fps(&mut self, state: &mut CameraState, dt: Duration) {
        let dt = dt.as_secs_f32();
        // Move forward/backward and left/right
        let mut rotation = Euler::from(state.rotation);
        let (yaw_sin, yaw_cos) = rotation.y.sin_cos();
        let forward = Vector3::new(yaw_sin, yaw_cos, 0.0).normalize(); // Adjusted for Z-up
        let right = Vector3::new(yaw_cos, -yaw_sin, 0.0).normalize();  // Adjusted for Z-up
        let (pitch_sin, pitch_cos) = rotation.x.sin_cos();
        state.position += forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
        state.position += right * (self.amount_right - self.amount_left) * self.speed * dt;

        let scrollward = Vector3::new(pitch_cos * yaw_sin, pitch_cos * yaw_cos, pitch_sin).normalize(); // Adjusted for Z-up
        state.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;
        self.scroll = 0.0;

        state.position.z += (self.amount_up - self.amount_down) * self.speed * dt;
        if state.position.z < 0.4 { state.position.z = 0.4; }

        rotation.y += Rad(self.rotate_horizontal) * self.sensitivity * dt;
        rotation.x += Rad(-self.rotate_vertical) * self.sensitivity * dt;
        // rotation.x = rotation.x.max(-Rad(SAFE_FRAC_PI_2)).min(Rad(SAFE_FRAC_PI_2));

        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;

        if rotation.x < -Rad(SAFE_FRAC_PI_2) { rotation.x = -Rad(SAFE_FRAC_PI_2); } 
        else if rotation.x > Rad(SAFE_FRAC_PI_2) { rotation.x = Rad(SAFE_FRAC_PI_2); }    
        state.rotation = Quaternion::from(rotation);
    }

    // pub fn update_orbit(&mut self, state: &mut CameraState, dt: Duration) 
    // {
    //     let dt = dt.as_secs_f32();
    //     let mut rotation = Euler::from(state.rotation);
    //     let (yaw_sin, yaw_cos) = rotation.y.sin_cos();
    //     let forward = Vector3::new(yaw_sin, yaw_cos, 0.0).normalize(); // Adjusted for Z-up
    //     let right = Vector3::new(yaw_cos, -yaw_sin, 0.0).normalize();  // Adjusted for Z-up
    //     let (pitch_sin, pitch_cos) = rotation.x.sin_cos();
    //     
    //     let cam_as_mat = Matrix4::look_to_rh(
    //         state.position,
    //         Vector3::new(
    //             cos_pitch * sin_yaw,
    //             cos_pitch * cos_yaw,
    //             sin_pitch,
    //         ).normalize(),
    //         Vector3::unit_z(),
    //     );
    //     let transform = Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0))
    //     
    //     state.position += forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
    //     state.position += right * (self.amount_right - self.amount_left) * self.speed * dt;
    //
    //     let scrollward = Vector3::unit_z(); 
    //     state.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;
    //     self.scroll = 0.0;
    // }

    // pub fn update_2d(&mut self, state: &mut CameraState, dt: Duration) 
    // {
    //     let dt = dt.as_secs_f32();
    //     // Move forward/backward and left/right
    //     let (yaw_sin, yaw_cos) = state.yaw.0.sin_cos();
    //     let forward = Vector3::new(yaw_sin, yaw_cos, 0.0).normalize(); // Adjusted for Z-up
    //     let right = Vector3::new(yaw_cos, -yaw_sin, 0.0).normalize();  // Adjusted for Z-up
    //     state.position += forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
    //     state.position += right * (self.amount_right - self.amount_left) * self.speed * dt;
    //
    //     let (pitch_sin, pitch_cos) = state.pitch.0.sin_cos();
    //     let scrollward = Vector3::new(pitch_cos * yaw_sin, pitch_cos * yaw_cos, pitch_sin).normalize(); // Adjusted for Z-up
    //     state.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;
    //     self.scroll = 0.0;
    // }
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

