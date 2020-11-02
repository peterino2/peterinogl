use peter_gl::ShaderPipe;
pub mod peter_gl;
use stb_image::image;
use gl::types as gt;
use std::ffi::{CString, CStr, c_void};

struct Game{
    win: sdl2::video::Window,
    _gl_context: sdl2::video::GLContext,
    sdl: sdl2::Sdl,
    rend: ShaderPipe,
    object: peter_gl::GraphicsObject
}

impl Game {
    pub fn start(&mut self) 
    {
        self.rend.activate();
        self.rend.configure_textures();
        let i = self.object.load_texture(std::path::Path::new("assets/textures/wall.jpg" ));
        self.rend.set_sampler("ourTexture", self.object.tex);

        'main: loop {
            let mut event_pump = self.sdl.event_pump().unwrap();
            for _event in event_pump.poll_iter() {
                match _event {
                    sdl2::event::Event::Quit {..} => break 'main,
                    // sdl2::event::Event::Window {timestamp, window_id, win_event} => match win_event {
                    sdl2::event::Event::Window {
                        timestamp, 
                        window_id,
                        win_event
                    } => match win_event {

                            sdl2::event::WindowEvent::Resized (x, y) => {
                                println!("[WindowEvent] Resized to {} {}", x, y);
                                unsafe{ 
                                    gl::Viewport(0,0, x as gl::types::GLint, y as gl::types::GLint)
                                }
                            },
                            _ => {},
                        }
                    _ => {},
                }
            }
            self.render();
        }
    }

    fn render(&mut self)
    {
        // render window contents here

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        self.rend.activate();
        self.object.draw();
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
    
    gl_attr.set_context_version(4, 0);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();

    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let vertices: Vec<f32> = vec![
            0.5,  0.5,  0.0,       1.0, 0.0, 0.0,      1.0, 1.0,
            0.5, -0.5,  0.0,       1.0, 1.0, 0.0,      1.0, 0.0,
        -0.5, -0.5,  0.0,       0.0, 0.0, 1.0,      0.0, 0.0, 
        -0.5,  0.5,  0.0,       1.0, 1.0, 0.0,      0.0, 1.0,
    ];

    let indices: Vec<i32> = vec![
            0, 1, 3,
            1, 2, 3,
    ];

    let mut rend = ShaderPipe::construct();
    rend.activate();
    Game{
        win: window,
        _gl_context: gl_context,
        sdl: sdl,
        rend: rend,
        object: peter_gl::GraphicsObject::new(vertices, indices)
    }
}

fn main() {
    let mut gamestate = init_game();

    /* --- enter the videogame  --- */
    /* | */ gamestate.start(); /* | */
    /* --- enter the videogame  --- */
}
