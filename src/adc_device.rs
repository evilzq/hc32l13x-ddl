#![allow(dead_code)]
use crate::delay::delay10us;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::en_result_t;
#[doc = "SCAN模式使用ADC CH0"]
const ADC_SCAN_CH0_EN: u8 = 1;
#[doc = "SCAN模式使用ADC CH1"]
const ADC_SCAN_CH1_EN: u8 = 0x1u8 << 1;
#[doc = "SCAN模式使用ADC CH2"]
const ADC_SCAN_CH2_EN: u8 = 0x1u8 << 2;
#[doc = "SCAN模式使用ADC CH3"]
const ADC_SCAN_CH3_EN: u8 = 0x1u8 << 3;
#[doc = "SCAN模式使用ADC CH4"]
const ADC_SCAN_CH4_EN: u8 = 0x1u8 << 4;
#[doc = "SCAN模式使用ADC CH5"]
const ADC_SCAN_CH5_EN: u8 = 0x1u8 << 5;
#[doc = "SCAN模式使用ADC CH6"]
const ADC_SCAN_CH6_EN: u8 = 0x1u8 << 6;
#[doc = "SCAN模式使用ADC CH7"]
const ADC_SCAN_CH7_EN: u8 = 0x1u8 << 7;

#[doc = "ADC转换模式"]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_adc_mode {
    AdcSglMode = 0,  //* 单输入通道单次转换模式 */
    AdcScanMode = 1, //* 多输入通道顺序/插队扫描转换模式*/
}

#[doc = "ADC时钟分频选择"]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_adc_clk_div {
    AdcMskClkDiv1 = 0 << 2, //* PCLK */
    AdcMskClkDiv2 = 1 << 2, //* 1/2 PCLK */
    AdcMskClkDiv4 = 2 << 2, //* 1/4 PCLK */
    AdcMskClkDiv8 = 3 << 2, //* 1/8 PCLK */
}

#[doc = "ADC参考电压"]
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_adc_ref_vol_sel {
    AdcMskRefVolSelInBgr1p5 = 0x0 << 9, //*内部参考电压1.5V(SPS<=200kHz)*/
    AdcMskRefVolSelInBgr2p5 = 0x1 << 9, //*内部参考电压2.5V(avdd>3V;SPS<=200kHz)*/
    AdcMskRefVolSelExtern1 = 0x2 << 9,  //*外部输入(max avdd)   PB01*/
    AdcMskRefVolSelAVDD = 0x3 << 9,     //*AVDD*/
}

#[doc = "ADC转换通道选择"]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_adc_samp_ch_sel {
    AdcExInputCH0 = 0,   //*使用PA00*/
    AdcExInputCH1 = 1,   //*使用PA01*/
    AdcExInputCH2 = 2,   //*使用PA02*/
    AdcExInputCH3 = 3,   //*使用PA03*/
    AdcExInputCH4 = 4,   //*使用PA04*/
    AdcExInputCH5 = 5,   //*使用PA05*/
    AdcExInputCH6 = 6,   //*使用PA06*/
    AdcExInputCH7 = 7,   //*使用PA07*/
    AdcExInputCH8 = 8,   //*使用PB00*/
    AdcExInputCH9 = 9,   //*使用PB01*/
    AdcExInputCH10 = 10, //*使用PC00*/
    AdcExInputCH11 = 11, //*使用PC01*/
    AdcExInputCH12 = 12, //*使用PC02*/
    AdcExInputCH13 = 13, //*使用PC03*/
    AdcExInputCH14 = 14, //*使用PC04*/
    AdcExInputCH15 = 15, //*使用PC05*/
    AdcExInputCH16 = 16, //*使用PB02*/
    AdcExInputCH17 = 17, //*使用PB10*/
    AdcExInputCH18 = 18, //*使用PB11*/
    AdcExInputCH19 = 19, //*使用PB12*/
    AdcExInputCH20 = 20, //*使用PB13*/
    AdcExInputCH21 = 21, //*使用PB14*/
    AdcExInputCH22 = 22, //*使用PB15*/
    AdcExInputCH23 = 23, //*使用PE15*/
    AdcOPA0Input = 24,   //*使用通道8输入OPA0*/
    AdcOPA1Input = 25,   //*使用通道8输入OPA1*/

    AdcOPA2Input = 26,     //*使用通道8输入OPA2*/
    AdcAVccdiv3Input = 27, //*使用1/3 AVCC(必须使用输入增益)*/
    AdcAiTsInput = 28,     //*使用内置温度传感器BGR_TS(必须使用输入增益)*/
    AdcVref1_2Input = 29,  //*使用内部基准1.2V(必须使用输入增益)*/
}

