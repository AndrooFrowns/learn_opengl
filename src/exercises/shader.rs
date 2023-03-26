use std::ffi::c_void;
use std::ptr;
use std::sync::mpsc::Receiver;
use gl::types::{GLfloat, GLsizeiptr, GLuint};
use glfw::{Action, Context, Key};
use crate::runner::Runner;
use crate::shader;

pub struct Shader;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const VERTICES: [f32; 24] = [
    // positions   // colors
    0.5, 0.5, 0.0, 0.0, 0.0, 1.0, // top right
    0.6, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom right
    -0.5, -0.5, 0.0, 0.0, 1.0, 0.0,// bottom left
    -0.5, 0.5, 0.0, 1.0, 0.0, 0.0, // top left
];

const INDEXES: [u32; 6] = [
    0, 1, 3, // first triangle
    1, 2, 3, // second triangle
];

//  // EXAMPLE of passing data from vertex shader to fragment shader
// const VERTEX_SHADER_SOURCE: &str = r#"
//     #version 330 core
//     layout (location = 0) in vec3 aPos; // the position variable has attribute position zero
//
//     out vec4 vertexColor; // specify a color output to the fragment shader
//
//     void main() {
//        gl_Position = vec4(aPos, 1.0); // see how we directly give a vec3 to vec4's consturctor
//        vertexColor = vec4(0.5, 0.0, 0.0, 1.0); // set the output variable to a dark red
//     }
// "#;

//  // EXAMPLE of using data from the vertex shader
// const FRAGMENT_SHADER_SOURCE: &str = r#"
//     #version 330 core
//     out vec4 FragColor;
//
//     in vec4 vertexColor; // the input variable from the vertex shader (same name)
//
//     void main() {
//        FragColor = vertexColor;
//     }
// "#;

//  for example changing color with time
// const VERTEX_SHADER_SOURCE: &str = r#"
//     #version 330 core
//     layout (location = 0) in vec3 aPos;
//
//     void main() {
//        gl_Position = vec4(aPos, 1.0);
//     }
// "#;

// change color with time example
// const FRAGMENT_SHADER_SOURCE: &str = r#"
//     #version 330 core
//     out vec4 FragColor;
//
//     uniform vec4 ourColor; // set from OpenGL code
//
//     void main() {
//        FragColor = ourColor;
//     }
// "#;
//
// const VERTEX_SHADER_SOURCE: &str = r#"
//     #version 330 core
//     layout (location = 0) in vec3 aPos;
//     layout (location = 1) in vec3 aColor;
//
//     out vec3 ourColor;
//
//     void main() {
//        gl_Position = vec4(aPos, 1.0);
//        ourColor = aColor;
//     }
// "#;
//
// const FRAGMENT_SHADER_SOURCE: &str = r#"
//     #version 330 core
//     out vec4 FragColor;
//     in vec3 ourColor;
//
//     void main() {
//        FragColor = vec4(ourColor, 1.0);
//     }
// "#;

impl Runner for Shader {
    fn chapter(&self) -> i32 { 1 }
    fn section(&self) -> i32 { 4 }
    fn name(&self) -> &'static str {
        "shader"
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

        let vertex_path = std::path::Path::new("shaders/firstShader.vert");
        let fragment_path = std::path::Path::new("shaders/firstShader.frag");


        // Build and compile the shader program
        #[allow(non_snake_case)]
        let (shader_program, VAO) = unsafe {

            let shader_program = shader::Shader::new(vertex_path, fragment_path);

            // feed in the data

            #[allow(non_snake_case)]
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

            // position attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 6 * std::mem::size_of::<GLfloat>() as gl::types::GLsizei, ptr::null());
            gl::EnableVertexAttribArray(0);
            // color attribute
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 6 * std::mem::size_of::<GLfloat>() as gl::types::GLsizei, (3 * std::mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(1);

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
                // clear the colorbuffer
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                // activate program
                shader_program.use_program();


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