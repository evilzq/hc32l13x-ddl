use crate::en_result_t;

pub enum en_bt_unit {
    TIM0 = 0, //* Timer 0
    TIM1 = 1, //* Timer 1
    TIM2 = 2, //* Timer 2
}

/**
 ******************************************************************************
 ** \brief 工作模式选择数据类型重定义 (MODE)(模式0/1/23)
 *****************************************************************************/
pub enum en_bt_work_mode {
    BtWorkMode0 = 0, //* 定时器模式
    BtWorkMode1 = 1, //* PWC模式
    BtWorkMode2 = 2, //* 锯齿波模式
    BtWorkMode3 = 3, //* 三角波模式
}

/**
 ******************************************************************************
 ** \brief 极性控制数据类型重定义 (GATE_P)(模式0)
 *****************************************************************************/
pub enum en_bt_m0_gatep {
    BtGatePositive = 0, //* 高电平有效
    BtGateOpposite = 1, //* 低电平有效
}

/**
 ******************************************************************************
 ** \brief TIM 预除频选择 (PRS)(模式0/1/23)
 *****************************************************************************/
pub enum en_bt_cr_timclkdiv {
    BtPCLKDiv1 = 0,   //* Div 1
    BtPCLKDiv2 = 1,   //* Div 2
    BtPCLKDiv4 = 2,   //* Div 4
    BtPCLKDiv8 = 3,   //* Div 8
    BtPCLKDiv16 = 4,  //* Div 16
    BtPCLKDiv32 = 5,  //* Div 32
    BtPCLKDiv64 = 6,  //* Div 64
    BtPCLKDiv256 = 7, //* Div 256
}

/**
 ******************************************************************************
 ** \brief 计数/定时器功能选择数据类型重定义 (CT)(模式0/1/23)
 *****************************************************************************/
pub enum en_bt_cr_ct {
    BtTimer = 0,   //* 定时器功能，计数时钟为内部PCLK
    BtCounter = 1, //* 计数器功能，计数时钟为外部ETR
}

/**
 ******************************************************************************
 ** \brief 定时器工作模式数据类型重定义 (MD)(模式0)
 *****************************************************************************/
pub enum en_bt_m0cr_md {
    Bt32bitFreeMode = 0, //* 32位计数器/定时器
    Bt16bitArrMode = 1,  //* 自动重载16位计数器/定时器
}

/**
 ******************************************************************************
** \brief BT0/BT1/BT2中断类型数据类型重定义(模式0/1/23)
 *****************************************************************************/
pub enum en_bt_irq_type {
    BtUevIrq = 0,   //* 溢出/事件更新中断
    BtCA0Irq = 2,   //* 捕获/比较中断A(仅模式1/23存在)
    BtCB0Irq = 5,   //* 捕获/比较中断B(仅模式23存在)
    BtCA0E = 8,     //* CH0A捕获数据丢失标志(仅模式23存在),不是中断
    BtCB0E = 11,    //* CH0B捕获数据丢失标志(仅模式23存在),不是中断
    BtBkIrq = 14,   //* 刹车中断(仅模式23存在)
    BtTrigIrq = 15, //* 触发中断(仅模式23存在)
}

/**
 ******************************************************************************
 ** \brief 测量开始结束数据类型重定义 (Edg1stEdg2nd)(模式1)
 *****************************************************************************/
pub enum en_bt_m1cr_Edge {
    BtPwcRiseToRise = 0, //* 上升沿到上升沿(周期)
    BtPwcFallToRise = 1, //* 下降沿到上升沿(低电平)
    BtPwcRiseToFall = 2, //* 上升沿到下降沿(高电平)
    BtPwcFallToFall = 3, //* 下降沿到下降沿(周期)
}

/**
 ******************************************************************************
 ** \brief PWC测量测试模式选择数据类型重定义 (Oneshot)(模式1)
 *****************************************************************************/
pub enum en_bt_m1cr_oneshot {
    BtPwcCycleDetect = 0,   //* PWC循环测量
    BtPwcOneShotDetect = 1, //* PWC单次测量
}

/**
 ******************************************************************************
 ** \brief PWC IA0选择数据类型重定义 (IA0S)(模式1)
 *****************************************************************************/
