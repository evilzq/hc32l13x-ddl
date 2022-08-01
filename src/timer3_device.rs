use crate::en_result_t;

pub enum en_tim3_channel {
    Tim3CH0 = 0, //* Timer3通道0
    Tim3CH1 = 1, //* Timer3通道1
    Tim3CH2 = 2, //* Timer3通道2
}

/**
 ******************************************************************************
 ** \brief 工作模式选择数据类型重定义 (MODE)(模式0/1/23)
 *****************************************************************************/
pub enum en_tim3_work_mode {
    Tim3WorkMode0 = 0, //* 定时器模式
    Tim3WorkMode1 = 1, //* PWC模式
    Tim3WorkMode2 = 2, //* 锯齿波模式
    Tim3WorkMode3 = 3, //* 三角波模式
}

/**
 ******************************************************************************
 ** \brief 极性控制数据类型重定义 (GATE_P)(模式0)
 *****************************************************************************/
pub enum en_tim3_m0cr_gatep {
    Tim3GatePositive = 0, //* 高电平有效
    Tim3GateOpposite = 1, //* 低电平有效
}

/**
 ******************************************************************************
 ** \brief TIM3 预除频选择 (PRS)(模式0/1/23)
 *****************************************************************************/
pub enum en_tim3_cr_timclkdiv {
    Tim3PCLKDiv1 = 0,   //* Div 1
    Tim3PCLKDiv2 = 1,   //* Div 2
    Tim3PCLKDiv4 = 2,   //* Div 4
    Tim3PCLKDiv8 = 3,   //* Div 8
    Tim3PCLKDiv16 = 4,  //* Div 16
    Tim3PCLKDiv32 = 5,  //* Div 32
    Tim3PCLKDiv64 = 6,  //* Div 64
    Tim3PCLKDiv256 = 7, //* Div 256
}

/**
 ******************************************************************************
 ** \brief 计数/定时器功能选择数据类型重定义 (CT)(模式0/1/23)
 *****************************************************************************/
pub enum en_tim3_cr_ct {
    Tim3Timer = 0,   //* 定时器功能，计数时钟为内部PCLK
    Tim3Counter = 1, //* 计数器功能，计数时钟为外部ETR
}

/**
 ******************************************************************************
 ** \brief 定时器工作模式数据类型重定义 (MD)(模式0)
 *****************************************************************************/
pub enum en_tim3_m0cr_md {
    Tim332bitFreeMode = 0, //* 32位计数器/定时器
    Tim316bitArrMode = 1,  //* 自动重装载16位计数器/定时器
}

/**
 ******************************************************************************
** \brief TIM3中断类型数据类型重定义(模式0/1/23)
 *****************************************************************************/
pub enum en_tim3_irq_type {
    Tim3UevIrq = 0,   //* 溢出/事件更新中断
    Tim3CA0Irq = 2,   //* CH0A捕获/比较中断(仅模式1/23存在)
    Tim3CA1Irq = 3,   //* CH1A捕获/比较中断(仅模式23存在)
    Tim3CA2Irq = 4,   //* CH2A捕获/比较中断(仅模式23存在)
    Tim3CB0Irq = 5,   //* CH0B捕获/比较中断(仅模式23存在)
    Tim3CB1Irq = 6,   //* CH1B捕获/比较中断(仅模式23存在)
    Tim3CB2Irq = 7,   //* CH2B捕获/比较中断(仅模式23存在)
    Tim3CA0E = 8,     //* CH0A捕获数据丢失标志(仅模式23存在)(不是中断)
    Tim3CA1E = 9,     //* CH1A捕获数据丢失标志(仅模式23存在)(不是中断)
    Tim3CA2E = 10,    //* CH2A捕获数据丢失标志(仅模式23存在)(不是中断)
    Tim3CB0E = 11,    //* CH0B捕获数据丢失标志(仅模式23存在)(不是中断)
    Tim3CB1E = 12,    //* CH1B捕获数据丢失标志(仅模式23存在)(不是中断)
    Tim3CB2E = 13,    //* CH2B捕获数据丢失标志(仅模式23存在)(不是中断)
    Tim3BkIrq = 14,   //* 刹车中断(仅模式23存在）
    Tim3TrigIrq = 15, //* 触发中断(仅模式23存在）
}

