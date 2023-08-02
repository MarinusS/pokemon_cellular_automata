use core::fmt;

use enum_iterator::all;
use rand::seq::SliceRandom;

mod pokemon_type;
pub use pokemon_type::PokemonType;

const INITIAL_SPAWN_HEALTH: i16 = 20;
const RESPAWN_HEALTH: i16 = 20;

#[derive(Clone)]
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

    fn take_damage(&mut self, opponent_type: PokemonType) {
        let damage = 2 * pokemon_type::PokemonType::get_damage_multiplier(
            &opponent_type,
            &self.pokemon_type,
        ) as i16;

        self.health -= damage;

        if self.health <= 0 {
            self.pokemon_type = opponent_type;
            self.health = RESPAWN_HEALTH;
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
        for y in 1..self.world_height - 1 {
            for x in 1..self.world_width - 1 {
                let pokemon_type = self.get_pokemon(x, y).unwrap().pokemon_type;

                for neighbour_coord in [
                    (x - 1, y - 1),
                    (x - 1, y + 1),
                    (x + 1, y - 1),
                    (x + 1, y + 1),
                ] {
                    self.get_pokemon_mut(neighbour_coord.0, neighbour_coord.1)
                        .unwrap()
                        .take_damage(pokemon_type);
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
}