pub enum en_bt_m1_mscr_ia0s {
    BtIA0Input = 0, //* IAO输入
    BtXORInput = 1, //* IA0 ETR GATE XOR(TIM0/1/2)/IA0 IA1 IA2 XOR(TIM3)
}

/**
 ******************************************************************************
 ** \brief PWC IB0选择数据类型重定义 (IA0S)(模式1)
 *****************************************************************************/
pub enum en_bt_m1_mscr_ib0s {
    BtIB0Input = 0, //* IBO输入
    BtTsInput = 1,  //* 内部触发TS选择信号
}

/**
 ******************************************************************************
 ** \brief 输出极性、输入相位 数据类型重定义 (CCPA0/CCPB0/ETP/BKP)(模式1/23)
 *****************************************************************************/
pub enum en_bt_port_polarity {
    BtPortPositive = 0, //* 正常输入输出
    BtPortOpposite = 1, //* 反向输入输出
}

/**
 ******************************************************************************
 ** \brief 滤波选择数据类型重定义 (FLTET/FLTA0/FLAB0)(模式1/23)
 *****************************************************************************/
pub enum en_bt_flt {
    BtFltNone = 0,          //* 无滤波
    BtFltPCLKCnt3 = 4,      //* PCLK 3个连续有效
    BtFltPCLKDiv4Cnt3 = 5,  //* PCLK/4 3个连续有效
    BtFltPCLKDiv16Cnt3 = 6, //* PCLK/16 3个连续有效
    BtFltPCLKDiv64Cnt3 = 7, //* PCLK/64 3个连续有效
}

/**
 ******************************************************************************
 ** \brief 通道比较控制 数据类型重定义 (OCMA/OCMB)(模式23)
 *****************************************************************************/
pub enum en_bt_m23_fltr_ocm {
    BtForceLow = 0,      //* 强制为0
    BtForceHigh = 1,     //* 强制为1
    BtCMPForceLow = 2,   //* 比较匹配时强制为0
    BtCMPForceHigh = 3,  //* 比较匹配时强制为1
    BtCMPInverse = 4,    //* 比较匹配时翻转电平
    BtCMPOnePrdHigh = 5, //* 比较匹配时输出一个计数周期的高电平
    BtPWMMode1 = 6,      //* 通道控制为PWM mode 1
    BtPWMMode2 = 7,      //* 通道控制为PWM mode 2
}

/**
 ******************************************************************************
 ** \brief 主从模式TS数据类型重定义 (TS)(模式1/23)
 *****************************************************************************/
pub enum en_bt_mscr_ts {
    BtTs0ETR = 0,      //* ETR外部输入滤波后的相位选择信号
    BtTs1TIM0TRGO = 1, //* Timer0的TRGO输出信号
    BtTs2TIM1TRGO = 2, //* Timer1的TRGO输出信号
    BtTs3TIM2TRGO = 3, //* Timer2的TRGO输出信号
    BtTs4TIM3TRGO = 4, //* Timer3的TRGO输出信号
    //BtTs5IA0ED    = 5,         //* 无效
    BtTs6IAFP = 6, //* CH0A 外部输输入滤波后的相位选择信号
    BtTs7IBFP = 7, //* CH0B 外部输输入滤波后的相位选择信
}

/**
 ******************************************************************************
 ** \brief PWM输出模式选择数据类型重定义 (COMP)(模式23)
 *****************************************************************************/
pub enum en_bt_m23cr_comp {
    BtIndependentPWM = 0,   //* 独立PWM输出
    BtComplementaryPWM = 1, //* 互补PWM输出
}

/**
 ******************************************************************************
 ** \brief 计数方向选择数据类型重定义 (DIR)(模式23)
 *****************************************************************************/
pub enum en_bt_m23cr_dir {
    BtCntUp = 0,   //* 向上计数
    BtCntDown = 1, //* 向下计数
}

/**
 ******************************************************************************
 ** \brief 计数方向选择数据类型重定义 (PWM2S)(模式23)
 *****************************************************************************/
pub enum en_bt_m23cr_pwm2s {
    BtDoublePointCmp = 0, //* 双点比较使能，使用CCRA,CCRB比较控制OCREFA输出
    BtSinglePointCmp = 1, //* 单点比较使能，使用CCRA比较控制OCREFA输出
}