#[doc = "ADC输入信号放大器控制"]
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_adc_op_buf {
    AdcMskBufEnable = 1 << 11, //* 打开放大器BUF */
    AdcMskBufDisable = 0,      //* 关闭放大器BUF */
}

#[doc = "ADC采样周期选择"]
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_adc_samp_cycle_sel {
    AdcMskSampCycle4Clk = 0 << 12,  //*4个采样时钟*/
    AdcMskSampCycle6Clk = 1 << 12,  //*6个采样时钟*/
    AdcMskSampCycle8Clk = 2 << 12,  //*8个采样时钟*/
    AdcMskSampCycle12Clk = 3 << 12, //*12个采样时钟*/
}

#[doc = "ADC内部参考电压使能控制"]
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_adc_in_ref {
    AdcMskInRefEnable = 1 << 14, //* 内部参考电压使能 */
    AdcMskInRefDisable = 0,      //* 内部参考电压关闭 */
}

#[doc = "ADC周边模块反射源选择"]
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_adc_trig_sel {
    AdcMskTrigTimer0 = 1 << 0,   //*选择timer0中断源，自动触发ADC采样*/
    AdcMskTrigTimer1 = 1 << 1,   //*选择timer1中断源，自动触发ADC采样*/
    AdcMskTrigTimer2 = 1 << 2,   //*选择timer2中断源，自动触发ADC采样*/
    AdcMskTrigTimer3 = 1 << 3,   //*选择timer3中断源，自动触发ADC采样*/
    AdcMskTrigTimer4 = 1 << 4,   //*选择timer4中断源，自动触发ADC采样*/
    AdcMskTrigTimer5 = 1 << 5,   //*选择timer5中断源，自动触发ADC采样*/
    AdcMskTrigTimer6 = 1 << 6,   //*选择timer6中断源，自动触发ADC采样*/
    AdcMskTrigUart0 = 1 << 7,    //*选择uart0中断源，自动触发ADC采样*/
    AdcMskTrigUart1 = 1 << 8,    //*选择uart1中断源，自动触发ADC采样*/
    AdcMskTrigLpuart0 = 1 << 9,  //*选择lpuart0中断源，自动触发ADC采样*/
    AdcMskTrigLpuart1 = 1 << 10, //*选择lpuart1中断源，自动触发ADC采样*/
    AdcMskTrigVC0 = 1 << 11,     //*选择VC0中断源，自动触发ADC采样*/
    AdcMskTrigVC1 = 1 << 12,     //*选择VC1中断源，自动触发ADC采样*/
    AdcMskTrigRTC = 1 << 13,     //*选择RTC中断源，自动触发ADC采样*/
    AdcMskTrigPCA = 1 << 14,     //*选择PCA中断源，自动触发ADC采样*/
    AdcMskTrigSPI0 = 1 << 15,    //*选择SPI0中断源，自动触发ADC采样*/
    AdcMskTrigSPI1 = 1 << 16,    //*选择SPI1中断源，自动触发ADC采样*/
    AdcMskTrigDMA = 1 << 17,     //*选择DMA中断源，自动触发ADC采样*/
    AdcMskTrigPA03 = 1 << 18,    //*选择PA03中断源，自动触发ADC采样*/
    AdcMskTrigPB03 = 1 << 19,    //*选择PB03中断源，自动触发ADC采样*/
    AdcMskTrigPC03 = 1 << 20,    //*选择PC03中断源，自动触发ADC采样*/
    AdcMskTrigPD03 = 1 << 21,    //*选择PD03中断源，自动触发ADC采样*/
    AdcMskTrigPA07 = 1 << 22,    //*选择PA07中断源，自动触发ADC采样*/
    AdcMskTrigPB07 = 1 << 23,    //*选择PB07中断源，自动触发ADC采样*/
    AdcMskTrigPC07 = 1 << 24,    //*选择PC07中断源，自动触发ADC采样*/
    AdcMskTrigPD07 = 1 << 25,    //*选择PD07中断源，自动触发ADC采样*/
    AdcMskTrigPA11 = 1 << 26,    //*选择PA11中断源，自动触发ADC采样*/
    AdcMskTrigPB11 = 1 << 27,    //*选择PB11中断源，自动触发ADC采样*/
    AdcMskTrigPC11 = 1 << 28,    //*选择PC11中断源，自动触发ADC采样*/
    AdcMskTrigPA15 = 1 << 29,    //*选择PA15中断源，自动触发ADC采样*/
    AdcMskTrigPB15 = 1 << 30,    //*选择PB15中断源，自动触发ADC采样*/
    AdcMskTrigPC15 = 1 << 31,    //*选择PC15中断源，自动触发ADC采样*/
}

