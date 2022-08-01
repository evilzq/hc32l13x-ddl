use crate::en_result_t;

const ADTIM_HW_STASTPCLR_EN: u8 = 31;
const ADTIM_HW_STASTPCLR_DIS: u32 = 0x7FFFFFFF;
const ADTIM_SS_TIM4: u8 = 1;
const ADTIM_SS_TIM5: u8 = 2;
const ADTIM_SS_TIM6: u8 = 4;
const ADTIM_PORT_BKE_NUM: u8 = 15;

pub enum en_adt_CHxX_port {
    AdtCHxA = 0, //*  CHx A通道
    AdtCHxB = 1, //*  CHx B通道
}

/**
 ******************************************************************************
 ** \brief ADT TRIG端口定义
 *****************************************************************************/
pub enum en_adt_trig_port {
    AdtTrigA = 0, //*  TIMx 触发A端口
    AdtTrigB = 1, //*  TIMx 触发B端口
    AdtTrigC = 2, //*  TIMx 触发C端口
    AdtTrigD = 3, //*  TIMx 触发D端口
}

/**
 ******************************************************************************
 ** \brief ADT通用控制 - Z相输入屏蔽周期数
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_gconr_zmsk {
    AdtZMaskDis = 0,   //*  Z相输入屏蔽功能无效
    AdtZMask4Cyl = 1,  //*  位置计数上溢后或下溢后的4个计数周期内的Z相输入被屏蔽
    AdtZMask8Cyl = 2,  //*  位置计数上溢后或下溢后的8个计数周期内的Z相输入被屏蔽
    AdtZMask16Cyl = 3, //*  位置计数上溢后或下溢后的16个计数周期内的Z相输入被屏蔽
}

/**
 ******************************************************************************
 ** \brief ADT通用控制 - 计数时钟选择
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_cnt_ckdiv {
    AdtClkPClk0 = 0,        //*  PCLK0
    AdtClkPClk0Div2 = 1,    //*  PCLK0/2
    AdtClkPClk0Div4 = 2,    //*  PCLK0/4
    AdtClkPClk0Div8 = 3,    //*  PCLK0/8
    AdtClkPClk0Div16 = 4,   //*  PCLK0/16
    AdtClkPClk0Div64 = 5,   //*  PCLK0/64
    AdtClkPClk0Div256 = 6,  //*  PCLK0/256
    AdtClkPClk0Div1024 = 7, //*  PCLK0/1024
}

/**
 ******************************************************************************
 ** \brief ADT计数模式
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_cnt_mode {
    AdtSawtoothMode = 0,  //*  锯齿波模式
    AdtTriangleModeA = 4, //*  三角波A模式
    AdtTriangleModeB = 5, //*  三角波B模式
}

/**
 ******************************************************************************
 ** \brief ADT计数方向
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_cnt_dir {
    AdtCntDown = 0, //*  递减计数
    AdtCntUp = 1,   //*  递加计数
}

/**
 ******************************************************************************
 ** \brief ADT通用比较基准
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_compare {
    AdtCompareA = 0, //*  通用比较基准A
    AdtCompareB = 1, //*  通用比较基准B
    AdtCompareC = 2, //*  通用比较基准C
    AdtCompareD = 3, //*  通用比较基准D
}

/**
 ******************************************************************************
 ** \brief ADT专用比较基准
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_special_compare {
    AdtSpclCompA = 0, //*  专用比较基准A
    AdtSpclCompB = 1, //*  专用比较基准B
}

/**
 ******************************************************************************
 ** \brief ADT端口控制 - TIMx输出状态控制
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_pconr_disval {
    AdtTIMxDisValNorm = 0, //*  强制输出无效条件0~3中被选择的条件成立时，CHx端口正常输出
    AdtTIMxDisValHiZ = 1,  //*  强制输出无效条件0~3中被选择的条件成立时，CHx端口输出高阻态
    AdtTIMxDisValLow = 2,  //*  强制输出无效条件0~3中被选择的条件成立时，CHx端口输出低电平
    AdtTIMxDisValHigh = 3, //*  强制输出无效条件0~3中被选择的条件成立时，CHx端口输出高电平
}

/**
 ******************************************************************************
 ** \brief ADT端口控制 - CHx强制输出无效条件选择
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_pconr_dissel {
    AdtCHxDisSel0 = 0, //*  选择强制输出无效条件0
    AdtCHxDisSel1 = 1, //*  选择强制输出无效条件1
    AdtCHxDisSel2 = 2, //*  选择强制输出无效条件2
    AdtCHxDisSel3 = 3, //*  选择强制输出无效条件3
}

/**
 ******************************************************************************
 ** \brief ADT端口控制 - CHx周期值匹配时端口状态设定
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_pconr_perc {
    AdtCHxPeriodLow = 0,  //*  计数器计数值与周期值相等时，CHx端口输出保持为低电平
    AdtCHxPeriodHigh = 1, //*  计数器计数值与周期值相等时，CHx端口输出设定为高电平
    AdtCHxPeriodKeep = 2, //*  计数器计数值与周期值相等时，CHx端口输出设定为先前状态
    AdtCHxPeriodInv = 3,  //*  计数器计数值与周期值相等时，CHx端口输出设定为反转电平
}

/**
 ******************************************************************************
 ** \brief ADT端口控制 - CHx比较值匹配时端口状态设定
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_pconr_cmpc {
    AdtCHxCompareLow = 0,  //*  计数器计数值与GCMxR相等时，CHx端口输出保持为低电平
    AdtCHxCompareHigh = 1, //*  计数器计数值与GCMxR相等时，CHx端口输出设定为高电平
    AdtCHxCompareKeep = 2, //*  计数器计数值与GCMxR相等时，CHx端口输出设定为先前状态
    AdtCHxCompareInv = 3,  //*  计数器计数值与GCMxR相等时，CHx端口输出设定为反转电平
}

/**
 ******************************************************************************
 ** \brief ADT端口控制 - CHx端口输出
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_pconr_port_out {
    AdtCHxPortOutLow = 0,  //*  CHx端口输出设定为低电平
    AdtCHxPortOutHigh = 1, //*  CHx端口输出设定为高电平
}

/**
 ******************************************************************************
 ** \brief ADT端口控制 - CHx端口功能模式选择
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_pconr_capc {
    AdtCHxCompareOutput = 0, //*  CHx端口设定为比较输出功能
    AdtCHxCompareInput = 1,  //*  CHx端口设定为捕获输入功能
}

/**
 ******************************************************************************
 ** \brief ADT端口控制 - CHx计数开始停止端口状态选择
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_pconr_stastps {
    AdtCHxStateSelSS = 0,   //*  计数开始或停止时，CHx端口输出由STACB、STPCB决定
    AdtCHxStateSelKeep = 1, //*  计数开始或停止时，CHx端口输出设定为先前状态
}

/**
 ******************************************************************************
 ** \brief ADT死区控制 - CHx死区分离设定
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_dconr_sepa {
    AdtCHxDtSeperate = 0, //*  DTUAR和DTDAR分别设定
    AdtCHxDtEqual = 1,    //*  DTDAR的值和DTUAR的值自动相等
}

/**
 ******************************************************************************
 ** \brief ADT滤波控制 - TRIx/TIMxIx端口滤波采样基准时钟选择
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_fconr_nofick {
    AdtFltClkPclk0 = 0,      //*  PCLK0
    AdtFltClkPclk0Div4 = 1,  //*  PCLK0/4
    AdtFltClkPclk0Div16 = 2, //*  PCLK0/16
    AdtFltClkPclk0Div64 = 3, //*  PCLK0/64
}

/**
 ******************************************************************************
 ** \brief ADT有效周期 - TIMx有效周期选择
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_vperr_pcnts {
    AdtPeriodCnts0 = 0, //*  有效周期选择功能无效
    AdtPeriodCnts1 = 1, //*  每隔1个周期有效一次
    AdtPeriodCnts2 = 2, //*  每隔2个周期有效一次
    AdtPeriodCnts3 = 3, //*  每隔3个周期有效一次
    AdtPeriodCnts4 = 4, //*  每隔4个周期有效一次
    AdtPeriodCnts5 = 5, //*  每隔5个周期有效一次
    AdtPeriodCnts6 = 6, //*  每隔6个周期有效一次
    AdtPeriodCnts7 = 7, //*  每隔7个周期有效一次
}

/**
 ******************************************************************************
 ** \brief ADT有效周期 - 计数条件选择
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_vperr_pcnte {
    AdtPeriodCnteDisable = 0, //*  有效周期选择功能无效
    AdtPeriodCnteMin = 1,     //*  锯齿波计数上、下溢点或三角波波谷做为计数条件
    AdtPeriodCnteMax = 2,     //*  锯齿波计数上、下溢点或三角波波峰做为计数条件
    AdtPeriodCnteBoth = 3,    //*  锯齿波计数上、下溢点或三角波波峰，波谷做为计数条件
}

/**
 ******************************************************************************
 ** \brief ADT端口触发控制 - 触发源选择
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_ttrig_trigxs {
    AdtTrigxSelPA3 = 0,   //*  PA3
    AdtTrigxSelPB3 = 1,   //*  PB3
    AdtTrigxSelPC3 = 2,   //*  PC3
    AdtTrigxSelPD3 = 3,   //*  PD3
    AdtTrigxSelPA7 = 4,   //*  PA7
    AdtTrigxSelPB7 = 5,   //*  PB7
    AdtTrigxSelPC7 = 6,   //*  PC7
    AdtTrigxSelPD7 = 7,   //*  PD7
    AdtTrigxSelPA11 = 8,  //*  PA11
    AdtTrigxSelPB11 = 9,  //*  PB11
    AdtTrigxSelPC11 = 10, //*  PC11
    AdtTrigxSelPD1 = 11,  //*  PD1
    AdtTrigxSelPA15 = 12, //*  PA15
    AdtTrigxSelPB15 = 13, //*  PB15
    AdtTrigxSelPC5 = 14,  //*  PC5
    AdtTrigxSelPD5 = 15,  //*  PD5
}

/**
 ******************************************************************************
 ** \brief ADT AOS触发控制 - AOSx触发源选择
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_itrig_iaosxs {
    AdtAosxTrigSelTim0Int = 0,   //*  TIM0_INT
    AdtAosxTrigSelTim1Int = 1,   //*  TIM1_INT
    AdtAosxTrigSelTim2Int = 2,   //*  TIM2_INT
    AdtAosxTrigSelLpTimInt = 3,  //*  LPTIMER_INT
    AdtAosxTrigSelTim4Int = 4,   //*  TIM4_INT
    AdtAosxTrigSelTim5Int = 5,   //*  TIM5_INT
    AdtAosxTrigSelTim6Int = 6,   //*  TIM6_INT
    AdtAosxTrigSelUart0Int = 7,  //*  UART0_INT
    AdtAosxTrigSelUart1Int = 8,  //*  UART1_INT
    AdtAosxTrigSelLpUartInt = 9, //*  LPUART_INT
    AdtAosxTrigSelVc0Int = 10,   //*  VC0_INT
    AdtAosxTrigSelVc1Int = 11,   //*  VC1_INT
    AdtAosxTrigSelRtcInt = 12,   //*  RTC_INT
    AdtAosxTrigSelPcaInt = 13,   //*  PCA_INT
    AdtAosxTrigSelSpiInt = 14,   //*  SPI_INT
    AdtAosxTrigSelAdcInt = 15,   //*  ADC_INT
}

/**
 ******************************************************************************
 ** \brief ADT硬件(启动/停止/清零/捕获)事件触发选择
 **
 ** \note
 ******************************************************************************/

