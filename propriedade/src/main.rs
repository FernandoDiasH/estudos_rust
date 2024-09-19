fn main() { 
    let s = String::from("hello");

    takes_ownership(s);
    //println!("{s}"); // Nesse caso é preciso fazer uma copia da variavel para passar a variavel
                     // para a funcao
                     // Ou podemos passar uma referencia para a funcao
    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string:String){
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {

    println!("{some_integer}");
}
