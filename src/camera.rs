use cgmath;
use cgmath::vec3;
use cgmath::prelude::*;

type Point3 = cgmath::Point3<f32>;
type Vector3 = cgmath::Vector3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;

// Defines several possible options for camera movement. Used as an abstraction to stay away from window-system specific input methods
#[derive(PartialEq, Clone, Copy)]
pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}

// Default Camera Values
const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 2.5;
const SENSITIVITY: f32 = 0.1;
const ZOOM: f32 = 45.0;

pub struct Camera {
    // Camera Atributes
    position: Point3,
    front: Vector3,
    up: Vector3,
    right: Vector3,
    world_up: Vector3,

    // Euler Angles
    yaw: f32,
    pitch: f32,

    // Camera Options
    movement_speed: f32,
    mouse_sensitivity: f32,
    zoom: f32,
}

use self::CameraMovement::*;

impl Default for Camera {
    fn default() -> Self {
        let mut camera = Camera {
            position: Point3::new(0.0, 0.0, 0.0),
            front: vec3(0.0, 0.0, -1.0),
            up: Vector3::zero(),
            right: Vector3::zero(),
            world_up: Vector3::unit_y(),
            yaw: YAW,
            pitch: PITCH,
            movement_speed: SPEED,
            mouse_sensitivity: SENSITIVITY,
            zoom: ZOOM,
        };

        camera.update_camera_vectors();
        camera
    }
}

impl Camera {
    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    /// creates a camera at the point described
    pub fn new(pt: Point3) -> Self {
        Camera {
            position: pt,
            ..Camera::default()
        }
    }

    /// Returns the view matrix calculated using Euler Angles and the look at matrix
    pub fn get_view_matrix(&self) -> Matrix4 {
        Matrix4::look_at_rh(self.position, self.position + self.front, self.up)
    }

    /// processes input recieved from any keyboard like input system. Accepts input parameter in the form of camera defined ENUM to abstract from windowing systems
    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;
        match direction {
            FORWARD => self.position += self.front * velocity,
            BACKWARD => self.position -= self.front * velocity,
            LEFT => self.position -= self.right * velocity,
            RIGHT => self.position += self.right * velocity,
        };
    }

    /// processes input received from a mouse input system. Expects the offset value in both the x and y directions.
    pub fn process_mouse_movement(&mut self, mut xoffset: f32, mut yoffset: f32, constrain_pitch: bool) {
        xoffset *= self.mouse_sensitivity;
        yoffset *= self.mouse_sensitivity;

        self.yaw += xoffset;
        self.yaw %= 360.0;

        self.pitch += yoffset;

        // make sure that pitch doesn't cause screen to flip
        if constrain_pitch {
            self.pitch = self.pitch.min(89.0);
            self.pitch = self.pitch.max(-89.0);
        }

        // update front right and up vectors using the update euler angles
        self.update_camera_vectors();
    }

    /// Processes input received from a mouse scroll-wheel event only requires input on the vertical wheel-axis
    pub fn process_mouse_scroll(&mut self, yoffset: f32) {
        self.zoom -= yoffset;
        self.zoom = self.zoom.max(1.0);
        self.zoom = self.zoom.min(45.0);
    }

    /// Calculates the front vector from the camera's euler angles
    fn update_camera_vectors(&mut self) {
        let front = Vector3 {
            x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            y: self.pitch.to_radians().sin(),
            z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        };

        self.front = front.normalize();
        // also re-calculate the Right and Up vector
        self.right = self.front.cross(self.world_up).normalize();
        self.up = self.right.cross(self.front).normalize();
    }
}