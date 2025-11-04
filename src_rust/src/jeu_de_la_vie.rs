use ndarray::Array2;

#[derive(Debug, Clone)]
pub struct GameOfLife {
    N: usize,
    vie: f64,
    grid: Array2<u8>,
}

impl GameOfLife {
    pub fn new(N: usize, vie: f64) -> Self {
        // Utilise `rand::random` pour éviter l'appel à la méthode `gen`
        // qui est devenu un mot-clé dans la nouvelle édition.
        let grid = Array2::from_shape_fn((N, N), |_| {
            if rand::random::<f64>() < vie {
                1u8
            } else {
                0u8
            }
        });
        Self { N, vie, grid }
    }

    pub fn step(&mut self) {
        let mut new_grid = Array2::from_elem((self.N, self.N), 0u8);
        for i in 0..self.N {
            for j in 0..self.N {
                let neighbor = self.neighbor_active_count(i, j);
                if self.grid[[i, j]] == 1 {
                    new_grid[[i, j]] = if neighbor == 2 || neighbor == 3 { 1 } else { 0 };
                } else {
                    new_grid[[i, j]] = if neighbor == 3 { 1 } else { 0 };
                }
            }
        }
        self.grid = new_grid;
    }

    fn neighbor_active_count(&self, x: usize, y: usize) -> u8 {
        let mut count: u8 = 0;
        let n = self.N as isize;
        for di in -1..=1 {
            for dj in -1..=1 {
                if di == 0 && dj == 0 {
                    continue;
                }
                let xi = x as isize + di;
                let yj = y as isize + dj;
                if xi >= 0 && yj >= 0 && xi < n && yj < n {
                    count += self.grid[[xi as usize, yj as usize]];
                }
            }
        }
        count
    }

    pub fn get_grid(&self) -> &Array2<u8> {
        &self.grid
    }

    pub fn get_grid_size(&self) -> usize {
        self.N
    }

    pub fn has_alive(&self) -> bool {
        self.grid.iter().any(|&x| x == 1)
    }
}