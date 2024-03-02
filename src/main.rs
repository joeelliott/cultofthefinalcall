// ███████╗██╗   ██╗██████╗ ███████╗██████╗  █████╗  ██████╗███████╗
// ██╔════╝██║   ██║██╔══██╗██╔════╝██╔══██╗██╔══██╗██╔════╝██╔════╝
// ███████╗██║   ██║██████╔╝███████╗██████╔╝███████║██║     █████╗  
// ╚════██║██║   ██║██╔══██╗╚════██║██╔═══╝ ██╔══██║██║     ██╔══╝  
// ███████║╚██████╔╝██████╔╝███████║██║     ██║  ██║╚██████╗███████╗
// ╚══════╝ ╚═════╝ ╚═════╝ ╚══════╝╚═╝     ╚═╝  ╚═╝ ╚═════╝╚══════╝
// subspace sniper is a project of the cult of the final call
// its goal is ridiculous, and as such, ought to be ignored
// s_d
// 
mod gl_setup;
mod event_handler;
mod game_logic;

// Use contents from the modules
use gl_setup::*;
use event_handler::*;
use game_logic::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, GLArea};

fn main() {
    let app = Application::new(Some("com.subspacesniper.opengl"), Default::default());
    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("SubSpaceSniper");
        window.set_default_size(800, 600);

        let gl_area = GLArea::new();
        gl_area.set_auto_render(true);
        gl_area.connect_realize(setup_gl);
        gl_area.connect_render(draw_gl);
        window.add(&gl_area);

        window.show_all();
    });

    app.run();
}

fn setup_gl(area: &GLArea) {
    area.make_current();
    gl::load_with(|ptr| area.get_proc_address(ptr) as *const _);
    // Set up your OpenGL context here (e.g., gl::ClearColor, gl::Enable...)
}

fn draw_gl(_area: &GLArea, _gl_context: &gtk::gdk::GLContext) -> gtk::Inhibit {
    unsafe {
        // Clear the screen to black
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    gtk::Inhibit(false)
}


