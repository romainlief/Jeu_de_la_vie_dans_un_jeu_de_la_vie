import numpy as np
import matplotlib.pyplot as plt
import matplotlib.animation as animation
from matplotlib.widgets import Button

from simulation import GameOfLifeInGameOfLife
from jeu_de_la_vie import GameOfLifeSimulation


def _biggrid_to_density(big_grid):
    """Convertit la `big_grid` (objets ou 0) en tableau float [0,1]

    Chaque cellule vide (0) -> 0.0. Chaque cellule contenant une
    GameOfLifeSimulation -> densité = fraction de cellules vivantes
    dans la sous-simulation.
    """
    big_N = big_grid.shape[0]
    dens = np.zeros((big_N, big_N), dtype=float)
    for i in range(big_N):
        for j in range(big_N):
            cell = big_grid[i, j]
            if cell == 0:
                dens[i, j] = 0.0
            elif isinstance(cell, GameOfLifeSimulation):
                try:
                    dens[i, j] = float(np.mean(cell.grid))
                except Exception:
                    dens[i, j] = 0.0
    return dens


def run_gui(size=50, alive_prob=0.2, interval=200):
    sim = GameOfLifeInGameOfLife(size=size, alive_prob=alive_prob)

    fig, ax = plt.subplots()
    plt.subplots_adjust(bottom=0.15)
    data = _biggrid_to_density(sim.grid)
    im = ax.imshow(data, cmap="viridis", interpolation="nearest", vmin=0.0, vmax=1.0)
    ax.set_title("Jeux de la Vie dans le Jeu de la Vie — densité")
    ax.set_xticks([])
    ax.set_yticks([])

    state = {"paused": False}

    def update(frame):
        if not state["paused"]:
            big = sim.step()
            dens = _biggrid_to_density(big)
            im.set_data(dens)
        return (im,)

    ani = animation.FuncAnimation(fig, update, interval=interval, blit=True)

    # Bouton Pause/Resume
    axpause = plt.axes([0.7, 0.03, 0.08, 0.05])
    bpause = Button(axpause, "Pause")

    def on_pause(event):
        state["paused"] = not state["paused"]
        bpause.label.set_text("Resume" if state["paused"] else "Pause")

    bpause.on_clicked(on_pause)

    # Bouton Step (avance d'une génération si en pause)
    axstep = plt.axes([0.59, 0.03, 0.08, 0.05])
    bstep = Button(axstep, "Step")

    def on_step(event):
        if state["paused"]:
            big = sim.step()
            dens = _biggrid_to_density(big)
            im.set_data(dens)
            fig.canvas.draw_idle()

    bstep.on_clicked(on_step)

    # Bouton Randomize (réinitialise la grille)
    axrand = plt.axes([0.44, 0.03, 0.125, 0.05])
    brand = Button(axrand, "Randomize")

    def on_randomize(event):
        sim.__init__(size=size, alive_prob=alive_prob)
        data = _biggrid_to_density(sim.grid)
        im.set_data(data)
        fig.canvas.draw_idle()

    brand.on_clicked(on_randomize)

    plt.show()
