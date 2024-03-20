use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, WebGlRenderingContext};
#[path = "render/color.rs"]
mod color_mod;
pub use color_mod::Color;

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
    vertices: &[f32],
    color: Color,
) {
    // log!("{color}");
    let timestamp = 1.0;

    glctx.bind_buffer(
        WebGlRenderingContext::ARRAY_BUFFER,
        Some(&glctx.create_buffer().unwrap()),
    );
    glctx.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::from(vertices),
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

pub fn draw_circle(
    glctx: &WebGlRenderingContext,
    circle_shader_prog: &web_sys::WebGlProgram,
    circle: maths::Circle,
    color: Color,
) {
    let canvasWidth = glctx.drawing_buffer_width() as f32;
    let canvasHeight = glctx.drawing_buffer_height() as f32;

    // Not used atm
    let to_clip_space = |x: f32, y: f32| -> (f32, f32) {
        // as i did in rect_to_vert?
        (x / canvasWidth, y / canvasHeight)
    };

    let vertices = circle_to_vert(
        circle,
        maths::Vec2::new(canvasWidth as f64, canvasHeight as f64),
    );

    glctx.bind_buffer(
        WebGlRenderingContext::ARRAY_BUFFER,
        Some(&glctx.create_buffer().unwrap()),
    );
    glctx.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &js_sys::Float32Array::from(vertices.as_slice()),
        WebGlRenderingContext::DYNAMIC_DRAW,
    );

    glctx.use_program(Some(&circle_shader_prog));

    // gl.blendFunc(gl.SRC_ALPHA, gl.ONE);
    // gl.enable(gl.BLEND);
    // gl.disable(gl.DEPTH_TEST);

    glctx.blend_func(WebGlRenderingContext::SRC_ALPHA, WebGlRenderingContext::ONE);
    glctx.enable(WebGlRenderingContext::BLEND);
    glctx.disable(WebGlRenderingContext::DEPTH_TEST);

    glctx.viewport(
        0,
        0,
        glctx.drawing_buffer_width(),
        glctx.drawing_buffer_height(),
    );

    // Attach the position vector as an attribute for the GL context.
    let position = glctx.get_attrib_location(&circle_shader_prog, "a_position") as u32;
    glctx.vertex_attrib_pointer_with_i32(position, 2, WebGlRenderingContext::FLOAT, true, 0, 0);
    glctx.enable_vertex_attrib_array(position);

    // Attach the time as a uniform for the GL context.
    glctx.uniform2fv_with_f32_array(
        glctx
            .get_uniform_location(&circle_shader_prog, "u_center")
            .as_ref(),
        &[
            // Shift the center pos to try to match glsl coords,
            circle.center.x as f32 + glctx.drawing_buffer_width() as f32,
            glctx.drawing_buffer_height() as f32 - circle.center.y as f32,
        ],
    );

    glctx.uniform1f(
        glctx
            .get_uniform_location(&circle_shader_prog, "u_radius")
            .as_ref(),
            circle.radius as f32,
    );

    // log!(format!(
    //     "Canvas size: {:?}",
    //     maths::Point::new(
    //         glctx.drawing_buffer_width() as f64,
    //         glctx.drawing_buffer_height() as f64,
    //     )
    // ));

    // log!(format!(
    //     "{:.0},{:.0} -> {:?}",
    //     pos.x,
    //     pos.y,
    //     to_clip_space(pos.x as f32, pos.y as f32)
    // ));

    // Other tests
    glctx.uniform2fv_with_f32_array(
        glctx
            .get_uniform_location(&circle_shader_prog, "u_resolution")
            .as_ref(),
        &[
            glctx.drawing_buffer_width() as f32,
            glctx.drawing_buffer_height() as f32,
        ],
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

pub fn rect_to_vert(rect: maths::Rect, canvas_size: maths::Vec2) -> [f32; 12] {
    let sized_rect = maths::Rect::new(
        maths::Point::new(
            rect.aa_topleft().x / canvas_size.x,
            rect.aa_topleft().y / canvas_size.y,
        ),
        maths::Point::new(rect.width() / canvas_size.x, rect.height() / canvas_size.y),
        0.,
    );
    let x0 = sized_rect.aa_topleft().x - sized_rect.size().x;
    let y0 = sized_rect.aa_topleft().y - sized_rect.size().y;
    let x1 = sized_rect.aa_topleft().x + sized_rect.size().x;
    let y1 = sized_rect.aa_topleft().y + sized_rect.size().y;

    let out = [
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
    .unwrap_or_else(|_| panic!("Failed to convert rectangle to triangles"));

    // log!(format!("{out:?}"));

    out
}

pub fn circle_to_vert(circle: maths::Circle, canvas_size: maths::Vec2) -> [f32; 12] {
    rect_to_vert(
        maths::Rect::new_from_center(circle.center(), circle.radius, 0.),
        canvas_size,
    )
}
