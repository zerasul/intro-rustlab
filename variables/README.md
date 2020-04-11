# Variables

En Rust, para declarar una variable se utiliza la palabra reservada ```let```.

```rust
let x = 1;
```

En este caso, hemos definido una variable sin tipado; el compilador infiere el tipo y lo asigna; siendo este de tipo ```i32``` (Entero con signo de 32 bits). Seguidamente se muestran los tipos de dato para números:

| Tipo | Descripcion |
| ---- | ----------- |
| ```i32```,```i64```,```i16```,... | Entero con signo (32,64,16 bits,...) |
| ```u32```,```u64```,```u16```,... | Entero sin signo (32,64,16 bits,...) |
| ```f32```,```f64```,... | Coma flotante (32,64 bits,...) |

Para definir un tipo de dato se usan los dos puntos después del nombre de la variable.

```rust
let x:u16=8;
```

## Mutabilidad

Por defecto las variables en Rust se definen como inmutables; esto quiere decir, que no puede cambiar su valor; ejemplo:

```rust

fn main(){
    let x=5;
    ...
    x=1;//ERROR; no se puede cambiar de valor
    ...
}
```

Para poder crear una variable para que pueda cambiar su valor, se añade la palabra reservada ```mut``` tras el ```let```.

```rust
fn main(){
    let mut x=5;
    ...
    x=1;//Ahora no da error.
    ...
}
```