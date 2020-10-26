use peter_gl::ShaderPipe;
pub mod peter_gl;

struct Game{
    win: sdl2::video::Window,
    _gl_context: sdl2::video::GLContext,
    sdl: sdl2::Sdl,
    rend: ShaderPipe,
    vbo: gl::types::GLuint,
    vao: gl::types::GLuint,
}


impl Game {

    pub fn start(&mut self) 
    {
        // Set up the shader pipeline

        unsafe {gl::ClearColor(0.3, 0.3, 0.5, 1.0);}


        unsafe {
            gl::GenBuffers(1, &mut self.vbo);
        }

        let triangle_vertices: Vec<f32> = vec![
            -0.5, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0, 0.5, 0.0
        ];

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, // target
                (triangle_vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                triangle_vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW, // usage
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
        }

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader

            gl::VertexAttribPointer(
                0, // index of the generic vertex attribute ("layout (location = 0)")
                3, // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null() // offset of the first component
            );
            // unbind the vertex array
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        'main: loop {
            self.render();
            let mut event_pump = self.sdl.event_pump().unwrap();
            for _event in event_pump.poll_iter() {
                match _event {
                    sdl2::event::Event::Quit {..} => break 'main,
                    sdl2::event::Event::Window {timestamp, window_id, win_event} => match win_event {

                            sdl2::event::WindowEvent::Resized (x, y) => {
                                println!("[WindowEvent] Resized to {} {}", x, y);
                                unsafe{ gl::Viewport(0,0, x as gl::types::GLint, y as gl::types::GLint)}
                            },

                            _ => {},

                        }
                    _ => {},
                }
            }
        }
    }

    fn render(&mut self)
    {
        // render window contents here
        self.rend.activate();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
        }
        self.win.gl_swap_window();
    }

}

fn init_game() -> Game
{
    let sdl = sdl2::init().unwrap();

    let video_subsystem = sdl.video().unwrap();
    video_subsystem.gl_load_library_default().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();

    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let rend = ShaderPipe::construct();
    Game{
        win: window,
        _gl_context: gl_context,
        sdl: sdl,
        rend: rend,
        vbo: 0,
        vao: 0,
    }
}

fn main() {
    let mut gamestate = init_game();

    /* --- enter the videogame  --- */
    /* | */ gamestate.start(); /* | */
    /* --- enter the videogame  --- */
}

