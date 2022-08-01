use crate::en_result_t;

/**
 ******************************************************************************
 ** \brief rtc周期中断的时间间隔                       RTC_CR0  PRDS
 *****************************************************************************/
pub enum en_rtc_prds_t {
    RtcNone = 0, //无周期中断
    Rtc05S = 1,  //0.5S中断
    Rtc1S = 2,   //1秒
    Rtc1Min = 3, //1分钟
    Rtc1H = 4,   //1小时
    Rtc1Day = 5, //1天
    Rtc1Mon = 6, //1月
}

/**
 ******************************************************************************
 ** \brief rtc 12h制或24h制方式选择                   RTC_CR0  AMPM
 *****************************************************************************/
pub enum en_rtc_ampm_t {
    RtcAm = 0, //12小时制
    RtcPm = 1, //24小时制
}

/**
 ******************************************************************************
 ** \brief 普通精度与高精度1Hz输出                   RTC_CR0  HZ1SEL
 *****************************************************************************/
pub enum en_rtc_hz1sel_t {
    RtcHz1selGeneralPricision = 0, //普通精度1Hz输出
    RtcHz1selHighPricision = 1,    //高精度1Hz输出
}

/**
 ******************************************************************************
 ** \brief 周期中断选择                             RTC_CR0  PRDSEL
 *****************************************************************************/
pub enum en_rtc_prdsel_t {
    RtcPrds = 0, //使用PRDS所设定的周期中断事件间隔
    RtcPrdx = 1, //使用PRDX所设定的周期中断事件间隔
}

/**
 ******************************************************************************
 ** \brief rtc的时钟选择                           RTC_CR1  CKSEL
 *****************************************************************************/
pub enum en_rtc_cksel_t {
    RtcClkXtl = 0,     //外部低速时钟XTL  32.768k
    RtcClkRcl = 2,     //内部低速时钟RCL  32k
    RtcClkXth128 = 4,  //外部晶振4M       XTH/128
    RtcClkXth256 = 5,  //外部晶振8M       XTH/256
    RtcClkXth512 = 6,  //外部晶振16M      XTH/512
    RtcClkXth1024 = 7, //外部晶振32M      XTH/1024
}

/**
 ******************************************************************************
 ** \brief 时钟误差补偿使能或禁止                      RTC_COMPEN  EN
 *****************************************************************************/
pub enum en_rtc_compen_t {
    RtcCompenDisable = 0,
    RtcCompenEnable = 1,
}

/**
 ******************************************************************************
** \brief 配置PRD中断使能及其周期类型                      
 *****************************************************************************/
pub struct stc_rtc_cyccfg_t {
    rtcPrdsel: en_rtc_prdsel_t,
    rtcPrdx: u8,
    rtcPrds: en_rtc_prds_t,
}

/**
 ******************************************************************************
 ** \brief 闹钟源配置
 *****************************************************************************/
pub struct stc_rtc_alarmtime_t {
    RtcAlarmMinute: u8, //闹钟分钟
    RtcAlarmHour: u8,   //闹钟小时
    RtcAlarmWeek: u8,   //闹钟周
}

/**
 ******************************************************************************
 ** \brief 时间
 *****************************************************************************/
/**
 ******************************************************************************
 ** \brief rtc时钟年、月、日、时、分、秒读写结构
 *****************************************************************************/
pub struct stc_rtc_time_t {
    u8Second: u8,    //时间：秒
    u8Minute: u8,    //时间：分
    u8Hour: u8,      //时间：时
    u8DayOfWeek: u8, //时间：周
    u8Day: u8,       //时间：日
    u8Month: u8,     //时间：月
    u8Year: u8,      //时间：年
}

/**
 ******************************************************************************
 ** \brief 初始化RTC的结构体
 *****************************************************************************/
pub struct stc_rtc_initstruct_t {
    rtcAmpm: en_rtc_ampm_t,      //小时的时制
    rtcPrdsel: stc_rtc_cyccfg_t, //确定PRDS或者PRDX所设定的周期中断时间间隔类型
    rtcClksrc: en_rtc_cksel_t,   //实时时钟的时钟源
    rtcCompen: en_rtc_compen_t,  //时钟误差补偿使能与禁止
    rtcCompValue: u16,           //使能补偿的情况下，补偿值取值范围为:0-255
    rtcTime: stc_rtc_time_t,     //要写入时间寄存器的时间
}

//RTC计数器的使能或停止
pub fn Rtc_Cmd(NewState: bool) {}
//RTC计数器启动等待函数
pub fn Rtc_StartWait() {}
//RTC的1Hz输出的使能或停止
pub fn Rtc_Hz1Cmd(pricision: en_rtc_hz1sel_t, NewState: bool) {}
//设置周期中断的类型(PRDSEL)及其所选类型的时间(PRDS或PRDX)
pub fn Rtc_SetCyc(pstCyc: &stc_rtc_cyccfg_t) -> en_result_t {
    en_result_t::Ok
}
//RTC闹钟中断的使能或停止
pub fn Rtc_AlmIeCmd(NewState: bool) {}
//RTC闹钟的使能或停止
pub fn Rtc_AlmEnCmd(NewState: bool) {}
//获取RTC闹钟中断状态位
pub fn Rtc_GetAlmfItStatus() -> bool {
    true
}
//清除RTC闹钟中断状态位
pub fn Rtc_ClearAlmfItStatus() {}
//清除RTC周期中断状态位
pub fn Rtc_ClearPrdfItStatus() {}
//获取RTC周期中断状态位
pub fn Rtc_GetPridItStatus() -> bool {
    true
}
//配置RTC的误差补偿寄存器
pub fn Rtc_CompCfg(CompVlue: u16, NewStatus: en_rtc_compen_t) -> en_result_t {
    en_result_t::Ok
}
//RTC根据日期计算周数
pub fn Check_BCD_Format(u8data: u8, u8limit_min: u8, u8limit_max: u8) -> en_result_t {
    en_result_t::Ok
}
//RTC获取时间函数
pub fn Rtc_ReadDateTime(time: &stc_rtc_time_t) -> en_result_t {
    en_result_t::Ok
}
//向RTC时间寄存器写入时间
pub fn Rtc_SetTime(time: &stc_rtc_time_t) -> en_result_t {
    en_result_t::Ok
}
//RTC闹钟中断时间获取
pub fn Rtc_GetAlarmTime(pstcAlarmTime: &stc_rtc_alarmtime_t) {}
//RTC闹钟设置
pub fn Rtc_SetAlarmTime(pstcAlarmTime: &stc_rtc_alarmtime_t) -> en_result_t {
    en_result_t::Ok
}
//初始化RTC
pub fn Rtc_Init(Rtc_InitStruct: &stc_rtc_initstruct_t) {}
