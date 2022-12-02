use std::fs::File;
use std::num::ParseIntError;
use std::path::Path;
use std::io::{self, BufRead};


fn main() {

    let mut vec = Vec::new();

    if let Ok(lines) = read_lines("./elf_calories.csv") {
        let mut calories_single_elf: i32 = 0;
        for line in lines {
            if let Ok(line) = line {
                let res: Result<i32, ParseIntError> = line.trim().parse();  
                /*
                Se il parser di int legge correttamente allora c'è un numero,
                altrimenti c'è uno spazio vuoto e vuol dire che l'elfo è cambiato
                */

                if let Ok(res) = res {
                    calories_single_elf = calories_single_elf + res;
                } else {

                    vec.push(calories_single_elf);
                    calories_single_elf = 0;
                }
            }
            
        }
    }

    vec.sort();

    let len = vec.len();

    let max_val = vec.get(len - 1).unwrap();

    let second_max_val = vec.get(len - 2).unwrap();

    let third_max_val = vec.get(len - 3).unwrap();

    println!("Elfo con più calorie: {}", max_val);

    println!("Calorie dei 3 elfi con più calorie: {}", max_val + second_max_val + third_max_val)

    


}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}