/**
 ******************************************************************************
 ** \brief GATE在PWM互补模式下捕获或比较功能 选择数据类型重定义 (CSG)(模式23)
 *****************************************************************************/
pub enum en_bt_m23cr_csg {
    BtPWMCompGateCmpOut = 0, //* 在PWM互补模式下，Gate作为比较输出
    BtPWMCompGateCapIn = 1,  //* 在PWM互补模式下，Gate作为捕获输入
}

/**
 ******************************************************************************
 ** \brief 比较捕获寄存器 数据类型重定义 (CCR0A,CCR0B)(模式23)
 *****************************************************************************/
pub enum en_bt_m23_ccrx {
    BtCCR0A = 0, //* CCR0A比较捕获寄存器
    BtCCR0B = 1, //* CCR0B比较捕获寄存器
}

/**
 ******************************************************************************
 ** \brief  OCREF清除源 选择数据类型重定义 (OCCS)(模式23)
 *****************************************************************************/
pub enum en_bt_m23ce_occs {
    BtOC_Ref_Clr = 0, //* 来自VC的OC_Ref_Clr
    BtETRf = 1,       //* 外部ETRf
}

/**
 ******************************************************************************
 ** \brief  比较匹配中断模式 选择数据类型重定义 (CIS/CISB)(模式23)
 *****************************************************************************/
pub enum en_bt_m23_cisa_cisb {
    BtCmpIntNone = 0,     //* 无比较匹配中断
    BtCmpIntRise = 1,     //* 比较匹配上升沿中断
    BtCmpIntFall = 2,     //* 比较匹配下降沿中断
    BtCmpIntRiseFall = 3, //* 比较匹配上升沿下降沿中断
}

/**
 ******************************************************************************
 ** \brief BT端口控制 - 刹车时CHx输出状态控制(BKSA/BKSB)(模式23)
 **
 ** \note
 ******************************************************************************/
pub enum en_bt_m23_crch0_bks {
    BtCHxBksHiZ = 0,  //* 刹车使能时，CHx端口输出高阻态
    BtCHxBksNorm = 1, //* 刹车使能时，CHx端口正常输出
    BtCHxBksLow = 2,  //* 刹车使能时，CHx端口输出低电平
    BtCHxBksHigh = 3, //* 刹车使能时，CHx端口输出高电平
}

/**
 ******************************************************************************
** \brief BT端口控制 - CHx上升沿下降沿捕获(CRx/CFx)(模式23)
 **
 ** \note
 ******************************************************************************/
pub enum en_bt_m23_crch0_cfx_crx {
    BtCHxCapNone = 0,     //* CHx通道捕获禁止
    BtCHxCapRise = 1,     //* CHx通道上升沿捕获使能
    BtCHxCapFall = 2,     //* CHx通道下降沿捕获使能
    BtCHxCapFallRise = 3, //* CHx通道上升沿下降沿捕获都使能
}

/**
 ******************************************************************************
** \brief BT端口控制 - CHx比较捕获模式(CSA/CSB)(模式23)
 **
 ** \note
 ******************************************************************************/
pub enum en_bt_m23_crch0_csa_csb {
    BtCHxCmpMode = 0, //* CHx通道设置为比较模式
    BtCHxCapMode = 1, //* CHx通道设置为捕获模式
}

/**
 ******************************************************************************
 ** \brief  比较模式下 DMA比较触发选择 数据类型重定义 (CCDS)(模式23)
 *****************************************************************************/
pub enum en_bt_m23_mscr_ccds {
    BtCmpTrigDMA = 0, //* 比较匹配触发DMA
    BtUEVTrigDMA = 1, //* 事件更新代替比较匹配触发DMA
}

/**
 ******************************************************************************
 ** \brief  主从模式选择 数据类型重定义 (MSM)(模式23)
 *****************************************************************************/
pub enum en_bt_m23_mscr_msm {
    BtSlaveMode = 0,  //* 从模式
    BtMasterMode = 1, //* 主模式
}

/**
 ******************************************************************************
 ** \brief  触发主模式输出源 数据类型重定义 (MMS)(模式23)
 *****************************************************************************/
