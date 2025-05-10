use std::io;

fn input() -> String {
    let mut entrada : String = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler a entrada!");
    entrada.to_string()
}

fn main(){
    let mut entrada : String = input();
    let c1 : String = entrada.trim().to_string();
    entrada.clear();

    let mut entrada : String = input();
    let c2 : String = entrada.trim().to_string();
    entrada.clear();

    let mut entrada : String = input();
    let c3 : String = entrada.trim().to_string();
    entrada.clear();

    let resultado: &str = if c1 == "vertebrado" {
        if c2 == "ave" {
            if c3 == "carnivoro" {
                "aguia"
            } else if c3 == "onivoro" {
                "pomba"
            } else {
                "entrada invalida"
            }
        } else if c2 == "mamifero" {
            if c3 == "onivoro" {
                "homem"
            } else if c3 == "herbivoro" {
                "vaca"
            } else {
                "entrada invalida"
            }
        } else {
            "entrada invalida"
        }
    } else if c1 == "invertebrado" {
        if c2 == "inseto" {
            if c3 == "hematofago" {
                "pulga"
            } else if c3 == "herbivoro" {
                "lagarta"
            } else {
                "entrada invalida"
            }
        } else if c2 == "anelideo" {
            if c3 == "hematofago" {
                "sanguessuga"
            } else if c3 == "onivoro" {
                "minhoca"
            } else {
                "entrada invalida"
            }
        } else {
            "entrada invalida"
        }
    } else {
        "entrada invalida"
    };

    println!("{}", resultado);
}