use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    // Leest volledige input in één string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Splitst op whitespace (werkt voor alle standaard programmeerwedstrijd-input)
    let mut iter = input.split_whitespace();

    // Aantal testcases
    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        // k = aantal rijen
        let k: usize = iter.next().unwrap().parse().unwrap();
        // n = aantal kolommen
        let n: usize = iter.next().unwrap().parse().unwrap();

        // Leest hoogtekaart in
        let mut height = vec![vec![0i32; n]; k];
        for i in 0..k {
            for j in 0..n {
                height[i][j] = iter.next().unwrap().parse().unwrap();
            }
        }

        // Print resultaat voor deze testcase
        println!("{}", trap_rain_water(&height, k, n));
    }
}

/// Berekent hoeveel water kan worden vastgehouden in een 2D hoogtekaart.
///
/// Idee:
/// - Start van de randen (water kan daar wegstromen).
/// - Gebruik een min-heap (priority queue).
/// - Werk van laag naar hoog, zoals Dijkstra.
/// - Houd de hoogste rand bij die we tot nu toe gezien hebben.
/// - Als een binnenpunt lager is dan die rand, kan het water vasthouden.
fn trap_rain_water(height: &Vec<Vec<i32>>, k: usize, n: usize) -> i32 {
    // Als er minder dan 3 rijen of kolommen zijn,
    // is er geen binnengebied om water vast te houden.
    if k <= 2 || n <= 2 {
        return 0;
    }

    // Houdt bij welke cellen al verwerkt zijn
    let mut visited = vec![vec![false; n]; k];

    // Min-heap via Reverse (BinaryHeap is standaard max-heap)
    let mut heap = BinaryHeap::new();

    // === Stap 1: Voeg alle randcellen toe ===
    // Randen bepalen het "lek-niveau" van de bak

    // Linker- en rechterkolom
    for i in 0..k {
        heap.push(Reverse((height[i][0], i, 0)));
        heap.push(Reverse((height[i][n - 1], i, n - 1)));
        visited[i][0] = true;
        visited[i][n - 1] = true;
    }

    // Bovenste en onderste rij
    for j in 0..n {
        heap.push(Reverse((height[0][j], 0, j)));
        heap.push(Reverse((height[k - 1][j], k - 1, j)));
        visited[0][j] = true;
        visited[k - 1][j] = true;
    }

    // 4 mogelijke richtingen (boven, onder, links, rechts)
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut water = 0;       // totale hoeveelheid water
    let mut max_height = 0;  // hoogste rand die we tot nu toe tegenkwamen

    // === Stap 2: Verwerk cellen van laag naar hoog ===
    while let Some(Reverse((h, x, y))) = heap.pop() {

        // Update hoogste randniveau
        max_height = max_height.max(h);

        // Bekijkt de 4 buren
        for (dx, dy) in directions.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            // Controleert of buur binnen rooster ligt
            if nx >= 0 && nx < k as i32 && ny >= 0 && ny < n as i32 {
                let nx = nx as usize;
                let ny = ny as usize;

                if !visited[nx][ny] {
                    visited[nx][ny] = true;

                    let nh = height[nx][ny];

                    // Als buur lager is dan huidige rand,
                    // kan daar water blijven staan
                    if nh < max_height {
                        water += max_height - nh;
                    }

                    // Voegt buur toe aan heap
                    // Belangrijk: we pushen zijn originele hoogte
                    heap.push(Reverse((nh, nx, ny)));
                }
            }
        }
    }

    water
}

