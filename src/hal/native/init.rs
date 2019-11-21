use glutin::{
    dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder, ContextBuilder,
};

#[cfg(not(target_arch = "wasm32"))]
pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    window_title: S,
) -> super::super::super::Rltk {
    use super::super::super::Rltk;
    use super::super::*;

    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title(window_title.to_string())
        .with_inner_size(LogicalSize::new(
            f64::from(width_pixels),
            f64::from(height_pixels),
        ));
    let windowed_context = ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
        .with_gl_profile(glutin::GlProfile::Core)
        .with_hardware_acceleration(Some(true))
        .with_vsync(true)
        .with_srgb(true)
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    //let gl = glow::glow::load_with(|ptr| windowed_context.get_proc_address(ptr) as *const _);
    let gl = glow::Context::from_loader_function(|ptr| {
        windowed_context.get_proc_address(ptr) as *const _
    });

    // Load our basic shaders
    let mut shaders: Vec<Shader> = Vec::new();

    shaders.push(Shader::new(
        &gl,
        shader_strings::UBERSHADER_VS,
        shader_strings::UBERSHADER_FS,
    ));

    // Build a simple quad rendering vao
    let quad_vao = setup_quad(&gl);

    Rltk {
        backend: RltkPlatform {
            gl,
            platform: PlatformGL {
                quad_vao,
                context_wrapper: Some(WrappedContext {
                    el,
                    wc: windowed_context,
                })
            },
        },
        width_pixels,
        height_pixels,
        fonts: Vec::new(),
        consoles: Vec::new(),
        shaders,
        fps: 0.0,
        frame_time_ms: 0.0,
        active_console: 0,
        key: None,
        mouse_pos: (0, 0),
        left_click: false,
        shift: false,
        control: false,
        alt: false,
        web_button: None,
        quitting: false,
        post_scanlines: false,
        post_screenburn: false,
    }
}
