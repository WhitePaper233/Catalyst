use std::{thread, time};
use sysinfo::{CpuRefreshKind, RefreshKind, SystemExt};
use tauri::Window;

use super::statusbar_info;

#[tauri::command]
pub fn emit_sysinfo(window: Window) {
    static mut FLAG: bool = false;

    unsafe {
        if FLAG {
            println!("Already emitted");
            return;
        }

        thread::spawn(move || loop {
            let mut usage_info = statusbar_info::StatusBarUsageInfo::new();
            let mut sys = sysinfo::System::new_with_specifics(
                RefreshKind::new()
                    .with_cpu(CpuRefreshKind::new().with_cpu_usage())
                    .with_memory(),
            );
            thread::sleep(time::Duration::from_secs_f32(0.5));
            usage_info.update(&mut sys);
            window.emit("statusbar-info-update", usage_info).unwrap();
            FLAG = true;
            thread::sleep(time::Duration::from_secs_f32(0.5));
        });
    }
}