pub enum en_bt_m23_mscr_mms {
    BtMasterUG = 0,      //* UG(软件更新)源
    BtMasterCTEN = 1,    //* CTEN源
    BtMasterUEV = 2,     //* UEV更新源
    BtMasterCMPSO = 3,   //* 比较匹配选择输出源
    BtMasterOCA0Ref = 4, //* OCA0_Ref源
    BtMasterOCB0Ref = 5, //* OCB0_Ref源
                         //BtMasterOCB0Ref  = 6,
                         //BtMasterOCB0Ref  = 7,
}

/**
 ******************************************************************************
 ** \brief  触发从模式选择 数据类型重定义 (SMS)(模式23)
 *****************************************************************************/
pub enum en_bt_m23_mscr_sms {
    BtSlaveIClk = 0,     //* 使用内部时钟
    BtSlaveResetTIM = 1, //* 复位功能
    BtSlaveTrigMode = 2, //* 触发模式
    BtSlaveEClk = 3,     //* 外部时钟模式
    BtSlaveCodeCnt1 = 4, //* 正交编码计数模式1
    BtSlaveCodeCnt2 = 5, //* 正交编码计数模式2
    BtSlaveCodeCnt3 = 6, //* 正交编码计数模式3
    BtSlaveGateCtrl = 7, //* 门控功能
}

/**
 ******************************************************************************
 ** \brief 定时器运行控制数据类型重定义 (CTEN)
 *****************************************************************************/
pub enum en_bt_start {
    BtCTENDisable = 0, //* 停止
    BtCTENEnable = 1,  //* 运行
}

/**
 ******************************************************************************
 ** \brief BaseTimer mode0 配置结构体定义(模式0)
 *****************************************************************************/
pub struct stc_bt_mode0_cfg {
    enWorkMode: en_bt_work_mode, //* 工作模式设置
    enGateP: en_bt_m0_gatep,     //* 门控极性控制
    bEnGate: bool,               //* 门控使能
    enPRS: en_bt_cr_timclkdiv,   //* 预除频配置
    bEnTog: bool,                //* 翻转输出使能
    enCT: en_bt_cr_ct,           //* 定时/计数功能选择
    enCntMode: en_bt_m0cr_md,    //* 计数模式配置

    pfnTim0Cb: fn() -> (), //* Timer0中断服务回调函数[void function(void)]
    pfnTim1Cb: fn() -> (), //* Timer1中断服务回调函数[void function(void)]
    pfnTim2Cb: fn() -> (), //* Timer2中断服务回调函数[void function(void)]
}

/**
 ******************************************************************************
 ** \brief BaseTimer mode1 配置结构体定义(模式1)
 *****************************************************************************/
pub struct stc_bt_mode1_cfg {
    enWorkMode: en_bt_work_mode,   //* 工作模式设置
    enPRS: en_bt_cr_timclkdiv,     //* 预除频配置
    enCT: en_bt_cr_ct,             //* 定时/计数功能选择
    enOneShot: en_bt_m1cr_oneshot, //* 单次测量/循环测量选择

    pfnTim0Cb: fn() -> (), //* Timer0中断服务回调函数[void function(void)]
    pfnTim1Cb: fn() -> (), //* Timer1中断服务回调函数[void function(void)]
    pfnTim2Cb: fn() -> (), //* Timer2中断服务回调函数[void function(void)]
}

/**
 ******************************************************************************
 ** \brief PWC输入配置结构体定义(模式1)
 *****************************************************************************/
pub struct stc_bt_pwc_input_cfg {
    enTsSel: en_bt_mscr_ts,          //* 触发输入源选择
    enIA0Sel: en_bt_m1_mscr_ia0s,    //* CHA0输入选择
    enIB0Sel: en_bt_m1_mscr_ib0s,    //* CHB0输入选择
    enETRPhase: en_bt_port_polarity, //* ETR相位选择
    enFltETR: en_bt_flt,             //* ETR滤波设置
    enFltIA0: en_bt_flt,             //* CHA0滤波设置
    enFltIB0: en_bt_flt,             //* CHB0滤波设置
}

