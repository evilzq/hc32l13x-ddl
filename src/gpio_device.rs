use crate::ddl::{GetBit, SetBit};
use crate::en_result_t;
use crate::gpio_device::en_gpio_af::GpioAf0;
use core::convert::TryFrom;
use core::ops::Add;
use core::ptr;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_port {
    GpioPortA = 0x00, //* GPIO PORT A
    GpioPortB = 0x40, //* GPIO PORT B
    GpioPortC = 0x80, //* GPIO PORT C
    GpioPortD = 0xc0, //* GPIO PORT D
}

/**
 *******************************************************************************
 ** \brief GPIO PIN类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_pin {
    GpioPin0 = 0,   //* GPIO PIN0
    GpioPin1 = 1,   //* GPIO PIN1
    GpioPin2 = 2,   //* GPIO PIN2
    GpioPin3 = 3,   //* GPIO PIN3
    GpioPin4 = 4,   //* GPIO PIN4
    GpioPin5 = 5,   //* GPIO PIN5
    GpioPin6 = 6,   //* GPIO PIN6
    GpioPin7 = 7,   //* GPIO PIN7
    GpioPin8 = 8,   //* GPIO PIN8
    GpioPin9 = 9,   //* GPIO PIN9
    GpioPin10 = 10, //* GPIO PIN10
    GpioPin11 = 11, //* GPIO PIN11
    GpioPin12 = 12, //* GPIO PIN12
    GpioPin13 = 13, //* GPIO PIN13
    GpioPin14 = 14, //* GPIO PIN14
    GpioPin15 = 15, //* GPIO PIN15
}

/**
 *******************************************************************************
 ** \brief GPIO 端口复用功能(AF-Alternate function)类型定义
 ** \note  具体功能及含义请参考用户手册GPIO复用表
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_af {
    GpioAf0 = 0, //* GPIO功能
    GpioAf1 = 1, //* GPIO AF1:复用功能1
    GpioAf2 = 2, //* GPIO AF2:复用功能2
    GpioAf3 = 3, //* GPIO AF3:复用功能3
    GpioAf4 = 4, //* GPIO AF4:复用功能4
    GpioAf5 = 5, //* GPIO AF5:复用功能5
    GpioAf6 = 6, //* GPIO AF6:复用功能6
    GpioAf7 = 7, //* GPIO AF7:复用功能7
}

/**
 *******************************************************************************
 ** \brief GPIO输入输出配置数据类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_dir {
    GpioDirOut = 0, //* GPIO 输出
    GpioDirIn = 1,  //* GPIO 输入
}

/**
 *******************************************************************************
 ** \brief GPIO端口上拉配置数据类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_pu {
    GpioPuDisable = 0, //* GPIO无上拉
    GpioPuEnable = 1,  //* GPIO上拉
}

/**
 *******************************************************************************
 ** \brief GPIO端口下拉配置数据类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_pd {
    GpioPdDisable = 0, //* GPIO无下拉
    GpioPdEnable = 1,  //* GPIO下拉
}
/**
 *******************************************************************************
 ** \brief GPIO端口输出驱动能力配置数据类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_drv {
    GpioDrvH = 0, //* GPIO高驱动能力
    GpioDrvL = 1, //* GPIO低驱动能力
}

/**
 *******************************************************************************
 ** \brief GPIO端口开漏输出控制数据类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_od {
    GpioOdDisable = 0, //* GPIO开漏输出关闭
    GpioOdEnable = 1,  //* GPIO开漏输出使能
}

/**
 *******************************************************************************
 ** \brief GPIO端口输入/输出值寄存器总线控制模式选择
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_ctrl_mode {
    GpioFastIO = 0, //* FAST IO 总线控制模式
    GpioAHB = 1,    //* AHB 总线控制模式
}

/**
 *******************************************************************************
 ** \brief GPIO中断触发方式类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_irqtype {
    GpioIrqHigh = 0,    //* GPIO高电平触发
    GpioIrqLow = 1,     //* GPIO低电平触发
    GpioIrqRising = 2,  //* GPIO上升沿触发
    GpioIrqFalling = 3, //* GPIO下降沿触发
}

/**
 *******************************************************************************
 ** \brief GPIO 辅助功能（SF-Secondary Function）端口中断模式类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_sf_irqmode {
    GpioSfIrqDpslpMode = 1,  //* Deep Sleep模式
    GpioSfIrqActSlpMode = 0, //* Active/Sleep模式
}

/**
 *******************************************************************************
 ** \brief GPIO 辅助功能（SF-Secondary Function）HCLK输出门控类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_sf_hclkout_g {
    GpioSfHclkOutDisable = 0, //* HCLK输出门控关闭
    GpioSfHclkOutEnable = 1,  //* HCLK输出门控使能
}

/**
 *******************************************************************************
 ** \brief GPIO 辅助功能（SF-Secondary Function）HCLK输出分频选择类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_sf_hclkout_div {
    GpioSfHclkOutDiv1 = 0, //* HCLK
    GpioSfHclkOutDiv2 = 1, //* HCLK/2
    GpioSfHclkOutDiv4 = 2, //* HCLK/4
    GpioSfHclkOutDiv8 = 3, //* HCLK/8
}
/**
 *******************************************************************************
 ** \brief GPIO 辅助功能（SF-Secondary Function）PCLK输出门控类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_sf_pclkout_g {
    GpioSfPclkOutDisable = 0, //* PCLK输出门控关闭
    GpioSfPclkOutEnable = 1,  //* PCLK输出门控使能
}

/**
 *******************************************************************************
 ** \brief GPIO 辅助功能（SF-Secondary Function）PCLK输出分频选择类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_sf_pclkout_div {
    GpioSfPclkOutDiv1 = 0, //* PCLK
    GpioSfPclkOutDiv2 = 1, //* PCLK/2
    GpioSfPclkOutDiv4 = 2, //* PCLK/4
    GpioSfPclkOutDiv8 = 3, //* PCLK/8
}

/**
 *******************************************************************************
 ** \brief GPIO 辅助功能（SF-Secondary Function）IR输出极性选择类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_sf_irpol {
    GpioSfIrP = 0, //* IR正向输出
    GpioSfIrN = 1, //* IR反向输出
}

/**
 *******************************************************************************
 ** \brief GPIO 辅助功能（SF-Secondary Function）SSN通道类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_sf_ssnspi {
    GpioSpi0 = 0, //* SPI0 SSN
    GpioSpi1 = 1, //* SPI1 SSN
}

/**
 *******************************************************************************
 ** \brief GPIO 辅助功能（SF-Secondary Function）SSN与外部时钟输入信号源选择类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_sf_ssn_extclk {
    GpioSfSsnExtClkH = 0,     //* 高电平
    GpioSfSsnExtClkPA03 = 1,  //* PA03
    GpioSfSsnExtClkPA04 = 2,  //* PA04
    GpioSfSsnExtClkPA06 = 3,  //* PA06
    GpioSfSsnExtClkPA08 = 4,  //* PA08
    GpioSfSsnExtClkPA09 = 5,  //* PA09
    GpioSfSsnExtClkPA12 = 6,  //* PA12
    GpioSfSsnExtClkPA15 = 7,  //* PA15
    GpioSfSsnExtClkPB01 = 8,  //* PB01
    GpioSfSsnExtClkPB02 = 9,  //* PB02
    GpioSfSsnExtClkPB05 = 10, //* PB05
    GpioSfSsnExtClkPB06 = 11, //* PB06
    GpioSfSsnExtClkPB09 = 12, //* PB09
    GpioSfSsnExtClkPB10 = 13, //* PB10
    GpioSfSsnExtClkPB12 = 14, //* PB12
    GpioSfSsnExtClkPB14 = 15, //* PB14
}

/**
 *******************************************************************************
 ** \brief GPIO 辅助功能（SF-Secondary Function）定时器互联功能选择类型定义
 ** \note  具体功能及含义请参考用户手册GPIO辅助寄存器描述
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_sf {
    GpioSf0 = 0, //* SF0:PX_SEL的配置功能
    GpioSf1 = 1, //* SF1:辅助功能1
    GpioSf2 = 2, //* SF2:辅助功能2
    GpioSf3 = 3, //* SF3:辅助功能3
    GpioSf4 = 4, //* SF4:辅助功能4
    GpioSf5 = 5, //* SF5:辅助功能5
    GpioSf6 = 6, //* SF6:辅助功能6
    GpioSf7 = 7, //* SF7:辅助功能7
}

/**
 *******************************************************************************
 ** \brief GPIO 辅助功能（SF-Secondary Function）定时器门控类型选择数据类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_sf_tim_g {
    GpioSfTim0G = 0,   //*Tim0定时器GATE输入选择
    GpioSfTim1G = 3,   //*Tim1定时器GATE输入选择
    GpioSfTim2G = 6,   //*Tim2定时器GATE输入选择
    GpioSfTim3G = 9,   //*Tim3定时器GATE输入选择
    GpioSfLpTimG = 12, //*LPTim定时器GATE输入选择
}

/**
 *******************************************************************************
 ** \brief GPIO 辅助功能（SF-Secondary Function）定时器ETR类型选择数据类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_sf_tim_e {
    GpioSfTim0E = 0,   //*Tim0定时器ETR输入选择
    GpioSfTim1E = 3,   //*Tim1定时器ETR输入选择
    GpioSfTim2E = 6,   //*Tim2定时器ETR输入选择
    GpioSfTim3E = 9,   //*Tim3定时器ETR输入选择
    GpioSfLpTimE = 12, //*LPTim定时器ETR输入选择
}

/**
 *******************************************************************************
 ** \brief GPIO 辅助功能（SF-Secondary Function）定时器捕获输入类型选择数据类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_sf_tim_c {
    GpioSfTim0CA = 0,  //*Tim0定时器CHA输入选择
    GpioSfTim1CA = 3,  //*Tim1定时器CHA输入选择
    GpioSfTim2CA = 6,  //*Tim2定时器CHA输入选择
    GpioSfTim3CA = 9,  //*Tim3定时器CH0A输入选择
    GpioSfTim3CB = 12, //*Tim3定时器CH0B输入选择
}

/**
 *******************************************************************************
 ** \brief GPIO 辅助功能（SF-Secondary Function）PCA捕获选择数据类型定义
 ******************************************************************************/
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_gpio_sf_pca {
    GpioSfPcaCH0 = 0, //*PCA_CH0捕获口输入选择
    GpioSfPcaECI = 3, //*PCA ECI时钟输入选择
}

