use crate::jeu_de_la_vie::GameOfLife;

use ndarray::Array2;

#[derive(Debug)]
enum Cell {
    Empty,
    Sim(Box<GameOfLife>),
}

#[derive(Debug)]
pub struct GameOfLifeInGameOfLife {
    big_N: usize,
    small_N: usize,
    vie: f64,
    big_grid: Array2<Cell>,
}

impl GameOfLifeInGameOfLife {
    pub fn new(big_N: usize, small_N: usize, vie: f64) -> Self {
        // Initialisation aléatoire : on utilise `rand::random` pour éviter
        // l'utilisation de la méthode `gen` qui est devenue un mot-clé.
        let big_grid = Array2::from_shape_fn((big_N, big_N), |_| {
            if rand::random::<f64>() < vie {
                Cell::Sim(Box::new(GameOfLife::new(small_N, vie)))
            } else {
                Cell::Empty
            }
        });

        Self {
            big_N,
            small_N,
            vie,
            big_grid,
        }
    }

    pub fn step(&mut self) {
        let snapshot_alive: Array2<bool> = Array2::from_shape_fn((self.big_N, self.big_N), |(i, j)| {
            match &self.big_grid[[i, j]] {
                Cell::Sim(sim) => sim.has_alive(),
                _ => false,
            }
        });

        // fonction locale pour compter voisins actifs à partir du snapshot
        let count_neighbors = |x: isize, y: isize| -> usize {
            let mut cnt = 0usize;
            for di in -1..=1 {
                for dj in -1..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let xi = x + di;
                    let yj = y + dj;
                    if xi >= 0 && yj >= 0 && (xi as usize) < self.big_N && (yj as usize) < self.big_N {
                        if snapshot_alive[[xi as usize, yj as usize]] {
                            cnt += 1;
                        }
                    }
                }
            }
            cnt
        };
        let mut new_big_grid: Array2<Cell> = Array2::from_shape_fn((self.big_N, self.big_N), |_| Cell::Empty);

        for i in 0..self.big_N {
            for j in 0..self.big_N {
                let neighbors = count_neighbors(i as isize, j as isize);

                // Déplacer la cellule courante hors de la grille pour en prendre possession
                let cell = std::mem::replace(&mut self.big_grid[[i, j]], Cell::Empty);

                match cell {
                    Cell::Empty => {
                        if neighbors == 3 {
                            new_big_grid[[i, j]] = Cell::Sim(Box::new(GameOfLife::new(self.small_N, self.vie)));
                        } else {
                            new_big_grid[[i, j]] = Cell::Empty;
                        }
                    }
                    Cell::Sim(mut sim_box) => {
                        // faire évoluer la sous-simulation
                        sim_box.step();
                        let alive = sim_box.has_alive();
                        if alive && (neighbors == 2 || neighbors == 3) {
                            // déplacer l'instance existante dans la nouvelle grille
                            new_big_grid[[i, j]] = Cell::Sim(sim_box);
                        } else {
                            if neighbors == 3 {
                                new_big_grid[[i, j]] = Cell::Sim(Box::new(GameOfLife::new(self.small_N, self.vie)));
                            } else {
                                new_big_grid[[i, j]] = Cell::Empty;
                            }
                        }
                    }
                }
            }
        }

        self.big_grid = new_big_grid;
    }

    pub fn get_big_grid(&self) -> &Array2<Cell> {
        &self.big_grid
    }

    pub fn get_big_N(&self) -> usize {
        self.big_N
    }

    pub fn get_small_N(&self) -> usize {
        self.small_N
    }

    pub fn density_grid(&self) -> Vec<f64> {
        let mut out = Vec::with_capacity(self.big_N * self.big_N);
        for i in 0..self.big_N {
            for j in 0..self.big_N {
                match &self.big_grid[[i, j]] {
                    Cell::Empty => out.push(0.0),
                    Cell::Sim(sim_box) => {
                        let grid = sim_box.get_grid();
                        let mut sum = 0usize;
                        for v in grid.iter() {
                            if *v == 1u8 { sum += 5; }
                        }
                        let area = (self.small_N * self.small_N) as f64;
                        out.push((sum as f64) / area);
                    }
                }
            }
        }
        out
    }
}