/**
 ******************************************************************************
 ** \brief BaseTimer mode23 配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_bt_mode23_cfg {
    enWorkMode: en_bt_work_mode,    //* 工作模式设置
    enCntDir: en_bt_m23cr_dir,      //* 计数方向
    enPRS: en_bt_cr_timclkdiv,      //* 时钟预除频配置
    enCT: en_bt_cr_ct,              //* 定时/计数功能选择
    enPWMTypeSel: en_bt_m23cr_comp, //* PWM模式选择（独立/互补）
    enPWM2sSel: en_bt_m23cr_pwm2s,  //* OCREFA双点比较功能选择
    bOneShot: bool,                 //* 单次触发模式使能/禁止
    bURSSel: bool,                  //* 更新源选择

    pfnTim0Cb: fn() -> (), //* Timer0中断服务回调函数[void function(void)]
    pfnTim1Cb: fn() -> (), //* Timer1中断服务回调函数[void function(void)]
    pfnTim2Cb: fn() -> (), //* Timer2中断服务回调函数[void function(void)]
}

/**
 ******************************************************************************
 ** \brief GATE在PWM互补模式下捕获或比较功能 配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_bt_m23_gate_cfg {
    enGateFuncSel: en_bt_m23cr_csg, //* Gate比较、捕获功能选择
    bGateRiseCap: bool,             //* GATE作为捕获功能时，上沿捕获有效控制
    bGateFallCap: bool,             //* GATE作为捕获功能时，下沿捕获有效控制
}

/**
 ******************************************************************************
 ** \brief CHA/CHB通道比较控制 配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_bt_m23_compare_cfg {
    enCh0ACmpCap: en_bt_m23_crch0_csa_csb, //* CH0A比较/捕获功能选择
    enCH0ACmpCtrl: en_bt_m23_fltr_ocm,     //* CH0A通道比较控制
    enCH0APolarity: en_bt_port_polarity,   //* CH0A输出极性控制
    bCh0ACmpBufEn: bool,                   //* 比较A缓存功能 使能/禁止
    enCh0ACmpIntSel: en_bt_m23_cisa_cisb,  //* CHA比较匹配中断选择

    enCh0BCmpCap: en_bt_m23_crch0_csa_csb, //* CH0B比较/捕获功能选择
    enCH0BCmpCtrl: en_bt_m23_fltr_ocm,     //* CH0B通道比较控制
    enCH0BPolarity: en_bt_port_polarity,   //* CH0B输出极性控制
    bCH0BCmpBufEn: bool,                   //* 比较B缓存功能 使能/禁止
    enCH0BCmpIntSel: en_bt_m23_cisa_cisb,  //* CHB0比较匹配中断选择
}

/**
 ******************************************************************************
 ** \brief CHA/CHB通道捕获控制 配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_bt_m23_input_cfg {
    enCh0ACmpCap: en_bt_m23_crch0_csa_csb, //* CH0A比较/捕获功能选择
    enCH0ACapSel: en_bt_m23_crch0_cfx_crx, //* CH0A捕获边沿选择
    enCH0AInFlt: en_bt_flt,                //* CH0A通道捕获滤波控制
    enCH0APolarity: en_bt_port_polarity,   //* CH0A输入相位

    enCh0BCmpCap: en_bt_m23_crch0_csa_csb, //* CH0B比较/捕获功能选择
    enCH0BCapSel: en_bt_m23_crch0_cfx_crx, //* HC0B捕获边沿选择
    enCH0BInFlt: en_bt_flt,                //* CH0B通道捕获滤波控制
    enCH0BPolarity: en_bt_port_polarity,   //* CH0B输入相位
}

/**
 ******************************************************************************
 ** \brief ETR输入相位滤波配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_bt_m23_etr_input_cfg {
    enETRPolarity: en_bt_port_polarity, //* ETR输入极性设置
    enETRFlt: en_bt_flt,                //* ETR滤波设置
}

/**
 ******************************************************************************
 ** \brief 刹车BK输入相位滤波配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_bt_m23_bk_input_cfg {
    bEnBrake: bool,                       //* 刹车使能
    bEnVC0Brake: bool,                    //* 使能VC0刹车
    bEnVC1Brake: bool,                    //* 使能VC1刹车
    bEnSafetyBk: bool,                    //* 使能safety刹车
    bEnBKSync: bool,                      //* TIM0/TIM1/TIM2刹车同步使能
    enBkCH0AStat: en_bt_m23_crch0_bks,    //* 刹车时CHA端口状态设置
    enBkCH0BStat: en_bt_m23_crch0_bks,    //* 刹车时CHB端口状态设置
    enBrakePolarity: en_bt_port_polarity, //* 刹车BK输入极性设置
    enBrakeFlt: en_bt_flt,                //* 刹车BK滤波设置
}

/**
 ******************************************************************************
** \brief 死区功能配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_bt_m23_dt_cfg {
    bEnDeadTime: bool,   //* 刹车时CHA端口状态设置
    u8DeadTimeValue: u8, //* 刹车时CHA端口状态设置
}

/**
 ******************************************************************************
 ** \brief 触发ADC配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_bt_m23_adc_trig_cfg {
    bEnTrigADC: bool,        //* 触发ADC全局控制
    bEnUevTrigADC: bool,     //* 事件更新触发ADC
    bEnCH0ACmpTrigADC: bool, //* CH0A比较匹配触发ADC
    bEnCH0BCmpTrigADC: bool, //* CH0B比较匹配触发ADC
}

/**
 ******************************************************************************
 ** \brief  DMA触发 配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_bt_m23_trig_dma_cfg {
    bUevTrigDMA: bool,                    //* 更新 触发DMA使能
    bTITrigDMA: bool,                     //* Trig 触发DMA功能
    bCmpATrigDMA: bool,                   //* A捕获比较触发DMA使能
    bCmpBTrigDMA: bool,                   //* B捕获比较触发DMA使能
    enCmpUevTrigDMA: en_bt_m23_mscr_ccds, //* 比较模式下DMA比较触发选择
}

/**
 ******************************************************************************
 ** \brief  主从模式 配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_bt_m23_master_slave_cfg {
    enMasterSlaveSel: en_bt_m23_mscr_msm, //* 主从模式选择
    enMasterSrc: en_bt_m23_mscr_mms,      //* 主模式触发源选择
    enSlaveModeSel: en_bt_m23_mscr_sms,   //* 从模式选择
    enTsSel: en_bt_mscr_ts,               //* 触发输入源选择
}

/**
 ******************************************************************************
 ** \brief  OCREF清除功能 配置结构体定义(模式23)
 *****************************************************************************/
