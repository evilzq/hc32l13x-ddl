#![warn(clippy::nonminimal_bool)]
use crate::delay::delay10us;
use crate::en_result_t;
use core::convert::TryFrom;
use core::ptr;
use num_enum::{IntoPrimitive, TryFromPrimitive};

const RC_TRIM_BASE_ADDR: u32 = 0x00100C00;
const RCH_CR_TRIM_24M_VAL: u32 = 0x00100C00;
const RCH_CR_TRIM_22_12M_VAL: u32 = 0x00100C02;
const RCH_CR_TRIM_16M_VAL: u32 = 0x00100C04;
const RCH_CR_TRIM_8M_VAL: u32 = 0x00100C06;
const RCL_CR_TRIM_38400_VAL: u32 = 0x00100C20;

fn _SysctrlUnlock() {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral.SYSCTRL.sysctrl2.write(|w| w.bits(0x5A5A));
        peripheral.SYSCTRL.sysctrl2.write(|w| w.bits(0xA5A5));
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_sysctrl_clk_source {
    #[num_enum(default)]
    SysctrlClkRCH = 0,
    //* 内部高速时钟
    SysctrlClkXTH = 1,
    //* 外部高速时钟
    SysctrlClkRCL = 2,
    //* 内部低速时钟
    SysctrlClkXTL = 3,
    //* 外部低速时钟
    SysctrlClkPLL = 4, //* PLL时钟
}

/**
 *******************************************************************************
 ** \brief RCH频率值枚举类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum en_sysctrl_rch_freq {
    SysctrlRchFreq4MHz = 4,     //* 4MHz
    SysctrlRchFreq8MHz = 3,     //* 8MHz
    SysctrlRchFreq16MHz = 2,    //* 16MHz
    SysctrlRchFreq22_12MHz = 1, //* 22.12MHz
    SysctrlRchFreq24MHz = 0,    //* 24MHz
}

/**
 *******************************************************************************
 ** \brief XTAL驱动能力类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum en_sysctrl_xtal_driver {
    SysctrlXtalDriver0 = 0, //* 最弱驱动能力
    SysctrlXtalDriver1 = 1, //* 弱驱动能力
    SysctrlXtalDriver2 = 2, //* 一般驱动能力
    SysctrlXtalDriver3 = 3, //* 最强驱动能力
}

/**
 *******************************************************************************
 ** \brief XTH频率值范围选择类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum en_sysctrl_xth_freq {
    SysctrlXthFreq4_6MHz = 0,   //* 4~6MHz
    SysctrlXthFreq6_12MHz = 1,  //* 6~12MHz
    SysctrlXthFreq12_20MHz = 2, //* 12~20MHz
    SysctrlXthFreq20_32MHz = 3, //* 20~32MHz
}

/**
 *******************************************************************************
 ** \brief XTH时钟稳定周期数类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum en_sysctrl_xth_cycle {
    SysctrlXthStableCycle256 = 0,   //* 256 个周期数
    SysctrlXthStableCycle1024 = 1,  //* 1024 个周期数
    SysctrlXthStableCycle4096 = 2,  //* 4096 个周期数
    SysctrlXthStableCycle16384 = 3, //* 16384 个周期数
}

/**
 *******************************************************************************
 ** \brief RCL频率值枚举类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum en_sysctrl_rcl_freq {
    SysctrlRclFreq32768 = 0x11, //* 32.768KHz
    SysctrlRclFreq38400 = 0x10, //* 38.4KHz
}

/**
 *******************************************************************************
 ** \brief RCL时钟稳定周期数类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum en_sysctrl_rcl_cycle {
    SysctrlRclStableCycle4 = 0,   //* 4 个周期数
    SysctrlRclStableCycle16 = 1,  //* 16 个周期数
    SysctrlRclStableCycle64 = 2,  //* 64 个周期数
    SysctrlRclStableCycle256 = 3, //* 256 个周期数
}

/**
 *******************************************************************************
 ** \brief XTL时钟稳定周期数类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum en_sysctrl_xtl_cycle {
    SysctrlXtlStableCycle256 = 0,   //* 256 个周期数
    SysctrlXtlStableCycle1024 = 1,  //* 1024 个周期数
    SysctrlXtlStableCycle4096 = 2,  //* 4096 个周期数
    SysctrlXtlStableCycle16384 = 3, //* 16384 个周期数
}

/**
 *******************************************************************************
 ** \brief XTL晶体振幅枚举类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum en_sysctrl_xtl_amp {
    SysctrlXtlAmp0 = 0, //* 最小振幅
    SysctrlXtlAmp1 = 1, //* 小振幅
    SysctrlXtlAmp2 = 2, //* 一般振幅
    SysctrlXtlAmp3 = 3, //* 最大振幅
}

/**
 *******************************************************************************
 ** \brief PLL时钟稳定周期数类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum en_sysctrl_pll_cycle {
    SysctrlPllStableCycle128 = 0,   //* 128个周期数
    SysctrlPllStableCycle256 = 1,   //* 256个周期数
    SysctrlPllStableCycle512 = 2,   //* 512个周期数
    SysctrlPllStableCycle1024 = 3,  //* 1024个周期数
    SysctrlPllStableCycle2048 = 4,  //* 2048个周期数
    SysctrlPllStableCycle4096 = 5,  //* 4096个周期数
    SysctrlPllStableCycle8192 = 6,  //* 8192个周期数
    SysctrlPllStableCycle16384 = 7, //* 16384个周期数
}

/**
 *******************************************************************************
 ** \brief PLL输入频率范围类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum en_sysctrl_pll_infreq {
    SysctrlPllInFreq4_6MHz = 0,   //* 4~16MHz
    SysctrlPllInFreq6_12MHz = 1,  //* 6~12MHz
    SysctrlPllInFreq12_20MHz = 2, //* 12~20MHz
    SysctrlPllInFreq20_24MHz = 3, //* 20~24MHz
}

/**
 *******************************************************************************
 ** \brief PLL输出频率范围类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum en_sysctrl_pll_outfreq {
    SysctrlPllOutFreq8_12MHz = 0,  //* 8~12MHz
    SysctrlPllOutFreq12_18MHz = 1, //* 12~18MHz
    SysctrlPllOutFreq18_24MHz = 2, //* 18~24MHz
    SysctrlPllOutFreq24_36MHz = 3, //* 24~36MHz
    SysctrlPllOutFreq36_48MHz = 4, //* 36~48MHz
}

/**
 *******************************************************************************
 ** \brief PLL输入时钟源类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum en_sysctrl_pll_clksource {
    SysctrlPllXthXtal = 0, //* XTH晶振输入的时钟
    SysctrlPllXthIn = 2,   //* XTH从端口输入的时钟
    SysctrlPllRch = 3,     //* RCH时钟
}

/**
 *******************************************************************************
 ** \brief PLL输入时钟源类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum en_sysctrl_pll_mul {
    SysctrlPllMul2 = 2,   //* 2倍频
    SysctrlPllMul3 = 3,   //* 3倍频
    SysctrlPllMul4 = 4,   //* 4倍频
    SysctrlPllMul5 = 5,   //* 5倍频
    SysctrlPllMul6 = 6,   //* 6倍频
    SysctrlPllMul7 = 7,   //* 7倍频
    SysctrlPllMul8 = 8,   //* 8倍频
    SysctrlPllMul9 = 9,   //* 9倍频
    SysctrlPllMul10 = 10, //* 10倍频
    SysctrlPllMul11 = 11, //* 11倍频
    SysctrlPllMul12 = 12, //* 12倍频
}

/**
 *******************************************************************************
 ** \brief HCLK时钟分频系数类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum en_sysctrl_hclk_div {
    SysctrlHclkDiv1 = 0,   //* SystemClk
    SysctrlHclkDiv2 = 1,   //* SystemClk/2
    SysctrlHclkDiv4 = 2,   //* SystemClk/4
    SysctrlHclkDiv8 = 3,   //* SystemClk/8
    SysctrlHclkDiv16 = 4,  //* SystemClk/16
    SysctrlHclkDiv32 = 5,  //* SystemClk/32
    SysctrlHclkDiv64 = 6,  //* SystemClk/64
    SysctrlHclkDiv128 = 7, //* SystemClk/128
}

/**
 *******************************************************************************
 ** \brief PCLK分频系数
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum en_sysctrl_pclk_div {
    SysctrlPclkDiv1 = 0, //* HCLK
    SysctrlPclkDiv2 = 1, //* HCLK/2
    SysctrlPclkDiv4 = 2, //* HCLK/4
    SysctrlPclkDiv8 = 3, //* HCLK/8
}

/**
 *******************************************************************************
 ** \brief RTC高速时钟补偿时钟频率数据类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum en_sysctrl_rtc_adjust {
    SysctrlRTC4MHz = 0,  //* 4MHz
    SysctrlRTC6MHz = 1,  //* 6MHz
    SysctrlRTC8MHz = 2,  //* 8MHz
    SysctrlRTC12MHz = 3, //* 12MHz
    SysctrlRTC16MHz = 4, //* 16MHz
    SysctrlRTC20MHz = 5, //* 20MHz
    SysctrlRTC24MHz = 6, //* 24MHz
    SysctrlRTC32MHz = 7, //* 32MHz
}

/**
 *******************************************************************************
 ** \brief 系统控制模块其他功能数据类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum en_sysctrl_func {
    SysctrlEXTHEn = 1,          //* 使能外部高速时钟从输入引脚输入
    SysctrlEXTLEn = 2,          //* 使能外部低速速时钟从输入引脚输入
    SysctrlXTLAlwaysOnEn = 3,   //* 使能后XTL_EN只可置位
    SysctrlClkFuncRTCLpmEn = 5, //* 使能RTC低功耗模式
    SysctrlCMLockUpEn = 6,      //* 使能后CPU执行无效指令会复位MCU
    SysctrlSWDUseIOEn = 8,      //* SWD端口设为IO功能
}

/**
 *******************************************************************************
 ** \brief 外设时钟门控开关类型枚举
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum en_sysctrl_peripheral_gate {
    SysctrlPeripheralUart0 = 0,   //* 串口0
    SysctrlPeripheralUart1 = 1,   //* 串口1
    SysctrlPeripheralLpUart0 = 2, //* 低功耗串口0
    SysctrlPeripheralLpUart1 = 3, //* 低功耗串口1
    SysctrlPeripheralI2c0 = 4,    //* I2C0
    SysctrlPeripheralI2c1 = 5,    //* I2C1
    SysctrlPeripheralSpi0 = 6,    //* SPI0
    SysctrlPeripheralSpi1 = 7,    //* SPI1
    SysctrlPeripheralBaseTim = 8, //* 基础定时器TIM0/1/2
    SysctrlPeripheralLpTim = 9,   //* 低功耗定时器
    SysctrlPeripheralAdvTim = 10, //* 高级定时器TIM4/5/6
    SysctrlPeripheralTim3 = 11,   //* 定时器3
    SysctrlPeripheralOpa = 13,    //* OPA
    SysctrlPeripheralPca = 14,    //* 可编程计数阵列
    SysctrlPeripheralWdt = 15,    //* 看门狗
    SysctrlPeripheralAdcBgr = 16, //* ADC&BGR
    SysctrlPeripheralVcLvd = 17,  //* VC和LVD
    SysctrlPeripheralRng = 18,    //* RNG
    SysctrlPeripheralPcnt = 19,   //* PCNT
    SysctrlPeripheralRtc = 20,    //* RTC
    SysctrlPeripheralTrim = 21,   //* 时钟校准
    SysctrlPeripheralLcd = 22,    //* LCD
    SysctrlPeripheralTick = 24,   //* 系统定时器
    SysctrlPeripheralSwd = 25,    //* SWD
    SysctrlPeripheralCrc = 26,    //* CRC
    SysctrlPeripheralAes = 27,    //* AES
    SysctrlPeripheralGpio = 28,   //* GPIO
    SysctrlPeripheralDma = 29,    //* DMA
    SysctrlPeripheralDiv = 30,    //* 除法器
    SysctrlPeripheralFlash = 31,  //* Flash
}

/**
 *******************************************************************************
 ** \brief 时钟初始化配置结构体定义
 ******************************************************************************/
#[derive(Copy, Clone)]
pub struct stc_sysctrl_clk_cfg {
    enClkSrc: en_sysctrl_clk_source, //* 时钟源选择
    enHClkDiv: en_sysctrl_hclk_div,  //* HCLK分频系数
    enPClkDiv: en_sysctrl_pclk_div,  //* PCLK分频系数
}

/**
 *******************************************************************************
 ** \brief 时钟初始化配置结构体定义
 ******************************************************************************/
#[derive(Copy, Clone)]
pub struct stc_sysctrl_pll_cfg {
    enInFreq: en_sysctrl_pll_infreq,       //* PLL输入时钟频率范围选择
    enOutFreq: en_sysctrl_pll_outfreq,     //* PLL输出时钟频率范围选择
    enPllClkSrc: en_sysctrl_pll_clksource, //* PLL输入时钟源选择
    enPllMul: en_sysctrl_pll_mul,          //* PLL倍频系数选择
}

///< 系统时钟初始化API:用于上电后，系统工作之前对主频及外设时钟进行初始化；
///< 注意1：使用该初始化函数前需要根据系统，必须优先设置目标内部时钟源的TRIM值或外部时钟源的频率范围，
///< 注意2：XTH、XTL的频率范围设定，需要根据外部晶振决定，
///< 注意3：本驱动默认宏定义：SYSTEM_XTH=32MHz,SYSTEM_XTL=32768Hz,如使用其它外部晶振，必须修改这两个宏定义的值。
pub fn Sysctrl_ClkInit(pstcCfg: &stc_sysctrl_clk_cfg) -> en_result_t {
    match pstcCfg.enClkSrc {
        en_sysctrl_clk_source::SysctrlClkRCH => {}
        en_sysctrl_clk_source::SysctrlClkXTH => {
            Sysctrl_XTHDriverCfg(en_sysctrl_xtal_driver::SysctrlXtalDriver3);
            Sysctrl_SetXTHStableTime(en_sysctrl_xth_cycle::SysctrlXthStableCycle16384);
        }
        en_sysctrl_clk_source::SysctrlClkRCL => {
            Sysctrl_SetRCLStableTime(en_sysctrl_rcl_cycle::SysctrlRclStableCycle256);
        }
        en_sysctrl_clk_source::SysctrlClkXTL => {
            Sysctrl_XTLDriverCfg(
                en_sysctrl_xtl_amp::SysctrlXtlAmp3,
                en_sysctrl_xtal_driver::SysctrlXtalDriver3,
            );
            Sysctrl_SetXTLStableTime(en_sysctrl_xtl_cycle::SysctrlXtlStableCycle16384);
        }
        en_sysctrl_clk_source::SysctrlClkPLL => {
            Sysctrl_SetPLLStableTime(en_sysctrl_pll_cycle::SysctrlPllStableCycle16384);
        }
    }
    Sysctrl_ClkSourceEnable(pstcCfg.enClkSrc, true);
    Sysctrl_SysClkSwitch(pstcCfg.enClkSrc);
    Sysctrl_SetHCLKDiv(pstcCfg.enHClkDiv);
    Sysctrl_SetPCLKDiv(pstcCfg.enPClkDiv);
    en_result_t::Ok
}

///< 系统时钟去初始化API:恢复为上电默认状态->PCLK=HCLK=SystemClk=RCH4MHz
pub fn Sysctrl_ClkDeInit() -> en_result_t {
    Sysctrl_SetRCHTrim(en_sysctrl_rch_freq::SysctrlRchFreq4MHz);
    Sysctrl_ClkSourceEnable(en_sysctrl_clk_source::SysctrlClkRCH, true);
    Sysctrl_SysClkSwitch(en_sysctrl_clk_source::SysctrlClkRCH);

    //其它时钟源使能关闭
    Sysctrl_ClkSourceEnable(en_sysctrl_clk_source::SysctrlClkXTH, false);
    Sysctrl_ClkSourceEnable(en_sysctrl_clk_source::SysctrlClkRCL, false);
    Sysctrl_ClkSourceEnable(en_sysctrl_clk_source::SysctrlClkXTL, false);
    Sysctrl_ClkSourceEnable(en_sysctrl_clk_source::SysctrlClkPLL, false);

    //时钟分频设置
    Sysctrl_SetHCLKDiv(en_sysctrl_hclk_div::SysctrlHclkDiv1);
    Sysctrl_SetPCLKDiv(en_sysctrl_pclk_div::SysctrlPclkDiv1);
    en_result_t::Ok
}

///< 系统时钟模块的基本功能设置
///< 注意：使能需要使用的时钟源之前，必须优先设置目标内部时钟源的TRIM值或外部时钟源的频率范围
pub fn Sysctrl_ClkSourceEnable(enSource: en_sysctrl_clk_source, bFlag: bool) -> en_result_t {
    _SysctrlUnlock();
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    let u32Temp = peripheral.SYSCTRL.peri_clken.read().bits();
    unsafe {
        match enSource {
            en_sysctrl_clk_source::SysctrlClkRCH => {
                peripheral.SYSCTRL.sysctrl0.write(|w| w.rch_en().bit(bFlag));
                while bFlag && (1 != peripheral.SYSCTRL.rch_cr.read().stable().bit() as i32) {}
            }
            en_sysctrl_clk_source::SysctrlClkXTH => {
                peripheral.SYSCTRL.peri_clken.write(|w| w.gpio().bit(true));
                peripheral
                    .GPIO
                    .pdads
                    .modify(|r, w| w.bits(r.bits() | 0x3u32));
                peripheral.SYSCTRL.sysctrl0.write(|w| w.xth_en().bit(bFlag));
                while bFlag && (1 != peripheral.SYSCTRL.xth_cr.read().stable().bit() as i32) {}
            }
            en_sysctrl_clk_source::SysctrlClkRCL => {
                peripheral.SYSCTRL.sysctrl0.write(|w| w.rcl_en().bit(bFlag));
                while bFlag && (1 != peripheral.SYSCTRL.rcl_cr.read().stable().bit() as i32) {}
            }
            en_sysctrl_clk_source::SysctrlClkXTL => {
                peripheral.SYSCTRL.peri_clken.write(|w| w.gpio().bit(true));
                peripheral
                    .GPIO
                    .pcads
                    .modify(|r, w| w.bits(r.bits() | 0xc000u32));
                peripheral.SYSCTRL.sysctrl0.write(|w| w.xtl_en().bit(bFlag));
                while bFlag && (1 != peripheral.SYSCTRL.xtl_cr.read().stable().bit() as i32) {}
            }
            en_sysctrl_clk_source::SysctrlClkPLL => {
                peripheral.SYSCTRL.peri_clken.write(|w| w.adc().bit(true));
                peripheral.BGR.cr.write(|w| w.bgr_en().bit(true));
                delay10us(20);
                peripheral.SYSCTRL.sysctrl0.write(|w| w.pll_en().bit(bFlag));
                while bFlag && (1 != peripheral.SYSCTRL.pll_cr.read().stable().bit() as i32) {}
            }
        }
        peripheral.SYSCTRL.peri_clken.write(|w| w.bits(u32Temp));
    }
    en_result_t::Ok
}

///<外部晶振驱动配置：系统初始化Sysctrl_ClkInit()之后，可根据需要配置外部晶振的驱动能力，时钟初始化Sysctrl_ClkInit()默认为最大值;
pub fn Sysctrl_XTHDriverCfg(enDriver: en_sysctrl_xtal_driver) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral
            .SYSCTRL
            .xth_cr
            .write(|w| w.driver().bits(enDriver as u8));
    }
    en_result_t::Ok
}

