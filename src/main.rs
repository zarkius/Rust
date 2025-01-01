use sys_info::{self};
extern crate glium;
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowBuilder;
use glium::Surface;

fn generar_ventana_linux() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("Reemplaza por tu título")
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(800.0, 600.0))
        .with_min_inner_size(glium::glutin::dpi::LogicalSize::new(400.0, 300.0));
    // Tamaño mínimo de la ventana
    
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



    fn imprimir_info_sistema() {
        let os_type = sys_info::os_type().unwrap_or_else(|_| "Desconocido".to_string());
        let mem_info = format!("{} KB total, {} KB libre", sys_info::mem_info().unwrap_or_else(|_| sys_info::MemInfo {
            total: 0,
            free: 0,
            avail: 0,
            buffers: 0,
            cached: 0,
            swap_total: 0,
            swap_free: 0,
        }).total, sys_info::mem_info().unwrap_or_else(|_| sys_info::MemInfo {
            total: 0,
            free: 0,
            avail: 0,
            buffers: 0,
            cached: 0,
            swap_total: 0,
            swap_free: 0,
        }).free);
        let disk_info = format!("{} KB total, {} KB libre", sys_info::disk_info().unwrap_or_else(|_| sys_info::DiskInfo {
            total: 0,
            free: 0,
        }).total, sys_info::disk_info().unwrap_or_else(|_| sys_info::DiskInfo {
            total: 0,
            free: 0,
        }).free);


        let array_strings: [String; 3] = [os_type, mem_info, disk_info];

        println!("Información del sistema: {} \n {} \n {} \n", array_strings[0], array_strings[1], array_strings[2]);
    }

#[cfg(target_os = "windows")]
fn main() {
    imprimir_info_sistema();
    generar_ventana_windows();
}

#[cfg(target_os = "linux")]
fn main() {
    imprimir_info_sistema();
    generar_ventana_linux();
}

#[cfg(target_os = "macos")]
fn main() {
    imprimir_info_sistema();
    generar_ventana_macos();
}