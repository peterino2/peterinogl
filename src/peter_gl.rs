use std::ffi::{CString, CStr, c_void};
use std::path;
use std::fs;
use std::io::Read;
use std::io;
use std::mem::size_of;
use stb_image::image;
use gl::types as gt;

pub struct ShaderPipe{
    frag_shader: Shader,
    vert_shader: Shader,
    pub prog_id: gl::types::GLuint,
}

impl ShaderPipe{

    pub fn set_sampler(&mut self, name: &str, texid: gt::GLuint)
    {
        self.activate();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texid);
            gl::Uniform1i(gl::GetUniformLocation(self.prog_id, name.as_ptr() as *mut i8), 0);
        }
    }

    pub fn configure_textures(&mut self)
    {
        let border_color: Vec<f32> = vec![ 1.0, 1.0, 0.0, 1.0];
        unsafe { 
            gl::TexParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, border_color.as_ptr());            // configure 
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as gl::types::GLint);

            // mipmaps
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as gl::types::GLint);
        }
    }

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

/// Opinionated texture parameters

pub struct GraphicsObject{
    pub ebo: gt::GLuint,
    pub vbo: gt::GLuint,
    pub vao: gt::GLuint,
    pub tex: gt::GLuint,
    vertices: Vec<f32>,
    indices: Vec<i32>,
}


// Class for holding Opengl handles and 
// helping with abstracting opengl functions
impl GraphicsObject
{

    fn gen_buffers(&mut self){
        unsafe {
            gl::GenBuffers(1, &mut self.ebo);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenTextures(1, &mut self.tex);
        }
        println!("[gen_buffers] buffers generated");
    }


    fn configure_vertex_formats(&mut self)
    {
        unsafe{
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER, // target
                (self.vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                self.vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW, // usage
            );

            // describe  texture co-ordinate attribute
            gl::VertexAttribPointer(
                0, // index of the generic vertex attribute ("layout (location = 0)")
                3, // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                (0 * std::mem::size_of::<f32>()) as *mut usize as *mut c_void// offset of the first component
            );
            gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader

            // describe Color attribute
            gl::VertexAttribPointer(
                1, // index of the generic vertex attribute ("layout (location = 1)")
                3, // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                (3 * std::mem::size_of::<f32>()) as *mut usize as *mut c_void// offset of the first component
            );
            gl::EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader

            // describe texture attribute
            gl::VertexAttribPointer(
                2, // index of the generic vertex attribute ("layout (location = 1)")
                2, // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                (6 * std::mem::size_of::<f32>()) as *mut usize as *mut c_void// offset of the first component
            );
            gl::EnableVertexAttribArray(2); // this is "layout (location = 0)" in vertex shader

        }
        println!("[configure] Format Objects completed");
    }

    fn configure_element_format(&mut self)
    {
        unsafe{
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER, (size_of::<f32>() as isize) * (self.indices.len() as isize), 
                self.indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );
        }

        println!("[configure] element array formatted");
    }

    pub fn draw(&mut self)
    {
        unsafe{
            gl::BindVertexArray(self.vao);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.tex);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

    }

    pub fn load_texture(&mut self, filepath: &path::Path) -> image::Image<u8>{
        let mut texture_image= match stb_image::image::load(filepath) {
            image::LoadResult::Error(e) => unsafe{
                println!("{}", CStr::from_ptr(stb_image::stb_image::bindgen::stbi_failure_reason()).to_string_lossy());
                panic!("{}", e)
            },
            image::LoadResult::ImageF32(i) => panic!("UNEXPECTED IMAGE FORMAT"),
            image::LoadResult::ImageU8(i) => i,
        };

        unsafe{
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.tex);
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::PixelStorei(gl::UNPACK_ROW_LENGTH, 0);
            gl::PixelStorei(gl::UNPACK_SKIP_PIXELS, 0);
            gl::PixelStorei(gl::UNPACK_SKIP_ROWS, 0);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as gt::GLint,
                texture_image.width as gt::GLint, 
                texture_image.height as gt::GLint, 
                0, gl::RGB, gl::UNSIGNED_BYTE, texture_image.data.as_mut_ptr() as *mut std::ffi::c_void
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        println!("[Load_texture]: Texture loaded from {}", filepath.to_string_lossy());
        return texture_image;
    }

    pub fn new(vertices: Vec<f32>, indices: Vec<i32>) -> GraphicsObject{
        // Construct a new Graphics Object
        let mut o: GraphicsObject = GraphicsObject{
            ebo: 0,
            vbo: 0,
            vao: 0,
            tex: 0,
            vertices: vertices,
            indices: indices,
        };

        o.gen_buffers();
        o.configure_vertex_formats();
        o.configure_element_format();

        return o
    }

}