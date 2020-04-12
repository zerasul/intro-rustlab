# Option y Result

En Rust, existen 2 tipos especiales que nos pueden ayudar la hora de manejar distintos resultados de una misma función; ya sea debido por ejemplo a un error o que pueda devolver un dato vacío.

## Option

Option es un tipo de dato que nos permite definir 2 opciones posibles; ```Some```y ```None```. De tal forma que en caso de que una operación tenga éxito, puede devolver el caso de ```Some``` o ```None``` en caso contrario. El tipo Option, tiene un tipo parametrizado que define en caso de que devuelva algo, el dato que tiene asignado.

```rust
fn mifuncion(x:i32)-> Option<i32>{

    if x>0{
        Some(4)
    }else{
        None
    }
}
```

En el caso anterior por ejeplo, vemos que la función ```mifuncion```, devuelve un tipo ```Option``` con un tipo ```i32```; vemos como la función, puede devolver ```Some``` o ```None```; para establecer que devuelve un ```Some``` podemos establecer el valor devuelto entre parentesis.

Para poder manejar los datos devueltos, podemos usar la estructura de control ```match```:

```rust
let x=mifuncion(4);

match x {
    Some(x) => println!("x vale {}", x),
    None => println!("Ha devuelto None")
}
```


## Result

Otro tipo de dato especial es ```Result```, el cual nos permite definir que ocurre cuando una operación devuelve un dato correcto o uno erroneo. En este caso se pueden definir los dos casos devolviendo dos tipos de datos en función si ha dado ```Ok``` o ```Err```. Ejemplo:

```rust
fn mifuncion(x:i32)-> Result<i32,String>{//A diferencia de Option, aquí hay dos tipos parametrizados; uno para OK y otro para Err.
    if(x>=0){
        Ok(4)
    }else{
        Err("Ha ocurrido un error".to_string())
    }
}
```

Este tipo de dato se utiliza para poder controlar tanto los casos de éxito como los errores a la hora de manipular estos.

```rust
match mifuncion(-1){
    Ok(x) => println!("Ha devuelto: {}", x),
    Err(err) => println!("Ha ocurrido un error: {}", err),
};
```