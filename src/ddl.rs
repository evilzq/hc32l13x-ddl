use core::ptr;
/// # Safety
/// 注意检查内存地址有效性，请务必核实芯片手册
pub unsafe fn SetBit(addr: *mut u32, offset: u32, bFlag: bool) {
    if bFlag {
        let mut dst = ptr::read(addr);
        dst |= (0x1 << offset);
        ptr::write(addr, dst);
    } else {
        let mut dst = ptr::read(addr);
        dst &= !(0x1 << offset);
        ptr::write(addr, dst);
    }
}
/// # Safety
/// 注意检查内存地址有效性，请务必核实芯片手册
pub unsafe fn GetBit(addr: *mut u32, offset: u32) -> bool {
    ((ptr::read(addr) >> offset) & 0x1) != 0
}