pub struct stc_bt_m23_OCREF_Clr_cfg {
    enOCRefClrSrcSel: en_bt_m23ce_occs, //* OCREF清除源选择
    bVCClrEn: bool,                     //* 是否使能来自VC的OCREF_Clr
}

//中断标志获取
pub fn Bt_GetIntFlag(enUnit: en_bt_unit, enBtIrq: en_bt_irq_type) -> bool {
    true
}
//中断标志清除
pub fn Bt_ClearIntFlag(enUnit: en_bt_unit, enBtIrq: en_bt_irq_type) -> en_result_t {
    en_result_t::Ok
}
//所有中断标志清除
pub fn Bt_ClearAllIntFlag(enUnit: en_bt_unit) -> en_result_t {
    en_result_t::Ok
}
//模式0中断使能
pub fn Bt_Mode0_EnableIrq(enUnit: en_bt_unit) -> en_result_t {
    en_result_t::Ok
}
//模式1中断使能
pub fn Bt_Mode1_EnableIrq(enUnit: en_bt_unit, enBtIrq: en_bt_irq_type) -> en_result_t {
    en_result_t::Ok
}
//模式2中断使能
pub fn Bt_Mode23_EnableIrq(enUnit: en_bt_unit, enBtIrq: en_bt_irq_type) -> en_result_t {
    en_result_t::Ok
}
//模式0中断禁止
pub fn Bt_Mode0_DisableIrq(enUnit: en_bt_unit) -> en_result_t {
    en_result_t::Ok
}
//模式1中断禁止
pub fn Bt_Mode1_DisableIrq(enUnit: en_bt_unit, enBtIrq: en_bt_irq_type) -> en_result_t {
    en_result_t::Ok
}
//模式2中断禁止
pub fn Bt_Mode23_DisableIrq(enUnit: en_bt_unit, enBtIrq: en_bt_irq_type) -> en_result_t {
    en_result_t::Ok
}

