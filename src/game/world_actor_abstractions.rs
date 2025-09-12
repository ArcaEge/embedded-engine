use super::collisions::CollisionTypes;
use crate::engine::{
    EngineInteractionLayer, Point, PreciseOffset, PrecisePoint, Sprite, Spritesheet,
    alloc::{Box, Rc, Vec},
    sound_player::SoundPlayer,
};
use core::{cell::RefCell, fmt::Error, ops::Range};

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

    fn get_camera(&self) -> &Camera;

    fn get_music(&mut self) -> &mut Vec<SoundPlayer>;
    fn get_sfx(&mut self) -> &mut Vec<SoundPlayer>;

    fn get_current_music(&self) -> Option<usize>;
    fn get_current_sfx(&self) -> Option<usize>;

    fn set_current_music(&mut self, music: Option<usize>);
    fn set_current_sfx(&mut self, sfx: Option<usize>);
}

/// Gives the World a create() function, used for keeping the core WorldTrait dyn compatible
pub trait ConstructableWorld {
    fn create(spritesheet: Rc<Spritesheet>) -> Box<dyn WorldTrait>;
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
        _self_index: Option<u32>,
        _world: &mut WorldInteractionLayer,
        _game: &mut GameInteractionLayer,
        _engine: &mut EngineInteractionLayer,
    ) {
    }

    /// Runs on every render, as long as the world that the actor is in is active
    fn render(
        &mut self,
        _tick_count: u64,
        world: &mut WorldInteractionLayer,
        _game: &mut GameInteractionLayer,
        engine: &mut EngineInteractionLayer,
    ) {
        if self.is_osd() {
            self.get_sprite()
                .render(self.get_location(), engine, true, true, self.is_flipped());
        } else {
            self.get_sprite().render(
                self.get_precise_location()
                    .apply_inverted_offset(world.camera.current_offset)
                    .into(),
                engine,
                true,
                true,
                self.is_flipped(),
            );
        }
    }

    /// Return the location of the Actor
    fn get_location(&self) -> Point {
        self.get_precise_location().into()
    }

    /// Return the precise location of the Actor
    fn get_precise_location(&self) -> PrecisePoint;

    /// Returns whether the Actor is part of the on-screen display (i.e. fixed in place even if the world camera moves)
    fn is_osd(&self) -> bool {
        false
    }

    /// Returns whether the sprite should be flipped or not in the x axis
    fn is_flipped(&self) -> bool {
        false
    }

    /// Return the sprite of the Actor
    fn get_sprite(&self) -> Rc<Sprite>;

    /// Return the CollisionType
    fn get_collision_type(&self) -> Rc<CollisionTypes>;

    /// TODO
    fn get_colliding_objects(
        &self,
        self_index: Option<u32>,
        world: &mut WorldInteractionLayer,
    ) -> Vec<Rc<RefCell<Box<dyn ActorTrait>>>> {
        let mut colliding_actors: Vec<Rc<RefCell<Box<dyn ActorTrait>>>> = Vec::new();

        let collision_type = self.get_collision_type();

        // Do nothing if collision type is None
        if *collision_type == CollisionTypes::None {
            return colliding_actors;
        } else if let CollisionTypes::BoundingBox(
            this_top_left_collision,
            this_bottom_right_collision,
        ) = *collision_type
        {
            let this_location = self.get_precise_location();

            let this_top_left = this_location.apply_offset(this_top_left_collision);
            let this_bottom_right = this_location.apply_offset(this_bottom_right_collision);

            let this_x_range = this_top_left.x..this_bottom_right.x;
            let this_y_range = this_top_left.y..this_bottom_right.y;

            for (index, actor) in world.actors.iter().enumerate() {
                if let Some(self_index_unwrapped) = self_index
                    && self_index_unwrapped == index as u32
                {
                    continue;
                }

                let borrowed_actor = actor.borrow();
                let inner_actor = borrowed_actor.as_ref();

                match inner_actor.get_collision_type().as_ref() {
                    CollisionTypes::None => continue,
                    CollisionTypes::BoundingBox(top_left_collision, bottom_right_collision) => {
                        let location = inner_actor.get_precise_location();
                        let top_left = location.apply_offset(*top_left_collision);
                        let bottom_right = location.apply_offset(*bottom_right_collision);

                        let x_range = top_left.x..bottom_right.x;
                        let y_range = top_left.y..bottom_right.y;

                        // Actual collision logic
                        if ranges_overlap(this_x_range.clone(), x_range)
                            && ranges_overlap(this_y_range.clone(), y_range)
                        {
                            colliding_actors.push(actor.clone());
                        }
                    }
                }
            }
        }

        colliding_actors
    }

    // fn is_colliding(&self) {}
}

/// Gives the Actor a create() function, used for keeping the core ActorTrait dyn compatible.
/// This is redundant for now, but I might use it in the future for something like loading actors from a vec
pub trait ConstructableActor {
    fn create(location: PrecisePoint, spritesheet: &Spritesheet) -> Box<dyn ActorTrait>;
}

/// Interaction layer used to pass data between the world and game
pub struct GameInteractionLayer<'a> {
    pub spritesheet: &'a Spritesheet,
}

/// Interaction layer used to pass data between the actor and world
pub struct WorldInteractionLayer<'a> {
    pub actors: &'a [Rc<RefCell<Box<dyn ActorTrait>>>],
    pub music: &'a mut Vec<SoundPlayer>,
    pub sfx: &'a mut Vec<SoundPlayer>,
    pub current_music: &'a mut Option<usize>,
    pub current_sfx: &'a mut Option<usize>,
    pub camera: &'a mut Camera,
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

pub struct Camera {
    pub current_offset: PreciseOffset,
    pub min_offset: PreciseOffset,
    pub max_offset: PreciseOffset,
}

/// Returns true if the given ranges overlap
fn ranges_overlap(a: Range<f32>, b: Range<f32>) -> bool {
    a.start.max(b.start) <= a.end.min(b.end)
}
