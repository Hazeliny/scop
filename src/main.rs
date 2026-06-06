extern crate gl as opengl;

mod math;
mod parser;
mod gl;
mod app;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    // Flush stdout immediately so we see output even if we crash
    use std::io::Write;
    let _ = std::io::stdout().flush();
    
    eprintln!("=== SCOP starting ===");
    let _ = std::io::stderr().flush();

    match parser::parse("assets/42.obj") {
        Ok(mesh) => eprintln!("Mesh loaded: {} vertices", mesh.vertices.len()),
        Err(e)   => eprintln!("No OBJ yet: {}", e),
    }

    eprintln!("Initializing SDL2...");
    let _ = std::io::stderr().flush();

    let sdl_context = match sdl2::init() {
        Ok(ctx) => { eprintln!("SDL2 OK"); ctx }
        Err(e) => { eprintln!("SDL2 init failed: {}", e); std::process::exit(1); }
    };

    eprintln!("Initializing video...");
    let _ = std::io::stderr().flush();

    let video_subsystem = match sdl_context.video() {
        Ok(v) => { eprintln!("Video OK"); v }
        Err(e) => { eprintln!("Video init failed: {}", e); std::process::exit(1); }
    };

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);
    gl_attr.set_double_buffer(true);
    gl_attr.set_depth_size(24);

    eprintln!("Creating window...");
    let _ = std::io::stderr().flush();

    let window = match video_subsystem
        .window("SCOP", 800, 600)
        .opengl()
        .resizable()
        .position_centered()
        .build()
    {
        Ok(w) => { eprintln!("Window OK"); w }
        Err(e) => { eprintln!("Window failed: {}", e); std::process::exit(1); }
    };

    eprintln!("Creating GL context...");
    let _ = std::io::stderr().flush();

    let _gl_context = match window.gl_create_context() {
        Ok(ctx) => { eprintln!("GL context OK"); ctx }
        Err(e) => { eprintln!("GL context failed: {}", e); std::process::exit(1); }
    };

    opengl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    let version = unsafe {
        let data = opengl::GetString(opengl::VERSION);
        std::ffi::CStr::from_ptr(data as *const _)
            .to_str().unwrap_or("unknown").to_owned()
    };
    eprintln!("OpenGL version: {}", version);
    eprintln!("Window open — press ESC to quit.");

    let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        unsafe {
            opengl::ClearColor(0.15, 0.15, 0.15, 1.0);
            opengl::Clear(opengl::COLOR_BUFFER_BIT | opengl::DEPTH_BUFFER_BIT);
        }
        window.gl_swap_window();
    }
    eprintln!("SCOP - exiting cleanly.");
}