pub enum en_adt_hw_trig {
    AdtHwTrigAos0 = 0,         //*  从AOS来的事件触发0有效
    AdtHwTrigAos1 = 1,         //*  从AOS来的事件触发1有效
    AdtHwTrigAos2 = 2,         //*  从AOS来的事件触发2有效
    AdtHwTrigAos3 = 3,         //*  从AOS来的事件触发3有效
    AdtHwTrigCHxARise = 4,     //*  CHxA端口上采样到上升沿
    AdtHwTrigCHxAFall = 5,     //*  CHxA端口上采样到下降沿
    AdtHwTrigCHxBRise = 6,     //*  CHxB端口上采样到上升沿
    AdtHwTrigCHxBFall = 7,     //*  CHxB端口上采样到下降沿
    AdtHwTrigTimTriARise = 8,  //*  TIMTRIA端口上采样到上升沿
    AdtHwTrigTimTriAFall = 9,  //*  TIMTRIA端口上采样到下降沿
    AdtHwTrigTimTriBRise = 10, //*  TIMTRIB端口上采样到上升沿
    AdtHwTrigTimTriBFall = 11, //*  TIMTRIB端口上采样到下降沿
    AdtHwTrigTimTriCRise = 12, //*  TIMTRIC端口上采样到上升沿
    AdtHwTrigTimTriCFall = 13, //*  TIMTRIC端口上采样到下降沿
    AdtHwTrigTimTriDRise = 14, //*  TIMTRID端口上采样到上升沿
    AdtHwTrigTimTriDFall = 15, //*  TIMTRID端口上采样到下降沿
    AdtHwTrigEnd = 16,
}

