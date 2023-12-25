fn main() {
    let pigify = exo_2("ecrire");
    println!("{}", pigify)
}



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
