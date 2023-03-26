use std::ffi::c_void;
use std::sync::mpsc::Receiver;
use gl::types::{GLfloat, GLsizeiptr};
use glfw::{Action, Context, Key};
use crate::runner::Runner;

pub struct Textures;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

impl Runner for Textures {
    fn chapter(&self) -> i32 { 1 }
    fn section(&self) -> i32 { 5 }
    fn name(&self) -> &'static str {
        "textures"
    }

    fn run(&self) {
        // initialize glfw
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        // glfw window creation

        let (mut window, events) =
            glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "LearnOpenGL: texture", glfw::WindowMode::Windowed)
                .expect("Failed to created glfw window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);


        // gl: load all OpenGL function pointers
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        #[allow(non_snake_case)]
        let (shaderProgram, VBO, VAO, EBO, texture1, texture2) = unsafe {
            // build and compile the shader program.
            let vert_path = std::path::Path::new("shaders/1.4.texture.vert");
            let frag_path = std::path::Path::new("shaders/1.4.texture.frag");
            let shaderProgram = crate::shader::Shader::new(vert_path, frag_path);

            // set up vertex data and buffeers and configure vertex attributes
            let vertices: [f32; 32] = [
                // positions       // colors        // texture coords
                 0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
                 0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
                -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
                -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0,  // top left
            ];

            let indices = [
                0, 1, 3, // first triangle
                1, 2, 3, // second triangle
            ];

            let mut VBO = 0;
            let mut VAO = 0;
            let mut EBO = 0;

            gl::GenVertexArrays(1, &mut VAO);
            gl::GenBuffers(1, &mut VBO);
            gl::GenBuffers(1, &mut EBO);

            gl::BindVertexArray(VAO);

            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                           &vertices as *const f32 as *const std::ffi::c_void,
                           gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           (indices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                           &indices[0] as *const i32 as *const c_void,
                           gl::STATIC_DRAW);

            let stride = 8 * std::mem::size_of::<GLfloat>() as gl::types::GLsizei;

            //positition attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            //color attribute
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * std::mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(1);
            //texture coord attribute
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * std::mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(2);

            let mut texture_1 = 0;
            gl::GenTextures(1, &mut texture_1);
            gl::BindTexture(gl::TEXTURE_2D, texture_1); // all upcoming GL_TEXTURE_2D ops now affect this texture object

            // set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            // set the texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            let img = image::open(std::path::Path::new("textures/container.jpg")).expect("failed to load texture");
            let data = img.as_bytes();
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGB as i32,
                           img.width() as i32,
                           img.height() as i32,
                           0,
                           gl::RGB,
                           gl::UNSIGNED_BYTE,
                           data.as_ptr() as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);

            let mut texture_2 = 0;
            gl::GenTextures(1, &mut texture_2);
            gl::BindTexture(gl::TEXTURE_2D, texture_2); // all upcoming GL_TEXTURE_2D ops now affect this texture object

            // set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            // set the texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            let img = image::open(std::path::Path::new("textures/awesomeface.png")).expect("failed to load texture");
            let img = img.flipv();
            let data = img.as_bytes();
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGB as i32,
                           img.width() as i32,
                           img.height() as i32,
                           0,
                           gl::RGBA,
                           gl::UNSIGNED_BYTE,
                           data.as_ptr() as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);

            // tell opelgl for each sampler which texture unit it belongs to.

            shaderProgram.use_program();
            // either set it manually like:
            // gl::uniform1i(gl::GetUniformLocation(shaderProgram.get_id(), c_str!("texture1").as_ptr()), 0); // using c_str! to avoid runtime overhead
            // or set it ivia the texture class
            shaderProgram.set_int(&std::ffi::CString::new("texture1").unwrap(), 0);
            shaderProgram.set_int(&std::ffi::CString::new("texture2").unwrap(), 1);

            shaderProgram.set_float(&std::ffi::CString::new("ratio").unwrap(), 0.0);



            (shaderProgram, VBO, VAO, EBO, texture_1, texture_2)
        };

        // render loop
        while !window.should_close() {
            process_events(&mut window, &events);

            // render
            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, texture1);
                gl::ActiveTexture(gl::TEXTURE1);
                gl::BindTexture(gl::TEXTURE_2D, texture2);

                let time = glfw.get_time();
                let ratio = (time.sin() + 1.0) as f32;


                shaderProgram.set_float(&std::ffi::CString::new("ratio").unwrap(), ratio );

                shaderProgram.use_program();

                gl::BindVertexArray(VAO);
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            }

            window.swap_buffers();
            glfw.poll_events();
        }

        unsafe {
            gl::DeleteVertexArrays(1, &VAO);
            gl::DeleteBuffers(1, &VBO);
            gl::DeleteBuffers(1, &EBO);
        }
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}