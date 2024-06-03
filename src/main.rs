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

    let mem = get_mem();
    table.add_row(row![Fyb -> "Memory", mem]);
    let swp = get_swap();
    table.add_row(row![Fcb -> "Swap Memory", swp]);

    table.printstd();
}