#[doc = "ADC外部触发源寄存器选择"]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_adc_ext_trig_sel {
    AdcExtTrig0 = 0, //*单次及顺序扫描转换 外部触发源选择寄存器*/
    AdcExtTrig1 = 1, //*插队扫描转换 外部触发源选择寄存器*/
}

#[doc = "ADC顺序转换通道"]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_adc_sqr_chmux {
    AdcSQRCH0MUX = 0,   //*顺序扫描模式转换通道0*/
    AdcSQRCH1MUX = 1,   //*顺序扫描模式转换通道1*/
    AdcSQRCH2MUX = 2,   //*顺序扫描模式转换通道2*/
    AdcSQRCH3MUX = 3,   //*顺序扫描模式转换通道3*/
    AdcSQRCH4MUX = 4,   //*顺序扫描模式转换通道4*/
    AdcSQRCH5MUX = 5,   //*顺序扫描模式转换通道5*/
    AdcSQRCH6MUX = 6,   //*顺序扫描模式转换通道6*/
    AdcSQRCH7MUX = 7,   //*顺序扫描模式转换通道7*/
    AdcSQRCH8MUX = 8,   //*顺序扫描模式转换通道8*/
    AdcSQRCH9MUX = 9,   //*顺序扫描模式转换通道9*/
    AdcSQRCH10MUX = 10, //*顺序扫描模式转换通道10*/
    AdcSQRCH11MUX = 11, //*顺序扫描模式转换通道11*/
    AdcSQRCH12MUX = 12, //*顺序扫描模式转换通道12*/
    AdcSQRCH13MUX = 13, //*顺序扫描模式转换通道13*/
    AdcSQRCH14MUX = 14, //*顺序扫描模式转换通道14*/
    AdcSQRCH15MUX = 15, //*顺序扫描模式转换通道15*/
}

#[doc = "ADC插队转换通道"]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_adc_jqr_chmux {
    AdcJQRCH0MUX = 0, //*转换通道0*/
    AdcJQRCH1MUX = 1, //*转换通道1*/
    AdcJQRCH2MUX = 2, //*转换通道2*/
    AdcJQRCH3MUX = 3, //*转换通道3*/
}

#[doc = "ADC结果对齐方式"]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_adc_align {
    AdcAlignRight = 0,
    AdcAlignLeft = 1,
}

#[doc = "ADC转换结果自动累加功能"]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_adc_result_acc {
    AdcResultAccEnable = 1,
    AdcResultAccDisable = 0,
}