/**
 *******************************************************************************
 ** \brief GPIO 端口配置结构体定义
 ******************************************************************************/

pub struct stc_gpio_cfg {
    enDir: en_gpio_dir,            //* 端口方向配置
    enDrv: en_gpio_drv,            //* 端口驱动能力配置
    enPu: en_gpio_pu,              //* 端口上拉配置
    enPd: en_gpio_pd,              //* 端口下拉配置
    enOD: en_gpio_od,              //* 端口开漏输出配置
    enCtrlMode: en_gpio_ctrl_mode, //* 端口输入/输出值寄存器总线控制模式配置
}

///< GPIO IO初始化/去初始化
pub fn Gpio_Init(
    enPort: en_gpio_port,
    enPin: en_gpio_pin,
    pstcGpioCfg: &stc_gpio_cfg,
) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        SetBit(
            peripheral.GPIO.paads.as_ptr().offset(enPort as isize),
            enPin as u32,
            false,
        );
        //方向配置
        SetBit(
            peripheral.GPIO.padir.as_ptr().offset(enPort as isize),
            enPin as u32,
            pstcGpioCfg.enDir as u8 != 0,
        );
        //驱动能力配置
        SetBit(
            peripheral.GPIO.padr.as_ptr().offset(enPort as isize),
            enPin as u32,
            pstcGpioCfg.enDrv as u8 != 0,
        );
        //上拉/下拉配置
        SetBit(
            peripheral.GPIO.papu.as_ptr().offset(enPort as isize),
            enPin as u32,
            pstcGpioCfg.enPu as u8 != 0,
        );
        SetBit(
            peripheral.GPIO.papd.as_ptr().offset(enPort as isize),
            enPin as u32,
            pstcGpioCfg.enPd as u8 != 0,
        );
        //开漏输出功能
        SetBit(
            peripheral.GPIO.paod.as_ptr().offset(enPort as isize),
            enPin as u32,
            pstcGpioCfg.enOD as u8 != 0,
        );
        peripheral
            .GPIO
            .ctrl2
            .write(|w| w.ahb_sel().bit(pstcGpioCfg.enCtrlMode as u8 != 0));
        ptr::write(
            peripheral
                .GPIO
                .pa00_sel
                .as_ptr()
                .offset(enPort as isize)
                .offset(((enPin as u32) << 2) as isize),
            GpioAf0 as u32,
        );
    }
    en_result_t::Ok
}