pub fn Sysctrl_XTLDriverCfg(
    enAmp: en_sysctrl_xtl_amp,
    enDriver: en_sysctrl_xtal_driver,
) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral
            .SYSCTRL
            .xtl_cr
            .write(|w| w.amp_sel().bits(enAmp as u8));
        peripheral
            .SYSCTRL
            .xtl_cr
            .write(|w| w.driver().bits(enDriver as u8));
    }
    en_result_t::Ok
}

///<时钟稳定周期设置:系统初始化Sysctrl_ClkInit()之后，可根据需要配置时钟开启后的稳定之间，默认为最大值;
pub fn Sysctrl_SetXTHStableTime(enCycle: en_sysctrl_xth_cycle) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral
            .SYSCTRL
            .xth_cr
            .write(|w| w.startup().bits(enCycle as u8));
    }
    en_result_t::Ok
}
pub fn Sysctrl_SetRCLStableTime(enCycle: en_sysctrl_rcl_cycle) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral
            .SYSCTRL
            .rcl_cr
            .write(|w| w.startup().bits(enCycle as u8));
    }
    en_result_t::Ok
}
pub fn Sysctrl_SetXTLStableTime(enCycle: en_sysctrl_xtl_cycle) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral
            .SYSCTRL
            .xtl_cr
            .write(|w| w.startup().bits(enCycle as u8));
    }
    en_result_t::Ok
}
pub fn Sysctrl_SetPLLStableTime(enCycle: en_sysctrl_pll_cycle) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral
            .SYSCTRL
            .pll_cr
            .write(|w| w.startup().bits(enCycle as u8));
    }
    en_result_t::Ok
}

