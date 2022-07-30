use crate::sysctrl_device::Sysctrl_GetHClkFreq;

pub fn delay1ms(mut u32delay: u32) {
    let mut u32send: u32 = 0;
    let mut peripheral = hc32l13x_pac::CorePeripherals::take().unwrap();

    peripheral.SYST.set_reload(u32send);
    peripheral.SYST.clear_current();
    peripheral.SYST.enable_counter();
    while u32delay > 0 {
        peripheral.SYST.clear_current();
        u32send = 0x1000000u32 - Sysctrl_GetHClkFreq() / 1000;
        while peripheral.SYST.cvr.read() > u32send {}
        u32delay -= 1;
    }
}

pub fn delay10us(mut u32delay: u32) {
    let mut u32send: u32 = 0;
    let mut peripheral = hc32l13x_pac::CorePeripherals::take().unwrap();

    peripheral.SYST.set_reload(u32send);
    peripheral.SYST.clear_current();
    peripheral.SYST.enable_counter();
    while u32delay > 0 {
        peripheral.SYST.clear_current();
        u32send = 0x1000000u32 - Sysctrl_GetHClkFreq() / 100000;
        while peripheral.SYST.cvr.read() > u32send {}
        u32delay -= 1;
    }
}

pub fn delay100us(mut u32delay: u32) {
    let mut u32send: u32 = 0;
    let mut peripheral = hc32l13x_pac::CorePeripherals::take().unwrap();

    peripheral.SYST.set_reload(u32send);
    peripheral.SYST.clear_current();
    peripheral.SYST.enable_counter();
    while u32delay > 0 {
        peripheral.SYST.clear_current();
        u32send = 0x1000000u32 - Sysctrl_GetHClkFreq() / 10000;
        while peripheral.SYST.cvr.read() > u32send {}
        u32delay -= 1;
    }
}
