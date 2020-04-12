/* Ejercicio 13: Structs en Rust
 * Mostrar la información de un struct que tenga de información el nombre y la edad.
 * Autor: Victor Suarez <zerasul@gmail.com>
*/

struct Persona{
    //TODO:Rellenar el struct
}

impl Persona{
    fn mostrarinfo(self){
        //TODO: Mostrar por pantalla el nombre y la edad de la persona
    }
}

fn main(){
 let p:Persona=Persona{nombre: String::from("Victor"), edad: 33};

 p.mostrarinfo();
}