///<系统时钟源切换并更新系统时钟：如果需要在系统时钟初始化Sysctrl_ClkInit()之后切换主频时钟源，则使用该函数；
///< 时钟切换前后，必须根据目标频率值设置Flash读等待周期，可配置插入周期为0、1、2，
///< 注意!!!：当HCLK大于24MHz时，FLASH等待周期插入必须至少为1,否则程序运行可能产生未知错误
pub fn Sysctrl_SysClkSwitch(enSource: en_sysctrl_clk_source) -> en_result_t {
    _SysctrlUnlock();
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral
            .SYSCTRL
            .sysctrl0
            .write(|w| w.clksw().bits(enSource as u8));
    }
    en_result_t::Ok
}

///< 时钟源频率设定：根据系统情况，单独设置不同时钟源的频率值;
///< 时钟频率设置前，必须根据目标频率值设置Flash读等待周期，可配置插入周期为0、1、2，
///< 其中XTL的时钟由外部晶振决定，无需设置。
pub fn Sysctrl_SetRCHTrim(enRCHFreq: en_sysctrl_rch_freq) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral.SYSCTRL.rch_cr.write(|w| {
            w.trim().bits(ptr::read(
                (RC_TRIM_BASE_ADDR + enRCHFreq as u32) as *const u16,
            ))
        })
    }
    en_result_t::Ok
}
pub fn Sysctrl_SetRCLTrim(enRCLFreq: en_sysctrl_rcl_freq) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral.SYSCTRL.rcl_cr.write(|w| {
            w.trim().bits(ptr::read(
                (RC_TRIM_BASE_ADDR + enRCLFreq as u32) as *const u16,
            ))
        })
    }
    en_result_t::Ok
}
pub fn Sysctrl_SetXTHFreq(enXTHFreq: en_sysctrl_xth_freq) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral
            .SYSCTRL
            .xth_cr
            .write(|w| w.xth_fsel().bits(enXTHFreq as u8));
    }
    en_result_t::Ok
}