/**
 ******************************************************************************
 ** \brief ADT硬件(递加/递减)事件触发选择
 **
 ** \note
 ******************************************************************************/

pub enum en_adt_hw_cnt {
    AdtHwCntCHxALowCHxBRise = 0, //*  CHxA端口为低电平时，CHxB端口上采样到上升沿
    AdtHwCntCHxALowCHxBFall = 1, //*  CHxA端口为低电平时，CHxB端口上采样到下降沿
    AdtHwCntCHxAHighCHxBRise = 2, //*  CHxA端口为高电平时，CHxB端口上采样到上升沿
    AdtHwCntCHxAHighCHxBFall = 3, //*  CHxA端口为高电平时，CHxB端口上采样到下降沿
    AdtHwCntCHxBLowCHxARise = 4, //*  CHxB端口为低电平时，CHxA端口上采样到上升沿
    AdtHwCntCHxBLowCHxAFall = 5, //*  CHxB端口为低电平时，CHxA端口上采样到下降沿
    AdtHwCntCHxBHighChxARise = 6, //*  CHxB端口为高电平时，CHxA端口上采样到上升沿
    AdtHwCntCHxBHighCHxAFall = 7, //*  CHxB端口为高电平时，CHxA端口上采样到下降沿
    AdtHwCntTimTriARise = 8,     //*  TIMTRIA端口上采样到上升沿
    AdtHwCntTimTriAFall = 9,     //*  TIMTRIA端口上采样到下降沿
    AdtHwCntTimTriBRise = 10,    //*  TIMTRIB端口上采样到上升沿
    AdtHwCntTimTriBFall = 11,    //*  TIMTRIB端口上采样到下降沿
    AdtHwCntTimTriCRise = 12,    //*  TIMTRIC端口上采样到上升沿
    AdtHwCntTimTriCFall = 13,    //*  TIMTRIC端口上采样到下降沿
    AdtHwCntTimTriDRise = 14,    //*  TIMTRID端口上采样到上升沿
    AdtHwCntTimTriDFall = 15,    //*  TIMTRID端口上采样到下降沿
    AdtHwCntAos0 = 16,           //*  从AOS来的事件触发0有效
    AdtHwCntAos1 = 17,           //*  从AOS来的事件触发1有效
    AdtHwCntAos2 = 18,           //*  从AOS来的事件触发2有效
    AdtHwCntAos3 = 19,           //*  从AOS来的事件触发3有效
    AdtHwCntMax = 20,
}