/**
 ******************************************************************************
 ** \brief 测量开始结束数据类型重定义 (Edg1stEdg2nd)(模式1)
 *****************************************************************************/
pub enum en_tim3_m1cr_Edge {
    Tim3PwcRiseToRise = 0, //* 上升沿到上升沿(周期)
    Tim3PwcFallToRise = 1, //* 下降沿到上升沿(低电平)
    Tim3PwcRiseToFall = 2, //* 上升沿到下降沿(高电平)
    Tim3PwcFallToFall = 3, //* 下降沿到下降沿(周期)
}

/**
 ******************************************************************************
 ** \brief PWC测量测试模式选择数据类型重定义 (Oneshot)(模式1)
 *****************************************************************************/
pub enum en_tim3_m1cr_oneshot {
    Tim3PwcCycleDetect = 0,   //* PWC循环测量
    Tim3PwcOneShotDetect = 1, //* PWC单次测量
}

/**
 ******************************************************************************
 ** \brief PWC IA0选择数据类型重定义 (IA0S)(模式1)
 *****************************************************************************/
pub enum en_tim3_m1_mscr_ia0s {
    Tim3IA0Input = 0, //* IAO输入
    Tim3XORInput = 1, //* IA0 ETR GATE XOR(TIM0/1/2)/IA0 IA1 IA2 XOR(TIM3)
}

/**
 ******************************************************************************
 ** \brief PWC IB0选择数据类型重定义 (IA0S)(模式1)
 *****************************************************************************/
pub enum en_tim3_m1_mscr_ib0s {
    Tim3IB0Input = 0, //* IBO输入
    Tim3TsInput = 1,  //* 内部触发TS选择信号
}

/**
 ******************************************************************************
 ** \brief 输出极性、输入相位 数据类型重定义 (CCPA0/CCPB0/ETP/BKP)(模式1/23)
 *****************************************************************************/
pub enum en_tim3_port_polarity {
    Tim3PortPositive = 0, //* 正常输入输出
    Tim3PortOpposite = 1, //* 反向输入输出
}

/**
 ******************************************************************************
 ** \brief 滤波选择数据类型重定义 (FLTET/FLTA0/FLAB0)(模式1/23)
 *****************************************************************************/
pub enum en_tim3_flt {
    Tim3FltNone = 0,          //* 无滤波
    Tim3FltPCLKCnt3 = 4,      //* PCLK 3个连续有效
    Tim3FltPCLKDiv4Cnt3 = 5,  //* PCLK/4 3个连续有效
    Tim3FltPCLKDiv16Cnt3 = 6, //* PCLK/16 3个连续有效
    Tim3FltPCLKDiv64Cnt3 = 7, //* PCLK/64 3个连续有效
}

/**
 ******************************************************************************
 ** \brief 通道比较控制 数据类型重定义 (OCMA/OCMB)(模式23)
 *****************************************************************************/
pub enum en_tim3_m23_fltr_ocm {
    Tim3ForceLow = 0,      //* 强制为0
    Tim3ForceHigh = 1,     //* 强制为1
    Tim3CMPForceLow = 2,   //* 比较匹配时强制为0
    Tim3CMPForceHigh = 3,  //* 比较匹配时强制为1
    Tim3CMPInverse = 4,    //* 比较匹配时翻转电平
    Tim3CMPOnePrdHigh = 5, //* 比较匹配时输出一个计数周期的高电平
    Tim3PWMMode1 = 6,      //* 通道控制为PWM mode 1
    Tim3PWMMode2 = 7,      //* 通道控制为PWM mode 2
}

/**
 ******************************************************************************
 ** \brief 主从模式TS数据类型重定义 (TS)(模式1/23)
 *****************************************************************************/
pub enum en_tim3_mscr_ts {
    Tim3Ts0ETR = 0,      //* ETR外部输入滤波后的相位选择信号
    Tim3Ts1TIM0TRGO = 1, //* Timer0的TRGO输出信号
    Tim3Ts2TIM1TRGO = 2, //* Timer1的TRGO输出信号
    Tim3Ts3TIM2TRGO = 3, //* Timer2的TRGO输出信号
    Tim3Ts4TIM3TRGO = 4, //* Timer3的TRGO输出信号
    //Tim3Ts5IA0ED    = 5,         //* 无效
    Tim3Ts6IAFP = 6, //* CH0A 外部输输入滤波后的相位选择信号
    Tim3Ts7IBFP = 7, //* CH0B 外部输输入滤波后的相位选择信
}

