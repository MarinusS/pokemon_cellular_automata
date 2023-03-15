use enum_iterator::all;
use rand::seq::SliceRandom;

mod pokemon_type;
pub use pokemon_type::PokemonType;

pub struct Pokemon {
    pub pokemon_type: PokemonType,
    pub health: u8,
}

impl Pokemon {
    pub fn new(pokemon_type: PokemonType, health: u8) -> Self {
        Pokemon {
            pokemon_type,
            health,
        }
    }
}

pub struct PokemonWorld {
    world: Vec<Vec<Pokemon>>,
    world_width: u32,
    world_height: u32,
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
                    .push(Pokemon::new(pokemon_type, 20));
            }
        }

        Self {
            world_width,
            world_height,
            world,
        }
    }

    pub fn update(&mut self) {}

    pub fn get_pokemon_color(&self, x: usize, y: usize) -> Option<(u8, u8, u8)> {
        let pokemon = self.world.get(y)?.get(x)?;
        Some(pokemon.pokemon_type.get_color())
    }
}
