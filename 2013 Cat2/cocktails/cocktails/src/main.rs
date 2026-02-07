use std::{
    collections::HashMap,
    io::{self, Read},
    vec,
};

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let vec: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    _second_attempt_function(vec);

    Ok(())
}

fn _third_attempt_function(vec: Vec<String>) {}

fn _second_attempt_function(vec: Vec<String>) {
    let mut iter = vec.into_iter();

    let aantal_opgaven = iter.next().unwrap().parse::<i64>().unwrap();

    for _ in 0..aantal_opgaven {
        let aantal_barmannen = iter.next().unwrap().parse::<usize>().unwrap();

        let barman_knowledge: Vec<String> = iter.by_ref().take(aantal_barmannen).collect();

        let aantal_orders = iter.next().unwrap().parse::<usize>().unwrap();

        let orders: Vec<String> = iter.by_ref().take(aantal_orders).collect();

        for i in 0..aantal_orders {
            let mut canmake_order = false;

            let current_order: Vec<char> = orders[i].chars().collect();
            let mut amount_of_orders_per_letter: HashMap<char, usize> = HashMap::new();
            for char in current_order.iter() {
                *amount_of_orders_per_letter.entry(*char).or_insert(0) += 1;
            }

            let mut pairs: Vec<(&char, &usize)> = amount_of_orders_per_letter.iter().collect();
            pairs.sort_by_key(|(_, count)| *count);

            let mut ordered_orders: Vec<char> = Vec::new();
            for (ch, count) in pairs {
                for _ in 0..*count {
                    ordered_orders.push(*ch);
                }
            }

            let mut barman_knowledge_copy = barman_knowledge.clone();

            for order in ordered_orders {
                if let Some(i) = barman_knowledge_copy
                    .iter()
                    .position(|knowledge| knowledge.chars().any(|ch| ch == order))
                {
                    barman_knowledge_copy.remove(i);
                    canmake_order = true;
                } else {
                    canmake_order = false;
                    break;
                }
            }

            // let mut barman_knowledge_copy = barman_knowledge.clone();
            // for order in ordered_orders {
            //     for (i, knowledge) in barman_knowledge_copy.iter().enumerate() {
            //         let knowledge_vec: Vec<char> = knowledge.chars().collect::<Vec<_>>();
            //         for ch in knowledge_vec {
            //             if order == ch {
            //                 barman_knowledge_copy.remove(i);
            //                 canmake_order = true;
            //                 break;
            //             } else {
            //                 canmake_order = false;
            //             }
            //         }
            //     }
            // }

            if canmake_order {
                println!("{} mogelijk", orders[i]);
            } else {
                println!("{} onmogelijk", orders[i]);
            }
        }
    }
}

fn _First_Attempt_function(vec: Vec<String>) {
    let mut iter = vec.into_iter();

    let aantal_opgaven = iter.next().unwrap().parse::<i64>().unwrap();

    for _ in 0..aantal_opgaven {
        let aantal_barmannen = iter.next().unwrap().parse::<usize>().unwrap();

        let cocktail_knowledge: Vec<String> = iter.by_ref().take(aantal_barmannen).collect();
        // for knowledge in cocktail_knowledge.iter() {
        //     println!("{knowledge}");
        // }

        let aantal_orders = iter.next().unwrap().parse::<usize>().unwrap();

        let orders: Vec<String> = iter.by_ref().take(aantal_orders).collect();

        // for knowledge in cocktail_knowledge.iter() {
        //     println!("Cocktail knowledge: {}", knowledge);
        // }
        // for cocktail in orders.iter() {
        //     println!("Orders: {}", cocktail);
        // }

        for i in 0..aantal_orders {
            let order_vec: Vec<char> = orders[i].chars().collect();
            let mut canmake_cocktail_in5 = true;
            for knowledge in cocktail_knowledge.iter() {
                let knowledge_vec: Vec<char> = knowledge.chars().collect();
                // for knowledge in knowledge_vec.iter() {
                //     println!("knowledge {}", knowledge);
                // }

                let mut canmake_current_coctail = false;
                let mut remaining_orders: Vec<char> = order_vec.clone();

                for char in knowledge_vec.iter() {
                    //println!("Current knowledge character being evaluated: {char}");
                    for (i, order) in remaining_orders.iter().enumerate() {
                        //println!("i: {}, order {}", i, order);
                        if *order == *char {
                            canmake_current_coctail = true;
                            remaining_orders.remove(i);
                            //println!("I am breaking and removing");
                            break;
                        } else {
                            canmake_current_coctail = false;
                        }
                        //println!("Cannmake Cocktail after check: {}", canmake_current_coctail);
                    }
                    if canmake_current_coctail {
                        break;
                    }
                }
                canmake_cocktail_in5 = canmake_current_coctail;
            }

            if canmake_cocktail_in5 {
                println!("{} mogelijk", orders[i]);
            } else {
                println!("{} onmogelijk", orders[i]);
            }
        }
    }
}