#[doc = "ADC中断类型定义"]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum en_adc_irq_type {
    AdcMskIrqJqr = 1 << 5, //*ADC插队扫描转换完成*/
    AdcMskIrqSqr = 1 << 4, //*ADC顺序扫描转换完成*/
    AdcMskIrqReg = 1 << 3, //*ADC转换结果比较区间内*/
    AdcMskIrqHt = 1 << 2,  //*ADC转换结果高于HT*/
    AdcMskIrqLt = 1 << 1,  //*ADC转换结果低于LT*/
    AdcMskIrqSgl = 1 << 0, //*ADC单次转换完成*/
}

#[doc = "ADC初始化配置结构体"]
pub struct stc_adc_cfg {
    enAdcMode: en_adc_mode,                   //* ADC转换模式*/
    enAdcClkDiv: en_adc_clk_div,              //* ADC时钟选择*/
    enAdcSampCycleSel: en_adc_samp_cycle_sel, //* ADC采样周期选择*/
    enAdcRefVolSel: en_adc_ref_vol_sel,       //* ADC参考电压选择*/
    enAdcOpBuf: en_adc_op_buf,                //* ADC输入信号放大器控制使能*/
    enInRef: en_adc_in_ref,                   //* ADC内部参考电压使能*/
    enAdcAlign: en_adc_align,                 //* ADC转换结果对齐控制*/
}

#[doc = "ADC顺序扫描模式配置结构体"]
pub struct stc_adc_sqr_cfg {
    u8SqrCnt: u8,                   //* ADC顺序扫描转换次数*/
    enResultAcc: en_adc_result_acc, //* ADC转换结果自动累加功能*/
    bSqrDmaTrig: bool,              //* ADC顺序扫描转换完成DMA触发使能*/
}

#[doc = "ADC插队扫描模式配置结构体"]
pub struct stc_adc_jqr_cfg {
    u8JqrCnt: u8,      //* ADC顺序扫描转换次数*/
    bJqrDmaTrig: bool, //* ADC插队扫描转换完成DMA触发使能*/
}

#[doc = "ADC比较功能配置结构体"]
pub struct stc_adc_threshold_cfg {
    bAdcRegCmp: bool, //*ADC区间使能*/
    bAdcHtCmp: bool,  //*ADC上超出区间使能*/
    bAdcLtCmp: bool,  //*ADC下超出区间使能*/

    u32AdcHighThd: u32, //*ADC比较上阈值*/
    u32AdcLowThd: u32,  //*ADC比较下阈值*/

    enSampChSel: en_adc_samp_ch_sel, //*ADC采样通道选择*/
}

#[doc = "ADC 初始化"]
pub fn Adc_Init(pstcAdcCfg: &stc_adc_cfg) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral.ADC.cr0.write(|w| w.bits(1));
        delay10us(2);
        peripheral.ADC.cr0.modify(|r, w| {
            w.bits(
                r.bits()
                    | (pstcAdcCfg.enAdcClkDiv as u32
                        | pstcAdcCfg.enAdcRefVolSel as u32
                        | pstcAdcCfg.enAdcOpBuf as u32
                        | pstcAdcCfg.enAdcSampCycleSel as u32
                        | pstcAdcCfg.enInRef as u32),
            )
        });
        peripheral
            .ADC
            .cr1
            .write(|w| w.mode().bit(pstcAdcCfg.enAdcMode as u8 != 0));
        peripheral
            .ADC
            .cr1
            .write(|w| w.align().bit(pstcAdcCfg.enAdcAlign as u8 != 0));
    }
    en_result_t::Ok
}

#[doc = "ADC 中断使能"]
pub fn Adc_EnableIrq() {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    peripheral.ADC.cr0.write(|w| w.ie().bit(true))
}
#[doc = "ADC 中断禁止"]
pub fn Adc_DisableIrq() {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    peripheral.ADC.cr0.write(|w| w.ie().bit(false))
}

