use std::{io::{self, BufRead}, fs::File, path::Path};



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {

  
    let mut mosse: Vec<(char, char)> = Vec::new();

    let mut punti: i32 = 0;

    let data = read_lines("./rock_paper_scissor.csv")
        .unwrap();

    for line in data {
        
        match line {
            Ok(l) => {
                mosse.push((l.chars().nth(0).unwrap(), l.chars().nth(2).unwrap()))
            },
            Err(e) => {
                println!("Error {}", e)
            }
        }
    }

    for mossa in &mosse {
        match mossa.1 {
            'X' => {
                // rock +1

                punti = punti + 1;

                if mossa.0 == 'A' {
                     
                    // pari
                    punti = punti + 3;           
                } 

                if mossa.0 == 'B' {
                    // perso
                    
                } 

                if mossa.0 == 'C' {
                    //vinto
                    punti = punti + 6;            

                    
                } 
            },
            'Y' => {
                // paper +2
                punti = punti + 2;

                if mossa.0 == 'A' {
                    // vinto        
                    punti = punti + 6;            
                } 

                if mossa.0 == 'B' {
                    // pari
                    punti = punti + 3;           
                } 

                if mossa.0 == 'C' {
                    //perso
                } 
            },
            'Z' => {
                // scissor +3
                punti = punti + 3;

                if mossa.0 == 'A' {
                    // perso
                } 

                if mossa.0 == 'B' {
                    // vinto
                    punti = punti + 6;            

                } 

                if mossa.0 == 'C' {
                    // pari
                    punti = punti + 3;           
                } 
            },
            _ => {
                println!("Errore")
            },
            
        }
        
    }

    println!("Parte 1:  {}", punti);

    punti = 0;

    for mossa in &mosse {
        match mossa.0 {
            'A' => {
                // rock
                if mossa.1 == 'X' {
                    // perso
                    // forbici + 3
                    punti = punti + 3
                }   
                
                if mossa.1 == 'Y' {
                    // pari + 3
                    // sasso + 1
                    punti = punti + 1;
                    punti = punti + 3;

                }  

                if mossa.1 == 'Z' {
                    // vinto
                    // carta + 2
                    punti = punti + 2;
                    punti = punti + 6;
                }  
            },
            'B' => {
                // paper
                if mossa.1 == 'X' {
                    // perso
                    // sasso + 1
                    punti = punti + 1

                }   
                
                if mossa.1 == 'Y' {
                    // pari + 3
                    // carta + 2
                    punti = punti + 2;
                    punti = punti + 3;


                }  

                if mossa.1 == 'Z' {
                    // vinto
                    // forbici + 3
                    punti = punti + 3;
                    punti = punti + 6;
                }  
            },
            'C' => {
                // scissor
                if mossa.1 == 'X' {
                    // perso
                    // carta + 2
                    punti = punti + 2

                }   
                
                if mossa.1 == 'Y' {
                    // pari + 3
                    // forbici + 3 
                    punti = punti + 3;
                    punti = punti + 3;
                }  

                if mossa.1 == 'Z' {
                    // vinto
                    // sasso + 1
                    punti = punti + 1;
                    punti = punti + 6;

                }  
            },
            _ => {
                println!("Errore")
            },
            
        }
        
    }

    println!("Parte 2:  {}", punti);

    /*
        mosse fegli elfi

        A = sasso
        B = carta
        C = forbici
    */

    /*
        Risposte alle loro mosse

        X = sasso
        Y = carta
        Z = forbici
    */

    /*
        punti:
        - sasso = 1
        - carta = 2
        - forbici = 3


        - sconfitta = 0
        - pareggio = 3
        - vittoria = 6
    */
/*
    for (elfo, dmo) in &mosse {
        if elfo == dmo {
            // pareggio
            punti = punti + 3;
        } else if (dmo == &'X' && elfo == &'C') ||  (dmo == &'Y' && elfo == &'A') ||  (dmo == &'Z' && elfo == &'B'){
            // vittoria
            punti = punti + 6;
        }
    }


    println!("Punti finale{}", punti);
*/



    /*
    
    
    for mossa in mosse_dmo {
        match mossa {
            'X' => {
                points = points + 1;
            },
            'Y' => {
                points = points + 2;
            },
            'Z' => {
                points = points + 3;
            },
            _   => println!("Error")
        }
        
    }

    let numero_mosse = mosse_elfi.len();


    println!("{}", numero_mosse);
    println!("{}", points);

    for (i, m_elf) in mosse_elfi.iter().enumerate() {
        match m_elf {
            'A' => {
                if Eq('X', 'Z') {
                    
                }
            },

            _ => {
                println!("Error")
            }
        }
    }*/

    
    /*
        mosse fegli elfi

        A = sasso
        B = carta
        C = forbici
    */

    /*
        Risposte alle loro mosse

        X = sasso
        Y = carta
        Z = forbici
    */

    /*
        punti:
        - sasso = 1
        - carta = 2
        - forbici = 3


        - sconfitta = 0
        - pareggio = 3
        - vittoria = 6
    */


    
}


