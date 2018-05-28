#![no_std]
#![feature(const_fn)]
#![feature(proc_macro)]
#![feature(used)]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate cortex_m_rtfm_macros;
extern crate panic_abort;
extern crate stm32f103xx_hal as hal;

mod cos;
mod sin;

use core::f32::consts::PI;
use cortex_m_rtfm_macros::app;
use cos::cos_normalised;
use hal::prelude::*;
use hal::pwm::{C1, C2, C3, C4, Pwm};
use hal::stm32f103xx::{TIM2, TIM3, TIM4};
use hal::timer::{Event, Timer};
use rtfm::Threshold;
use sin::sin_normalised;

// type Timer2Resource = (Pwm<TIM2, C1>, Pwm<TIM2, C2>, Pwm<TIM2, C3>, Pwm<TIM2, C4>);
// type Timer3Resource = (Pwm<TIM3, C1>, Pwm<TIM3, C2>, Pwm<TIM3, C3>, Pwm<TIM3, C4>);

type PwmOutputs = (
    Pwm<TIM2, C1>,
    Pwm<TIM2, C2>,
    Pwm<TIM2, C3>,
    Pwm<TIM2, C4>,
    Pwm<TIM3, C1>,
    Pwm<TIM3, C2>,
    Pwm<TIM3, C3>,
    Pwm<TIM3, C4>,
);

// Tasks and resources
app! {
    device: hal::stm32f103xx,

    resources: {
        static PWM: PwmOutputs;
        static MS: u32;
        static MAX_DUTY: u16;
    },

    tasks: {
        SYS_TICK: {
            path: tick,
            resources: [PWM, MS, MAX_DUTY],
        },
    }
}

fn init(p: init::Peripherals) -> init::LateResources {
    let mut flash = p.device.FLASH.constrain();
    let mut rcc = p.device.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = p.device.AFIO.constrain(&mut rcc.apb2);
    let mut gpiob = p.device.GPIOB.split(&mut rcc.apb2);
    let mut gpioa = p.device.GPIOA.split(&mut rcc.apb2);

    Timer::syst(p.core.SYST, 1.khz(), clocks).listen(Event::Update);

    // TIM2
    let t2c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let t2c2 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);
    let t2c3 = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let t2c4 = gpioa.pa3.into_alternate_push_pull(&mut gpioa.crl);

    // TIM3
    let t3c1 = gpioa.pa6.into_alternate_push_pull(&mut gpioa.crl);
    let t3c2 = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    let t3c3 = gpiob.pb0.into_alternate_push_pull(&mut gpiob.crl);
    let t3c4 = gpiob.pb1.into_alternate_push_pull(&mut gpiob.crl);

    // TIM4
    // let c1 = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    // let c2 = gpiob.pb7.into_alternate_push_pull(&mut gpiob.crl);
    // let c3 = gpiob.pb8.into_alternate_push_pull(&mut gpiob.crh);
    // let c4 = gpiob.pb9.into_alternate_push_pull(&mut gpiob.crh);

    let mut t2pwm = p.device.TIM2.pwm(
        (t2c1, t2c2, t2c3, t2c4),
        &mut afio.mapr,
        1.khz(),
        clocks,
        &mut rcc.apb1,
    );

    let mut t3pwm = p.device.TIM3.pwm(
        (t3c1, t3c2, t3c3, t3c4),
        &mut afio.mapr,
        1.khz(),
        clocks,
        &mut rcc.apb1,
    );

    let max = t2pwm.0.get_max_duty();

    t2pwm.0.enable();
    t2pwm.1.enable();
    t2pwm.2.enable();
    t2pwm.3.enable();

    t3pwm.0.enable();
    t3pwm.1.enable();
    t3pwm.2.enable();
    t3pwm.3.enable();

    init::LateResources {
        PWM: (
            t2pwm.0, t2pwm.1, t2pwm.2, t2pwm.3, t3pwm.0, t3pwm.1, t3pwm.2, t3pwm.3,
        ),
        MS: 0,
        MAX_DUTY: max,
    }
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

fn tick(_t: &mut Threshold, mut r: SYS_TICK::Resources) {
    let ms: u32 = *r.MS;

    r.PWM
        .0
        .set_duty((*r.MAX_DUTY as f32 * sin_normalised(ms, 0.0)) as u16);
    r.PWM
        .1
        .set_duty((*r.MAX_DUTY as f32 * sin_normalised(ms, 0.25)) as u16);
    r.PWM
        .2
        .set_duty((*r.MAX_DUTY as f32 * sin_normalised(ms, 0.5)) as u16);
    r.PWM
        .3
        .set_duty((*r.MAX_DUTY as f32 * sin_normalised(ms, 0.75)) as u16);

    r.PWM
        .4
        .set_duty((*r.MAX_DUTY as f32 * sin_normalised(ms, 0.0)) as u16);
    r.PWM
        .5
        .set_duty((*r.MAX_DUTY as f32 * sin_normalised(ms, 0.25)) as u16);
    r.PWM
        .6
        .set_duty((*r.MAX_DUTY as f32 * sin_normalised(ms, 0.5)) as u16);
    r.PWM
        .7
        .set_duty((*r.MAX_DUTY as f32 * sin_normalised(ms, 0.75)) as u16);

    *r.MS += 1;
}
