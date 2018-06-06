use super::sin::{sin_normalised, SINE_TABLE_LEN};
use super::{PwmOutputs, ITERATIONS_PER_PATTERN};
use hal::prelude::*;

const MAX_COUNT: u32 = ITERATIONS_PER_PATTERN * SINE_TABLE_LEN;

pub fn red_wave(max_duty: u16, ms: u32, out: &mut PwmOutputs) {
    out.r1
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.0)) as u16);
    out.r2
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.25)) as u16);
    out.r3
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.5)) as u16);
    out.r4
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.75)) as u16);

    out.g1.set_duty(0);
    out.g2.set_duty(0);
    out.g3.set_duty(0);
    out.g4.set_duty(0);
}

pub fn green_wave(max_duty: u16, ms: u32, out: &mut PwmOutputs) {
    out.r1.set_duty(0);
    out.r2.set_duty(0);
    out.r3.set_duty(0);
    out.r4.set_duty(0);

    out.g1
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.0)) as u16);
    out.g2
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.25)) as u16);
    out.g3
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.5)) as u16);
    out.g4
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.75)) as u16);
}

pub fn red_to_green_wave(max_duty: u16, ms: u32, out: &mut PwmOutputs) {
    let green_mul: f32 = ms as f32 / MAX_COUNT as f32;
    let red_mul: f32 = 1.0 - green_mul;

    out.r1
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.0) * red_mul) as u16);
    out.r2
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.25) * red_mul) as u16);
    out.r3
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.5) * red_mul) as u16);
    out.r4
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.75) * red_mul) as u16);

    out.g1
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.0) * green_mul) as u16);
    out.g2
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.25) * green_mul) as u16);
    out.g3
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.5) * green_mul) as u16);
    out.g4
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.75) * green_mul) as u16);
}

pub fn green_to_red_wave(max_duty: u16, ms: u32, out: &mut PwmOutputs) {
    let red_mul: f32 = ms as f32 / MAX_COUNT as f32;
    let green_mul: f32 = 1.0 - red_mul;

    out.r1
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.0) * red_mul) as u16);
    out.r2
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.25) * red_mul) as u16);
    out.r3
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.5) * red_mul) as u16);
    out.r4
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.75) * red_mul) as u16);

    out.g1
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.0) * green_mul) as u16);
    out.g2
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.25) * green_mul) as u16);
    out.g3
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.5) * green_mul) as u16);
    out.g4
        .set_duty((max_duty as f32 * sin_normalised(ms, 0.75) * green_mul) as u16);
}