/**
 ******************************************************************************
 ** \brief ADT端口刹车极性控制
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_ptbrk_polarity {
    AdtPtBrkHigh = 0, //*  端口刹车极性高电平有效
    AdtPtBrkLow = 1,  //*  端口刹车极性低电平有效
}

/**
 ******************************************************************************
 ** \brief ADT PWM展频计数选择
 **
 ** \note
 ******************************************************************************/
pub enum en_adt_pwm_dither_type {
    AdtPwmDitherUnderFlow = 0, //*  PWM展频计数下溢出
    AdtPwmDitherOverFlow = 1,  //*  PWM展频计数上溢出
}

/**
 ******************************************************************************
 ** \brief ADT中断类型
 **
 ** \note
 ******************************************************************************/

pub enum en_adt_irq_type {
    AdtCMAIrq = 0,   //*  计数匹配A（或捕获输入）中断
    AdtCMBIrq = 1,   //*  计数匹配B（或捕获输入）中断
    AdtCMCIrq = 2,   //*  计数匹配C中断
    AdtCMDIrq = 3,   //*  计数匹配D中断
    AdtOVFIrq = 6,   //*  上溢匹配中断
    AdtUDFIrq = 7,   //*  下溢匹配中断
    AdtDTEIrq = 8,   //*  死区时间错误中断
    AdtSAMLIrq = 14, //*  同低中断
    AdtSAMHIrq = 15, //*  同高中断
}

pub enum en_adt_state_type {
    AdtCMAF = 0,    //*  计数匹配A标志
    AdtCMBF = 1,    //*  计数匹配B标志
    AdtCMCF = 2,    //*  计数匹配C标志
    AdtCMDF = 3,    //*  计数匹配D标志
    AdtOVFF = 6,    //*  上溢匹配标志
    AdtUDFF = 7,    //*  下溢匹配标志
    AdtDTEF = 8,    //*  死区时间错误标志
    AdtCMSAUF = 9,  //*  向上计数专用比较基准值匹配A标志
    AdtCMSADF = 10, //*  向下计数专用比较基准值匹配B标志
    AdtCMSBUF = 11, //*  向上计数专用比较基准值匹配A标志
    AdtCMSBDF = 12, //*  向下计数专用比较基准值匹配B标志
    AdtCntDir = 31, //*  计数方向
}

/**
 ******************************************************************************
 ** \brief ADT软件同步配置
 ** \note
 ******************************************************************************/
pub struct stc_adt_sw_sync {
    bAdTim4: bool, //*  Timer 4
    bAdTim5: bool, //*  Timer 5
    bAdTim6: bool, //*  Timer 6
}

/**
 ******************************************************************************
 ** \brief ADT AOS触发配置
 ** \note
 ******************************************************************************/
pub struct stc_adt_aos_trig_cfg {
    enAos0TrigSrc: en_adt_itrig_iaosxs, //*  AOS0触发源选择
    enAos2TrigSrc: en_adt_itrig_iaosxs, //*  AOS2触发源选择
    enAos3TrigSrc: en_adt_itrig_iaosxs, //*  AOS3触发源选择
}

/**
 ******************************************************************************
 ** \brief ADT 中断触发配置
 ** \note
 ******************************************************************************/
