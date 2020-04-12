# Modulos

Un modulo en Rust, es una coleccion de funciones y datos (structs,enums,etc...); que nos permiten organizar nuestro código; gracias a estos modulos podemos utilizar en caso necesario otros modulos y reutilizar nuestro código de forma sencilla; seguidamente se muestra como crear un modulo en Rust:

```rust
mod mimodulo{
    ....
}
```

Los modulos normalmente, se almacenan en distintas carpetas y pueden estar anidados para organizar mejor las distintas funcionalidades de nuestro modulo.

```rust
mod modulopadre{
    mod modulohijo{
        ...
    }
}
```

Para poder cargar y usar todas las funcionalidades que nos provee un modulo usaremos la instrucción ```use```:

```rust
use std::convert::f64
```

Otro aspecto a tener en cuenta es la visibilidad de las funciones y modulos; ya que muchas veces solo queremos que ciertas funciones puedan ser usada por nuestro modulo; es por ello, que se utiliza la palabra reservada ```pub``` para definir las funciones y modulos publicos. Por defecto son privados y solo pueden ser usados por su modulo y sus hijos.

```rust
pub mod modulopadre{ //Modulo publico

    pub fn mifuncion(x:i32)->i32{//funcion publica
        ...
    }
    mod modulohijo{//modulo privado
        ...
    }
}
```

