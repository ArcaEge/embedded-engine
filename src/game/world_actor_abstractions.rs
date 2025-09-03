use core::fmt::Error;

use crate::engine::{
    EngineInteractionLayer, Point, Sprite,
    alloc::{Box, Vec},
    sound_player::SoundPlayer,
};

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

    fn handle_sound(&mut self, engine: &mut EngineInteractionLayer) {
        let current_sfx = self.get_current_sfx();
        let current_music = self.get_current_music();

        // SFX takes priority over music
        if let Some(current_sfx) = current_sfx {
            let sfx = self.get_sfx();
            sfx.get_mut(current_sfx).unwrap().play_tick(engine);

            // Set to None if finished
            if sfx.get_mut(current_sfx).unwrap().is_finished() {
                self.set_current_sfx(None);
            }
        } else if let Some(current_music) = current_music {
            let music = self.get_music();
            music.get_mut(current_music).unwrap().play_tick(engine);

            // Set to None if finished
            if music.get_mut(current_music).unwrap().is_finished() {
                self.set_current_music(None);
            }
        }
    }

    fn get_music(&mut self) -> &mut Vec<SoundPlayer>;
    fn get_sfx(&mut self) -> &mut Vec<SoundPlayer>;

    fn get_current_music(&self) -> Option<usize>;
    fn get_current_sfx(&self) -> Option<usize>;

    fn set_current_music(&mut self, music: Option<usize>);
    fn set_current_sfx(&mut self, sfx: Option<usize>);
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
pub struct WorldInteractionLayer<'a> {
    pub music: &'a mut Vec<SoundPlayer>,
    pub sfx: &'a mut Vec<SoundPlayer>,
    pub current_music: &'a mut Option<usize>,
    pub current_sfx: &'a mut Option<usize>,
}

impl<'a> WorldInteractionLayer<'a> {
    /// Set the currently playing music. If `reset_current` is `true`, the currently playing music is reset before switching.
    /// If `pause_current` is `true`, the currently playing music is paused before switching. Returns `Error` if `index` is invalid.
    pub fn set_music(
        &mut self,
        index: usize,
        reset_current: bool,
        pause_current: bool,
        engine: &mut EngineInteractionLayer,
    ) -> Result<(), Error> {
        if self.music.len() <= index {
            Err(Error)
        } else {
            if reset_current {
                self.music.as_mut_slice()[index].reset(engine);
            } else if pause_current {
                self.music.as_mut_slice()[index].pause(engine);
            }

            *self.current_music = Some(index);
            Ok(())
        }
    }

    /// Set the currently playing sfx. If `reset_current` is `true`, the currently playing sfx is reset before switching.
    /// If `pause_current` is `true`, the currently playing sfx is paused before switching. Returns `Error` if `index` is invalid.
    pub fn set_sfx(
        &mut self,
        index: usize,
        reset_current: bool,
        pause_current: bool,
        engine: &mut EngineInteractionLayer,
    ) -> Result<(), Error> {
        if self.sfx.len() <= index {
            Err(Error)
        } else {
            if reset_current {
                self.sfx.as_mut_slice()[index].reset(engine);
            } else if pause_current {
                self.sfx.as_mut_slice()[index].pause(engine);
            }

            *self.current_music = Some(index);
            Ok(())
        }
    }
}
