use once_cell::sync::Lazy;
use prettytable;
use std::env;
use std::process::Command;
use sysinfo;
use whoami;

static SYS: Lazy<sysinfo::System> = Lazy::new(|| {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();
    sys
});

static TABLE_FORMAT: Lazy<prettytable::format::TableFormat> = Lazy::new(|| {
    use prettytable::format;
    format::FormatBuilder::new()
        .column_separator('│')
        .borders('│')
        .separator(
            format::LinePosition::Top,
            format::LineSeparator::new('─', '┬', '┌', '┐'),
        )
        .separator(
            format::LinePosition::Title,
            format::LineSeparator::new('─', '┼', '├', '┤'),
        )
        .separator(
            format::LinePosition::Bottom,
            format::LineSeparator::new('─', '┴', '└', '┘'),
        )
        .separator(
            format::LinePosition::Intern,
            format::LineSeparator::new('─', '┼', '├', '┤'),
        )
        .padding(1, 1)
        .build()
});

fn format_size(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = KIB * 1024;
    const GIB: u64 = MIB * 1024;

    if bytes >= GIB {
        format!("{:.2} GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.2} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.2} KiB", bytes as f64 / KIB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn get_table() -> prettytable::Table {
    let mut table = prettytable::Table::new();
    table.set_format(*TABLE_FORMAT);
    table
}

pub fn get_mem() -> String {
    let total_memory = SYS.total_memory();
    let used_memory = SYS.used_memory();

    format!(
        "{} / {}",
        format_size(used_memory),
        format_size(total_memory)
    )
}

pub fn get_swap() -> String {
    let total_swap = SYS.total_swap();
    let used_swap = SYS.used_swap();

    format!("{} / {}", format_size(used_swap), format_size(total_swap))
}

pub fn get_host() -> String {
    let user = whoami::username();
    let hostname = sysinfo::System::host_name().unwrap();
    format!("{}@{}", user, hostname)
}

pub fn get_os() -> String {
    let os_name = sysinfo::System::name().unwrap();
    let os_version = sysinfo::System::os_version().unwrap();
    let os_arch = whoami::arch().to_string();
    format!("{} {} {}", os_name, os_version, os_arch)
}

pub fn get_kernel() -> String {
    let kernel_version = sysinfo::System::kernel_version().unwrap();
    kernel_version
}

pub fn get_uptime() -> String {
    let uptime = sysinfo::System::uptime();
    let days = uptime / 86400;
    let hours = (uptime % 86400) / 3600;
    let minutes = (uptime % 3600) / 60;
    let seconds = uptime % 60;

    format!(
        "{} days, {} hours, {} mins, {} secs",
        days, hours, minutes, seconds
    )
}

pub fn get_cpu() -> String {
    let cpu = SYS.cpus().first().unwrap();
    cpu.brand().to_string()
}

#[cfg(target_os = "windows")]
pub fn get_gpu() -> Vec<String> {
    let output = Command::new("cmd")
        .args(&["/C", "wmic path win32_videocontroller get caption"])
        .output()
        .expect("Failed to execute command");
    let res = String::from_utf8_lossy(&output.stdout).to_string();
    let res = res.split("\r\r\n").collect::<Vec<&str>>();
    let mut gpus = Vec::new();
    for i in res {
        if !i.is_empty() {
            gpus.push(i.trim().to_string());
        }
    }
    gpus
}

#[cfg(target_os = "linux")]
pub fn get_gpu() -> Vec<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("lspci | grep VGA")
        .output()
        .expect("Failed to execute command");

    let res = String::from_utf8_lossy(&output.stdout).to_string();
    let res = res.split("\r\r\n").collect::<Vec<&str>>();
    let mut gpus = Vec::new();
    for i in res {
        if !i.is_empty() {
            gpus.push(i.trim().to_string());
        }
    }
    gpus
}

#[cfg(target_os = "macos")]
pub fn get_gpu() -> Vec<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("system_profiler SPDisplaysDataType | grep Chipset")
        .output()
        .expect("Failed to execute command");

    let res = String::from_utf8_lossy(&output.stdout).to_string();
    let res = res.split("\r\r\n").collect::<Vec<&str>>();
    let mut gpus = Vec::new();
    for i in res {
        if !i.is_empty() {
            gpus.push(i.trim().to_string());
        }
    }

    gpus
}
