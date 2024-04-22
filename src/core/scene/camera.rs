use cgmath::*;
use winit::event::*;
use winit::dpi::PhysicalPosition;
use winit::keyboard::*;
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
pub struct Camera 
{
    pub position: Point3<f32>,
    pub rotation: Quaternion<f32>,
    pub velocity: Vector3<f32>,
    pub controller: CameraController,
    pub projection: Projection,
}

#[derive(Debug)]
pub struct CameraController 
{
    force: Vector3<f32>,
    mouse_dx: f32,
    mouse_dy: f32,
    mouse_pos: PhysicalPosition<f64>,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
}

#[derive(Debug)] 
pub struct Projection 
{
    aspect: f32,
    fovy: Rad<f32>,
    znear: f32,
    zfar: f32,
}


impl Camera {
    pub fn new< V: Into<Point3<f32>>, Y: Into<Rad<f32>>, P: Into<Rad<f32>>, >(
        position: V,
        yaw: Y,
        pitch: P,
        projection: Projection,
        controller: CameraController,
    ) -> Self
    {
        let rotation = Quaternion::from(Euler::new(pitch.into() - Rad(FRAC_PI_2), yaw.into(), Rad(0.0)));
        Self 
        {
            position: position.into(),
            rotation,
            velocity: Vector3::zero(),
            controller,
            projection,
        }
    }


    pub fn update_fps(&mut self, dt: Duration) 
    {
        let c = &mut self.controller;
        let v = &mut self.velocity;
        *v += (c.force * c.speed * 0.06 ) / (v.magnitude() + 1.0);
        let dt = dt.as_secs_f32();
        let forward = self.rotation * Vector3::unit_z();
        let right = self.rotation * Vector3::unit_x();
        let yaw =  Quaternion::from_angle_z(Rad(-c.mouse_dx) * c.sensitivity * dt);  // world z
        let pitch =  Quaternion::from_angle_x(Rad(-c.mouse_dy) * c.sensitivity * dt);  // world x

        self.position += forward * v.z * c.speed * dt;
        self.position += right * v.x * c.speed * dt;
        self.position += forward * c.scroll * c.speed * c.sensitivity * dt;
        let mut new_fovy = self.projection.fovy + Rad(1.0) * c.scroll * c.speed * c.sensitivity * dt * 0.1;
        if new_fovy < Rad(0.1) { new_fovy = Rad(0.1); }
        if new_fovy > Rad(1.5) { new_fovy = Rad(1.5); }
        self.projection.set_fovy(new_fovy);
        self.position.z += v.y * c.speed * dt;
        if self.position.z < 0.4 { self.position.z = 0.4; }

        c.scroll = 0.0;
        c.mouse_dx = 0.0;
        c.mouse_dy = 0.0;
        *v *= 0.9;
        self.rotation = yaw * self.rotation * pitch; 

    }


    pub fn update_orbit(&mut self, dt: Duration) 
    {
        let c = &mut self.controller;
        let v = &mut self.velocity;
        *v += (c.force * c.speed * 0.06 ) / (v.magnitude() + 1.0);
        let dt = dt.as_secs_f32();
        let forward = self.rotation * Vector3::unit_z();
        let yaw =  Quaternion::from_angle_z(Rad(-c.mouse_dx) * c.sensitivity * dt);                                         // world z
        let pitch =  Quaternion::from_axis_angle(self.rotation * Vector3::unit_x(), Rad(-c.mouse_dy) * c.sensitivity * dt); // current x
        let rotation = yaw * pitch;
        let point_as_quat = Quaternion::new(0.0, self.position.x, self.position.y, self.position.z);
        let rotated_quat = rotation * point_as_quat * rotation.invert();
        let rotated_point = Point3::new(rotated_quat.v.x, rotated_quat.v.y, rotated_quat.v.z);

        self.position = rotated_point;
        self.position += forward * v.z * c.speed * dt;
        self.position.z += v.y * c.speed * dt;
        self.position.x += v.x * c.speed * dt;
        self.position.y += c.scroll * c.speed * c.sensitivity * dt;

        c.scroll *= 0.0;
        c.mouse_dx = 0.0;
        c.mouse_dy = 0.0;
        *v *= 0.9;
        self.rotation = rotation * self.rotation;

    }


    pub fn calc_matrix(&self) -> [[f32; 4]; 4]
    {        
        // world is z-up, camera is y-up z-forward
        Matrix4::look_to_rh
        (
            self.position,
            -self.rotation * Vector3::unit_z(),
            Vector3::unit_z(),
        ).into()
    }

}






impl Projection 
{
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

    pub fn new_orthographic<F: Into<Rad<f32>>>(
        width: u32,
        height: u32,
        fovy: F,
    ) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear: 0.1,
            zfar: 100.0,
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


    pub fn calc_matrix(&self) -> Matrix4<f32> 
    {
        OPENGL_TO_WGPU_MATRIX * perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }
}





 
impl CameraController 
{
    pub fn new(speed: f32, sensitivity: f32) -> Self 
    {
        Self {
            force: Vector3::zero(),
            mouse_dx: 0.0,
            mouse_dy: 0.0,
            mouse_pos: PhysicalPosition::new(0.0, 0.0),
            scroll: 0.0,
            speed,
            sensitivity,
        }
    }

    pub fn process_keyboard(&mut self, key: KeyEvent, state: ElementState)
    {
        let amount = match state 
        {
            ElementState::Pressed => 1.0,
            ElementState::Released => 0.0,
        };
        if let PhysicalKey::Code(keycode) = key.physical_key 
        {
            match keycode {
                KeyCode::KeyW | KeyCode::ArrowUp => {
                    self.force.z = amount;
                }
                KeyCode::KeyS | KeyCode::ArrowDown => {
                    self.force.z = -amount;
                }
                KeyCode::KeyA | KeyCode::ArrowLeft => {
                    self.force.x = -amount;
                }
                KeyCode::KeyD | KeyCode::ArrowRight => {
                    self.force.x = amount;
                }
                KeyCode::Space => {
                    self.force.y = amount;
                }
                KeyCode::ShiftLeft => {
                    self.force.y = -amount;
                }
                _ => {}
            }
        } 
        else { }
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.mouse_dx = mouse_dx as f32;
        self.mouse_dy = mouse_dy as f32;
    }


    pub fn process_mouse_pos(&mut self, x:f64, y:f64 ) {
        self.mouse_pos.x = x as f64;
        self.mouse_pos.y = y as f64;
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scroll = -match delta {
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition {
                y: scroll,
                ..
            }) => *scroll as f32,
        };
    }
}
