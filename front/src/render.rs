use wasm_bindgen::JsCast;
use web_sys::{window, WebGlRenderingContext};

#[path = "render/color.rs"]
mod color_mod;
pub use color_mod::Color;

static mut BUFFER_ID: Option<web_sys::WebGlBuffer> = None;

pub fn init(glctx: &WebGlRenderingContext) {
    if unsafe { BUFFER_ID.is_some() } {
        panic!("Buffer already initialized");
    }
    unsafe {
        BUFFER_ID = Some(glctx.create_buffer().unwrap());
    }
}

pub fn setup_shader(glctx: &WebGlRenderingContext, name: &str) -> web_sys::WebGlProgram {
    let vert_code = match name {
        "circle" => include_str!("../resources/shaders/circle.vert"),
        "rect" => include_str!("../resources/shaders/rect.vert"),
        _ => panic!("Not loaded"),
    };
    let frag_code = match name {
        "circle" => include_str!("../resources/shaders/circle.frag"),
        "rect" => include_str!("../resources/shaders/rect.frag"),
        _ => panic!("Not loaded"),
    };

    let vert_shader = glctx
        .create_shader(WebGlRenderingContext::VERTEX_SHADER)
        .unwrap();
    glctx.shader_source(&vert_shader, vert_code);
    glctx.compile_shader(&vert_shader);

    let frag_shader = glctx
        .create_shader(WebGlRenderingContext::FRAGMENT_SHADER)
        .unwrap();
    glctx.shader_source(&frag_shader, frag_code);
    glctx.compile_shader(&frag_shader);

    let shader_program = glctx.create_program().unwrap();
    glctx.attach_shader(&shader_program, &vert_shader);
    glctx.attach_shader(&shader_program, &frag_shader);
    glctx.link_program(&shader_program);

    shader_program
}

pub fn draw_rect(
    glctx: &WebGlRenderingContext,
    rect_shader_prog: &web_sys::WebGlProgram,
    rect: maths::Rect,
    color: Color,
) {
    let vertices = rect_to_vert(
        rect,
        maths::Vec2::new(
            glctx.drawing_buffer_width() as f64,
            glctx.drawing_buffer_height() as f64,
        ),
    );

    let buffer = unsafe { BUFFER_ID.clone().unwrap() };

    glctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    glctx.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::from(vertices.as_slice()),
        WebGlRenderingContext::DYNAMIC_DRAW,
    );

    glctx.use_program(Some(rect_shader_prog));

    glctx.blend_func(
        WebGlRenderingContext::SRC_ALPHA,
        WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
    );
    glctx.enable(WebGlRenderingContext::BLEND);
    glctx.disable(WebGlRenderingContext::DEPTH_TEST);

    glctx.viewport(
        0,
        0,
        glctx.drawing_buffer_width(),
        glctx.drawing_buffer_height(),
    );

    // Attach the position vector as an attribute for the GL context.
    let position = glctx.get_attrib_location(rect_shader_prog, "a_position") as u32;
    glctx.vertex_attrib_pointer_with_i32(position, 2, WebGlRenderingContext::FLOAT, true, 16, 0);
    glctx.enable_vertex_attrib_array(position);

    let uv = glctx.get_attrib_location(rect_shader_prog, "a_uv") as u32;
    glctx.vertex_attrib_pointer_with_i32(uv, 2, WebGlRenderingContext::FLOAT, true, 16, 8);
    glctx.enable_vertex_attrib_array(uv);

    // Attach the time as a uniform for the GL context.
    glctx.uniform4fv_with_f32_array(
        glctx
            .get_uniform_location(rect_shader_prog, "u_color")
            .as_ref(),
        &[
            color.r() as f32 / 255.,
            color.g() as f32 / 255.,
            color.b() as f32 / 255.,
            color.a() as f32 / 255.,
        ],
    );

    glctx.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
}

