use std::ffi::c_void;
use std::sync::mpsc::Receiver;
use glfw::{Action, Context, Key};
use crate::runner::Runner;

use cgmath::{Matrix4, Vector3, vec3, Deg, perspective, Point3};
use cgmath::prelude::*;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use crate::common::process_input;
use crate::shader;

pub struct Camera;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

impl Runner for Camera {
    fn chapter(&self) -> i32 { 1 }
    fn section(&self) -> i32 { 8 }
    fn name(&self) -> &'static str {
        "camera"
    }

    fn run(&self) {
        let mut camera = crate::camera::Camera::new(Point3::new(0.0, 0.0, 3.0));

        let mut firstMouse = true;
        let mut lastX = SCR_WIDTH as f32 / 2.0;
        let mut lastY = SCR_HEIGHT as f32 / 2.0;

        // timing
        let mut deltaTime: f32;
        let mut lastFrameTime: f32 = 0.0;

        // Setup window initialization and configuration
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::openGlForwardCompat(true));

        // glfw window creation
        let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "Camera Controls", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);

        // tell GLFW to capture our mouse
        window.set_cursor_mode(glfw::CursorMode::Disabled);

        // gl: load all OpenGL function pointers
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let (shaderProgram, VBO, VAO, texture1, texture2, cubePositions) = unsafe {
            // configure the global opengl state
            gl::Enable(gl::DEPTH_TEST);

            // build and compile our shader program
            let shaderProgram = shader::Shader::new(
                &std::path::Path::new("shaders/1.8.camera.vert"),
                &std::path::Path::new("shaders/1.8.camera.frag"),
            );

            // set up vertex data and buffer(s) for cube
            let vertices = [
                -0.5f32, -0.5, -0.5, 0.0, 0.0,
                0.5, -0.5, -0.5, 1.0, 0.0,
                0.5, 0.5, -0.5, 1.0, 1.0,
                0.5, 0.5, -0.5, 1.0, 1.0,
                -0.5, 0.5, -0.5, 0.0, 1.0,
                -0.5, -0.5, -0.5, 0.0, 0.0,
                -0.5, -0.5, 0.5, 0.0, 0.0,
                0.5, -0.5, 0.5, 1.0, 0.0,
                0.5, 0.5, 0.5, 1.0, 1.0,
                0.5, 0.5, 0.5, 1.0, 1.0,
                -0.5, 0.5, 0.5, 0.0, 1.0,
                -0.5, -0.5, 0.5, 0.0, 0.0,
                -0.5, 0.5, 0.5, 1.0, 0.0,
                -0.5, 0.5, -0.5, 1.0, 1.0,
                -0.5, -0.5, -0.5, 0.0, 1.0,
                -0.5, -0.5, -0.5, 0.0, 1.0,
                -0.5, -0.5, 0.5, 0.0, 0.0,
                -0.5, 0.5, 0.5, 1.0, 0.0,
                0.5, 0.5, 0.5, 1.0, 0.0,
                0.5, 0.5, -0.5, 1.0, 1.0,
                0.5, -0.5, -0.5, 0.0, 1.0,
                0.5, -0.5, -0.5, 0.0, 1.0,
                0.5, -0.5, 0.5, 0.0, 0.0,
                0.5, 0.5, 0.5, 1.0, 0.0,
                -0.5, -0.5, -0.5, 0.0, 1.0,
                0.5, -0.5, -0.5, 1.0, 1.0,
                0.5, -0.5, 0.5, 1.0, 0.0,
                0.5, -0.5, 0.5, 1.0, 0.0,
                -0.5, -0.5, 0.5, 0.0, 0.0,
                -0.5, -0.5, -0.5, 0.0, 1.0,
                -0.5, 0.5, -0.5, 0.0, 1.0,
                0.5, 0.5, -0.5, 1.0, 1.0,
                0.5, 0.5, 0.5, 1.0, 0.0,
                0.5, 0.5, 0.5, 1.0, 0.0,
                -0.5, 0.5, 0.5, 0.0, 0.0,
                -0.5, 0.5, -0.5, 0.0, 1.0
            ];

            // get world space positions of cubes
            let cubePositions = [
                vec3(0.0f32, 0.0, 0.0),
                vec3(2.0, 5.0, -15.0),
                vec3(-1.5, -2.2, -2.5),
                vec3(-3.8, -2.0, -12.3),
                vec3(2.4, -0.4, -3.5),
                vec3(-1.7, 3.0, -7.5),
                vec3(1.3, -2.0, -2.5),
                vec3(1.5, 2.0, -2.5),
                vec3(1.5, 0.2, -1.5),
                vec3(-1.3, 1.0, -1.5),
            ];

            let (mut VBO, mut VAO) = (0, 0);

            gl::GenVertexArrays(1, &mut VAO);
            gl::GenBuffers(1, &mut VBO);

            gl::BindVertexArray(VAO);

            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                           &vertices[0] as *const f32 as *const c_void,
                           gl::STATIC_DRAW,
            );

            let stride = 5 * std::mem::size_of::<GLfloat>() as GLsizei;
            // position attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            // texture coordinate attribute
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * std::mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(1);

            // load and create texture
            let (mut texture1, mut texture2) = (0, 0);

            // texture 1
            gl::GenTextures(1, &mut texture1);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            // set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // Set the texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            // load image, create texture and generate mipmaps
            let img = image::open(&std::path::Path::new("textures/container.jpg")).expect("Failed to load texture.");
            let data = img.as_bytes();
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGB as i32,
                           img.width() as i32,
                           img.height() as i32,
                           0,
                           gl::RGB,
                           gl::UNSIGNED_BYTE,
                           &data[0] as *const u8 as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            // texture 2
            gl::GenTextures(1, &mut texture2);
            gl::BindTexture(gl::TEXTURE_2D, texture2);
            // set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // Set the texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            // load image create texture and generate mipmaps
            let img = image::open(&std::path::Path::new("textures/awesomeface.png")).expect("Failed to load texture");
            let img = img.flipv();
            let data = img.as_bytes();
            // note that the awesomeface.png has transparency and thus and alpha channel, so make sure to tell OpenGL the data type is of GL_RGBA
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGB as i32,
                           img.width() as i32,
                           img.height() as i32,
                           0,
                           gl::RGBA,
                           gl::UNSIGNED_BYTE,
                           &data[0] as *const u8 as *const c_void,
            );

            // tell opegl for each sampler to which texture unit it belongs to
            shaderProgram.useProgram();
            shaderProgram.set_int(&std::ffi::CString::new("texture1").unwrap(), 0);
            shaderProgram.set_int(&std::ffi::CString::new("texture2").unwrap(), 1);

            (shaderProgram, VBO, VAO, texture1, texture2, cubePositions)
        };

        while !window.should_close() {
            let currentFrameTime = glfw.get_time() as f32;
            deltaTime = currentFrameTime - lastFrameTime;
            lastFrameTime = currentFrameTime;

            // event handle
            crate::common::process_events(&events, &mut firstMouse, &mut lastX, &mut lastY, &mut camera);

            //input
            process_input(&mut window, deltaTime, &mut camera);

            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                // bind textures on corresponding texture units
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, texture1);
                gl::ActiveTexture(gl::TEXTURE1);
                gl::BindTexture(gl::TEXTURE_2D, texture2);

                // activate shader
                shaderProgram.useProgram();

                // pass projection matrix to shader (note that in thiis case it cound change every frame)
                let projection: Matrix4<f32> = perspective(Deg(camera.get_zoom()), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
                shaderProgram.set_mat4(&std::ffi::CString::new("projection").unwrap(), &projection);

                // camera view transformation
                let view = camera.GetViewMatrix();
                shaderProgram.set_mat4(&std::ffi::CString::new("view").unwrap(), &view);

                // render boxes
                gl::BindVertexArray(VAO);
                for (i, position) in cubePositions.iter().enumerate() {
                    let mut model: Matrix4<f32> = Matrix4::from_translation(*position);
                    let angle = 20.0 * i as f32;
                    model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(angle));
                    shaderProgram.set_mat4(&std::ffi::CString::new("model").unwrap(), &model);

                    gl::DrawArrays(gl::TRIANGLES, 0, 36);
                }
            }

            window.swap_buffers();
            glfw.poll_events();
        }

        unsafe {
            gl::DeleteVertexArrays(1, &VAO);
            gl::DeleteBuffers(1, &VBO);
        }
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {}