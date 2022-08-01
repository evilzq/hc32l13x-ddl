use crate::en_result_t;

pub enum en_lptim_tcksel_t {
    LptimPclk = 0,
    LptimXtl = 2,
    LptimRcl = 3,
}

/**
 *******************************************************************************
 ** \brief LPTIMx  GATE极性控制位
 ** \note  LPTIMx_CR  GATE_P   
 ******************************************************************************/
pub enum en_lptim_gatep_t {
    LptimGatePLow = 0,
    LptimGatePHigh = 1,
}

/**
 *******************************************************************************
 ** \brief LPTIMx  时钟选择
 ** \note  LPTIMx_CR  GATE   
 ******************************************************************************/
pub enum en_lptim_gate_t {
    LptimGateLow = 0,
    LptimGateHigh = 1,
}

/**
 *******************************************************************************
 ** \brief LPTIMx  TOG输出使能位
 ** \note  LPTIMx_CR  TOG  
 ******************************************************************************/
pub enum en_lptim_togen_t {
    LptimTogEnLow = 0,
    LptimTogEnHigh = 1,
}

/**
 *******************************************************************************
 ** \brief LPTIMx  CT计数器/定时器功能选择
 ** \note  LPTIMx_CR  CT
 ******************************************************************************/
pub enum en_lptim_ct_t {
    LptimTimerFun = 0, //警示器功能，定时器使用TCK_SEL选择的时钟进行计数
    LptimCntFun = 1, //计数器功能，计数器使用外部输入的下降沿进行计数，采样时钟使用TCK_SEL选择的时钟
}

/**
 *******************************************************************************
 ** \brief LPTIMx  定时器工作模式
 ** \note  LPTIMx_CR  MD
 ******************************************************************************/
pub enum en_lptim_md_t {
    LptimMode1 = 0, //模式1无重载16位计数器/定时器
    LptimMode2 = 1, //模式2自动重载16位计数器/定时器
}

pub enum M0P_LPTIMER_TypeDef {
    LpTimer0,
}

/**
 *******************************************************************************
 ** \brief LPTIMx 初始化配置的结构体
 ** \note       
 ******************************************************************************/
pub struct stc_lptim_cfg_t {
    enTcksel: en_lptim_tcksel_t,
    enGatep: en_lptim_gatep_t,
    enGate: en_lptim_gate_t,
    enTogen: en_lptim_togen_t,
    enCt: en_lptim_ct_t,
    enMd: en_lptim_md_t,
    u16Arr: u16,
}

pub fn Lptim_ConfIt(Lptimx: M0P_LPTIMER_TypeDef, NewStatus: bool) {}
pub fn Lptim_Cmd(Lptimx: M0P_LPTIMER_TypeDef, NewStatus: bool) {}
pub fn Lptim_GetItStatus(Lptimx: M0P_LPTIMER_TypeDef) -> bool {
    true
}
pub fn Lptim_ClrItStatus(Lptimx: M0P_LPTIMER_TypeDef) {}
pub fn Lptim_Init(Lptimx: M0P_LPTIMER_TypeDef, InitStruct: &stc_lptim_cfg_t) -> en_result_t {
    en_result_t::Ok
}