///< GPIO 获取端口输入电平
pub fn Gpio_GetInputIO(enPort: en_gpio_port, enPin: en_gpio_pin) -> bool {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        GetBit(
            peripheral.GPIO.pain.as_ptr().offset(enPort as isize),
            enPin as u32,
        )
    }
}
pub fn Gpio_GetInputData(enPort: en_gpio_port) -> u16 {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    match enPort {
        en_gpio_port::GpioPortA => peripheral.GPIO.pain.read().bits() as u16,
        en_gpio_port::GpioPortB => peripheral.GPIO.pbin.read().bits() as u16,
        en_gpio_port::GpioPortC => peripheral.GPIO.pcin.read().bits() as u16,
        en_gpio_port::GpioPortD => peripheral.GPIO.pdin.read().bits() as u16,
    }
}

///< GPIO 设置端口输出
///< GPIO 端口输出电平配置及获取
pub fn Gpio_WriteOutputIO(enPort: en_gpio_port, enPin: en_gpio_pin, bVal: bool) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        SetBit(
            peripheral.GPIO.paout.as_ptr().offset(enPort as isize),
            enPin as u32,
            bVal,
        );
    }
    en_result_t::Ok
}
pub fn Gpio_ReadOutputIO(enPort: en_gpio_port, enPin: en_gpio_pin) -> bool {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        GetBit(
            peripheral.GPIO.paout.as_ptr().offset(enPort as isize),
            enPin as u32,
        )
    }
}
///< GPIO 端口/引脚输出电平置位
pub fn Gpio_SetPort(enPort: en_gpio_port, u16ValMsk: u16) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        match enPort {
            en_gpio_port::GpioPortA => {
                peripheral.GPIO.pabset.write(|w| w.bits(u16ValMsk as u32));
            }
            en_gpio_port::GpioPortB => {
                peripheral.GPIO.pbbset.write(|w| w.bits(u16ValMsk as u32));
            }
            en_gpio_port::GpioPortC => {
                peripheral.GPIO.pcbset.write(|w| w.bits(u16ValMsk as u32));
            }
            en_gpio_port::GpioPortD => {
                peripheral.GPIO.pdbset.write(|w| w.bits(u16ValMsk as u32));
            }
        }
    }
    en_result_t::Ok
}
pub fn Gpio_SetIO(enPort: en_gpio_port, enPin: en_gpio_pin) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        SetBit(
            peripheral.GPIO.pabset.as_ptr().offset(enPort as isize),
            enPin as u32,
            true,
        );
    }
    en_result_t::Ok
}
///< GPIO 端口/引脚输出电平清零
pub fn Gpio_ClrPort(enPort: en_gpio_port, u16ValMsk: u16) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        match enPort {
            en_gpio_port::GpioPortA => {
                peripheral.GPIO.pabclr.write(|w| w.bits(u16ValMsk as u32));
            }
            en_gpio_port::GpioPortB => {
                peripheral.GPIO.pbbclr.write(|w| w.bits(u16ValMsk as u32));
            }
            en_gpio_port::GpioPortC => {
                peripheral.GPIO.pcbclr.write(|w| w.bits(u16ValMsk as u32));
            }
            en_gpio_port::GpioPortD => {
                peripheral.GPIO.pdbclr.write(|w| w.bits(u16ValMsk as u32));
            }
        }
    }
    en_result_t::Ok
}
pub fn Gpio_ClrIO(enPort: en_gpio_port, enPin: en_gpio_pin) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        SetBit(
            peripheral.GPIO.pabclr.as_ptr().offset(enPort as isize),
            enPin as u32,
            true,
        );
    }
    en_result_t::Ok
}
///< GPIO 端口输出电平置位与清零设置
pub fn Gpio_SetClrPort(enPort: en_gpio_port, u32ValMsk: u32) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {}
    en_result_t::Ok
}

