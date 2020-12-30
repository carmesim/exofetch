use libc::sysinfo;
use std::mem;
#[derive(Debug)]
pub struct SysInfo {
    // Total usable main memory size
    pub uptime:     usize,
    pub total_ram:  usize,
    // Available memory size
    pub free_ram:   usize,
    pub shared_ram: usize,
    // Other possible info we could get from sysinfo():
    //    * Shared RAM, total and free swap
}

impl SysInfo {
    pub fn new () -> SysInfo {
        let mut sysinfo_s: sysinfo = unsafe { mem::zeroed() };

        let ret_val = unsafe { libc::sysinfo(& mut sysinfo_s) };

        if ret_val < 0 {
            panic!("libc::sysinfo failed.");
        };

        SysInfo {
            uptime:      sysinfo_s.uptime    as usize,
            total_ram:   sysinfo_s.totalram  as usize,
            free_ram:    sysinfo_s.freeram   as usize,
            shared_ram:  sysinfo_s.sharedram as usize,
        }
        
    }
}