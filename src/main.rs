use prettytable::row;

use selfetch::*;

fn main() {
    // ========================
    let mut table = get_table();

    let host = get_host();
    table.add_row(row![Frb -> "Host", host]);

    let os = get_os();
    table.add_row(row![Fbb -> "OS", os]);

    let kernel = get_kernel();
    table.add_row(row![Fgb -> "Kernel", kernel]);

    let uptime = get_uptime();
    table.add_row(row![Fbb -> "Uptime", uptime]);

    let cpu = get_cpu();
    table.add_row(row![Fyb -> "CPU", cpu]);

    let gpus = get_gpu();
    for i in 0..gpus.len() {
        let gpu = gpus[i].to_string();
        let gpu_idx = format!("GPU{}", i);
        table.add_row(row![Fcb -> gpu_idx, gpu]);
    }

    let mem = get_mem();
    table.add_row(row![Fyb -> "Memory", mem]);

    let swp = get_swap();
    table.add_row(row![Fcb -> "Swap Memory", swp]);

    table.printstd();

    println!("{}", get_shell().unwrap());
}