/**
 ******************************************************************************
 ** \brief PWM输出模式选择数据类型重定义 (COMP)(模式23)
 *****************************************************************************/
pub enum en_tim3_m23cr_comp {
    Tim3IndependentPWM = 0,   //* 独立PWM输出
    Tim3ComplementaryPWM = 1, //* 互补PWM输出
}

/**
 ******************************************************************************
 ** \brief 计数方向选择数据类型重定义 (DIR)(模式23)
 *****************************************************************************/
pub enum en_tim3_m23cr_dir {
    Tim3CntUp = 0,   //* 向上计数
    Tim3CntDown = 1, //* 向下计数
}

/**
 ******************************************************************************
 ** \brief 计数方向选择数据类型重定义 (PWM2S)(模式23)
 *****************************************************************************/
pub enum en_tim3_m23cr_pwm2s {
    Tim3DoublePointCmp = 0, //* 双点比较使能，使用CCRA,CCRB比较控制OCREFA输出
    Tim3SinglePointCmp = 1, //* 单点比较使能，使用CCRA比较控制OCREFA输出
}

/**
 ******************************************************************************
 ** \brief GATE在PWM互补模式下捕获或比较功能 选择数据类型重定义 (CSG)(模式23)
 *****************************************************************************/
pub enum en_tim3_m23cr_csg {
    Tim3PWMCompGateCmpOut = 0, //* 在PWM互补模式下，Gate作为比较输出
    Tim3PWMCompGateCapIn = 1,  //* 在PWM互补模式下，Gate作为捕获输入
}

/**
 ******************************************************************************
 ** \brief 比较捕获寄存器 数据类型重定义 (CCR0A,CCR0B)(模式23)
 *****************************************************************************/
pub enum en_tim3_m23_ccrx {
    Tim3CCR0A = 0, //* CCR0A比较捕获寄存器
    Tim3CCR0B = 1, //* CCR0B比较捕获寄存器
    Tim3CCR1A = 2, //* CCR1A比较捕获寄存器
    Tim3CCR1B = 3, //* CCR1B比较捕获寄存器
    Tim3CCR2A = 4, //* CCR2A比较捕获寄存器
    Tim3CCR2B = 5, //* CCR2B比较捕获寄存器
}

/**
 ******************************************************************************
 ** \brief  OCREF清除源 选择数据类型重定义 (OCCS)(模式23)
 *****************************************************************************/
pub enum en_tim3_m23ce_occs {
    Tim3OC_Ref_Clr = 0, //* 来自VC的OC_Ref_Clr
    Tim3ETRf = 1,       //* 外部ETRf
}

/**
 ******************************************************************************
 ** \brief  比较匹配中断模式 选择数据类型重定义 (CIS/CISB)(模式23)
 *****************************************************************************/
pub enum en_tim3_m23_cisa_cisb {
    Tim3CmpIntNone = 0,     //* 无比较匹配中断
    Tim3CmpIntRise = 1,     //* 比较匹配上升沿中断
    Tim3CmpIntFall = 2,     //* 比较匹配下降沿中断
    Tim3CmpIntRiseFall = 3, //* 比较匹配上升沿下降沿中断
}

/**
 ******************************************************************************
 ** \brief TIM3端口控制 - 刹车时CHx输出状态控制(BKSA/BKSB)(模式23)
 **
 ** \note
 ******************************************************************************/
pub enum en_tim3_m23_crchx_bks {
    Tim3CHxBksHiZ = 0,  //* 刹车使能时，CHx端口输出高阻态
    Tim3CHxBksNorm = 1, //* 刹车使能时，CHx端口正常输出
    Tim3CHxBksLow = 2,  //* 刹车使能时，CHx端口输出低电平
    Tim3CHxBksHigh = 3, //* 刹车使能时，CHx端口输出高电平
}

/**
 ******************************************************************************
** \brief TIM3端口控制 - CHx上升沿下降沿捕获(CRx/CFx)(模式23)
 **
 ** \note
 ******************************************************************************/