///< GPIO 设置端口为模拟功能
pub fn Gpio_SetAnalogMode(enPort: en_gpio_port, enPin: en_gpio_pin) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {}
    en_result_t::Ok
}

///< GPIO 设置端口为端口复用功能
pub fn Gpio_SetAfMode(enPort: en_gpio_port, enPin: en_gpio_pin, enAf: en_gpio_af) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {}
    en_result_t::Ok
}

///< GPIO 端口中断控制功能使能/关闭
pub fn Gpio_EnableIrq(
    enPort: en_gpio_port,
    enPin: en_gpio_pin,
    enType: en_gpio_irqtype,
) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {}
    en_result_t::Ok
}
pub fn Gpio_DisableIrq(
    enPort: en_gpio_port,
    enPin: en_gpio_pin,
    enType: en_gpio_irqtype,
) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {}
    en_result_t::Ok
}
///< GPIO 中断状态获取
pub fn Gpio_GetIrqStatus(enPort: en_gpio_port, enPin: en_gpio_pin) -> bool {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {}
    true
}
///< GPIO 中断标志清除
pub fn Gpio_ClearIrq(enPort: en_gpio_port, enPin: en_gpio_pin) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {}
    en_result_t::Ok
}

///< GPIO 端口辅助功能配置
///< GPIO 中断模式配置
pub fn Gpio_SfIrqModeCfg(enIrqMode: en_gpio_sf_irqmode) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {}
    en_result_t::Ok
}
///< GPIO IR输出极性配置
pub fn Gpio_SfIrPolCfg(enIrPolMode: en_gpio_sf_irpol) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {}
    en_result_t::Ok
}
///< GPIO HCLK输出配置
pub fn Gpio_SfHClkOutputCfg(
    enGate: en_gpio_sf_hclkout_g,
    enDiv: en_gpio_sf_hclkout_div,
) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {}
    en_result_t::Ok
}
///< GPIO PCLK输出配置
pub fn Gpio_SfPClkOutputCfg(
    enGate: en_gpio_sf_pclkout_g,
    enDiv: en_gpio_sf_pclkout_div,
) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {}
    en_result_t::Ok
}
///< GPIO 外部时钟输入配置
pub fn Gpio_SfExtClkCfg(enExtClk: en_gpio_sf_ssn_extclk) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {}
    en_result_t::Ok
}
///< GPIO SPI SSN输入配置
pub fn Gpio_SfSsnCfg(enSpi: en_gpio_sf_ssnspi, enSsn: en_gpio_sf_ssn_extclk) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {}
    en_result_t::Ok
}
///< GPIO Timer 门控输入配置
pub fn Gpio_SfTimGCfg(enTimG: en_gpio_sf_tim_g, enSf: en_gpio_sf) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {}
    en_result_t::Ok
}
///< GPIO Timer ETR选择配置
pub fn Gpio_SfTimECfg(enTimE: en_gpio_sf_tim_e, enSf: en_gpio_sf) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {}
    en_result_t::Ok
}
///< GPIO Timer 捕获输入配置
pub fn Gpio_SfTimCCfg(enTimC: en_gpio_sf_tim_c, enSf: en_gpio_sf) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {}
    en_result_t::Ok
}
///< GPIO PCA捕获选择配置
pub fn Gpio_SfPcaCfg(enPca: en_gpio_sf_pca, enSf: en_gpio_sf) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {}
    en_result_t::Ok
}
