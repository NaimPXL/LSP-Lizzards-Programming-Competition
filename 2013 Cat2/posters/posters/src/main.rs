use std::{io::{self, Read}};

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let vec: Vec<String> = input.lines().map(|s|s.to_string()).collect();
    
    /*for numbers in vec {
    println!("{numbers}");
    }*/
    
    let mut iter = vec.into_iter();
    
    let aantal_opgaven = iter.next().unwrap().parse::<i64>().unwrap();

    for _ in 0..aantal_opgaven {
        
        let aantal_posters = iter.next().unwrap().parse::<i64>().unwrap();

        let mut array = [0i64; 10000];

        let  coordinaten:Vec<String> = iter.by_ref().take(aantal_posters as usize).collect();

        for (i,position) in coordinaten.iter().enumerate().map(|(i, v)| (i + 1, v)) {
            let positie_lengte_vec:Vec<String> = position.split_whitespace().map(|s| s.to_string()).collect();
            
            //let pos = positie_lengte_vec.get(0).map(|s| s.parse::<i32>().unwrap()).unwrap();
            //let len = positie_lengte_vec.get(0).map(|s| s.parse::<i32>().unwrap()).unwrap();

            let poster_number = i;

            let pos:i64 = positie_lengte_vec[0].parse().unwrap();
            let len:i64 = positie_lengte_vec[1].parse().unwrap();
            for i in pos..len {
                array[i as usize] = poster_number as i64;
            }

            //array[pos as usize..=len as usize].fill(poster_number as i64);
        }

        let mut counter = 0;

        for number in array {
            if number > counter 
            {
                counter += 1;
            }
        }

        println!("{counter}");
    }

    /*
    for number in iter  {
        
        println!("{number}");
    }*/
    
    Ok(())
}