pub enum en_tim3_m23_crch0_cfx_crx {
    Tim3CHxCapNone = 0,     //* CHx通道捕获禁止
    Tim3CHxCapRise = 1,     //* CHx通道上升沿捕获使能
    Tim3CHxCapFall = 2,     //* CHx通道下降沿捕获使能
    Tim3CHxCapFallRise = 3, //* CHx通道上升沿下降沿捕获都使能
}

/**
 ******************************************************************************
** \brief TIM3端口控制 - CHx比较捕获模式(CSA/CSB)(模式23)
 **
 ** \note
 ******************************************************************************/
pub enum en_tim3_m23_crch0_csa_csb {
    Tim3CHxCmpMode = 0, //* CHx通道设置为比较模式
    Tim3CHxCapMode = 1, //* CHx通道设置为捕获模式
}

/**
 ******************************************************************************
 ** \brief  比较模式下 DMA比较触发选择 数据类型重定义 (CCDS)(模式23)
 *****************************************************************************/
pub enum en_tim3_m23_mscr_ccds {
    Tim3CmpTrigDMA = 0, //* 比较匹配触发DMA
    Tim3UEVTrigDMA = 1, //* 事件更新代替比较匹配触发DMA
}

/**
 ******************************************************************************
 ** \brief  主从模式选择 数据类型重定义 (MSM)(模式23)
 *****************************************************************************/
pub enum en_tim3_m23_mscr_msm {
    Tim3SlaveMode = 0,  //* 从模式
    Tim3MasterMode = 1, //* 主模式
}

/**
 ******************************************************************************
 ** \brief  触发主模式输出源 数据类型重定义 (MMS)(模式23)
 *****************************************************************************/
pub enum en_tim3_m23_mscr_mms {
    Tim3MasterUG = 0,      //* UG(软件更新)源
    Tim3MasterCTEN = 1,    //* CTEN源
    Tim3MasterUEV = 2,     //* UEV更新源
    Tim3MasterCMPSO = 3,   //* 比较匹配选择输出源
    Tim3MasterOCA0Ref = 4, //* OCA0_Ref源
    Tim3MasterOCB0Ref = 5, //* OCB0_Ref源
                           //Tim3MasterOCB0Ref  = 6,
                           //Tim3MasterOCB0Ref  = 7,
}

/**
 ******************************************************************************
 ** \brief  触发从模式选择 数据类型重定义 (SMS)(模式23)
 *****************************************************************************/
pub enum en_tim3_m23_mscr_sms {
    Tim3SlaveIClk = 0,     //* 使用内部时钟
    Tim3SlaveResetTIM = 1, //* 复位功能
    Tim3SlaveTrigMode = 2, //* 触发模式
    Tim3SlaveEClk = 3,     //* 外部时钟模式
    Tim3SlaveCodeCnt1 = 4, //* 正交编码计数模式1
    Tim3SlaveCodeCnt2 = 5, //* 正交编码计数模式2
    Tim3SlaveCodeCnt3 = 6, //* 正交编码计数模式3
    Tim3SlaveGateCtrl = 7, //* 门控功能
}

/**
 ******************************************************************************
 ** \brief 定时器运行控制数据类型重定义 (CTEN)
 *****************************************************************************/
pub enum en_tim3_start {
    Tim3CTENDisable = 0, //* 停止
    Tim3CTENEnable = 1,  //* 运行
}

/**
 ******************************************************************************
 ** \brief TIM3 mode0 配置结构体定义(模式0)
 *****************************************************************************/
pub struct stc_tim3_mode0_cfg {
    enWorkMode: en_tim3_work_mode, //* 工作模式设置
    enGateP: en_tim3_m0cr_gatep,   //* 门控极性控制
    bEnGate: bool,                 //* 门控使能
    enPRS: en_tim3_cr_timclkdiv,   //* 预除频配置
    bEnTog: bool,                  //* 翻转输出使能
    enCT: en_tim3_cr_ct,           //* 定时/计数功能选择
    enCntMode: en_tim3_m0cr_md,    //* 计数模式配置
}

/**
 ******************************************************************************
 ** \brief TIM3 mode1 配置结构体定义(模式1)
 *****************************************************************************/
pub struct stc_tim3_mode1_cfg {
    enWorkMode: en_tim3_work_mode,   //* 工作模式设置
    enPRS: en_tim3_cr_timclkdiv,     //* 预除频配置
    enCT: en_tim3_cr_ct,             //* 定时/计数功能选择
    enOneShot: en_tim3_m1cr_oneshot, //* 单次测量/循环测量选择
}