pub struct stc_adt_irq_trig_cfg {
    bAdtSpecilMatchBTrigDmaEn: bool, //*  专用比较基准值匹配B使能触发DMA
    bAdtSpecilMatchATrigDmaEn: bool, //*  专用比较基准值匹配A使能触发DMA
    bAdtUnderFlowTrigDmaEn: bool,    //*  下溢匹配使能触发DMA
    bAdtOverFlowTrigDmaEn: bool,     //*  上溢匹配使能触发DMA
    bAdtCntMatchDTrigDmaEn: bool,    //*  计数匹配D使能触发DMA
    bAdtCntMatchCTrigDmaEn: bool,    //*  计数匹配C使能触发DMA
    bAdtCntMatchBTrigDmaEn: bool,    //*  计数匹配B使能触发DMA
    bAdtCntMatchATrigDmaEn: bool,    //*  计数匹配A使能触发DMA
    bAdtSpecilMatchBTrigEn: bool,    //*  专用比较基准值匹配B使能触发ADC
    bAdtSpecilMatchATrigEn: bool,    //*  专用比较基准值匹配A使能触发ADC
    bAdtUnderFlowTrigEn: bool,       //*  下溢匹配使能触发ADC
    bAdtOverFlowTrigEn: bool,        //*  上溢匹配使能触发ADC
    bAdtCntMatchDTrigEn: bool,       //*  计数匹配D使能触发ADC
    bAdtCntMatchCTrigEn: bool,       //*  计数匹配C使能触发ADC
    bAdtCntMatchBTrigEn: bool,       //*  计数匹配B使能触发ADC
    bAdtCntMatchATrigEn: bool,       //*  计数匹配A使能触发ADC
}

/**
 ******************************************************************************
 ** \brief ADT Trig端口配置
 ** \note
 ******************************************************************************/
pub struct stc_adt_port_trig_cfg {
    enTrigSrc: en_adt_ttrig_trigxs, //*  触发源选择
    bFltEn: bool,                   //*  触发源捕获输入滤波使能
    enFltClk: en_adt_fconr_nofick,  //*  滤波采样基准时钟
}

/**
 ******************************************************************************
 ** \brief ADT Z相输入屏蔽功能配置
 ** \note
 ******************************************************************************/
pub struct stc_adt_zmask_cfg {
    enZMaskCycle: en_adt_gconr_zmsk, //*  Z相输入屏蔽计数周期选择
    bFltPosCntMaksEn: bool, //*  Z相输入时的屏蔽周期内，位置计数器的清零功能不屏蔽（FALSE）或屏蔽(TRUE)
    bFltRevCntMaksEn: bool, //*  Z相输入时的屏蔽周期内，公转计数器的计数功能不屏蔽（FALSE）或屏蔽(TRUE)
}

/**
 ******************************************************************************
 ** \brief ADT TIMxX端口配置
 ** \note
 ******************************************************************************/
pub struct stc_adt_TIMxX_port_cfg {
    enCap: en_adt_pconr_capc,        //*  端口功能模式
    bOutEn: bool,                    //*  输出使能
    enPerc: en_adt_pconr_perc,       //*  周期值匹配时端口状态
    enCmpc: en_adt_pconr_cmpc,       //*  比较值匹配时端口状态
    enStaStp: en_adt_pconr_stastps,  //*  计数开始停止端口状态选择
    enStaOut: en_adt_pconr_port_out, //*  计数开始端口输出状态
    enStpOut: en_adt_pconr_port_out, //*  计数停止端口输出状态
    enDisVal: en_adt_pconr_disval,   //*  强制输出无效时输出状态控制
    enDisSel: en_adt_pconr_dissel,   //*  强制输出无效条件选择
    bFltEn: bool,                    //*  端口捕获输入滤波使能
    enFltClk: en_adt_fconr_nofick,   //*  端口滤波采样基准时钟
}

/**
 ******************************************************************************
 ** \brief ADT刹车端口配置
 ** \note
 ******************************************************************************/
pub struct stc_adt_break_port_cfg {
    bPortEn: bool,                //*  端口使能
    enPol: en_adt_ptbrk_polarity, //*  极性选择
}

/**
 ******************************************************************************
 ** \brief ADT无效条件3配置
 ** \note
 ******************************************************************************/
pub struct stc_adt_disable_3_cfg {
    stcBrkPtCfg: [stc_adt_break_port_cfg; 16], //*  刹车端口配置
    bFltEn: bool,                              //*  刹车端口滤波使能
    enFltClk: en_adt_fconr_nofick,             //*  滤波采样基准时钟
}

/**
 ******************************************************************************
 ** \brief ADT无效条件1配置
 ** \note
 ******************************************************************************/
pub struct stc_adt_disable_1_cfg {
    bTim6OutSH: bool, //*  TIM6输出同高
    bTim5OutSH: bool, //*  TIM5输出同高
    bTim4OutSH: bool, //*  TIM4输出同高
    bTim6OutSL: bool, //*  TIM6输出同低
    bTim5OutSL: bool, //*  TIM5输出同低
    bTim4OutSL: bool, //*  TIM4输出同低
}

/**
 ******************************************************************************
 ** \brief ADT PWM展频计数配置
 ** \note
 ******************************************************************************/
pub struct stc_adt_pwm_dither_cfg {
    enAdtPDType: en_adt_pwm_dither_type, //*  PWM展频计数选择
    bTimxBPDEn: bool,                    //*  PWM通道B展频使能
    bTimxAPDEn: bool,                    //*  PWM通道A展频使能
}

/**
 ******************************************************************************
 ** \brief ADT基本计数配置
 ** \note
 ******************************************************************************/
