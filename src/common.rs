use std::ffi::c_void;
use std::path::Path;
use std::sync::mpsc::Receiver;
use glfw::Action;
use glfw::Key::Escape;
use glfw::WindowEvent::Key;
use image::DynamicImage;
use image::DynamicImage::{ImageLuma8, ImageLumaA8, ImageRgb8, ImageRgba8};
use crate::camera::Camera;
use crate::camera::Camera_Movement::{BACKWARD, FORWARD, LEFT, RIGHT};

/// Event processing function use for the camera class and later tutorials
pub fn process_events(
    events: &Receiver<(f64, glfw::WindowEvent)>,
    firstMouse: &mut bool,
    lastX: &mut f32,
    lastY: &mut f32,
    camera: &mut Camera,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::CursorPos(xpos, ypos) => {
                let (xpos, ypos) = (xpos as f32, ypos as f32);
                if *firstMouse {
                    *lastX = xpos;
                    *lastY = ypos;
                    *firstMouse = false;
                }

                let xoffset = xpos - *lastX;
                let yoffset = *lastY - ypos; // reverse since y-coordinates go from bottom to top

                *lastX = xpos;
                *lastY = ypos;

                camera.ProcessMouseMovement(xoffset, yoffset, true);
            }
            glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                camera.ProcessMouseScroll(yoffset as f32);
            }
            _ => {}
        }
    }
}


/// Input processing function as introduced for camera
pub fn process_input(window: &mut glfw::Window, deltaTime: f32, camera: &mut Camera) {
    if window.get_key(glfw::Key::Escape) == Action::Press {
        window.set_should_close(true)
    }

    if window.get_key(glfw::Key::W) == Action::Press {
        camera.ProcessKeyboard(FORWARD, deltaTime);
    }
    if window.get_key(glfw::Key::S) == Action::Press {
        camera.ProcessKeyboard(BACKWARD, deltaTime);
    }
    if window.get_key(glfw::Key::A) == Action::Press {
        camera.ProcessKeyboard(LEFT, deltaTime);
    }
    if window.get_key(glfw::Key::D) == Action::Press {
        camera.ProcessKeyboard(RIGHT, deltaTime);
    }
}

/// utility function for loading a 2D texture from file
#[allow(dead_code)]
pub unsafe fn loadTexture(path: &str) -> u32 {
    let mut textureID = 0;

    gl::GenTextures(1, &mut textureID);
    let img = image::open(&Path::new(path)).expect("Texture failed to load");
    let format = match img {
        ImageLuma8(_) => gl::RED,
        ImageLumaA8(_) => gl::RG,
        ImageRgb8(_) => gl::RGB,
        ImageRgba8(_) => gl::RGBA,
        _ => {panic!("invalid image format in: {path}")}
    };

    let data = img.as_bytes();

    gl::BindTexture(gl::TEXTURE_2D, textureID);
    gl::TexImage2D(gl::TEXTURE_2D,
                   0,
                   format as i32,
                   img.width() as i32,
                   img.height() as i32,
                   0,
                   format,
                   gl::UNSIGNED_BYTE,
                   &data[0] as *const u8 as *const c_void);
    gl::GenerateMipmap(gl::TEXTURE_2D);

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    textureID
}

