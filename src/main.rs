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
use hal::pwm::{C4, Pwm};
use hal::stm32f103xx::TIM4;
use hal::timer::{Event, Timer};
use rtfm::Threshold;
use sin::sin_normalised;

type PwmResource = Pwm<TIM4, C4>;

// Tasks and resources
app! {
    device: hal::stm32f103xx,

    resources: {
        static PWMOUT: PwmResource;
        static MS: u32;
        static MAX_DUTY: u16;
    },

    tasks: {
        SYS_TICK: {
            path: tick,
            resources: [PWMOUT, MS, MAX_DUTY],
        },
    }
}

fn init(p: init::Peripherals) -> init::LateResources {
    let mut flash = p.device.FLASH.constrain();
    let mut rcc = p.device.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = p.device.AFIO.constrain(&mut rcc.apb2);
    let mut gpiob = p.device.GPIOB.split(&mut rcc.apb2);

    Timer::syst(p.core.SYST, 1.khz(), clocks).listen(Event::Update);

    // TIM4
    let c1 = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    let c2 = gpiob.pb7.into_alternate_push_pull(&mut gpiob.crl);
    let c3 = gpiob.pb8.into_alternate_push_pull(&mut gpiob.crh);
    let c4 = gpiob.pb9.into_alternate_push_pull(&mut gpiob.crh);

    let mut pwm = p.device
        .TIM4
        .pwm(
            (c1, c2, c3, c4),
            &mut afio.mapr,
            1.khz(),
            clocks,
            &mut rcc.apb1,
        )
        .3;

    let max = pwm.get_max_duty();

    pwm.enable();

    init::LateResources {
        PWMOUT: pwm,
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

    let sin = sin_normalised(ms);

    r.PWMOUT.set_duty((*r.MAX_DUTY as f32 * sin) as u16);

    *r.MS += 1;
}