pub struct stc_adt_basecnt_cfg {
    enCntMode: en_adt_cnt_mode,    //*  计数模式
    enCntDir: en_adt_cnt_dir,      //*  计数方向
    enCntClkDiv: en_adt_cnt_ckdiv, //*  计数时钟选择
}

/**
 ******************************************************************************
 ** \brief ADT计数状态
 ** \note
 ******************************************************************************/
pub struct stc_adt_cntstate_cfg {
    u16Counter: u16,   //*  当前计数器的计数值
    enCntDir: bool,    //*  计数方向
    u8ValidPeriod: u8, //*  有效周期计数
    bCMSBDF: bool,     //*  向下计数专用比较基准值匹配B标志
    bCMSBUF: bool,     //*  向上计数专用比较基准值匹配A标志
    bCMSADF: bool,     //*  向下计数专用比较基准值匹配B标志
    bCMSAUF: bool,     //*  向上计数专用比较基准值匹配A标志
    bDTEF: bool,       //*  死区时间错误标志
    bUDFF: bool,       //*  下溢匹配标志
    bOVFF: bool,       //*  上溢匹配标志
    bCMDF: bool,       //*  计数匹配D标志
    bCMCF: bool,       //*  计数匹配C标志
    bCMBF: bool,       //*  计数匹配B标志
    bCMAF: bool,       //*  计数匹配A标志
}

/**
 ******************************************************************************
 ** \brief ADT有效计数周期
 ** \note
 ******************************************************************************/
pub struct stc_adt_validper_cfg {
    enValidCnt: en_adt_vperr_pcnts, //*  有效周期选择
    enValidCdt: en_adt_vperr_pcnte, //*  有效周期计数条件
    bPeriodD: bool,                 //*  通用信号有效周期选择D
    bPeriodC: bool,                 //*  通用信号有效周期选择C
    bPeriodB: bool,                 //*  通用信号有效周期选择B
    bPeriodA: bool,                 //*  通用信号有效周期选择A
}

pub enum M0P_ADTIM_TypeDef {
    ADTIM4,
    ADTIM5,
    ADTIM6,
}