/**
 ******************************************************************************
 ** \brief PWC输入配置结构体定义(模式1)
 *****************************************************************************/
pub struct stc_tim3_pwc_input_cfg {
    enTsSel: en_tim3_mscr_ts,          //* 触发输入源选择
    enIA0Sel: en_tim3_m1_mscr_ia0s,    //* CHA0输入选择
    enIB0Sel: en_tim3_m1_mscr_ib0s,    //* CHB0输入选择
    enETRPhase: en_tim3_port_polarity, //* ETR相位选择
    enFltETR: en_tim3_flt,             //* ETR滤波设置
    enFltIA0: en_tim3_flt,             //* CHA0滤波设置
    enFltIB0: en_tim3_flt,             //* CHB0滤波设置
}

/**
 ******************************************************************************
 ** \brief TIM3 mode23 配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_tim3_mode23_cfg {
    enWorkMode: en_tim3_work_mode,    //* 工作模式设置
    enCntDir: en_tim3_m23cr_dir,      //* 计数方向
    enPRS: en_tim3_cr_timclkdiv,      //* 时钟预除频配置
    enCT: en_tim3_cr_ct,              //* 定时/计数功能选择
    enPWMTypeSel: en_tim3_m23cr_comp, //* PWM模式选择（独立/互补）
    enPWM2sSel: en_tim3_m23cr_pwm2s,  //* OCREFA双点比较功能选择
    bOneShot: bool,                   //* 单次触发模式使能/禁止
    bURSSel: bool,                    //* 更新源选择
}

/**
 ******************************************************************************
 ** \brief GATE在PWM互补模式下捕获或比较功能 配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_tim3_m23_gate_cfg {
    enGateFuncSel: en_tim3_m23cr_csg, //* Gate比较、捕获功能选择
    bGateRiseCap: bool,               //* GATE作为捕获功能时，上沿捕获有效控制
    bGateFallCap: bool,               //* GATE作为捕获功能时，下沿捕获有效控制
}

/**
 ******************************************************************************
 ** \brief CHA/CHB通道比较控制 配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_tim3_m23_compare_cfg {
    enCHxACmpCap: en_tim3_m23_crch0_csa_csb, //* CH0A比较/捕获功能选择
    enCHxACmpCtrl: en_tim3_m23_fltr_ocm,     //* CH0A通道比较控制
    enCHxAPolarity: en_tim3_port_polarity,   //* CH0A输出极性控制
    bCHxACmpBufEn: bool,                     //* 比较A缓存功能 使能/禁止
    enCHxACmpIntSel: en_tim3_m23_cisa_cisb,  //* CHA比较匹配中断选择

    enCHxBCmpCap: en_tim3_m23_crch0_csa_csb, //* CH0B比较/捕获功能选择
    enCHxBCmpCtrl: en_tim3_m23_fltr_ocm,     //* CH0B通道比较控制
    enCHxBPolarity: en_tim3_port_polarity,   //* CH0B输出极性控制
    bCHxBCmpBufEn: bool,                     //* 比较B缓存功能 使能/禁止
    enCHxBCmpIntSel: en_tim3_m23_cisa_cisb,  //* CHB0比较匹配中断选择
}

/**
 ******************************************************************************
 ** \brief CHA/CHB通道捕获控制 配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_tim3_m23_input_cfg {
    enCHxACmpCap: en_tim3_m23_crch0_csa_csb, //* CH0A比较/捕获功能选择
    enCHxACapSel: en_tim3_m23_crch0_cfx_crx, //* CH0A捕获边沿选择
    enCHxAInFlt: en_tim3_flt,                //* CH0A通道捕获滤波控制
    enCHxAPolarity: en_tim3_port_polarity,   //* CH0A输入相位

    enCHxBCmpCap: en_tim3_m23_crch0_csa_csb, //* CH0A比较/捕获功能选择
    enCHxBCapSel: en_tim3_m23_crch0_cfx_crx, //* CH0B捕获边沿选择
    enCHxBInFlt: en_tim3_flt,                //* CH0B通道捕获滤波控制
    enCHxBPolarity: en_tim3_port_polarity,   //* CH0B输入相位
}

/**
 ******************************************************************************
 ** \brief ETR输入相位滤波配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_tim3_m23_etr_input_cfg {
    enETRPolarity: en_tim3_port_polarity, //* ETR输入极性设置
    enETRFlt: en_tim3_flt,                //* ETR滤波设置
}

/**
 ******************************************************************************
 ** \brief 刹车BK输入相位滤波配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_tim3_m23_bk_input_cfg {
    bEnBrake: bool,                         //* 刹车使能
    bEnVC0Brake: bool,                      //* 使能VC0刹车
    bEnVC1Brake: bool,                      //* 使能VC1刹车
    bEnSafetyBk: bool,                      //* 使能safety刹车
    bEnBKSync: bool,                        //* TIM0/TIM1/TIM2刹车同步使能
    enBkCH0AStat: en_tim3_m23_crchx_bks,    //* 刹车时CHA端口状态设置
    enBkCH0BStat: en_tim3_m23_crchx_bks,    //* 刹车时CHB端口状态设置
    enBkCH1AStat: en_tim3_m23_crchx_bks,    //* 刹车时CHA端口状态设置
    enBkCH1BStat: en_tim3_m23_crchx_bks,    //* 刹车时CHB端口状态设置
    enBkCH2AStat: en_tim3_m23_crchx_bks,    //* 刹车时CHA端口状态设置
    enBkCH2BStat: en_tim3_m23_crchx_bks,    //* 刹车时CHB端口状态设置
    enBrakePolarity: en_tim3_port_polarity, //* 刹车BK输入极性设置
    enBrakeFlt: en_tim3_flt,                //* 刹车BK滤波设置
}

/**
 ******************************************************************************
** \brief 死区功能配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_tim3_m23_dt_cfg {
    bEnDeadTime: bool,   //* 刹车时CHA端口状态设置
    u8DeadTimeValue: u8, //* 刹车时CHA端口状态设置
}

/**
 ******************************************************************************
 ** \brief 触发ADC配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_tim3_m23_adc_trig_cfg {
    bEnTrigADC: bool,        //* 触发ADC全局控制
    bEnUevTrigADC: bool,     //* 事件更新触发ADC
    bEnCH0ACmpTrigADC: bool, //* CH0A比较匹配触发ADC
    bEnCH0BCmpTrigADC: bool, //* CH0B比较匹配触发ADC
    bEnCH1ACmpTrigADC: bool, //* CH0A比较匹配触发ADC
    bEnCH1BCmpTrigADC: bool, //* CH0B比较匹配触发ADC
    bEnCH2ACmpTrigADC: bool, //* CH0A比较匹配触发ADC
    bEnCH2BCmpTrigADC: bool, //* CH0B比较匹配触发ADC
}

/**
 ******************************************************************************
 ** \brief  DMA触发 配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_tim3_m23_trig_dma_cfg {
    bUevTrigDMA: bool,                      //* 更新 触发DMA使能
    bTITrigDMA: bool,                       //* Trig 触发DMA功能
    bCmpA0TrigDMA: bool,                    //* CH0A捕获比较触发DMA使能
    bCmpB0TrigDMA: bool,                    //* CH0B捕获比较触发DMA使能
    bCmpA1TrigDMA: bool,                    //* CH1A捕获比较触发DMA使能
    bCmpB1TrigDMA: bool,                    //* CH1B捕获比较触发DMA使能
    bCmpA2TrigDMA: bool,                    //* CH2A捕获比较触发DMA使能
    bCmpB2TrigDMA: bool,                    //* CH2B捕获比较触发DMA使能
    enCmpUevTrigDMA: en_tim3_m23_mscr_ccds, //* 比较模式下DMA比较触发选择
}

/**
 ******************************************************************************
 ** \brief  主从模式 配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_tim3_m23_master_slave_cfg {
    enMasterSlaveSel: en_tim3_m23_mscr_msm, //* 主从模式选择
    enMasterSrc: en_tim3_m23_mscr_mms,      //* 主模式触发源选择
    enSlaveModeSel: en_tim3_m23_mscr_sms,   //* 从模式选择
    enTsSel: en_tim3_mscr_ts,               //* 触发输入源选择
}

/**
 ******************************************************************************
 ** \brief  OCREF清除功能 配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_tim3_m23_OCREF_Clr_cfg {
    enOCRefClrSrcSel: en_tim3_m23ce_occs, //* OCREF清除源选择
    bVCClrEn: bool,                       //* 是否使能来自VC的OCREF_Clr
}

//中断标志获取
pub fn Tim3_GetIntFlag(enTim3Irq: en_tim3_irq_type) -> bool {
    true
}
//中断标志清除
pub fn Tim3_ClearIntFlag(enTim3Irq: en_tim3_irq_type) -> en_result_t {
    en_result_t::Ok
}
//所有中断标志清除
pub fn Tim3_ClearAllIntFlag() -> en_result_t {
    en_result_t::Ok
}
//模式0中断使能
pub fn Tim3_Mode0_EnableIrq() -> en_result_t {
    en_result_t::Ok
}
//模式1中断使能
pub fn Tim3_Mode1_EnableIrq(enTim3Irq: en_tim3_irq_type) -> en_result_t {
    en_result_t::Ok
}
//模式2中断使能
pub fn Tim3_Mode23_EnableIrq(enTim3Irq: en_tim3_irq_type) -> en_result_t {
    en_result_t::Ok
}
//模式0中断禁止
pub fn Tim3_Mode0_DisableIrq() -> en_result_t {
    en_result_t::Ok
}
//模式1中断禁止
pub fn Tim3_Mode1_DisableIrq(enTim3Irq: en_tim3_irq_type) -> en_result_t {
    en_result_t::Ok
}
//模式2中断禁止
pub fn Tim3_Mode23_DisableIrq(enTim3Irq: en_tim3_irq_type) -> en_result_t {
    en_result_t::Ok
}

//模式0初始化及相关功能操作

//timer配置及初始化
pub fn Tim3_Mode0_Init(pstcCfg: &stc_tim3_mode0_cfg) -> en_result_t {
    en_result_t::Ok
}
//timer 启动/停止
pub fn Tim3_M0_Run() -> en_result_t {
    en_result_t::Ok
}
pub fn Tim3_M0_Stop() -> en_result_t {
    en_result_t::Ok
}
//重载值设置
pub fn Tim3_M0_ARRSet(u16Data: u16) -> en_result_t {
    en_result_t::Ok
}
//16位计数值设置/获取
pub fn Tim3_M0_Cnt16Set(u16Data: u16) -> en_result_t {
    en_result_t::Ok
}
pub fn Tim3_M0_Cnt16Get() -> u16 {
    0
}
//32位计数值设置/获取
pub fn Tim3_M0_Cnt32Set(u32Data: u32) -> en_result_t {
    en_result_t::Ok
}
pub fn Tim3_M0_Cnt32Get() -> u32 {
    0
}
//端口输出使能/禁止设定
pub fn Tim3_M0_Enable_Output(bEnOutput: bool) -> en_result_t {
    en_result_t::Ok
}
//翻转使能/禁止（低电平）设定
pub fn Tim3_M0_EnTOG(bEnTOG: bool) -> en_result_t {
    en_result_t::Ok
}

//模式1初始化及相关功能操作

//timer配置及初始化
pub fn Tim3_Mode1_Init(pstcCfg: &stc_tim3_mode1_cfg) -> en_result_t {
    en_result_t::Ok
}
//PWC 输入配置
pub fn Tim3_M1_Input_Cfg(pstcCfg: &stc_tim3_pwc_input_cfg) -> en_result_t {
    en_result_t::Ok
}
//PWC测量边沿起始结束选择
pub fn Tim3_M1_PWC_Edge_Sel(enEdgeSel: en_tim3_m1cr_Edge) -> en_result_t {
    en_result_t::Ok
}
//timer 启动/停止
pub fn Tim3_M1_Run() -> en_result_t {
    en_result_t::Ok
}
pub fn Tim3_M1_Stop() -> en_result_t {
    en_result_t::Ok
}
//16位计数值设置/获取
pub fn Tim3_M1_Cnt16Set(u16Data: u16) -> en_result_t {
    en_result_t::Ok
}
pub fn Tim3_M1_Cnt16Get() -> u16 {
    0
}
//脉冲宽度测量结果数值获取
pub fn Tim3_M1_PWC_CapValueGet() -> u16 {
    0
}

pub fn Tim3_Mode23_Init(pstcCfg: &stc_tim3_mode23_cfg) -> en_result_t {
    en_result_t::Ok
}
//timer 启动/停止
pub fn Tim3_M23_Run() -> en_result_t {
    en_result_t::Ok
}
pub fn Tim3_M23_Stop() -> en_result_t {
    en_result_t::Ok
}
//PWM输出使能
pub fn Tim3_M23_EnPWM_Output(bEnOutput: bool, bEnAutoOutput: bool) -> en_result_t {
    en_result_t::Ok
}
//重载值设置
pub fn Tim3_M23_ARRSet(u16Data: u16, bArrBufEn: bool) -> en_result_t {
    en_result_t::Ok
}
//16位计数值设置/获取
pub fn Tim3_M23_Cnt16Set(u16Data: u16) -> en_result_t {
    en_result_t::Ok
}
pub fn Tim3_M23_Cnt16Get() -> u16 {
    0
}
//比较捕获寄存器CCR0A/CCR0B设置/读取
pub fn Tim3_M23_CCR_Set(enCCRSel: en_tim3_m23_ccrx, u16Data: u16) -> en_result_t {
    en_result_t::Ok
}
pub fn Tim3_M23_CCR_Get(enCCRSel: en_tim3_m23_ccrx) -> u16 {
    0
}
//PWM互补输出模式下，GATE功能选择
pub fn Tim3_M23_GateFuncSel(pstcCfg: &stc_tim3_m23_gate_cfg) -> en_result_t {
    en_result_t::Ok
}
//主从模式配置
pub fn Tim3_M23_MasterSlave_Set(pstcCfg: &stc_tim3_m23_master_slave_cfg) -> en_result_t {
    en_result_t::Ok
}
//CH0A/CH0B比较通道控制
pub fn Tim3_M23_PortOutput_Cfg(
    enTim3Chx: en_tim3_channel,
    pstcCfg: &stc_tim3_m23_compare_cfg,
) -> en_result_t {
    en_result_t::Ok
}
//CH0A/CH0B输入控制
pub fn Tim3_M23_PortInput_Cfg(
    enTim3Chx: en_tim3_channel,
    pstcCfg: &stc_tim3_m23_input_cfg,
) -> en_result_t {
    en_result_t::Ok
}
//ERT输入控制
pub fn Tim3_M23_ETRInput_Cfg(pstcCfg: &stc_tim3_m23_etr_input_cfg) -> en_result_t {
    en_result_t::Ok
}
//刹车BK输入控制
pub fn Tim3_M23_BrakeInput_Cfg(pstcBkCfg: &stc_tim3_m23_bk_input_cfg) -> en_result_t {
    en_result_t::Ok
}
//触发ADC控制
pub fn Tim3_M23_TrigADC_Cfg(pstcCfg: &stc_tim3_m23_adc_trig_cfg) -> en_result_t {
    en_result_t::Ok
}
//死区功能
pub fn Tim3_M23_DT_Cfg(pstcCfg: &stc_tim3_m23_dt_cfg) -> en_result_t {
    en_result_t::Ok
}
//重复周期设置
pub fn Tim3_M23_SetValidPeriod(u8ValidPeriod: u8) -> en_result_t {
    en_result_t::Ok
}
//OCREF清除功能
pub fn Tim3_M23_OCRefClr(pstcCfg: &stc_tim3_m23_OCREF_Clr_cfg) -> en_result_t {
    en_result_t::Ok
}
//使能DMA传输
pub fn Tim3_M23_EnDMA(pstcCfg: &stc_tim3_m23_trig_dma_cfg) -> en_result_t {
    en_result_t::Ok
}
//捕获比较A软件触发
pub fn Tim3_M23_EnSwTrigCapCmpA(enTim3Chx: en_tim3_channel) -> en_result_t {
    en_result_t::Ok
}
//捕获比较B软件触发
pub fn Tim3_M23_EnSwTrigCapCmpB(enTim3Chx: en_tim3_channel) -> en_result_t {
    en_result_t::Ok
}
//软件更新使能
pub fn Tim3_M23_EnSwUev() -> en_result_t {
    en_result_t::Ok
}
//软件触发使能
pub fn Tim3_M23_EnSwTrig() -> en_result_t {
    en_result_t::Ok
}
//软件刹车使能
pub fn Tim3_M23_EnSwBk() -> en_result_t {
    en_result_t::Ok
}
