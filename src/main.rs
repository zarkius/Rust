use std::io;
use sys_info::{self};
extern crate glium;
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowBuilder;
use glium::Surface;

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

    println!("Sistema Operativo: {} {}", os_type, os_version);
    println!("CPU: {} @ {} MHz, {} núcleos", cpu_vendor, cpu_frequency, cpu_num_cores);
    println!("Memoria: {} KB total, {} KB libre", mem_info.total, mem_info.free);
    println!("Disco: {} KB total, {} KB libre", disk_info.total, disk_info.free);
}

fn main() {
    imprimir_info_sistema();
    println!("\n\n\n");
    let mut nombre_usuario = String::new();
    println!("Hola, ¿cómo te llamas?");
    io::stdin().read_line(&mut nombre_usuario).expect("Error al leer la entrada");
    println!("Hola, {} , tu aventura te aguarda!", nombre_usuario.trim());
    
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("Ventana Nativa con Glium y Winit")
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(800.0, 600.0))
        .with_min_inner_size(glium::glutin::dpi::LogicalSize::new(400.0, 300.0)); // Tamaño mínimo de la ventana

    let context_builder = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = glium::glutin::event_loop::ControlFlow::Wait;

        match event {
            glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                glium::glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                },
                _ => (),
            },
            _ => (),
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();
    });
}