#[doc = "ADC 中断/采样完成状态获取"]
pub fn Adc_GetIrqStatus(enAdcIrq: en_adc_irq_type) -> bool {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    match enAdcIrq {
        en_adc_irq_type::AdcMskIrqJqr => peripheral.ADC.ifr.read().jqrif().bit(),
        en_adc_irq_type::AdcMskIrqSqr => peripheral.ADC.ifr.read().sqrif().bit(),
        en_adc_irq_type::AdcMskIrqReg => peripheral.ADC.ifr.read().regif().bit(),
        en_adc_irq_type::AdcMskIrqHt => peripheral.ADC.ifr.read().htif().bit(),
        en_adc_irq_type::AdcMskIrqLt => peripheral.ADC.ifr.read().ltif().bit(),
        en_adc_irq_type::AdcMskIrqSgl => peripheral.ADC.ifr.read().sglif().bit(),
    }
}
#[doc = "ADC 中断/采样完成状态清除"]
pub fn Adc_ClrIrqStatus(enAdcIrq: en_adc_irq_type) {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    match enAdcIrq {
        en_adc_irq_type::AdcMskIrqJqr => {
            peripheral.ADC.icr.write(|w| w.jqric().bit(false));
        }
        en_adc_irq_type::AdcMskIrqSqr => {
            peripheral.ADC.icr.write(|w| w.sqric().bit(false));
        }
        en_adc_irq_type::AdcMskIrqReg => {
            peripheral.ADC.icr.write(|w| w.regic().bit(false));
        }
        en_adc_irq_type::AdcMskIrqHt => {
            peripheral.ADC.icr.write(|w| w.htic().bit(false));
        }
        en_adc_irq_type::AdcMskIrqLt => {
            peripheral.ADC.icr.write(|w| w.ltic().bit(false));
        }
        en_adc_irq_type::AdcMskIrqSgl => {
            peripheral.ADC.icr.write(|w| w.sglic().bit(false));
        }
    }
}

#[doc = "ADC 使能"]
pub fn Adc_Enable() {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    peripheral.ADC.cr0.write(|w| w.en().bit(true));
}
#[doc = "ADC 禁止"]
pub fn Adc_Disable() {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    peripheral.ADC.cr0.write(|w| w.en().bit(false));
}

#[doc = "ADC 顺序扫描模式配置"]
pub fn Adc_SqrModeCfg(pstcAdcSqrCfg: &stc_adc_sqr_cfg) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral.ADC.cr1.write(|w| w.raccclr().bit(false));
        peripheral
            .ADC
            .cr1
            .write(|w| w.raccen().bit(pstcAdcSqrCfg.enResultAcc as u8 != 0));
        peripheral
            .ADC
            .cr1
            .write(|w| w.dmasqr().bit(pstcAdcSqrCfg.bSqrDmaTrig as u8 != 0));
        peripheral
            .ADC
            .sqr2
            .write(|w| w.cnt().bits(pstcAdcSqrCfg.u8SqrCnt as u8 - 1));
    }
    en_result_t::Ok
}
#[doc = "ADC 插队扫描模式配置"]
pub fn Adc_JqrModeCfg(pstcAdcJqrCfg: &stc_adc_jqr_cfg) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral
            .ADC
            .cr1
            .write(|w| w.dmasqr().bit(pstcAdcJqrCfg.bJqrDmaTrig as u8 != 0));
        peripheral
            .ADC
            .jqr
            .write(|w| w.bits(pstcAdcJqrCfg.bJqrDmaTrig as u32));
    }
    en_result_t::Ok
}