pub fn Sysctrl_SetPLLFreq(pstcPLLCfg: &stc_sysctrl_pll_cfg) -> en_result_t {
    let mut u16Trim: [u16; 5] = [0, 0, 0, 0, 0];
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        u16Trim[1] = ptr::read(RCH_CR_TRIM_8M_VAL as *const u16);
        u16Trim[2] = ptr::read(RCH_CR_TRIM_16M_VAL as *const u16);
        u16Trim[3] = ptr::read(RCH_CR_TRIM_22_12M_VAL as *const u16);
        u16Trim[4] = ptr::read(RCH_CR_TRIM_24M_VAL as *const u16);
        match pstcPLLCfg.enPllClkSrc {
            en_sysctrl_pll_clksource::SysctrlPllXthXtal => {
                if 32000000u32 * pstcPLLCfg.enPllMul as u32 > 48000000 {
                    return en_result_t::ErrorInvalidMode;
                }
            }
            en_sysctrl_pll_clksource::SysctrlPllXthIn => {
                if 32000000u32 * pstcPLLCfg.enPllMul as u32 > 48000000 {
                    return en_result_t::ErrorInvalidMode;
                }
            }
            en_sysctrl_pll_clksource::SysctrlPllRch => {
                if (u16Trim[4] == peripheral.SYSCTRL.rcl_cr.read().trim().bits())
                    && (pstcPLLCfg.enPllMul as u16 > 2)
                    || (pstcPLLCfg.enPllMul as u16 > 2)
                        && peripheral.SYSCTRL.rcl_cr.read().trim().bits() == u16Trim[3]
                    || (peripheral.SYSCTRL.rcl_cr.read().trim().bits() == u16Trim[2])
                        && (pstcPLLCfg.enPllMul as u16 > 3)
                    || ((peripheral.SYSCTRL.rcl_cr.read().trim().bits() == u16Trim[1])
                        && (pstcPLLCfg.enPllMul as u16 > 6))
                {
                    return en_result_t::ErrorInvalidMode;
                }
            }
        }
        peripheral
            .SYSCTRL
            .pll_cr
            .write(|w| w.frsel().bits(pstcPLLCfg.enInFreq as u8));
        peripheral
            .SYSCTRL
            .pll_cr
            .write(|w| w.fosc().bits(pstcPLLCfg.enOutFreq as u8));
        peripheral
            .SYSCTRL
            .pll_cr
            .write(|w| w.divn().bits(pstcPLLCfg.enPllMul as u8));
        peripheral
            .SYSCTRL
            .pll_cr
            .write(|w| w.refsel().bits(pstcPLLCfg.enPllClkSrc as u8));
    }
    en_result_t::Ok
}

