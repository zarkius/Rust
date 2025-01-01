use std::io;

fn main() {
    let mut nombre_usuario = String::new();
    println!("Hola, ¿cómo te llamas?");
    io::stdin().read_line(&mut nombre_usuario).expect("Error al leer la entrada");
    println!("Hola, {} , tu aventura te aguarda!", nombre_usuario.trim());
}