//配置硬件递加事件
pub fn Adt_CfgHwCntUp(ADTx: &M0P_ADTIM_TypeDef, enAdtHwCntUp: en_adt_hw_cnt) -> en_result_t {
    en_result_t::Ok
}
//清除硬件递加事件
pub fn Adt_ClearHwCntUp(ADTx: &M0P_ADTIM_TypeDef) -> en_result_t {
    en_result_t::Ok
}
//配置硬件递减事件
pub fn Adt_CfgHwCntDwn(ADTx: &M0P_ADTIM_TypeDef, enAdtHwCntDwn: en_adt_hw_cnt) -> en_result_t {
    en_result_t::Ok
}
//清除硬件递减事件
pub fn Adt_ClearHwCntDwn(ADTx: &M0P_ADTIM_TypeDef) -> en_result_t {
    en_result_t::Ok
}
//配置硬件启动事件
pub fn Adt_CfgHwStart(ADTx: &M0P_ADTIM_TypeDef, enAdtHwStart: en_adt_hw_trig) -> en_result_t {
    en_result_t::Ok
}
//清除硬件启动事件
pub fn Adt_ClearHwStart(ADTx: &M0P_ADTIM_TypeDef) -> en_result_t {
    en_result_t::Ok
}
//使能硬件启动事件
pub fn Adt_EnableHwStart(ADTx: &M0P_ADTIM_TypeDef) -> en_result_t {
    en_result_t::Ok
}
//禁止硬件启动事件
pub fn Adt_DisableHwStart(ADTx: &M0P_ADTIM_TypeDef) -> en_result_t {
    en_result_t::Ok
}
//配置硬件停止事件
pub fn Adt_CfgHwStop(ADTx: &M0P_ADTIM_TypeDef, enAdtHwStop: en_adt_hw_trig) -> en_result_t {
    en_result_t::Ok
}
//清除硬件停止事件
pub fn Adt_ClearHwStop(ADTx: &M0P_ADTIM_TypeDef) -> en_result_t {
    en_result_t::Ok
}
//使能硬件停止事件
pub fn Adt_EnableHwStop(ADTx: &M0P_ADTIM_TypeDef) -> en_result_t {
    en_result_t::Ok
}
//禁止硬件停止事件
pub fn Adt_DisableHwStop(ADTx: &M0P_ADTIM_TypeDef) -> en_result_t {
    en_result_t::Ok
}
//配置硬件清零事件
pub fn Adt_CfgHwClear(ADTx: &M0P_ADTIM_TypeDef, enAdtHwClear: en_adt_hw_trig) -> en_result_t {
    en_result_t::Ok
}
//清除硬件清零事件
pub fn Adt_ClearHwClear(ADTx: &M0P_ADTIM_TypeDef) -> en_result_t {
    en_result_t::Ok
}
//使能硬件清零事件
pub fn Adt_EnableHwClear(ADTx: &M0P_ADTIM_TypeDef) -> en_result_t {
    en_result_t::Ok
}
//禁止硬件清零事件
pub fn Adt_DisableHwClear(ADTx: &M0P_ADTIM_TypeDef) -> en_result_t {
    en_result_t::Ok
}
//配置A通道硬件捕获事件
pub fn Adt_CfgHwCaptureA(ADTx: &M0P_ADTIM_TypeDef, enAdtHwCaptureA: en_adt_hw_trig) -> en_result_t {
    en_result_t::Ok
}
//清除A通道硬件捕获事件
pub fn Adt_ClearHwCaptureA(ADTx: &M0P_ADTIM_TypeDef) -> en_result_t {
    en_result_t::Ok
}
//配置B通道硬件捕获事件
pub fn Adt_CfgHwCaptureB(ADTx: &M0P_ADTIM_TypeDef, enAdtHwCaptureB: en_adt_hw_trig) -> en_result_t {
    en_result_t::Ok
}
//清除B通道硬件捕获事件
pub fn Adt_ClearHwCaptureB(ADTx: &M0P_ADTIM_TypeDef) -> en_result_t {
    en_result_t::Ok
}
//软件同步启动
pub fn Adt_SwSyncStart(pstcAdtSwSyncStart: &stc_adt_sw_sync) -> en_result_t {
    en_result_t::Ok
}
//软件同步停止
pub fn Adt_SwSyncStop(pstcAdtSwSyncStop: &stc_adt_sw_sync) -> en_result_t {
    en_result_t::Ok
}
//软件同步清零
pub fn Adt_SwSyncClear(pstcAdtSwSyncClear: &stc_adt_sw_sync) -> en_result_t {
    en_result_t::Ok
}
//获取软件同步状态
pub fn Adt_GetSwSyncState(pstcAdtSwSyncState: &stc_adt_sw_sync) -> en_result_t {
    en_result_t::Ok
}
//AOS触发配置
pub fn Adt_AosTrigCfg(pstcAdtAosTrigCfg: &stc_adt_aos_trig_cfg) -> en_result_t {
    en_result_t::Ok
}
//中断触发配置
pub fn Adt_IrqTrigCfg(
    ADTx: &M0P_ADTIM_TypeDef,
    pstcAdtIrqTrigCfg: &stc_adt_irq_trig_cfg,
) -> en_result_t {
    en_result_t::Ok
}
//端口触发配置
pub fn Adt_PortTrigCfg(
    enAdtTrigPort: en_adt_trig_port,
    pstcAdtPortTrigCfg: &stc_adt_port_trig_cfg,
) -> en_result_t {
    en_result_t::Ok
}
//CHxX端口配置
pub fn Adt_CHxXPortCfg(
    ADTx: &M0P_ADTIM_TypeDef,
    enAdtCHxXPort: en_adt_CHxX_port,
    pstcAdtCHxXCfg: &stc_adt_TIMxX_port_cfg,
) -> en_result_t {
    en_result_t::Ok
}
//使能端口刹车
pub fn Adt_EnableBrakePort(port: u8, pstcAdtBrkPtCfg: &stc_adt_break_port_cfg) -> en_result_t {
    en_result_t::Ok
}
//清除端口刹车
pub fn Adt_ClearBrakePort() {}
//无效条件3配置(端口刹车)
pub fn Adt_Disable3Cfg(pstcAdtDisable3: &stc_adt_disable_3_cfg) -> en_result_t {
    en_result_t::Ok
}
//软件刹车 Enable/Disable(仅适用于无效条件3使能的情况下)
pub fn Adt_SwBrake(bSwBrk: bool) -> en_result_t {
    en_result_t::Ok
}
//获取端口刹车标志
pub fn Adt_GetPortBrakeFlag() -> bool {
    true
}
//清除端口刹车标志
pub fn Adt_ClearPortBrakeFlag() {}
//无效条件1配置(同高同低刹车)
pub fn Adt_Disable1Cfg(pstcAdtDisable1: &stc_adt_disable_1_cfg) -> en_result_t {
    en_result_t::Ok
}
//获取同高同低刹车标志
pub fn Adt_GetSameBrakeFlag() -> bool {
    true
}
//清除同高同低刹车标志
pub fn Adt_ClearSameBrakeFlag() {}
//PWM展频配置
pub fn Adt_PwmDitherCfg(
    ADTx: &M0P_ADTIM_TypeDef,
    pstcAdtPwmDitherCfg: &stc_adt_pwm_dither_cfg,
) -> en_result_t {
    en_result_t::Ok
}
//AdvTimer初始化
pub fn Adt_Init(ADTx: &M0P_ADTIM_TypeDef, pstcAdtBaseCntCfg: &stc_adt_basecnt_cfg) -> en_result_t {
    en_result_t::Ok
}
//AdvTimer去初始化
pub fn Adt_DeInit(ADTx: &M0P_ADTIM_TypeDef) -> en_result_t {
    en_result_t::Ok
}
//AdvTimert启动
pub fn Adt_StartCount(ADTx: &M0P_ADTIM_TypeDef) -> en_result_t {
    en_result_t::Ok
}
//AdvTimert停止
pub fn Adt_StopCount(ADTx: &M0P_ADTIM_TypeDef) -> en_result_t {
    en_result_t::Ok
}
//设置计数值
pub fn Adt_SetCount(ADTx: &M0P_ADTIM_TypeDef, u16Value: u16) -> en_result_t {
    en_result_t::Ok
}
//获取计数值
pub fn Adt_GetCount(ADTx: &M0P_ADTIM_TypeDef) -> u16 {
    0
}
//清除计数值
pub fn Adt_ClearCount(ADTx: &M0P_ADTIM_TypeDef) -> en_result_t {
    en_result_t::Ok
}
//获取有效周期计数值
pub fn Adt_GetVperNum(ADTx: &M0P_ADTIM_TypeDef) -> u8 {
    0
}
//获取状态标志
pub fn Adt_GetState(ADTx: &M0P_ADTIM_TypeDef, enstate: en_adt_state_type) -> bool {
    true
}
//配置计数周期
pub fn Adt_SetPeriod(ADTx: &M0P_ADTIM_TypeDef, u16Period: u16) -> en_result_t {
    en_result_t::Ok
}
//配置计数周期缓冲
pub fn Adt_SetPeriodBuf(ADTx: &M0P_ADTIM_TypeDef, u16PeriodBuf: u16) -> en_result_t {
    en_result_t::Ok
}
//清除计数周期缓冲
pub fn Adt_ClearPeriodBuf(ADTx: &M0P_ADTIM_TypeDef) -> en_result_t {
    en_result_t::Ok
}
//配置有效计数周期
pub fn Adt_SetValidPeriod(
    ADTx: &M0P_ADTIM_TypeDef,
    pstcAdtValidPerCfg: &stc_adt_validper_cfg,
) -> en_result_t {
    en_result_t::Ok
}
//配置比较输出计数基准值
pub fn Adt_SetCompareValue(
    ADTx: &M0P_ADTIM_TypeDef,
    enAdtCompare: en_adt_compare,
    u16Compare: u16,
) -> en_result_t {
    en_result_t::Ok
}
//配置通用比较值/捕获值的缓存传送
pub fn Adt_EnableValueBuf(
    ADTx: &M0P_ADTIM_TypeDef,
    enAdtCHxXPort: en_adt_CHxX_port,
    bCompareBufEn: bool,
) -> en_result_t {
    en_result_t::Ok
}
//清除比较输出计数值/捕获值缓存
pub fn Adt_ClearValueBuf(ADTx: &M0P_ADTIM_TypeDef, enAdtCHxXPort: en_adt_CHxX_port) -> en_result_t {
    en_result_t::Ok
}
//获取捕获值
pub fn Adt_GetCaptureValue(
    ADTx: &M0P_ADTIM_TypeDef,
    enAdtCHxXPort: en_adt_CHxX_port,
    pu16Capture: &mut u16,
) -> en_result_t {
    en_result_t::Ok
}
//获取捕获缓存值
pub fn Adt_GetCaptureBuf(
    ADTx: &M0P_ADTIM_TypeDef,
    enAdtCHxXPort: en_adt_CHxX_port,
    pu16CaptureBuf: &mut u16,
) -> en_result_t {
    en_result_t::Ok
}
//设置死区时间上基准值
pub fn Adt_SetDTUA(ADTx: &M0P_ADTIM_TypeDef, u16Value: u16) -> en_result_t {
    en_result_t::Ok
}
//设置死区时间下基准值
pub fn Adt_SetDTDA(ADTx: &M0P_ADTIM_TypeDef, u16Value: u16) -> en_result_t {
    en_result_t::Ok
}
//配置死区时间功能
pub fn Adt_CfgDT(ADTx: &M0P_ADTIM_TypeDef, bDTEn: bool, bEqual: bool) -> en_result_t {
    en_result_t::Ok
}
//配置中断
pub fn Adt_CfgIrq(ADTx: &M0P_ADTIM_TypeDef, enAdtIrq: en_adt_irq_type, bEn: bool) -> en_result_t {
    en_result_t::Ok
}
//获取中断标志
pub fn Adt_GetIrqFlag(ADTx: &M0P_ADTIM_TypeDef, enAdtIrq: en_adt_irq_type) -> bool {
    true
}
//清除中断标志
pub fn Adt_ClearIrqFlag(ADTx: &M0P_ADTIM_TypeDef, enAdtIrq: en_adt_irq_type) -> en_result_t {
    en_result_t::Ok
}
//清除所有中断标志
pub fn Adt_ClearAllIrqFlag(ADTx: &M0P_ADTIM_TypeDef) -> en_result_t {
    en_result_t::Ok
}
//Z相输入屏蔽设置
pub fn Adt_CfgZMask(ADTx: &M0P_ADTIM_TypeDef, pstcAdtZMaskCfg: &stc_adt_zmask_cfg) -> en_result_t {
    en_result_t::Ok
}