///< 时钟分频设置:根据系统情况，单独设置HCLK、PCLK的分配值;
pub fn Sysctrl_SetHCLKDiv(enHCLKDiv: en_sysctrl_hclk_div) -> en_result_t {
    _SysctrlUnlock();
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral
            .SYSCTRL
            .sysctrl0
            .write(|w| w.hclk_prs().bits(enHCLKDiv as u8));
    }
    en_result_t::Ok
}
pub fn Sysctrl_SetPCLKDiv(enPCLKDiv: en_sysctrl_pclk_div) -> en_result_t {
    _SysctrlUnlock();
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral
            .SYSCTRL
            .sysctrl0
            .write(|w| w.pclk_prs().bits(enPCLKDiv as u8));
    }
    en_result_t::Ok
}

///< 时钟频率获取：根据系统需要，获取当前HCLK及PCLK的频率值
pub fn Sysctrl_GetHClkFreq() -> u32 {
    let mut u32Val: u32 = 0;
    let u32hcr_tbl = [4000000, 8000000, 16000000, 22120000, 24000000];
    let u32lcr_tbl = [32768, 38400];
    let mut u16Trim: [u16; 5] = [0, 0, 0, 0, 0];
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        u16Trim[4] = ptr::read(RCH_CR_TRIM_24M_VAL as *const u16);
        u16Trim[3] = ptr::read(RCH_CR_TRIM_22_12M_VAL as *const u16);
        u16Trim[2] = ptr::read(RCH_CR_TRIM_16M_VAL as *const u16);
        u16Trim[1] = ptr::read(RCH_CR_TRIM_8M_VAL as *const u16);
        u16Trim[0] = ptr::read(RCL_CR_TRIM_38400_VAL as *const u16);
        let clkSrc = peripheral.SYSCTRL.sysctrl0.read().clksw().bits();
        let enSrc =
            en_sysctrl_clk_source::try_from(clkSrc).unwrap_or(en_sysctrl_clk_source::SysctrlClkRCH);
        match enSrc {
            en_sysctrl_clk_source::SysctrlClkRCH => {
                if peripheral.SYSCTRL.rch_cr.read().trim().bits() == u16Trim[4] {
                    u32Val = u32hcr_tbl[4];
                } else if peripheral.SYSCTRL.rch_cr.read().trim().bits() == u16Trim[3] {
                    u32Val = u32hcr_tbl[3];
                } else if peripheral.SYSCTRL.rch_cr.read().trim().bits() == u16Trim[2] {
                    u32Val = u32hcr_tbl[2];
                } else if peripheral.SYSCTRL.rch_cr.read().trim().bits() == u16Trim[1] {
                    u32Val = u32hcr_tbl[1];
                } else {
                    u32Val = u32hcr_tbl[0];
                }
            }
            en_sysctrl_clk_source::SysctrlClkXTH => {
                u32Val = 32000000;
            }
            en_sysctrl_clk_source::SysctrlClkRCL => {
                if u16Trim[0] == peripheral.SYSCTRL.rcl_cr.read().trim().bits() {
                    u32Val = u32lcr_tbl[1];
                } else {
                    u32Val = u32lcr_tbl[0];
                }
            }
            en_sysctrl_clk_source::SysctrlClkXTL => {
                u32Val = 32768;
            }
            en_sysctrl_clk_source::SysctrlClkPLL => {}
        }
    }
    u32Val >>= peripheral.SYSCTRL.sysctrl0.read().hclk_prs().bits();
    u32Val
}
pub fn Sysctrl_GetPClkFreq() -> u32 {
    let mut u32Val: u32;
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    u32Val = Sysctrl_GetHClkFreq();
    u32Val >>= peripheral.SYSCTRL.sysctrl0.read().pclk_prs().bits();
    u32Val
}

