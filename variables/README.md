# Variables

En Rust, para declarar una variable se utiliza la palabra reservada ```let```.

```rust
let x = 1;
```

En este caso, hemos definido una variable sin tipado; el compilador infiere el tipo y lo asigna; siendo este de tipo ```i32``` (Entero con signo de 32 bits). Seguidamente se muestran los tipos de dato para números:

| Tipo | Descripcion |
| ```i32```,```i64```,```i16```,... | Entero con signo (32,64,16 bits,...) |
| ```u32```,```u64```,```u16```,... | Entero sin signo (32,64,16 bits,...) |
| ```f32```,```f64```,... | Coma flotante (32,64 bits,...) |