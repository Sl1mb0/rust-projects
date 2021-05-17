use std::ffi::{CStr, CString};

pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        match success {
            1 => {
                for shader in shaders {
                    unsafe {
                        gl::DetachShader(program_id, shader.id());
                    }
                }
                Ok(Program { id: program_id })
            }
            _ => {
                let mut len: gl::types::GLint = 0;
                unsafe {
                    gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
                }

                let error = create_whitespace_cstring(len as usize);

                unsafe {
                    gl::GetProgramInfoLog(
                        program_id,
                        len,
                        std::ptr::null_mut(),
                        error.as_ptr() as *mut gl::types::GLchar,
                    );
                }
                return Err(error.to_string_lossy().into_owned());
            }
        }
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub struct Shader {
    id: gl::types::GLuint,
}

// impl keyword is used to define implementations on types
//  inherent implementations are standalone,
//  trait implementations are used to implement traits for types, or other traits
//
impl Shader {
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

// as it currently stands,
// the Shader type leaks shader id without cleaning it up
// to remedy this, implement the Drop trait for Shader
// for more on how drops work:
// https://doc.rust-lang.org/book/ch15-03-drop.html
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            // Rust ensures that DeleteShader is called
            // exactly once for every Shader object id
            gl::DeleteShader(self.id);
        }
    }
}

// helper function to
// compile shader from a string
// return shader id as an int
// otherwise handle error
fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success) }

    match success {
        1 => Ok(id),
        _ => {
            // we were not successful
            // in compiling the shader
            // we now must handle the error

            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring(len as usize);

            unsafe {
                // write information from
                // shader log regarding
                // compilation error into 'error'
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }
            // error is a cstring
            // to_string_lossy converts cstring -> rstring
            // since to_string_lossy() also returns value
            // that can be either a String or a &str
            // into_owned to obtain a definite string
            return Err(error.to_string_lossy().into_owned());
        }
    }
}

// returns cstring with length 'len'
// filled with blank spaces
fn create_whitespace_cstring(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    buffer.extend([b' '].iter().cycle().take(len as usize));
    unsafe { CString::from_vec_unchecked(buffer) }
}
