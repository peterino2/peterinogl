use std::ffi::{CString, CStr};
use std::path;
use std::fs;
use std::io::Read;
use std::io;

pub struct ShaderPipe{
    frag_shader: Shader,
    vert_shader: Shader,
    prog_id: gl::types::GLuint,
}

impl ShaderPipe{

    pub fn construct() -> ShaderPipe
    {
        let frag_shader_src = load_file_as_cstr(path::Path::new("frag.shader")).unwrap();
        let vert_shader_src = load_file_as_cstr(path::Path::new("vert.shader")).unwrap();

        let frag_shader = Shader::from_frag_source(&frag_shader_src).unwrap();
        let vert_shader = Shader::from_vert_source(&vert_shader_src).unwrap();

        let prog_id = unsafe {gl::CreateProgram()};

        let mut success: gl::types::GLint = 1;

        unsafe{
            gl::AttachShader(prog_id, frag_shader.id);
            gl::AttachShader(prog_id, vert_shader.id);
            gl::LinkProgram(prog_id);
            gl::DetachShader(prog_id, frag_shader.id);
            gl::DetachShader(prog_id, vert_shader.id);
        }
        

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe{
                gl::GetProgramiv(prog_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let msg: Vec<u8> = vec![0x00; len as usize];
            unsafe{
                gl::GetProgramInfoLog(
                    prog_id, 
                    len, 
                    std::ptr::null_mut(), 
                    msg.as_ptr() as *mut gl::types::GLchar
                );
                panic!("Shader Linkage failed with error: {}", String::from_utf8(msg).unwrap())
            }
        }

        ShaderPipe{
            frag_shader, vert_shader, prog_id
        }
    }
    
    pub fn activate(&mut self)
    {
        unsafe{gl::UseProgram(self.prog_id);}
    }
}

impl Drop for ShaderPipe{

    fn drop(&mut self)
    {
        unsafe{
            gl::DeleteProgram(self.prog_id);
        }
    }
}

fn load_file_as_cstr(filepath: &path::Path) -> Result<CString, String>
{

    let display = filepath.display();
    let mut f = match fs::File::open(&filepath)
    {
        Err(why) => return Err(format!("couldn't open {}: {}", display, why)),
        Ok(filehandle) => filehandle,
    };

    let metadata = match fs::metadata(&filepath){
        Err(why) => return Err(format!("couldn't load metadata for file {}: {}", display, why)),
        Ok(meta) => meta,
    };

    let mut buffer = vec![0x00; metadata.len() as usize];
    let _unused = match f.read(&mut buffer) {
        Ok(f) => f,
        Err(why) => return Err(format!("Buffer overflow"))
    };

    let cstr = match CString::new(buffer)
    {
        Ok(cstr) => cstr,
        Err(failure) => return Err(format!("Invalid encoding."))
    };

    return Ok(cstr);
}

pub struct Shader{
    id: gl::types::GLuint
}

impl Shader{

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> 
        { Shader::from_source(source, gl::VERTEX_SHADER) }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> 
        { Shader::from_source(source, gl::FRAGMENT_SHADER) }

    pub fn from_source( 
        source: &CStr,
        kind: gl::types::GLuint
    ) -> Result<Shader, String>
    {
        let id = shader_from_source(source, kind)?;
        Ok(Shader{id})
    }
}

impl Drop for Shader{
    fn drop(&mut self)
    {
        unsafe{ 
            gl::DeleteShader(self.id); 
        }
    }
}

fn shader_from_source(
    source: &CStr,
    kind: gl::types::GLuint
) -> Result<gl::types::GLuint, String> 
{

    let id = unsafe {
        let id = gl::CreateShader(kind);
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
        id
    };
    let mut success: gl::types::GLint = 1;
    
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len)};
        let msg: Vec<u8> = vec![b' '; len as usize];

        unsafe{
            gl::GetShaderInfoLog(
                id, 
                len, 
                std::ptr::null_mut(), 
                msg.as_ptr() as *mut gl::types::GLchar
            );
        }
        return Err(
            String::from_utf8(msg).unwrap()
        );
    }
    Ok(id)
}

