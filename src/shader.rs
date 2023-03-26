use std::ffi::CString;
use std::io::Read;
use gl::types::{GLchar, GLint, GLuint};
use image::codecs::png::CompressionType::Default;

pub struct Shader {
    uid: GLuint,
}

impl Shader {
    pub fn get_id(&self) -> GLuint {
        self.uid
    }

    pub fn new(vertex_path: &std::path::Path, fragment_path: &std::path::Path) -> Self {
        let vertex_shader = read_file_to_Cstring(vertex_path);
        let fragment_shader = read_file_to_Cstring(fragment_path);

        Shader { uid: compile_shader_program(&vertex_shader, &fragment_shader) }
    }

    pub fn useProgram(&self) {
        unsafe {
            gl::UseProgram(self.uid);
        }
    }

    // Utility uniform functions
    pub fn set_bool(&self, name: &CString, value: bool) {
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.uid, name.as_ptr()), value as i32);
        }
    }

    pub fn set_int(&self, name: &CString, value: i32) {
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.uid, name.as_ptr()), value);
        }
    }

    pub fn set_float(&self, name: &CString, value: f32) {
        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self.uid, name.as_ptr()), value);
        }
    }
}

fn read_file_to_Cstring(path: &std::path::Path) -> CString {
    let mut file = std::fs::File::open(path).expect(&*format!("Failed to open: {}", path.display()));
    let mut text = std::default::Default::default();
    file.read_to_string(&mut text).expect(&*format!("Failed to read: {}", path.display()));

    CString::new(text.as_bytes()).expect(&*format!("Failed to convert: {}", path.display()))
}

fn compile_shader_program(vertex_shader: &CString, fragment_shader: &CString) -> GLuint {
    let id;
    unsafe {
        let vertex = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex, 1, &vertex_shader.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex);
        check_compile_errors(vertex, "VERTEX");

        let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment, 1, &fragment_shader.as_ptr(), std::ptr::null());
        gl::CompileShader(fragment);
        check_compile_errors(fragment, "FRAGMENT");

        id = gl::CreateProgram();
        gl::AttachShader(id, vertex);
        gl::AttachShader(id, fragment);
        gl::LinkProgram(id);
        check_compile_errors(id, "PROGRAM");

        gl::DeleteShader(vertex);
        gl::DeleteShader(fragment);
    }

    id
}

fn check_compile_errors(id: GLuint, name: &str) {
    let mut success = gl::FALSE as GLint;
    let mut info_log = std::vec::Vec::with_capacity(1024);
    unsafe {
        info_log.set_len(1024 - 1); // subtract 1 to keep a trailing null char

        if name != "PROGRAM" {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(id, 1024, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                let info_log = std::str::from_utf8(&info_log).unwrap();
                panic!("ERROR::{name}:\n{info_log}\n");
            }
        } else {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(id, 1024, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                let info_log = std::str::from_utf8(&info_log).unwrap();
                panic!("ERROR::{name}:\n{info_log}\n");
            };
        }
    }
}