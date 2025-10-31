from jeu_de_la_vie import GameOfLifeSimulation
import numpy as np

class GameOfLifeInGameOfLife():
    def __init__(self, size=50, alive_prob=0.2):
        self.big_N = int(size)
        self.vie = float(alive_prob)
        self.big_grid = np.empty((self.big_N, self.big_N), dtype=object)
        for i in range(self.big_N):
            for j in range(self.big_N):
                if np.random.random() < self.vie:
                    self.big_grid[i, j] = GameOfLifeSimulation(size=self.big_N, alive_prob=self.vie)
                else:
                    self.big_grid[i, j] = 0
    
    def step(self):
        new_big_grid = np.empty((self.big_N, self.big_N), dtype=object)
        for i in range(self.big_N):
            for j in range(self.big_N):
                neighbors = self.neighbor_active_count(i, j)
                cell = self.big_grid[i, j]

                if cell == 0:
                    # naissance : exactement 3 voisins
                    if neighbors == 3:
                        new_big_grid[i, j] = GameOfLifeSimulation(size=self.small_N if hasattr(self, "small_N") else self.big_N,
                                                              alive_prob=self.vie)
                    else:
                        new_big_grid[i, j] = 0

                elif isinstance(cell, GameOfLifeSimulation):
                    sim = cell
                    # on fait évoluer la sous-simulation
                    sim.step()
                    has_alive = (sim.grid == 1).any()

                    # si la sous-sim a encore des cellules vivantes ET la règle de survie du grand-jeu est satisfaite -> on garde sim
                    if has_alive and (neighbors == 2 or neighbors == 3):
                        new_big_grid[i, j] = sim
                    else:
                        # soit la sous-sim est morte, soit elle ne survit pas par le grand-jeu
                        # mais si le grand-jeu donne naissance (3 voisins), on recrée une nouvelle sous-sim
                        if neighbors == 3:
                            new_big_grid[i, j] = GameOfLifeSimulation(size=self.small_N if hasattr(self, "small_N") else self.big_N,
                                                                  alive_prob=self.vie)
                        else:
                            new_big_grid[i, j] = 0
                else:
                    # cas inattendu : copie sûre
                    new_big_grid[i, j] = 0

        self.big_grid = new_big_grid
        return self.big_grid
    
    def _neighbor_active_count(self, x, y):
        count = 0
        for i in (-1, 0, 1):
            for j in (-1, 0, 1):
                if i == 0 and j == 0:
                    continue
                xi = (x + i) % self.big_N
                yj = (y + j) % self.big_N
                neighbor = self.big_grid[xi, yj]
                # actif seulement si c'est une sous-simulation avec au moins
                # une cellule vivante
                if isinstance(neighbor, GameOfLifeSimulation):
                    try:
                        if (neighbor.grid == 1).any():
                            count += 1
                    except Exception:
                        # en cas d'erreur inattendue, ne pas compter
                        pass
        return count
    
    def neighbor_active_count(self, x, y):
        count = 0
        for i in (-1, 0, 1):
            for j in (-1, 0, 1):
                if i == 0 and j == 0:
                    continue
                xi, yj = x + i, y + j
                if 0 <= xi < self.big_N and 0 <= yj < self.big_N:
                    neighbor = self.big_grid[xi, yj]
                    # actif seulement si c'est une sous-simulation avec au moins
                    # une cellule vivante
                    if isinstance(neighbor, GameOfLifeSimulation):
                        try:
                            if (neighbor.grid == 1).any():
                                count += 1
                        except Exception:
                            # en cas d'erreur inattendue, ne pas compter
                            pass
        return count
    
    @property
    def grid(self):
        return self.big_grid

    @property
    def grid_size(self):
        return (self.big_N, self.big_N)

