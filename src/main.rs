use crate::DiaDaSemana::{Quarta, Sabado};

fn main() {
    // notas();
    // matriz();
    println!("É fim de semana ? : {}", final_de_semana(dia));
}


fn notas() {
    let notas: [f32; 5] = [10f32, 8f32, 6f32, 5f32, 7.5];

    for i in 0..notas.len() {
        println!("A nota {} é {}", i + 1, notas[i])
    }
}

fn matriz() {
    let x = [
        [1, 2, 3],
        [5, 6, 7]
    ];

    for i in 0..x.len() {
        println!("Linha: {}", i+1);
        for j in 0..x[i].len() {
            println!("Coluna: {}", x[i][j]);
        }
    }
}
enum DiaDaSemana{
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado
}

fn final_de_semana(dia:DiaDaSemana) -> bool{
    let res:bool = match dia{
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ => false
    };
    res
}


