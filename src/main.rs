use prettytable::row;
use selfetch::*;
fn main() {
    use sysinfo::{Components, Disks, Networks, System};

    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // We display all disks' information:
    println!("=> disks:");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("{disk:?}");
    }

    // ========================
    let mut table = get_table();

    let host = get_host();
    table.add_row(row!["Host", host]);

    let os = get_os();
    table.add_row(row!["OS", os]);

    let kernel = get_kernel();
    table.add_row(row!["Kernel", kernel]);

    let mem = get_mem();
    table.add_row(row!["Memory", mem]);
    let swp = get_swap();
    table.add_row(row!["Swap Memory", swp]);

    table.printstd();
}
