
#[path="lib/variables.rs"]
mod variables; //Importamos el módulo variables.
#[path="lib/operaciones.rs"]
mod operaciones; //Importamos el módulo operaciones.
#[path="lib/adivinar_numero.rs"]
mod adivinar_numero; //Importamos el módulo adivinar_numero. REMINDER: Usamos snake_case.


fn main() {
    println!("Hello, world!");

    println!("Imprimimos la variable importada de 'variables': {}", variables::MI_CONSTANTE);
    variables::saludar(); //En Java sería algo como variables.saludar(), aquí se usan los dobles dos puntos "::".
    variables::desc_variables();

    operaciones::operaciones();
    
    adivinar_numero::adivinar_numero();

    /*let mut edad: String=String::new();

    println!("Inserta tu edad: ");

    //io::stdin().read_line(&mut edad)
        //.expect("Error al leer la entrada");

    //VER SI HAY QUE METER UN CRATE PARA IO O QUE PASA AQUI, VER SI CAMBIANDO EL NOMBRE DE MAIN.RS TAMBIEN FUNCIONA EL MAIN.
    //Cambiamos a la rama main. 

    let edad: i32 = edad.trim().parse()
        .expect("Error al convertir la edad a un número");

    if edad < 18 {
        println!("Eres MENOR de edad.");
    } else {
        println!("Eres MAYOR de edad.");
    }*/

    println!("El programa ha terminado correctamente.");
}
