use std::{collections::btree_map::Keys, fs::{File, read}, io::{self, BufRead, Read}};

fn main() -> std::io::Result<()> {

    
    /*let mut string_numbers_vector :Vec<String> = Vec::new();

    string_numbers_vector.push(String::from("3"));
    string_numbers_vector.push(String::from("3"));
    string_numbers_vector.push(String::from("23"));
    string_numbers_vector.push("42".to_string());
    string_numbers_vector.push("13".to_string());
    string_numbers_vector.push("2".to_string());
    string_numbers_vector.push("100".to_string());
    string_numbers_vector.push("97".to_string());
    string_numbers_vector.push("1".to_string());
    string_numbers_vector.push("42".to_string());
    */

    /*let file: File = File::open("listNumbers.txt")?;
    let reader = io::BufReader::new(file);
    
    for line_result in reader.lines() {
        let line: String = line_result?;
        string_numbers_vector.push(line);
    }*/

    
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    //eprintln!("RAW INPUT:\n{}", input);

    let int_numbers_vector: Vec<i32> = input.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
    
    let mut iter = int_numbers_vector.into_iter();
    let numberofarrays = iter.next().unwrap();


    let mut arrays = Vec::with_capacity(numberofarrays as usize);

    for _ in 0..numberofarrays {

        let arraysize = iter.next().expect("missing array size") as usize;

        let mut arr:Vec<i32> = iter.by_ref().take(arraysize).collect();

        arrays.push(arr);
    }

    for (array_index, array) in arrays.iter().enumerate() {
        print!("{} ", array_index + 1);
        let mut min = 0;
        let mut max= 0;
        for numbers in array
        {
            if (min == 0 || *numbers < min) {
                min = *numbers
            }
            if (*numbers > max) {
                max = *numbers;
            }
        }
        print!("{} {}\n", min, max)

    }


    /*for numbers in string_numbers_vector {
        println!("{numbers}");
    }

    for numbers in int_numbers_vector {
        let stringnumber:String = numbers.to_string();
        println!("{stringnumber}");
    }

    println!("{}", numberofarrays);
    
    for aray in arrays {
        print!("[");
        for numbers in aray
        {
            print!("{} ", numbers);
        }
        print!("], ");
    }*/

    Ok(())
}
