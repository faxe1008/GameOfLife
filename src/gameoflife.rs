extern crate rand;

use crossbeam_utils::thread;
use rand::Rng;
use std::collections::HashSet;
use std::sync::Arc;

pub struct GameOfLife {
    width: u32,
    height: u32,
    live_cells: HashSet<u32>,
}

impl GameOfLife {
    pub fn new(width: u32, height: u32) -> Self {
        GameOfLife {
            width,
            height,
            live_cells: HashSet::with_capacity((width * height / 2) as usize),
        }
    }

    fn live_neighbours(&self, pos: u32) -> usize {
        let indicies: [i64; 8] = [
            pos as i64 - self.width as i64 - 1,
            pos as i64 - self.width as i64,
            pos as i64 - self.width as i64 + 1,
            pos as i64 - 1,
            pos as i64 + 1,
            pos as i64 + self.width as i64 - 1,
            pos as i64 + self.width as i64,
            pos as i64 + self.width as i64 + 1,
        ];

        indicies
            .iter()
            .filter(|&&i| {
                i >= 0
                    && i < (self.width * self.height) as i64
                    && self.live_cells.contains(&(i as u32))
            })
            .count()
    }

    fn next_gen(&self, start: u32, stop: u32) -> HashSet<u32> {
        (start..stop)
            .into_iter()
            .filter(
                |&i| match (self.live_neighbours(i), self.live_cells.contains(&i)) {
                    (2, true) | (3, _) => true,
                    _ => false,
                },
            )
            .collect::<HashSet<u32>>()
    }

    pub fn next_generation(&mut self) {
        let size = self.width * self.height;
        let half = (size / 2) as u32;
        let mut new_cells = HashSet::<u32>::with_capacity(half as usize);
        thread::scope(|scope| {
            // `move` is not necessary for `self`, because the closure only uses it by shared reference, which is fine in scoped threads.
            // If you needed `move` to capture other variables, you might have to add it back.
            let h1 = scope.spawn(|_| self.next_gen(0, half));
            let h2 = scope.spawn(|_| self.next_gen(half, size));

            new_cells.extend(&h1.join().unwrap());
            new_cells.extend(&h2.join().unwrap());
        })
        .unwrap();
        self.live_cells = new_cells;
    }

    pub fn seed(&mut self, n: u32) {
        let mut rng = rand::thread_rng();
        for _i in 0..n {
            self.live_cells
                .insert(rng.gen_range(0..(self.width * self.height)));
        }
    }

    pub fn show(&self) {
        let mut out = String::with_capacity(((self.width + 2) * self.height) as usize);
        for i in 0..self.height {
            for j in 0..self.width {
                if self.live_cells.contains(&(i * self.width + j)) {
                    out.push('█');
                } else {
                    out.push(' ');
                }
            }
            out.push('\n');
        }
        print!("\x1B[2J");
        print!("{}", out);
    }
}
