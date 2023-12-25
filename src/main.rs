fn main() {
    exo_3()
}


use std::char::decode_utf16;
use std::collections::HashMap;
fn exo_1(){
    let mut list = vec![31,83,23,90,263];

    list.sort();

    println!("La médiane de la liste triée est {:?}", list[2]);

    let mut hashed_list = HashMap::new();

    for element in list {
        let counter = hashed_list.entry(element).or_insert(0);
        *counter +=1
    }

    for (key,value) in hashed_list {
        println!("La clé {} a pour valeur {}",key,value)
    }

}

fn exo_2( mut word : &str)->String{

    let mut chars = word.chars();

    let letter = match chars.next() {
        Some(c) => c,
        None => return String::new(),
    };

    match letter {
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' =>{
            format!("l{}muche",word)
        }
        _=> format!("l{}{}em",chars.as_str(),letter)
    }

}

use std::io;

fn exo_3(){

    let mut people_commercial:Vec<String> = vec![];
    let mut people_etude:Vec<String> = vec![];
    let mut people_pre_press:Vec<String> = vec![];

    let mut company_hierarchy = HashMap::new();

    company_hierarchy.insert("commercialService",&people_commercial);
    company_hierarchy.insert("EtudeService",&people_etude);
    company_hierarchy.insert("PrePressService",&people_pre_press);


    println!("Bienvenue dans l'entreprise!");
    println!("Voici les services disponibles au sein de l'entreprise:");
    let mut counter = 1;
    for (key,value) in &company_hierarchy {
        println!("-{} ({})",key,counter);
        counter+=1
    }


    println!("A quel service voulez vous ajouter quelqu'un? 1/2/3");
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("oups petite erreur");


    println!("Et quel sera le nom du nouvel employé?");
    let mut employee_name = String::new();
    io::stdin()
        .read_line(&mut employee_name)
        .expect("oups erreur");

    match answer.trim() {


        "1" =>{
            println!("Service Commercial");
            people_commercial.push(employee_name)
        },
        "2" =>{
            println!("Service Etude");
            people_etude.push(employee_name)
        },
        "3" =>{
            println!("Service Etude");
            people_pre_press.push(employee_name)
        },
        _=>{println!("nope")}
    }

}