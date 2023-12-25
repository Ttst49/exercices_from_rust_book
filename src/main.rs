fn main() {
    exo_1()
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

