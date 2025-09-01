use crate::engine::{EngineInteractionLayer, Point, Sprite, alloc::Box};

/// World = scene
/// Not a ripoff of Greenfoot Java's system
pub trait WorldTrait {
    /// Runs on Game init() or when switching to the World
    fn init(&mut self, game: &mut GameInteractionLayer, engine: &mut EngineInteractionLayer);

    /// Runs on every tick, as long as the world is active
    fn tick(
        &mut self,
        tick_count: u64,
        game: &mut GameInteractionLayer,
        engine: &mut EngineInteractionLayer,
    );

    /// Runs on every render, as long as the world is active
    fn render(
        &mut self,
        tick_count: u64,
        game: &mut GameInteractionLayer,
        engine: &mut EngineInteractionLayer,
    );
}

/// Gives the World a create() function, used for keeping the core WorldTrait dyn compatible
pub trait ConstructableWorld {
    fn create() -> Box<dyn WorldTrait>;
}

/// Actor = object in the game
pub trait ActorTrait {
    /// Runs on World init()
    fn init(
        &mut self,
        _world: &mut WorldInteractionLayer,
        _game: &mut GameInteractionLayer,
        _engine: &mut EngineInteractionLayer,
    ) {
    }

    /// Runs on every tick, as long as the world that the actor is in is active
    fn tick(
        &mut self,
        _tick_count: u64,
        _world: &mut WorldInteractionLayer,
        _game: &mut GameInteractionLayer,
        _engine: &mut EngineInteractionLayer,
    ) {
    }

    /// Runs on every render, as long as the world that the actor is in is active
    fn render(
        &mut self,
        _tick_count: u64,
        _world: &mut WorldInteractionLayer,
        _game: &mut GameInteractionLayer,
        engine: &mut EngineInteractionLayer,
    ) {
        self.get_sprite()
            .render(self.get_location(), engine, true, true);
    }

    /// Return the location of the Actor
    fn get_location(&self) -> Point;

    /// Return the sprite of the Actor
    fn get_sprite(&self) -> &Sprite;
}

/// Gives the Actor a create() function, used for keeping the core ActorTrait dyn compatible
pub trait ConstructableActor {
    fn create() -> Box<dyn ActorTrait>;
}

/// Interaction layer used to pass data between the world and game
pub struct GameInteractionLayer {}

/// Interaction layer used to pass data between the actor and world
pub struct WorldInteractionLayer {}
