use std::io::{self, Read};

/// Zet een karakter om bij een rotatie van 90° in wijzerzin.
///
/// Bij een rotatie veranderen sommige ASCII-symbolen van richting:
/// - '-' wordt '|'
/// - '|' wordt '-'
/// - '/' wordt '\'
/// - '\' wordt '/'
/// - '+' blijft hetzelfde (symmetrisch)
fn rotate_char(c: char) -> char {
    match c {
        '-' => '|',
        '|' => '-',
        '/' => '\\',
        '\\' => '/',
        '+' => '+',
        _ => c, // Volgens de opgave komen geen andere tekens voor
    }
}

/// Draait een volledig patroon 90° in wijzerzin.
///
/// Een patroon is een 2D matrix (Vec<Vec<char>>).
/// Als de oorspronkelijke grootte h x w is, dan wordt de nieuwe grootte w x h.
/// Formule voor 90° klokwijzerzin-rotatie:
/// element (i, j) → (j, h - 1 - i)
fn rotate(pattern: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let h = pattern.len();        // aantal rijen (hoogte)
    let w = pattern[0].len();     // aantal kolommen (breedte)

    // Nieuwe matrix met omgewisselde dimensies
    let mut result = vec![vec![' '; h]; w];

    // Loop over alle elementen van de originele matrix
    for i in 0..h {
        for j in 0..w {
            // Plaats geroteerd element op correcte positie
            // én pas het teken zelf aan via rotate_char
            result[j][h - 1 - i] = rotate_char(pattern[i][j]);
        }
    }

    result
}

/// Naait twee patronen horizontaal aan elkaar.
///
/// De eerste parameter (left) komt links.
/// De tweede parameter (right) komt rechts.
///
/// Voorbeeld:
/// AB      CD
/// EF  +   GH
///
/// Wordt:
/// ABCD
/// EFGH
///
/// We combineren dus rij per rij.
fn sew(left: Vec<Vec<char>>, right: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    // Neemt telkens één rij van links en één rij van rechts
    for (row_l, row_r) in left.into_iter().zip(right.into_iter()) {
        let mut new_row = row_l;

        // Voegt de rechterrij toe achter de linkerrij
        new_row.extend(row_r);

        result.push(new_row);
    }

    result
}

fn main() {
    // Leest volledige input in één keer in een string
    // (sneller en eenvoudiger voor wedstrijdproblemen)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Splits input per lijn
    let mut lines = input.lines();

    // Aantal testcases
    let t: usize = lines.next().unwrap().trim().parse().unwrap();

    // Zorgt ervoor dat we geen extra lege regel
    // vóór de eerste output of na de laatste printen
    let mut first_output = true;

    // Verwerkt elke testcase apart
    for _ in 0..t {

        // Leest aantal basispatronen
        let p: usize = lines.next().unwrap().trim().parse().unwrap();

        // Opslag van alle basispatronen
        // patterns[i] bevat patroon nummer i+1
        let mut patterns: Vec<Vec<Vec<char>>> = Vec::new();

        // Elk patroon bestaat uit exact 2 regels van 2 tekens
        for _ in 0..p {
            let row1: Vec<char> = lines.next().unwrap().chars().collect();
            let row2: Vec<char> = lines.next().unwrap().chars().collect();
            patterns.push(vec![row1, row2]);
        }

        // Leest aantal bevelen
        let b: usize = lines.next().unwrap().trim().parse().unwrap();

        // Stapel (LIFO-structuur)
        // Hierop voeren we de stack-machine uit
        let mut stack: Vec<Vec<Vec<char>>> = Vec::new();

        // Verwerkt alle bevelen
        for _ in 0..b {
            let cmd = lines.next().unwrap().trim();

            if cmd == "stop" {
                // Programma stopt voor deze testcase
                break;
            } 
            else if cmd == "naai" {
                // Neemt bovenste twee quilts van de stapel
                let top = stack.pop().unwrap();      // bovenste
                let second = stack.pop().unwrap();   // daaronder

                // Volgens opgave:
                // bovenste komt LINKS van de andere
                stack.push(sew(top, second));
            } 
            else if cmd == "draai" {
                // Draait bovenste quilt 90° wijzerzin
                let top = stack.pop().unwrap();
                stack.push(rotate(top));
            } 
            else if cmd == "teken" {
                // Print lege regel tussen outputs
                if !first_output {
                    println!();
                }
                first_output = false;

                // Toont huidige quilt (top van de stapel)
                let top = stack.last().unwrap();

                for row in top {
                    // Converteert rij van Vec<char> naar String
                    let line: String = row.iter().collect();
                    println!("{}", line);
                }
            } 
            else {
                // Anders is het een nummer → basispatroon pushen
                let idx: usize = cmd.parse().unwrap();

                // idx-1 omdat vector 0-indexed is
                stack.push(patterns[idx - 1].clone());
            }
        }
    }
}
