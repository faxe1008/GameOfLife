mod gameoflife;
use gameoflife::GameOfLife;
use std::{thread, time};

fn main() {
    let mut gol = GameOfLife::new(150, 60);
    gol.seed(2000);

    for i in 0..100000 {
        gol.show();
        gol.next_generation();
        if i % 1000 == 0 {
            gol.seed(1500);
        }
        thread::sleep(time::Duration::from_millis(50));
    }
}
