// Explicitly declare the external gl crate BEFORE our local mod gl
// This tells Rust: when we write gl:: we mean the external crate
extern crate gl as opengl;

mod math;
mod parser;
mod gl;       // our local module src/gl/
mod app;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    // ── Test OBJ parser (will be wired to real file in Step 6) ───────────────
    // For now just verify it handles a missing file gracefully
    match parser::parse("assets/42.obj") {
        Ok(mesh) => println!("Mesh loaded: {} vertices", mesh.vertices.len()),
        Err(e)   => println!("No OBJ yet (expected): {}", e),
    }
    // ── 1. Init SDL2 ─────────────────────────────────────────────────────────
    let sdl_context = match sdl2::init() {
        Ok(ctx) => ctx,
        Err(e) => { eprintln!("SDL2 init failed: {}", e); std::process::exit(1); }
    };

    let video_subsystem = match sdl_context.video() {
        Ok(v) => v,
        Err(e) => { eprintln!("SDL2 video init failed: {}", e); std::process::exit(1); }
    };

    // ── 2. Request OpenGL 4.1 Core Profile ───────────────────────────────────
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);
    gl_attr.set_double_buffer(true);
    gl_attr.set_depth_size(24);

    // ── 3. Create window ─────────────────────────────────────────────────────
    let window = match video_subsystem
        .window("SCOP", 800, 600)
        .opengl()
        .resizable()
        .position_centered()
        .build()
    {
        Ok(w) => w,
        Err(e) => { eprintln!("Window creation failed: {}", e); std::process::exit(1); }
    };

    // ── 4. Create OpenGL context ──────────────────────────────────────────────
    let _gl_context = match window.gl_create_context() {
        Ok(ctx) => ctx,
        Err(e) => { eprintln!("OpenGL context creation failed: {}", e); std::process::exit(1); }
    };

    // ── 5. Load OpenGL function pointers via alias 'opengl' ──────────────────
    opengl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    // Print OpenGL version — confirms everything works
    let version = unsafe {
        let data = opengl::GetString(opengl::VERSION);
        std::ffi::CStr::from_ptr(data as *const _)
            .to_str()
            .unwrap_or("unknown")
            .to_owned()
    };
    println!("OpenGL version: {}", version);
    println!("Window open — press ESC to quit.");

    // ── 6. Event loop ─────────────────────────────────────────────────────────
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

    println!("SCOP - exiting cleanly.");
}