pub fn draw_circle(
    glctx: &WebGlRenderingContext,
    circle_shader_prog: &web_sys::WebGlProgram,
    circle: maths::Circle,
    color: Color,
    gradient: bool,
    ring: bool,
) {
    let vertices = circle_to_vert(
        circle,
        maths::Vec2::new(
            glctx.drawing_buffer_width() as f64,
            glctx.drawing_buffer_height() as f64,
        ),
    );

    let buffer = unsafe { BUFFER_ID.clone().unwrap() };

    glctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    glctx.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::from(vertices.as_slice()),
        WebGlRenderingContext::STREAM_DRAW, // DYNAMIC_DRAW
    );

    glctx.use_program(Some(circle_shader_prog));

    // gl.blendFunc(gl.SRC_ALPHA, gl.ONE);
    // gl.enable(gl.BLEND);
    // gl.disable(gl.DEPTH_TEST);

    glctx.blend_func(
        WebGlRenderingContext::SRC_ALPHA,
        WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
    );
    glctx.enable(WebGlRenderingContext::BLEND);
    glctx.disable(WebGlRenderingContext::DEPTH_TEST);

    glctx.viewport(
        0,
        0,
        glctx.drawing_buffer_width(),
        glctx.drawing_buffer_height(),
    );

    // Attach the position vector as an attribute for the GL context.
    let position = glctx.get_attrib_location(circle_shader_prog, "a_position") as u32;
    glctx.vertex_attrib_pointer_with_i32(position, 2, WebGlRenderingContext::FLOAT, true, 16, 0);
    glctx.enable_vertex_attrib_array(position);

    let uv = glctx.get_attrib_location(circle_shader_prog, "a_uv") as u32;
    glctx.vertex_attrib_pointer_with_i32(uv, 2, WebGlRenderingContext::FLOAT, true, 16, 8);
    glctx.enable_vertex_attrib_array(uv);

    // Attach the time as a uniform for the GL context.
    glctx.uniform4fv_with_f32_array(
        glctx
            .get_uniform_location(circle_shader_prog, "u_color")
            .as_ref(),
        &[
            color.r() as f32 / 255.,
            color.g() as f32 / 255.,
            color.b() as f32 / 255.,
            color.a() as f32 / 255.,
        ],
    );

    glctx.uniform1i(
        glctx
            .get_uniform_location(circle_shader_prog, "u_gradient")
            .as_ref(),
        gradient as i32,
    );

    glctx.uniform1i(
        glctx
            .get_uniform_location(circle_shader_prog, "u_ring")
            .as_ref(),
        ring as i32,
    );

    glctx.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
}

pub fn end_frame(f: &wasm_bindgen::closure::Closure<dyn FnMut()>) {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn rect_to_vert(rect: maths::Rect, canvas_size: maths::Vec2) -> [f32; 24] {
    let sized_rect = maths::Rect::new_from_center(
        maths::Point::new(
            rect.center().x / canvas_size.x,
            rect.center().y / canvas_size.y,
        ),
        maths::Point::new(rect.width() / canvas_size.x, rect.height() / canvas_size.y),
        0.,
    );

    let x0 = sized_rect.aa_topleft().x;
    let y0 = sized_rect.aa_topleft().y;
    let x1 = sized_rect.aa_botright().x;
    let y1 = sized_rect.aa_botright().y;

    // log!(format!("{:?}", sized_rect));

    let out = [
        // First triangle
        (x0, y0, 0.0, 0.0),
        (x1, y0, 1.0, 0.0),
        (x0, y1, 0.0, 1.0),
        // Second triangle
        (x0, y1, 0.0, 1.0),
        (x1, y0, 1.0, 0.0),
        (x1, y1, 1.0, 1.0),
    ]
    .iter()
    .flat_map(|&(x, y, u, v)| vec![x, y, u, v])
    .map(|x| x as f32)
    .collect::<Vec<_>>()
    .try_into()
    .unwrap_or_else(|_| panic!("Failed to convert rectangle to triangles"));

    // log!(format!("{out:?}"));

    out
}

pub fn circle_to_vert(circle: maths::Circle, canvas_size: maths::Vec2) -> [f32; 24] {
    rect_to_vert(
        maths::Rect::new_from_center(circle.center(), circle.radius, 0.),
        canvas_size,
    )
}
