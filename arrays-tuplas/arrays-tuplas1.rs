fn main(){
    let a: Vec<i32> = (0..20).collect();

    //TODO: mostrar el doble del 0 al 10 y el triple del 11 al 20.
    let first_ten = &a[0..10];
    let next_ten = &a[10..20];

    for &num in first_ten {
        println!("Doble de {}: {}", num, num * 2);
    }

    for &num in next_ten {
        println!("Triple de {}: {}", num, num * 3);
    }
}
