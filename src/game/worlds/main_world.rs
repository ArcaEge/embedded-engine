use super::super::world_actor_abstractions::{
    ConstructableWorld, GameInteractionLayer, WorldTrait,
};
use crate::{
    engine::{
        EngineInteractionLayer, Point,
        alloc::{Box, Vec},
    },
    game::{ActorTrait, WorldInteractionLayer, actors::Player},
};

pub struct MainWorld {
    pub actors: Vec<Box<dyn ActorTrait>>,
    pub player: Player,
}

impl ConstructableWorld for MainWorld {
    fn create() -> Box<dyn WorldTrait> {
        Box::new(Self {
            actors: Vec::new(),
            player: Player::new(Point { x: 0, y: 0 }),
        })
    }
}

impl WorldTrait for MainWorld {
    fn init(&mut self, _game: &mut GameInteractionLayer, _engine: &mut EngineInteractionLayer) {}

    fn tick(
        &mut self,
        tick_count: u64,
        game: &mut GameInteractionLayer,
        engine: &mut EngineInteractionLayer,
    ) {
        let mut world = WorldInteractionLayer {};

        // Tick Player
        self.player.tick(tick_count, &mut world, game, engine);

        // Tick Actors
        for actor in &mut self.actors {
            actor.as_mut().tick(tick_count, &mut world, game, engine);
        }
    }

    fn render(
        &mut self,
        tick_count: u64,
        game: &mut GameInteractionLayer,
        engine: &mut EngineInteractionLayer,
    ) {
        let mut world = WorldInteractionLayer {};

        // Render Actors
        for actor in &mut self.actors {
            actor.as_mut().render(tick_count, &mut world, game, engine);
        }

        // Render Player
        self.player.render(tick_count, &mut world, game, engine);
    }
}
