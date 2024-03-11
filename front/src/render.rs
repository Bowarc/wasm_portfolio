use gloo::console::log;
use web_sys::{window, HtmlCanvasElement, WebGlRenderingContext};
#[path = "render/color.rs"]
mod color_mod;
pub use color_mod::Color;

pub fn setup(glctx: &WebGlRenderingContext) -> web_sys::WebGlProgram {
    let vert_code = include_str!("./basic.vert");
    let frag_code = include_str!("./basic.frag");

    // This list of vertices will draw two triangles to cover the entire canvas.

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

pub fn draw(
    glctx: &WebGlRenderingContext,
    rect_shader_prog: &web_sys::WebGlProgram,
    vertices: &[f32],
    color: Color,
) {
    // log!("{color}");
    let timestamp = 1.0;

    let vertex_buffer = glctx.create_buffer().unwrap();
    let verts = js_sys::Float32Array::from(vertices);

    glctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    glctx.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &verts,
        WebGlRenderingContext::DYNAMIC_DRAW,
    );

    glctx.use_program(Some(&rect_shader_prog));

    // Attach the position vector as an attribute for the GL context.
    let position = glctx.get_attrib_location(&rect_shader_prog, "a_position") as u32;
    glctx.vertex_attrib_pointer_with_i32(position, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
    glctx.enable_vertex_attrib_array(position);

    // Attach the time as a uniform for the GL context.
    glctx.uniform1f(
        glctx
            .get_uniform_location(&rect_shader_prog, "u_r")
            .as_ref(),
        color.r() as f32 / 255.,
    );
    glctx.uniform1f(
        glctx
            .get_uniform_location(&rect_shader_prog, "u_g")
            .as_ref(),
        color.g() as f32 / 255.,
    );
    glctx.uniform1f(
        glctx
            .get_uniform_location(&rect_shader_prog, "u_b")
            .as_ref(),
        color.b() as f32 / 255.,
    );
    glctx.uniform1f(
        glctx
            .get_uniform_location(&rect_shader_prog, "u_a")
            .as_ref(),
        color.a() as f32 / 255.,
    );

    glctx.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
}

pub fn end_frame(f: &wasm_bindgen::closure::Closure<dyn FnMut()>) {
    use wasm_bindgen::JsCast;
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn rect_to_vert(rect: crate::maths::Rect, canvas_size: crate::maths::Vec2) -> [f32; 12] {
    let sized_rect = crate::maths::Rect::new(
        crate::maths::Point::new(
            rect.aa_topleft().x / canvas_size.x,
            rect.aa_topleft().y / canvas_size.y,
        ),
        crate::maths::Point::new(rect.width() / canvas_size.x, rect.height() / canvas_size.y),
        0.,
    );
    let x0 = sized_rect.aa_topleft().x - sized_rect.size().x;
    let y0 = sized_rect.aa_topleft().y - sized_rect.size().y;
    let x1 = sized_rect.aa_topleft().x + sized_rect.size().x;
    let y1 = sized_rect.aa_topleft().y + sized_rect.size().y;

    [
        // First triangle
        (x0, y0),
        (x1, y0),
        (x0, y1),
        // Second triangle
        (x0, y1),
        (x1, y0),
        (x1, y1),
    ]
    .iter()
    .flat_map(|&(x, y)| vec![x as f32, y as f32])
    .collect::<Vec<_>>()
    .try_into()
    .unwrap_or_else(|_| panic!("Failed to convert rectangle to triangles"))
}
