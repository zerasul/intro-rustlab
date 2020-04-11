# Bucles

En Rust, existen 3 tipos de bucles; ```for```, ```while``` y ```loop```.

## For

El bucle for, permite realizar una serie de acciones de forma repetitiva un número definido de veces.

```rust
for n in 0..10{
    println!("Iteración {}", n);
}
```

aunque puede usarse para iterar en colecciones:

```rust
for a in collection.iter(){
    println!("Objeto de la coleccion: {}",a)
}
```

## While

Este bucle permite realizar acciones mientras se cumpla una condición:

```rust
let mut a:u16=0;

while a < 18{
    println!("a vale: {}",a);
    a+=1;
}
```

## loop

Este es un bucle infinito que estará funcionando mientras no se rompa el bucle con la instrucción ```break```.

```rust
let mut a:u16=0;
loop{

    println!("a vale: {}",a);
    a+=1;
    if a>=18{
        break;
    }

}
```

**NOTA:** También existe la instrucción ```continue``` que parará la ejecución de la pasada actual del bucle y continuará a la siguiente.