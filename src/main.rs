use std::io;
use sys_info::{self};

fn imprimir_info_sistema() {
    let os_version = sys_info::os_release().unwrap_or_else(|_| "Desconocida".to_string());
    let os_type = sys_info::os_type().unwrap_or_else(|_| "Desconocido".to_string());
    let cpu_vendor = "Desconocido".to_string();
    let cpu_frequency = sys_info::cpu_speed().unwrap_or_else(|_| 0);
    let cpu_num_cores = sys_info::cpu_num().unwrap_or_else(|_| 0);
    let mem_info = sys_info::mem_info().unwrap_or_else(|_| sys_info::MemInfo {
        total: 0,
        free: 0,
        avail: 0,
        buffers: 0,
        cached: 0,
        swap_total: 0,
        swap_free: 0,
    });
    let disk_info = sys_info::disk_info().unwrap_or_else(|_| sys_info::DiskInfo {
        total: 0,
        free: 0,
    });
    let hostname = sys_info::hostname().unwrap_or_else(|_| "Desconocido".to_string());
    let loadavg = sys_info::loadavg().unwrap_or_else(|_| sys_info::LoadAvg {
        one: 0.0,
        five: 0.0,
        fifteen: 0.0,
    });

    println!("Información del sistema:");

    println!("Versión del SO: {}", os_version);
    println!("Tipo de SO: {}", os_type);
    println!("Memoria total: {} KB", mem_info.total);
    println!("CPU: {} MHz, {} cores", cpu_frequency, cpu_num_cores);
    println!("Disco total: {} KB", disk_info.total);
    println!("Disco libre: {} KB", disk_info.free);
    println!("Nombre del host: {}", hostname);
    println!("Promedio de carga: {} {} {}", loadavg.one, loadavg.five, loadavg.fifteen);
    println!("Vendor de CPU: {}", cpu_vendor);
}

fn main() {
        imprimir_info_sistema();
        println!("\n\n\n");
    let mut nombre_usuario = String::new();
    println!("Hola, ¿cómo te llamas?");
    io::stdin().read_line(&mut nombre_usuario).expect("Error al leer la entrada");
    println!("Hola, {} , tu aventura te aguarda!", nombre_usuario.trim());
}