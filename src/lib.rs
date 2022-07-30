#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
pub mod adc_device;
pub mod adt_device;
pub mod aes_device;
pub mod bgr_device;
pub mod bt_device;
pub mod crc_device;
pub mod ddl;
pub mod debug;
pub mod delay;
pub mod dmac_device;
pub mod flash_device;
pub mod gpio_device;
pub mod hdiv_device;
pub mod i2c_device;
pub mod lcd_device;
pub mod lpm;
pub mod lptimer_device;
pub mod lpuart_device;
pub mod lvd_device;
pub mod opa_device;
pub mod pca_device;
pub mod pcnt_device;
pub mod reset;
pub mod rtc_device;
pub mod spi_device;
pub mod sysctrl_device;
pub mod timer3_device;
pub mod trim_device;
pub mod uart_device;
pub mod vc_device;
pub mod wdt_device;

pub enum en_result_t {
    Ok,
    Error,
    ErrorAddressAlignment,
    ErrorAccessRights,
    ErrorInvalidParameter,
    ErrorOperationInProgress,
    ErrorInvalidMode,
    ErrorUninitialized,
    ErrorBufferFull,
    ErrorTimeout,
    ErrorNotReady,
    OperationInProgress,
}

impl Default for en_result_t {
    fn default() -> Self {
        Self::Ok
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub use hc32l13x_pac;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