#[doc = "ADC Sgl 单次转换模式通道选择配置"]
pub fn Adc_CfgSglChannel(enstcAdcSampCh: en_adc_samp_ch_sel) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral
            .ADC
            .cr0
            .write(|w| w.sglmux().bits(enstcAdcSampCh as u8));
    }
    en_result_t::Ok
}
#[doc = "ADC SQR 顺序扫描转换模式通道选择配置"]
pub fn Adc_CfgSqrChannel(
    enstcAdcSqrChMux: en_adc_sqr_chmux,
    enstcAdcSampCh: en_adc_samp_ch_sel,
) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        match enstcAdcSqrChMux {
            en_adc_sqr_chmux::AdcSQRCH0MUX => {
                peripheral
                    .ADC
                    .sqr0
                    .write(|w| w.ch0mux().bits(enstcAdcSampCh as u8));
            }
            en_adc_sqr_chmux::AdcSQRCH1MUX => {
                peripheral
                    .ADC
                    .sqr0
                    .write(|w| w.ch1mux().bits(enstcAdcSampCh as u8));
            }
            en_adc_sqr_chmux::AdcSQRCH2MUX => {
                peripheral
                    .ADC
                    .sqr0
                    .write(|w| w.ch2mux().bits(enstcAdcSampCh as u8));
            }
            en_adc_sqr_chmux::AdcSQRCH3MUX => {
                peripheral
                    .ADC
                    .sqr0
                    .write(|w| w.ch3mux().bits(enstcAdcSampCh as u8));
            }
            en_adc_sqr_chmux::AdcSQRCH4MUX => {
                peripheral
                    .ADC
                    .sqr0
                    .write(|w| w.ch4mux().bits(enstcAdcSampCh as u8));
            }
            en_adc_sqr_chmux::AdcSQRCH5MUX => {
                peripheral
                    .ADC
                    .sqr0
                    .write(|w| w.ch5mux().bits(enstcAdcSampCh as u8));
            }
            en_adc_sqr_chmux::AdcSQRCH6MUX => {
                peripheral
                    .ADC
                    .sqr1
                    .write(|w| w.ch6mux().bits(enstcAdcSampCh as u8));
            }
            en_adc_sqr_chmux::AdcSQRCH7MUX => {
                peripheral
                    .ADC
                    .sqr1
                    .write(|w| w.ch7mux().bits(enstcAdcSampCh as u8));
            }
            en_adc_sqr_chmux::AdcSQRCH8MUX => {
                peripheral
                    .ADC
                    .sqr1
                    .write(|w| w.ch8mux().bits(enstcAdcSampCh as u8));
            }
            en_adc_sqr_chmux::AdcSQRCH9MUX => {
                peripheral
                    .ADC
                    .sqr1
                    .write(|w| w.ch9mux().bits(enstcAdcSampCh as u8));
            }
            en_adc_sqr_chmux::AdcSQRCH10MUX => {
                peripheral
                    .ADC
                    .sqr1
                    .write(|w| w.ch10mux().bits(enstcAdcSampCh as u8));
            }
            en_adc_sqr_chmux::AdcSQRCH11MUX => {
                peripheral
                    .ADC
                    .sqr1
                    .write(|w| w.ch11mux().bits(enstcAdcSampCh as u8));
            }
            en_adc_sqr_chmux::AdcSQRCH12MUX => {
                peripheral
                    .ADC
                    .sqr2
                    .write(|w| w.ch12mux().bits(enstcAdcSampCh as u8));
            }
            en_adc_sqr_chmux::AdcSQRCH13MUX => {
                peripheral
                    .ADC
                    .sqr2
                    .write(|w| w.ch13mux().bits(enstcAdcSampCh as u8));
            }
            en_adc_sqr_chmux::AdcSQRCH14MUX => {
                peripheral
                    .ADC
                    .sqr2
                    .write(|w| w.ch14mux().bits(enstcAdcSampCh as u8));
            }
            en_adc_sqr_chmux::AdcSQRCH15MUX => {
                peripheral
                    .ADC
                    .sqr2
                    .write(|w| w.ch15mux().bits(enstcAdcSampCh as u8));
            }
        }
    }
    en_result_t::Ok
}
#[doc = "ADC JQR 插队扫描转换模式通道选择配置"]
pub fn Adc_CfgJqrChannel(
    enstcAdcJqrChMux: en_adc_jqr_chmux,
    enstcAdcSampCh: en_adc_samp_ch_sel,
) -> en_result_t {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        match enstcAdcJqrChMux {
            en_adc_jqr_chmux::AdcJQRCH0MUX => {
                peripheral
                    .ADC
                    .jqr
                    .write(|w| w.ch0mux().bits(enstcAdcSampCh as u8));
            }
            en_adc_jqr_chmux::AdcJQRCH1MUX => {
                peripheral
                    .ADC
                    .jqr
                    .write(|w| w.ch1mux().bits(enstcAdcSampCh as u8));
            }
            en_adc_jqr_chmux::AdcJQRCH2MUX => {
                peripheral
                    .ADC
                    .jqr
                    .write(|w| w.ch2mux().bits(enstcAdcSampCh as u8));
            }
            en_adc_jqr_chmux::AdcJQRCH3MUX => {
                peripheral
                    .ADC
                    .jqr
                    .write(|w| w.ch3mux().bits(enstcAdcSampCh as u8));
            }
        }
    }
    en_result_t::Ok
}