///< 外设门控开关/状态获取：用于控制外设模块的使能，使用该模块的功能之前，必须使能该模块的门控时钟；
pub fn Sysctrl_SetPeripheralGate(
    enPeripheral: en_sysctrl_peripheral_gate,
    bFlag: bool,
) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    match enPeripheral {
        en_sysctrl_peripheral_gate::SysctrlPeripheralUart0 => {
            peripheral
                .SYSCTRL
                .peri_clken
                .write(|w| w.uart0().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralUart1 => {
            peripheral
                .SYSCTRL
                .peri_clken
                .write(|w| w.uart1().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralLpUart0 => {
            peripheral
                .SYSCTRL
                .peri_clken
                .write(|w| w.lpuart0().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralLpUart1 => {
            peripheral
                .SYSCTRL
                .peri_clken
                .write(|w| w.lpuart1().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralI2c0 => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.i2c0().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralI2c1 => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.i2c1().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralSpi0 => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.spi0().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralSpi1 => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.spi1().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralBaseTim => {
            peripheral
                .SYSCTRL
                .peri_clken
                .write(|w| w.basetim().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralLpTim => {
            peripheral
                .SYSCTRL
                .peri_clken
                .write(|w| w.lptim().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralAdvTim => {
            peripheral
                .SYSCTRL
                .peri_clken
                .write(|w| w.advtim().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralTim3 => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.tim3().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralOpa => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.opa().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralPca => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.pca().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralWdt => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.wdt().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralAdcBgr => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.adc().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralVcLvd => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.vc().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralRng => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.rng().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralPcnt => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.pcnt().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralRtc => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.rtc().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralTrim => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.trim().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralLcd => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.lcd().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralTick => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.tick().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralSwd => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.swd().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralCrc => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.crc().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralAes => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.aes().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralGpio => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.gpio().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralDma => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.dma().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralDiv => {
            peripheral.SYSCTRL.peri_clken.write(|w| w.div().bit(bFlag));
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralFlash => {
            peripheral
                .SYSCTRL
                .peri_clken
                .write(|w| w.flash().bit(bFlag));
        }
    }
    en_result_t::Ok
}

pub fn Sysctrl_GetPeripheralGate(enPeripheral: en_sysctrl_peripheral_gate) -> bool {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    match enPeripheral {
        en_sysctrl_peripheral_gate::SysctrlPeripheralUart0 => {
            peripheral.SYSCTRL.peri_clken.read().uart0().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralUart1 => {
            peripheral.SYSCTRL.peri_clken.read().uart1().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralLpUart0 => {
            peripheral.SYSCTRL.peri_clken.read().lpuart0().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralLpUart1 => {
            peripheral.SYSCTRL.peri_clken.read().lpuart1().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralI2c0 => {
            peripheral.SYSCTRL.peri_clken.read().i2c0().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralI2c1 => {
            peripheral.SYSCTRL.peri_clken.read().i2c1().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralSpi0 => {
            peripheral.SYSCTRL.peri_clken.read().spi0().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralSpi1 => {
            peripheral.SYSCTRL.peri_clken.read().spi1().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralBaseTim => {
            peripheral.SYSCTRL.peri_clken.read().basetim().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralLpTim => {
            peripheral.SYSCTRL.peri_clken.read().lptim().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralAdvTim => {
            peripheral.SYSCTRL.peri_clken.read().advtim().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralTim3 => {
            peripheral.SYSCTRL.peri_clken.read().tim3().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralOpa => {
            peripheral.SYSCTRL.peri_clken.read().opa().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralPca => {
            peripheral.SYSCTRL.peri_clken.read().uart0().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralWdt => {
            peripheral.SYSCTRL.peri_clken.read().wdt().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralAdcBgr => {
            peripheral.SYSCTRL.peri_clken.read().adc().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralVcLvd => {
            peripheral.SYSCTRL.peri_clken.read().vc().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralRng => {
            peripheral.SYSCTRL.peri_clken.read().rng().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralPcnt => {
            peripheral.SYSCTRL.peri_clken.read().pcnt().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralRtc => {
            peripheral.SYSCTRL.peri_clken.read().rtc().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralTrim => {
            peripheral.SYSCTRL.peri_clken.read().trim().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralLcd => {
            peripheral.SYSCTRL.peri_clken.read().lcd().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralTick => {
            peripheral.SYSCTRL.peri_clken.read().tick().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralSwd => {
            peripheral.SYSCTRL.peri_clken.read().swd().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralCrc => {
            peripheral.SYSCTRL.peri_clken.read().crc().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralAes => {
            peripheral.SYSCTRL.peri_clken.read().aes().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralGpio => {
            peripheral.SYSCTRL.peri_clken.read().gpio().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralDma => {
            peripheral.SYSCTRL.peri_clken.read().dma().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralDiv => {
            peripheral.SYSCTRL.peri_clken.read().div().bit()
        }
        en_sysctrl_peripheral_gate::SysctrlPeripheralFlash => {
            peripheral.SYSCTRL.peri_clken.read().flash().bit()
        }
    }
}

///< 系统功能配置：用于设置其他系统相关特殊功能；
pub fn Sysctrl_SetFunc(enFunc: en_sysctrl_func, bFlag: bool) -> en_result_t {
    _SysctrlUnlock();
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    match enFunc {
        en_sysctrl_func::SysctrlEXTHEn => {
            peripheral
                .SYSCTRL
                .sysctrl1
                .write(|w| w.exth_en().bit(bFlag));
        }
        en_sysctrl_func::SysctrlEXTLEn => {
            peripheral
                .SYSCTRL
                .sysctrl1
                .write(|w| w.extl_en().bit(bFlag));
        }
        en_sysctrl_func::SysctrlXTLAlwaysOnEn => {
            peripheral
                .SYSCTRL
                .sysctrl1
                .write(|w| w.xtl_always_on().bit(bFlag));
        }
        en_sysctrl_func::SysctrlClkFuncRTCLpmEn => {
            peripheral
                .SYSCTRL
                .sysctrl1
                .write(|w| w.rtc_lpw().bit(bFlag));
        }
        en_sysctrl_func::SysctrlCMLockUpEn => {
            peripheral
                .SYSCTRL
                .sysctrl1
                .write(|w| w.lockup_en().bit(bFlag));
        }
        en_sysctrl_func::SysctrlSWDUseIOEn => {
            peripheral
                .SYSCTRL
                .sysctrl1
                .write(|w| w.swd_use_io().bit(bFlag));
        }
    }
    en_result_t::Ok
}

///< RTC高速时钟补偿:用于设置RTC高速时钟下的频率补偿
pub fn Sysctrl_SetRTCAdjustClkFreq(enRtcAdj: en_sysctrl_rtc_adjust) -> en_result_t {
    _SysctrlUnlock();
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral
            .SYSCTRL
            .sysctrl1
            .write(|w| w.rtc_freq_adjust().bits(enRtcAdj as u8));
    }
    en_result_t::Ok
}
