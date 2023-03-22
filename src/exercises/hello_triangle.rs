use std::ffi::{c_void, CString};
use std::ptr;
use std::sync::mpsc::Receiver;
use gl::types::{GLfloat, GLint, GLsizei, GLsizeiptr, GLuint};
use glfw::{Action, Context, Key};
use crate::runner::Runner;

pub struct HelloTriangle;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const VERTICES: [f32; 12] = [
    0.5, 0.5, 0.0, // top right
    0.6, -0.5, 0.0, // bottom right
    -0.5, -0.5, 0.0, // bottom left
    -0.5, 0.5, 0.0, // top left
];

const INDEXES: [u32; 6] = [
    0, 1, 3, // first triangle
    1, 2, 3, // second triangle
];

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;


const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

impl Runner for HelloTriangle {
    fn chapter(&self) -> i32 { 1 }
    fn section(&self) -> i32 { 3 }
    fn name(&self) -> &'static str {
        "hello triangle"
    }

    fn run(&self) {
        // glfw: initalize and configure
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        // glfw window creation
        let (mut window, events) =
            glfw.create_window(SCR_WIDTH, SCR_HEIGHT, self.name(), glfw::WindowMode::Windowed)
                .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        // gl: load all OpenGL function pointers
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        // Build and compile the shader program
        let (shader_program,  VAO) = unsafe {

            // vertex shader

            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);

            // check for shader compile errors
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(vertex_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut gl::types::GLchar);
                println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", std::str::from_utf8(&info_log).unwrap());
            }

            // fragment shader

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);

            // check for shader compile errors
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(fragment_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut gl::types::GLchar);
                println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", std::str::from_utf8(&info_log).unwrap());
            }

            // link shaders

            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
            // check for linking errors
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shader_program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut gl::types::GLchar);
                println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", std::str::from_utf8(&info_log).unwrap());
            }
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            // gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<GLfloat>() as gl::types::GLsizei, ptr::null());
            // gl::EnableVertexAttribArray(0);

            // feed in the data

            let (mut VBO, mut VAO, mut EBO) = (0, 0, 0);
            gl::GenVertexArrays(1, &mut VAO);
            gl::GenBuffers(1, &mut VBO);
            gl::GenBuffers(1, &mut EBO);

            // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
            gl::BindVertexArray(VAO);

            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (VERTICES.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                           &VERTICES[0] as *const f32 as *const c_void,
                           gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           (INDEXES.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
                           &INDEXES[0] as *const u32 as *const c_void,
                           gl::STATIC_DRAW);

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<GLfloat>() as gl::types::GLsizei, ptr::null());
            gl::EnableVertexAttribArray(0);

            // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
            // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
            gl::BindVertexArray(0);

            // uncomment this call to draw in wireframe polygons.
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

            (shader_program, VAO)
        };

        // uncomment this call to draw in wireframe polygons.
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        // render loop
        while !window.should_close() {
            // events
            process_events(&mut window, &events);

            // Render
            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                gl::UseProgram(shader_program);
                gl::BindVertexArray(VAO);
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            }

            // glfw: swap buffers and poll IO events (keys presssed/released, mouse moved, etc)
            window.swap_buffers();
            glfw.poll_events();
        }
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimesnions.
                // note that the width and height will be significantly larger than specified on retina displays
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}