#[doc = "ADC 单次转换外部触发源配置"]
pub fn Adc_SglExtTrigCfg(enAdcTrigSel: en_adc_trig_sel, bValue: bool) {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        if bValue {
            peripheral
                .ADC
                .exttrigger0
                .modify(|r, w| w.bits(r.bits() | enAdcTrigSel as u32));
        } else {
            peripheral
                .ADC
                .exttrigger0
                .modify(|r, w| w.bits(r.bits() & !(enAdcTrigSel as u32)));
        }
    }
}
#[doc = "ADC 顺序扫描转换外部触发源配置"]
pub fn Adc_SqrExtTrigCfg(enAdcTrigSel: en_adc_trig_sel, bValue: bool) {
    Adc_SglExtTrigCfg(enAdcTrigSel, bValue)
}
#[doc = "ADC 插队扫描转换外部触发源配置"]
pub fn Adc_JqrExtTrigCfg(enAdcTrigSel: en_adc_trig_sel, bValue: bool) {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        if bValue {
            peripheral
                .ADC
                .exttrigger1
                .modify(|r, w| w.bits(r.bits() | enAdcTrigSel as u32));
        } else {
            peripheral
                .ADC
                .exttrigger1
                .modify(|r, w| w.bits(r.bits() & !(enAdcTrigSel as u32)));
        }
    }
}

#[doc = "ADC 阈值比较功能配置"]
pub fn Adc_ThresholdCfg(pstcAdcThrCfg: &stc_adc_threshold_cfg) {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral
            .ADC
            .ht
            .write(|w| w.bits(pstcAdcThrCfg.u32AdcHighThd as u32));
        peripheral
            .ADC
            .lt
            .write(|w| w.bits(pstcAdcThrCfg.u32AdcLowThd as u32));
        peripheral
            .ADC
            .cr1
            .write(|w| w.thch().bits(pstcAdcThrCfg.enSampChSel as u8));
        peripheral
            .ADC
            .cr1
            .write(|w| w.regcmp().bit(pstcAdcThrCfg.bAdcRegCmp as u8 != 0));
        peripheral
            .ADC
            .cr1
            .write(|w| w.htcmp().bit(pstcAdcThrCfg.bAdcHtCmp as u8 != 0));
        peripheral
            .ADC
            .cr1
            .write(|w| w.ltcmp().bit(pstcAdcThrCfg.bAdcLtCmp as u8 != 0));
    }
}

#[doc = "ADC 单次转换模式启动"]
pub fn Adc_SGL_Start() {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral.ADC.sglstart.write(|w| w.bits(1));
    }
}
#[doc = "ADC 单次转换模式停止"]
pub fn Adc_SGL_Stop() {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral.ADC.sglstart.write(|w| w.bits(0));
    }
}

#[doc = "ADC 顺序扫描转换模式启动"]
pub fn Adc_SQR_Start() {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral.ADC.sqrstart.write(|w| w.bits(1));
    }
}
#[doc = "ADC 顺序扫描转换模式停止"]
pub fn Adc_SQR_Stop() {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral.ADC.sqrstart.write(|w| w.bits(0));
    }
}

