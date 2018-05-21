#![no_std]
#![feature(const_fn)]
#![feature(proc_macro)]
#![feature(used)]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate cortex_m_rtfm_macros;
extern crate panic_abort;
extern crate stm32f103xx_hal as hal;

use cortex_m_rtfm_macros::app;
use hal::prelude::*;
use hal::pwm::{C4, Pwm};
use hal::stm32f103xx::TIM4;
use rtfm::Threshold;

type PwmResource = Pwm<TIM4, C4>;

// Tasks and resources
app! {
    device: hal::stm32f103xx,

    resources: {
        static PWMOUT: PwmResource;
    },

    tasks: {
        SYS_TICK: {
            path: tick,
            resources: [PWMOUT],
        },
    }
}

fn init(p: init::Peripherals) -> init::LateResources {
    let mut flash = p.device.FLASH.constrain();
    let mut rcc = p.device.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = p.device.AFIO.constrain(&mut rcc.apb2);
    let mut gpiob = p.device.GPIOB.split(&mut rcc.apb2);

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

    // full
    pwm.set_duty(max);

    init::LateResources { PWMOUT: pwm }
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

fn tick(_t: &mut Threshold, mut _r: SYS_TICK::Resources) {
    // ...
}
