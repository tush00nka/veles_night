use raylib::{
    ffi::{
        BlendMode::{BLEND_ALPHA, BLEND_CUSTOM},
        RenderTexture, rlDrawRenderBatchActive, rlSetBlendFactors, rlSetBlendMode,
    },
    prelude::*,
};

// Custom Blend Modes
const RLGL_SRC_ALPHA: i32 = 0x0302;
const RLGL_MIN: i32 = 0x8007;
const RLGL_MAX: i32 = 0x8008;

const MAX_BOXES: usize = 20;
const MAX_SHADOWS: usize = MAX_BOXES * 3; // MAX_BOXES*3 - Each box can cast up to two shadow volumes for the edges it is away from, and one for the box itself
pub const MAX_LIGHTS: usize = 16;

#[derive(Copy, Clone, Default)]
pub struct ShadowGeometry {
    vertices: [Vector2; 4],
}

#[derive(Copy, Clone)]
pub struct LightInfo {
    active: bool, // Is this light slot active?
    dirty: bool,  // Does this light need to be updated?
    valid: bool,  // Is this light in a valid position?

    position: Vector2,   // Light position
    mask: Option<RenderTexture>, // Alpha mask for the light
    outer_radius: f32,   // The distance the light touches
    bounds: Rectangle,   // A cached rectangle of the light bounds to help with culling

    shadows: [ShadowGeometry; MAX_SHADOWS],
    shadow_count: i32,
}

impl Default for LightInfo {
    fn default() -> Self {
        Self {
            active: false,
            dirty: false,
            valid: false,
            position: Vector2::default(),
            mask: None,
            outer_radius: f32::default(),
            bounds: Rectangle::default(),
            shadows: [ShadowGeometry::default(); MAX_SHADOWS],
            shadow_count: 0,
        }
    }
}

pub fn move_light(lights: &mut [LightInfo; MAX_LIGHTS], slot: usize, x: f32, y: f32) {
    lights[slot].dirty = true;
    lights[slot].position.x = x;
    lights[slot].position.y = y;
}

pub fn setup_light(
    lights: &mut [LightInfo; MAX_LIGHTS],
    slot: usize,
    x: f32,
    y: f32,
    radius: f32,
    rl: &mut RaylibDrawHandle,
    thread: &RaylibThread,
) {
    lights[slot].active = true;
    lights[slot].valid = false; // The light must prove it is valid

    let w = rl.get_screen_width();
    let h = rl.get_screen_height();

    lights[slot].mask = Some(*rl
        .load_render_texture(thread, w as u32, h as u32)
        .expect("error"));
    lights[slot].outer_radius = radius;

    lights[slot].bounds.width = radius * 2.;
    lights[slot].bounds.height = radius * 2.;

    move_light(lights, slot, x, y);

    // Force the render texture to have something in it
    draw_light_mask(lights, slot, rl, thread);
}

pub fn draw_light_mask(
    lights: &mut [LightInfo; MAX_LIGHTS],
    slot: usize,
    rl: &mut RaylibDrawHandle,
    thread: &RaylibThread,
) {
    let mut tex = lights[slot].mask.unwrap();
    let mut tm = rl.begin_texture_mode(thread, &mut tex);
    tm.clear_background(Color::WHITE);

    unsafe {
        rlSetBlendFactors(RLGL_SRC_ALPHA, RLGL_SRC_ALPHA, RLGL_MIN);
        rlSetBlendMode(BLEND_CUSTOM as i32)
    }

    if lights[slot].valid {
        tm.draw_circle_gradient(
            lights[slot].position.x as i32,
            lights[slot].position.y as i32,
            lights[slot].outer_radius,
            Color::WHITE.alpha(0.0),
            Color::WHITE,
        );
    }

    unsafe {
        rlDrawRenderBatchActive();
        // Cut out the shadows from the light radius by forcing the alpha to maximum
        rlSetBlendMode(BLEND_ALPHA as i32);
        rlSetBlendFactors(RLGL_SRC_ALPHA, RLGL_SRC_ALPHA, RLGL_MAX);
        rlSetBlendMode(BLEND_CUSTOM as i32);
    }

    let range = lights[slot].shadow_count as usize;
    for i in 0..range {
        tm.draw_triangle_fan(&lights[slot].shadows[i].vertices, Color::WHITE);
    }

    unsafe { rlDrawRenderBatchActive() };

    // Go back to normal blend mode
    unsafe { rlSetBlendMode(BLEND_ALPHA as i32) };
}
