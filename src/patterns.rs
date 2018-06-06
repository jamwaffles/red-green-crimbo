use super::sin::sin_normalised;
use super::PwmOutputs;
use hal::prelude::*;

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
