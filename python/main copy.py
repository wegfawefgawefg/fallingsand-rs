import random
import sys
import pygame
from pygame.locals import *

# Game constants
SCREEN_WIDTH = 800
SCREEN_HEIGHT = 600
FPS = 30
GRID_SIZE = 4

# Colors
WHITE = (255, 255, 255)
BLACK = (0, 0, 0)

# Elements
SAND = 1
WATER = 2
GAS = 3
FIRE = 4
SMOKE = 5
STEAM = 6

# SOME BEHAVIOR CONSTANTS
FIRE_LIFETIME = 5  # The number of time steps a fire particle lasts
SMOKE_PROBABILITY = 0.3
SMOKE_LIFETIME = 30

ELEMENT_COLORS = {
    SAND: (255, 255, 0),
    WATER: (0, 0, 255),
    GAS: (128, 128, 128),
    FIRE: (255, 0, 0),
    SMOKE: (128, 128, 128),
    STEAM: (200, 200, 255),
}

element_names = {
    SAND: "Sand",
    WATER: "Water",
    GAS: "Gas",
    FIRE: "Fire",
    SMOKE: "Smoke",
    STEAM: "Steam",
}


class Particle:
    def __init__(self, x, y, element):
        self.x = x
        self.y = y
        self.element = element
        self.age = 0

    def update(self):
        self.age += 1

    def draw(self, screen):
        pygame.draw.rect(
            screen,
            ELEMENT_COLORS[self.element],
            (self.x * GRID_SIZE, self.y * GRID_SIZE, GRID_SIZE, GRID_SIZE),
        )


def create_text_surface(text, font, color):
    return font.render(text, True, color)


def is_valid_position(x, y, particles_grid):
    return 0 <= x < len(particles_grid) and 0 <= y < len(particles_grid[0])


def get_neighboring_positions(x, y):
    return [
        (x, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x, y + 1),
    ]


