use cgmath;
use cgmath::vec3;
use cgmath::prelude::*;

type Point3 = cgmath::Point3<f32>;
type Vector3 = cgmath::Vector3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;

// Defines several possible options for camera movement. Used as an abstraction to stay away from window-system specific input methods
#[derive(PartialEq, Clone, Copy)]
pub enum Camera_Movement {
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
    Position: Point3,
    Front: Vector3,
    Up: Vector3,
    Right: Vector3,
    WorldUp: Vector3,

    // Euler Angles
    Yaw: f32,
    Pitch: f32,

    // Camera Options
    MovementSpeed: f32,
    MouseSensitivity: f32,
    Zoom: f32,
}

use self::Camera_Movement::*;

impl Default for Camera {
    fn default() -> Self {
        let mut camera = Camera {
            Position: Point3::new(0.0, 0.0, 0.0),
            Front: vec3(0.0, 0.0, -1.0),
            Up: Vector3::zero(),
            Right: Vector3::zero(),
            WorldUp: Vector3::unit_y(),
            Yaw: YAW,
            Pitch: PITCH,
            MovementSpeed: SPEED,
            MouseSensitivity: SENSITIVITY,
            Zoom: ZOOM,
        };

        camera.updateCameraVectors();
        camera
    }
}

impl Camera {
    pub fn get_zoom(&self) -> f32 {
        self.Zoom
    }

    /// creates a camera at the point described
    pub fn new(pt: Point3) -> Self {
        Camera {
            Position: pt,
            ..Camera::default()
        }
    }

    /// Returns the view matrix calculated using Euler Angles and the look at matrix
    pub fn GetViewMatrix(&self) -> Matrix4 {
        Matrix4::look_at(self.Position, self.Position + self.Front, self.Up)
    }

    /// processes input recieved from any keyboard like input system. Accepts input parameter in the form of camera defined ENUM to abstract from windowing systems
    pub fn ProcessKeyboard(&mut self, direction: Camera_Movement, delta_time: f32) {
        let velocity = self.MovementSpeed * delta_time;
        match direction {
            FORWARD => self.Position += self.Front * velocity,
            BACKWARD => self.Position -= self.Front * velocity,
            LEFT => self.Position -= self.Right * velocity,
            RIGHT => self.Position += self.Right * velocity,
        };
    }

    /// processes input received from a mouse input system. Expects the offset value in both the x and y directions.
    pub fn ProcessMouseMovement(&mut self, mut xoffset: f32, mut yoffset: f32, constrain_pitch: bool) {
        xoffset *= self.MouseSensitivity;
        yoffset *= self.MouseSensitivity;

        self.Yaw += xoffset;
        self.Yaw %= 360.0;

        self.Pitch += yoffset;

        // make sure that pitch doesn't cause screen to flip
        if constrain_pitch {
            self.Pitch = self.Pitch.min(89.0);
            self.Pitch = self.Pitch.max(-89.0);
        }

        self.Yaw;

        // update front right and up vectors using the update euler angles
        self.updateCameraVectors();
    }

    /// Processes input received from a mouse scroll-wheel event only requires input on the vertical wheel-axis
    pub fn ProcessMouseScroll(&mut self, yoffset: f32) {
        self.Zoom -= yoffset;
        self.Zoom = self.Zoom.max(1.0);
        self.Zoom = self.Zoom.min(45.0);
    }

    /// Calculates the front vector from the camera's euler angles
    fn updateCameraVectors(&mut self) {
        let front = Vector3 {
            x: self.Yaw.to_radians().cos() * self.Pitch.to_radians().cos(),
            y: self.Pitch.to_radians().sin(),
            z: self.Yaw.to_radians().sin() * self.Pitch.to_radians().cos(),
        };

        self.Front = front.normalize();
        // also re-calculate the Right and Up vector
        self.Right = self.Front.cross(self.WorldUp).normalize();
        self.Up = self.Right.cross(self.Front).normalize();
    }
}