//模式0初始化及相关功能操作

//timer配置及初始化
pub fn Bt_Mode0_Init(enUnit: en_bt_unit, pstcCfg: &stc_bt_mode0_cfg) -> en_result_t {
    en_result_t::Ok
}
//timer 启动/停止
pub fn Bt_M0_Run(enUnit: en_bt_unit) -> en_result_t {
    en_result_t::Ok
}
pub fn Bt_M0_Stop(enUnit: en_bt_unit) -> en_result_t {
    en_result_t::Ok
}
//重载值设置
pub fn Bt_M0_ARRSet(enUnit: en_bt_unit, u16Data: u16) -> en_result_t {
    en_result_t::Ok
}
//16位计数值设置/获取
pub fn Bt_M0_Cnt16Set(enUnit: en_bt_unit, u16Data: u16) -> en_result_t {
    en_result_t::Ok
}
pub fn Bt_M0_Cnt16Get(enUnit: en_bt_unit) -> u16 {
    0
}
//32位计数值设置/获取
pub fn Bt_M0_Cnt32Set(enUnit: en_bt_unit, u32Data: u32) -> en_result_t {
    en_result_t::Ok
}
pub fn Bt_M0_Cnt32Get(enUnit: en_bt_unit) -> u32 {
    0
}
//翻转输出使能/禁止（低电平）设定
pub fn Bt_M0_EnTOG_Output(enUnit: en_bt_unit, bEnTOG: bool) -> en_result_t {
    en_result_t::Ok
}
//端口输出使能/禁止设定
pub fn Bt_M0_Enable_Output(enUnit: en_bt_unit, bEnOutput: bool) -> en_result_t {
    en_result_t::Ok
}

//模式1初始化及相关功能操作

//timer配置及初始化
pub fn Bt_Mode1_Init(enUnit: en_bt_unit, pstcCfg: &stc_bt_mode1_cfg) -> en_result_t {
    en_result_t::Ok
}
//PWC 输入配置
pub fn Bt_M1_Input_Cfg(enUnit: en_bt_unit, pstcCfg: &stc_bt_pwc_input_cfg) -> en_result_t {
    en_result_t::Ok
}
//PWC测量边沿起始结束选择
pub fn Bt_M1_PWC_Edge_Sel(enUnit: en_bt_unit, enEdgeSel: en_bt_m1cr_Edge) -> en_result_t {
    en_result_t::Ok
}
//timer 启动/停止
pub fn Bt_M1_Run(enUnit: en_bt_unit) -> en_result_t {
    en_result_t::Ok
}
pub fn Bt_M1_Stop(enUnit: en_bt_unit) -> en_result_t {
    en_result_t::Ok
}
//16位计数值设置/获取
pub fn Bt_M1_Cnt16Set(enUnit: en_bt_unit, u16Data: u16) -> en_result_t {
    en_result_t::Ok
}
pub fn Bt_M1_Cnt16Get(enUnit: en_bt_unit) -> u16 {
    0
}
//脉冲宽度测量结果数值获取
pub fn Bt_M1_PWC_CapValueGet(enUnit: en_bt_unit) -> u16 {
    0
}

//模式23初始化及相关功能操作

