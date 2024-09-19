fn main() {
    const THREE_HORS_IN_SECONDS: u32 = 60 * 60 * 3;
    let mut x = 5;
    println!("The value of x is is: {x}");
    x = 6;
    println!("The vaue of x is: {x}");
    println!("{THREE_HORS_IN_SECONDS}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is {x}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", tup.0);
    
    another_function();
    teste_variaveis();
    teste_retorno_funcao(5);
    teste_condicao(20);
    variavel_retorno_loop();
    loop_com_label();
}

fn loop_com_label(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remainig = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

}

fn variavel_retorno_loop(){

    let mut counter = 0;
    let result = loop {
        counter += 1;
        
        if counter == 10{
            break counter * 2;
        }
    };
    println!("O resultado é {result}");
}
fn teste_condicao(valor: i32){

    if valor < 50 {
        println!("valor é menor que 50");
    }
    else {
        println!("valor é maior que 50");
    }
}

fn teste_retorno_funcao(x: i32) -> i32{
    x + 1
}

fn teste_variaveis(){
    let y = {
        let x = 3;
        x + 1
    };
    println!("a variavel de dentro foi somada {y}");
}


fn another_function() {


    println!("another function");

}