def update_particles(particles):
    particles_grid = [
        [None for _ in range(SCREEN_HEIGHT // GRID_SIZE)]
        for _ in range(SCREEN_WIDTH // GRID_SIZE)
    ]

    for particle in particles:
        particles_grid[particle.x][particle.y] = particle

    new_particles = []
    particles_to_remove = set()
    for particle in particles:
        particle.update()
        if particle.element == SAND:
            x, y = particle.x, particle.y
            if (
                is_valid_position(x, y + 1, particles_grid)
                and particles_grid[x][y + 1] is None
            ):
                particle.y += 1
                particles_grid[x][y], particles_grid[x][y + 1] = None, particle
            else:
                for dx in [-1, 1]:
                    if (
                        is_valid_position(x + dx, y + 1, particles_grid)
                        and particles_grid[x + dx][y + 1] is None
                    ):
                        particle.x += dx
                        particle.y += 1
                        particles_grid[x][y], particles_grid[x + dx][y + 1] = (
                            None,
                            particle,
                        )
                        break

        elif particle.element == STEAM:
            x, y = particle.x, particle.y

            # Move upwards with some randomness
            dy = random.choice([-1, -2])  # Move up 1 or 2 steps
            dx = random.choice([-1, 0, 1])  # Move randomly to the left or right

            if (
                is_valid_position(x + dx, y + dy, particles_grid)
                and particles_grid[x + dx][y + dy] is None
            ):
                particle.x += dx
                particle.y += dy
                particles_grid[x][y], particles_grid[x + dx][y + dy] = None, particle
            else:
                # If steam can't move up, move randomly to the left or right
                dx = random.choice([-1, 1])
                if (
                    is_valid_position(x + dx, y, particles_grid)
                    and particles_grid[x + dx][y] is None
                ):
                    particle.x += dx
                    particles_grid[x][y], particles_grid[x + dx][y] = None, particle

        elif particle.element == WATER:
            x, y = particle.x, particle.y
            moved = False

            # Check for fire in neighboring positions
            for nx, ny in get_neighboring_positions(x, y):
                if (
                    is_valid_position(nx, ny, particles_grid)
                    and particles_grid[nx][ny] is not None
                    and particles_grid[nx][ny].element == FIRE
                ):
                    particle.element = STEAM  # Turn the water particle into steam
                    break

            # Try to move down
            if (
                is_valid_position(x, y + 1, particles_grid)
                and particles_grid[x][y + 1] is None
            ):
                particle.y += 1
                particles_grid[x][y], particles_grid[x][y + 1] = None, particle
                moved = True
            else:
                # Try to move left or right
                for dx in [-1, 1]:
                    if (
                        is_valid_position(x + dx, y, particles_grid)
                        and particles_grid[x + dx][y] is None
                    ):
                        particle.x += dx
                        particles_grid[x][y], particles_grid[x + dx][y] = None, particle
                        moved = True
                        break

        elif particle.element == FIRE:
            x, y = particle.x, particle.y

            # Extinguish fire after a certain number of time steps
            if particle.age >= FIRE_LIFETIME:
                particles_to_remove.add(particle)
                continue  # Skip the rest of the fire behavior for extinguished particles

            # Extinguish fire in contact with water
            for nx, ny in get_neighboring_positions(x, y):
                if (
                    is_valid_position(nx, ny, particles_grid)
                    and particles_grid[nx][ny] is not None
                    and particles_grid[nx][ny].element == WATER
                ):
                    particles_to_remove.add(particle)
                    break

            # Occasionally spawn smoke particles above the fire
            if random.random() < SMOKE_PROBABILITY:
                if (
                    is_valid_position(x, y - 1, particles_grid)
                    and particles_grid[x][y - 1] is None
                ):
                    smoke_particle = Particle(x, y - 1, SMOKE)
                    particles_grid[x][y - 1] = smoke_particle
                    new_particles.append(smoke_particle)

            # Move upwards and randomly to the left or right
            dx = random.choice([-1, 0, 1])
            if (
                is_valid_position(x + dx, y - 1, particles_grid)
                and particles_grid[x + dx][y - 1] is None
            ):
                particle.x += dx
                particle.y -= 1
                particles_grid[x][y], particles_grid[x + dx][y - 1] = None, particle

            # Spread fire to adjacent flammable particles (e.g., sand)
            for nx, ny in get_neighboring_positions(x, y):
                if (
                    is_valid_position(nx, ny, particles_grid)
                    and particles_grid[nx][ny] is not None
                    and particles_grid[nx][ny].element == SAND
                ):
                    particles_grid[nx][ny].element = FIRE
                    particles_grid[nx][ny].age = 0
                    break

        elif particle.element == SMOKE:
            x, y = particle.x, particle.y

            # Dissipate smoke after a certain number of time steps
            if particle.age >= SMOKE_LIFETIME:
                particles_to_remove.add(particle)
                continue  # Skip the rest of the smoke behavior for dissipated particles

            # Move upwards
            if (
                is_valid_position(x, y - 1, particles_grid)
                and particles_grid[x][y - 1] is None
            ):
                particle.y -= 1
                particles_grid[x][y], particles_grid[x][y - 1] = None, particle

        new_particles.append(particle)
        # Add more particle behavior here

        # Remove particles marked for removal (e.g., extinguished fire)
        new_particles = [
            particle
            for particle in new_particles
            if particle not in particles_to_remove
        ]

    return new_particles


def main():
    pygame.init()

    screen = pygame.display.set_mode((SCREEN_WIDTH, SCREEN_HEIGHT))
    pygame.display.set_caption("Falling Sand Simulation")
    clock = pygame.time.Clock()

    particles = []

    # Manage the selected particle
    element_options = [SAND, WATER, FIRE, GAS, SMOKE, STEAM]
    current_element_index = 0

    # Font rendering
    font_small = pygame.font.Font(None, 24)
    font_large = pygame.font.Font(None, 32)

    left_mouse_button_pressed = False

    while True:
        screen.fill(BLACK)

        for event in pygame.event.get():
            if event.type == QUIT:
                pygame.quit()
                sys.exit()

            # Update the selected particle with keyboard input
            elif event.type == KEYDOWN:
                if event.key == K_w:
                    current_element_index = (current_element_index - 1) % len(
                        element_options
                    )
                elif event.key == K_s:
                    current_element_index = (current_element_index + 1) % len(
                        element_options
                    )
                # quit on q or escape press
                elif event.key == K_q or event.key == K_ESCAPE:
                    pygame.quit()
                    sys.exit()

            #   same but mouse drag
            elif event.type == MOUSEBUTTONDOWN:
                if event.button == 1:  # Left mouse button
                    left_mouse_button_pressed = True
                    x, y = event.pos
                    particles.append(
                        Particle(
                            x // GRID_SIZE,
                            y // GRID_SIZE,
                            element_options[current_element_index],
                        )
                    )

            elif event.type == MOUSEBUTTONUP:
                if event.button == 1:  # Left mouse button
                    left_mouse_button_pressed = False

            elif event.type == MOUSEMOTION:
                if left_mouse_button_pressed:
                    x, y = event.pos
                    particles.append(
                        Particle(
                            x // GRID_SIZE,
                            y // GRID_SIZE,
                            element_options[current_element_index],
                        )
                    )

        particles = update_particles(particles)

        for particle in particles:
            particle.draw(screen)

        # Display the list of options on the left
        for i, element in enumerate(element_options):
            font = font_large if i == current_element_index else font_small
            element_text = create_text_surface(
                element_names[element], font, ELEMENT_COLORS[element]
            )
            screen.blit(element_text, (10, 10 + i * 40))

        pygame.display.flip()
        clock.tick(FPS)


if __name__ == "__main__":
    main()