#[doc = "ADC 插队扫描转换模式启动"]
pub fn Adc_JQR_Start() {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral.ADC.jqrstart.write(|w| w.bits(1));
    }
}
#[doc = "ADC 插队扫描转换模式停止"]
pub fn Adc_JQR_Stop() {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    unsafe {
        peripheral.ADC.jqrstart.write(|w| w.bits(0));
    }
}

#[doc = "获取单次转换采样值"]
pub fn Adc_GetSglResult() -> u32 {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    peripheral.ADC.result.read().bits()
}
#[doc = "获取顺序扫描采样值"]
pub fn Adc_GetSqrResult(enstcAdcSqrChMux: en_adc_sqr_chmux) -> u32 {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    match enstcAdcSqrChMux {
        en_adc_sqr_chmux::AdcSQRCH0MUX => peripheral.ADC.sqrresult0.read().bits(),
        en_adc_sqr_chmux::AdcSQRCH1MUX => peripheral.ADC.sqrresult1.read().bits(),
        en_adc_sqr_chmux::AdcSQRCH2MUX => peripheral.ADC.sqrresult2.read().bits(),
        en_adc_sqr_chmux::AdcSQRCH3MUX => peripheral.ADC.sqrresult3.read().bits(),
        en_adc_sqr_chmux::AdcSQRCH4MUX => peripheral.ADC.sqrresult4.read().bits(),
        en_adc_sqr_chmux::AdcSQRCH5MUX => peripheral.ADC.sqrresult5.read().bits(),
        en_adc_sqr_chmux::AdcSQRCH6MUX => peripheral.ADC.sqrresult6.read().bits(),
        en_adc_sqr_chmux::AdcSQRCH7MUX => peripheral.ADC.sqrresult7.read().bits(),
        en_adc_sqr_chmux::AdcSQRCH8MUX => peripheral.ADC.sqrresult8.read().bits(),
        en_adc_sqr_chmux::AdcSQRCH9MUX => peripheral.ADC.sqrresult9.read().bits(),
        en_adc_sqr_chmux::AdcSQRCH10MUX => peripheral.ADC.sqrresult10.read().bits(),
        en_adc_sqr_chmux::AdcSQRCH11MUX => peripheral.ADC.sqrresult11.read().bits(),
        en_adc_sqr_chmux::AdcSQRCH12MUX => peripheral.ADC.sqrresult12.read().bits(),
        en_adc_sqr_chmux::AdcSQRCH13MUX => peripheral.ADC.sqr_result13.read().bits(),
        en_adc_sqr_chmux::AdcSQRCH14MUX => peripheral.ADC.sqrresult14.read().bits(),
        en_adc_sqr_chmux::AdcSQRCH15MUX => peripheral.ADC.sqrresult15.read().bits(),
    }
}
#[doc = "获取插队扫描采样值"]
pub fn Adc_GetJqrResult(enstcAdcJqrChMux: en_adc_jqr_chmux) -> u32 {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    match enstcAdcJqrChMux {
        en_adc_jqr_chmux::AdcJQRCH0MUX => peripheral.ADC.jqrresult0.read().bits(),
        en_adc_jqr_chmux::AdcJQRCH1MUX => peripheral.ADC.jqrresult1.read().bits(),
        en_adc_jqr_chmux::AdcJQRCH2MUX => peripheral.ADC.jqrresult2.read().bits(),
        en_adc_jqr_chmux::AdcJQRCH3MUX => peripheral.ADC.jqrresult3.read().bits(),
    }
}

#[doc = "获取累加采样值"]
pub fn Adc_GetAccResult() -> u32 {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    peripheral.ADC.resultacc.read().bits()
}
#[doc = "ADC 累加寄存器结果清除"]
pub fn Adc_ClrAccResult() {
    let peripheral = hc32l13x_pac::Peripherals::take().unwrap();
    peripheral.ADC.cr1.write(|w| w.raccclr().bit(false))
}
