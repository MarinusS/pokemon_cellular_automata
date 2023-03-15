use core::fmt;

use enum_iterator::all;
use rand::seq::SliceRandom;

mod pokemon_type;
pub use pokemon_type::PokemonType;

const INITIAL_SPAWN_HEALTH: i16 = 20;
const RESPAWN_HEALTH: i16 = 20;

pub struct Pokemon {
    pub pokemon_type: PokemonType,
    pub health: i16,
}

impl Pokemon {
    pub fn new(pokemon_type: PokemonType, health: i16) -> Self {
        Pokemon {
            pokemon_type,
            health,
        }
    }
}

impl fmt::Display for Pokemon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Pokemon( Health: {}, Type: {:?} )",
            self.health, self.pokemon_type
        )
    }
}

pub struct PokemonWorld {
    world: Vec<Vec<Pokemon>>,
    world_width: usize,
    world_height: usize,

    tick_duration: std::time::Duration,
    last_update: std::time::Instant,
}

impl PokemonWorld {
    pub fn new(world_width: u32, world_height: u32) -> Self {
        let mut world = Vec::new();
        for _ in 0..world_height {
            world.push(Vec::new());

            for _ in 0..world_width {
                let pokemon_type = *all::<PokemonType>()
                    .collect::<Vec<_>>()
                    .choose(&mut rand::thread_rng())
                    .unwrap_or(&PokemonType::Normal);
                world
                    .last_mut()
                    .unwrap()
                    .push(Pokemon::new(pokemon_type, INITIAL_SPAWN_HEALTH));
            }
        }

        Self {
            world_width: world_width as usize,
            world_height: world_height as usize,
            world,

            //tick_duration: std::time::Duration::new(0, 500000000),
            tick_duration: std::time::Duration::new(0, 0),
            last_update: std::time::Instant::now(),
        }
    }

    pub fn update(&mut self) {
        let now = std::time::Instant::now();
        if now - self.last_update > self.tick_duration {
            self.battle();

            self.last_update = now;
        }
    }

    fn battle(&mut self) {
        for y in 0..self.world_height {
            for x in 0..self.world_width {
                let pokemon_type = self.world.get(y).unwrap().get(x).unwrap().pokemon_type;
                let neighbours = self.get_neighbours_coordinates(x, y);

                for neighbour_coord in neighbours {
                    let neighbour = self
                        .get_pokemon_mut(neighbour_coord.0, neighbour_coord.1)
                        .unwrap();

                    neighbour.health -= 2 * pokemon_type::PokemonType::get_damage_multiplier(
                        &pokemon_type,
                        &neighbour.pokemon_type,
                    ) as i16;

                    if neighbour.health <= 0 {
                        neighbour.pokemon_type = pokemon_type;
                        neighbour.health = RESPAWN_HEALTH;
                    }
                }
            }
        }
    }

    pub fn get_pokemon(&self, x: usize, y: usize) -> Option<&Pokemon> {
        self.world.get(y)?.get(x)
    }

    pub fn get_pokemon_mut(&mut self, x: usize, y: usize) -> Option<&mut Pokemon> {
        self.world.get_mut(y)?.get_mut(x)
    }

    fn get_neighbours_coordinates(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours = vec![];
        if x > 0 {
            if y > 0 {
                neighbours.push((x - 1, y - 1));
            }
            if y < self.world_height - 1 {
                neighbours.push((x - 1, y + 1));
            }
        }
        if x < self.world_width - 1 {
            if y > 0 {
                neighbours.push((x + 1, y - 1));
            }
            if y < self.world_height - 1 {
                neighbours.push((x + 1, y + 1));
            }
        }

        neighbours
    }
}
