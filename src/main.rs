mod gameoflife;
use gameoflife::GameOfLife;
use std::{thread, time};

fn main() {
    let mut gol = GameOfLife::new(1920, 1080);
    gol.seed(2000);

    for _i in 0..100 {
        gol.next_generation();
    }
}
