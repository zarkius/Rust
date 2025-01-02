use glium::index::PrimitiveType;
use glium::{implement_vertex, Surface};
use sys_info::{self};
use glium::glutin::{event_loop::EventLoop, window::WindowBuilder};

fn generar_ventana_linux() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("Reemplazar por tu titulo")
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
    
        // Definir los vértices del triángulo
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
        }
    
        implement_vertex!(Vertex, position);
    
        let vertex1 = Vertex { position: [-0.5, -0.5] };
        let vertex2 = Vertex { position: [ 0.0,  0.5] };
        let vertex3 = Vertex { position: [ 0.5, -0.25] };
        let shape = vec![vertex1, vertex2, vertex3];
    
        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
        let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0u16, 1, 2]).unwrap();
    
        // Definir los shaders
        let vertex_shader_src = r#"
            #version 140
            in vec2 position;
            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;
    
        let fragment_shader_src = r#"
            #version 140
            out vec4 color;
            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;
    
        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 1.0, 1.0);
            target.draw(&vertex_buffer, &index_buffer, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
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