//timer配置及初始化
pub fn Bt_Mode23_Init(enUnit: en_bt_unit, pstcCfg: &stc_bt_mode23_cfg) -> en_result_t {
    en_result_t::Ok
}
//timer 启动/停止
pub fn Bt_M23_Run(enUnit: en_bt_unit) -> en_result_t {
    en_result_t::Ok
}
pub fn Bt_M23_Stop(enUnit: en_bt_unit) -> en_result_t {
    en_result_t::Ok
}
//PWM输出使能
pub fn Bt_M23_EnPWM_Output(
    enUnit: en_bt_unit,
    bEnOutput: bool,
    bEnAutoOutput: bool,
) -> en_result_t {
    en_result_t::Ok
}
//重载值设置
pub fn Bt_M23_ARRSet(enUnit: en_bt_unit, u16Data: u16, bArrBufEn: bool) -> en_result_t {
    en_result_t::Ok
}
//16位计数值设置/获取
pub fn Bt_M23_Cnt16Set(enUnit: en_bt_unit, u16Data: u16) -> en_result_t {
    en_result_t::Ok
}
pub fn Bt_M23_Cnt16Get(enUnit: en_bt_unit) -> u16 {
    0
}
//比较捕获寄存器CCR0A/CCR0B设置/读取
pub fn Bt_M23_CCR_Set(enUnit: en_bt_unit, enCCRSel: en_bt_m23_ccrx, u16Data: u16) -> en_result_t {
    en_result_t::Ok
}
pub fn Bt_M23_CCR_Get(enUnit: en_bt_unit, enCCRSel: en_bt_m23_ccrx) -> u16 {
    0
}
//PWM互补输出模式下，GATE功能选择
pub fn Bt_M23_GateFuncSel(enUnit: en_bt_unit, pstcCfg: &stc_bt_m23_gate_cfg) -> en_result_t {
    en_result_t::Ok
}
//主从模式配置
pub fn Bt_M23_MasterSlave_Set(
    enUnit: en_bt_unit,
    pstcCfg: &stc_bt_m23_master_slave_cfg,
) -> en_result_t {
    en_result_t::Ok
}
//CH0A/CH0B比较通道控制
pub fn Bt_M23_PortOutput_Cfg(enUnit: en_bt_unit, pstcCfg: &stc_bt_m23_compare_cfg) -> en_result_t {
    en_result_t::Ok
}
//CH0A/CH0B输入控制
pub fn Bt_M23_PortInput_Cfg(enUnit: en_bt_unit, pstcCfg: &stc_bt_m23_input_cfg) -> en_result_t {
    en_result_t::Ok
}
//ERT输入控制
pub fn Bt_M23_ETRInput_Cfg(enUnit: en_bt_unit, pstcCfg: &stc_bt_m23_etr_input_cfg) -> en_result_t {
    en_result_t::Ok
}
//刹车BK输入控制
pub fn Bt_M23_BrakeInput_Cfg(enUnit: en_bt_unit, pstcCfg: &stc_bt_m23_bk_input_cfg) -> en_result_t {
    en_result_t::Ok
}
//触发ADC控制
pub fn Bt_M23_TrigADC_Cfg(enUnit: en_bt_unit, pstcCfg: &stc_bt_m23_adc_trig_cfg) -> en_result_t {
    en_result_t::Ok
}
//死区功能
pub fn Bt_M23_DT_Cfg(enUnit: en_bt_unit, pstcCfg: &stc_bt_m23_dt_cfg) -> en_result_t {
    en_result_t::Ok
}
//重复周期设置
pub fn Bt_M23_SetValidPeriod(enUnit: en_bt_unit, u8ValidPeriod: u8) -> en_result_t {
    en_result_t::Ok
}
//OCREF清除功能
pub fn Bt_M23_OCRefClr(enUnit: en_bt_unit, pstcCfg: &stc_bt_m23_OCREF_Clr_cfg) -> en_result_t {
    en_result_t::Ok
}
//使能DMA传输
pub fn Bt_M23_EnDMA(enUnit: en_bt_unit, pstcCfg: &stc_bt_m23_trig_dma_cfg) -> en_result_t {
    en_result_t::Ok
}
//捕获比较A软件触发
pub fn Bt_M23_EnSwTrigCapCmpA(enUnit: en_bt_unit) -> en_result_t {
    en_result_t::Ok
}
//捕获比较B软件触发
pub fn Bt_M23_EnSwTrigCapCmpB(enUnit: en_bt_unit) -> en_result_t {
    en_result_t::Ok
}
//软件更新使能
pub fn Bt_M23_EnSwUev(enUnit: en_bt_unit) -> en_result_t {
    en_result_t::Ok
}
//软件触发使能
pub fn Bt_M23_EnSwTrig(enUnit: en_bt_unit) -> en_result_t {
    en_result_t::Ok
}
//软件刹车使能
pub fn Bt_M23_EnSwBk(enUnit: en_bt_unit) -> en_result_t {
    en_result_t::Ok
}
