# Structs y Enums

## Structs
En Rust, como en otros lenguajes como C, se permite definir estructuras de datos compuestas de tal forma que podemos definir nuestros propios tipos. 

```rust
struct Persona{
    nombre: String,
    edad: i32
};
```

Como vemos en el anterior ejemplo, para definir un Struct, se usa la palabra reservada ```struct``` seguido del nombre y la definición de este entre ```{}```.

Para definir una variable de un tipo de Struct, se usa el nombre de este:

```rust
let p:Persona;
```

Además, para acceder a los campos de un struct, usaremos el operador ```.``` seguido del nombre de este.

```rust
let p:Persona;

p.nombre=String::from("Victor");
```

Aunque también podemos inicializar directamente el Struct:

```rust
let p =Persona { nombre: String::from("Victor"), edad:33};
```

### Ampliar funcionalidad a un struct

Podemos añadir metodos a un Struct para que puedan darnos funcionalidades extras; como por ejemplo inicializar un nuevo struct,etc... Estos metodos pueden ser estáticos o funciones del struct.

```rust
impl Persona{
    fn new()->Persona{ //Metodo Estatico
        let p =Persona{ nombre:String::new(), edad=0};
        p 
    }

    fn show_properties(self){
        println!("Nombre: {}, edad: {}", self.nombre, self.edad);
    }
}
```

Para poder usar estos metodos:

```rust
...
let p:Persona= Persona::new();//Uso del metodo estatico para generar una nueva persona
p.show_properties();//Uso del metodo del struct persona
...
```

## Enums

Un Enum, es un tipo de dato que puede tener distintos estados posibles; En Rust se define de la siguiente forma:

```rust
enum Mienum{
    Valor1,
    Valor2,
    Valor3
};
```
Sin embargo, en Rust podemos asociar información adicional a cada estado de un Enum:

```rust
enum Mienum{
    Valor1(i32),
    Valor2{x:i32,y:i32},
    Valor3
};
```

Como podemos ver se puede definir en formato Struct o simplemente añadiendo un dato entre parentesis (como una tupla).

Ahora vamos a ver como se crea una variable con un valor de un Enum:

```rust
let v1:Mienum=Mienum::Valor1(3); //V1 es de tipo Mienum con valor 3.
```

También podemos añadir funciones a un Enum:

```rust
impl Mienum{
    fn show(&self){
        println!("{:?}", &self);
    }
}
```

### Match

A la hora de hablar de las estructuras de control condicionales, hemos mencionado la estructura ```match```; esta estructura nos va a permitir, en función del valor de una variable realizar una serie de acciones; e incluso poder establecer el valor de otra variable. Este es una de las estructura de control más importantes de Rust y que nos permitirá controlar por ejemplo los errores de nuestros programas, como veremos más adelante con las estructuras de ```Option```o ```Result```.

```rust
...
let m=Mienum::Valor1(3);

match m{
    Mienum::Valor1(n) => println!("El valor de n es: {}", n),
    Mienum::Valor2{x:x,y:y} => pritnln!("x vale: {} e y vale: {}", x,y),
    Mienum::Valor3 => println!("Valor 3")
};
```
