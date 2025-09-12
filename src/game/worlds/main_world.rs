use super::super::{
    ActorTrait, Camera, WorldInteractionLayer,
    actors::{MOVEMENT_SPEED, Player, StaticActor},
    collisions::CollisionTypes,
    world_actor_abstractions::{ConstructableWorld, GameInteractionLayer, WorldTrait},
};
use crate::engine::{
    EngineInteractionLayer, PreciseOffset, PrecisePoint, Sound, Spritesheet,
    alloc::{Box, Rc, Vec},
    sound_player::SoundPlayer,
};
use core::cell::RefCell;
use postcard::from_bytes;

pub struct MainWorld {
    pub actors: Vec<Rc<RefCell<Box<dyn ActorTrait>>>>,
    pub player: Player,
    music: Vec<SoundPlayer>,
    sfx: Vec<SoundPlayer>,
    current_music: Option<usize>,
    current_sfx: Option<usize>,
    camera: Camera,
    spritesheet: Rc<Spritesheet>,
}

const PLAYER_SCREEN_X_MIN: f32 = 16.0;
const PLAYER_SCREEN_X_MAX: f32 = 112.0;

impl ConstructableWorld for MainWorld {
    fn create(spritesheet: Rc<Spritesheet>) -> Box<dyn WorldTrait> {
        Box::new(Self {
            actors: Vec::new(),
            player: Player::create(PrecisePoint { x: 8.0, y: 3.0 }, spritesheet.clone()),
            music: Vec::new(),
            sfx: Vec::new(),
            current_music: None,
            current_sfx: None,
            camera: Camera {
                current_offset: PreciseOffset { x: 0.0, y: 0.0 },
                min_offset: PreciseOffset {
                    x: -100.0,
                    y: -100.0,
                },
                max_offset: PreciseOffset { x: 100.0, y: 100.0 },
            },
            spritesheet,
        })
    }
}

impl WorldTrait for MainWorld {
    fn init(&mut self, game: &mut GameInteractionLayer, engine: &mut EngineInteractionLayer) {
        let soundtrack_bytes = include_bytes!("../sounds/doom.embsound");
        let soundtrack: Sound =
            from_bytes(soundtrack_bytes).expect("Failed to parse sound file, invalid format");

        self.music.push(SoundPlayer::new(soundtrack));
        self.set_current_music(Some(0));
        self.music.get_mut(0).unwrap().repeat = true;

        self.actors.push(Rc::new(RefCell::new(StaticActor::create(
            PrecisePoint { x: 8.0, y: 24.0 },
            self.spritesheet.clone(),
            Vec::from([(75, 100)]),
            false,
            CollisionTypes::BoundingBox(
                PreciseOffset { x: 0.0, y: 4.0 },
                PreciseOffset { x: 8.0, y: 8.0 },
            ),
        ))));

        self.actors.push(Rc::new(RefCell::new(StaticActor::create(
            PrecisePoint { x: 8.0, y: 48.0 },
            self.spritesheet.clone(),
            Vec::from([(90, 100)]),
            false,
            CollisionTypes::BoundingBox(
                PreciseOffset { x: 0.0, y: 1.0 },
                PreciseOffset { x: 8.0, y: 8.0 },
            ),
        ))));
        self.actors.push(Rc::new(RefCell::new(StaticActor::create(
            PrecisePoint { x: 16.0, y: 48.0 },
            self.spritesheet.clone(),
            Vec::from([(91, 100)]),
            false,
            CollisionTypes::BoundingBox(
                PreciseOffset { x: 0.0, y: 1.0 },
                PreciseOffset { x: 8.0, y: 8.0 },
            ),
        ))));
        self.actors.push(Rc::new(RefCell::new(StaticActor::create(
            PrecisePoint { x: 24.0, y: 48.0 },
            self.spritesheet.clone(),
            Vec::from([(91, 100)]),
            false,
            CollisionTypes::BoundingBox(
                PreciseOffset { x: 0.0, y: 1.0 },
                PreciseOffset { x: 8.0, y: 8.0 },
            ),
        ))));
        self.actors.push(Rc::new(RefCell::new(StaticActor::create(
            PrecisePoint { x: 32.0, y: 48.0 },
            self.spritesheet.clone(),
            Vec::from([(91, 100)]),
            false,
            CollisionTypes::BoundingBox(
                PreciseOffset { x: 0.0, y: 1.0 },
                PreciseOffset { x: 8.0, y: 8.0 },
            ),
        ))));
        self.actors.push(Rc::new(RefCell::new(StaticActor::create(
            PrecisePoint { x: 40.0, y: 48.0 },
            self.spritesheet.clone(),
            Vec::from([(91, 100)]),
            false,
            CollisionTypes::BoundingBox(
                PreciseOffset { x: 0.0, y: 1.0 },
                PreciseOffset { x: 8.0, y: 8.0 },
            ),
        ))));
        self.actors.push(Rc::new(RefCell::new(StaticActor::create(
            PrecisePoint { x: 48.0, y: 48.0 },
            self.spritesheet.clone(),
            Vec::from([(92, 100)]),
            false,
            CollisionTypes::BoundingBox(
                PreciseOffset { x: 0.0, y: 1.0 },
                PreciseOffset { x: 8.0, y: 8.0 },
            ),
        ))));

        self.actors.push(Rc::new(RefCell::new(StaticActor::create(
            PrecisePoint { x: 8.0, y: 56.0 },
            self.spritesheet.clone(),
            Vec::from([(107, 100)]),
            false,
            CollisionTypes::BoundingBox(
                PreciseOffset { x: 0.0, y: 0.0 },
                PreciseOffset { x: 8.0, y: 8.0 },
            ),
        ))));
        self.actors.push(Rc::new(RefCell::new(StaticActor::create(
            PrecisePoint { x: 48.0, y: 56.0 },
            self.spritesheet.clone(),
            Vec::from([(108, 100)]),
            false,
            CollisionTypes::BoundingBox(
                PreciseOffset { x: 0.0, y: 0.0 },
                PreciseOffset { x: 8.0, y: 8.0 },
            ),
        ))));

        self.actors.push(Rc::new(RefCell::new(StaticActor::create(
            PrecisePoint { x: 32.0, y: 40.0 },
            self.spritesheet.clone(),
            Vec::from([(65, 100)]),
            false,
            CollisionTypes::None,
        ))));

        let mut world = WorldInteractionLayer {
            actors: &self.actors,
            music: &mut self.music,
            sfx: &mut self.sfx,
            current_music: &mut self.current_music,
            current_sfx: &mut self.current_sfx,
            camera: &mut self.camera,
        };

        // Call the init() method of the actors
        self.player.init(&mut world, game, engine);

        for actor in &self.actors {
            actor.borrow_mut().init(&mut world, game, engine);
        }
    }

    fn tick(
        &mut self,
        tick_count: u64,
        game: &mut GameInteractionLayer,
        engine: &mut EngineInteractionLayer,
    ) {
        let mut world = WorldInteractionLayer {
            actors: &self.actors,
            music: &mut self.music,
            sfx: &mut self.sfx,
            current_music: &mut self.current_music,
            current_sfx: &mut self.current_sfx,
            camera: &mut self.camera,
        };

        // Tick Player
        self.player.tick(tick_count, None, &mut world, game, engine);

        // Tick Actors
        for (index, actor) in self.actors.iter().enumerate() {
            actor
                .borrow_mut()
                .tick(tick_count, Some(index as u32), &mut world, game, engine);
        }

        // Camera position
        let player_screen_position = self
            .player
            .get_precise_location()
            .apply_inverted_offset(self.camera.current_offset);

        if player_screen_position.x < PLAYER_SCREEN_X_MIN {
            self.camera.current_offset.x -= MOVEMENT_SPEED + 0.5;
            self.camera.current_offset.x = self
                .camera
                .current_offset
                .x
                .max(self.camera.min_offset.x)
                .min(self.camera.max_offset.x)
        }
        if player_screen_position.x > PLAYER_SCREEN_X_MAX {
            self.camera.current_offset.x += MOVEMENT_SPEED + 0.5;
            self.camera.current_offset.x = self
                .camera
                .current_offset
                .x
                .max(self.camera.min_offset.x)
                .min(self.camera.max_offset.x)
        }

        self.handle_sound(engine);
    }

    fn render(
        &mut self,
        tick_count: u64,
        game: &mut GameInteractionLayer,
        engine: &mut EngineInteractionLayer,
    ) {
        let mut world = WorldInteractionLayer {
            actors: &self.actors,
            music: &mut self.music,
            sfx: &mut self.sfx,
            current_music: &mut self.current_music,
            current_sfx: &mut self.current_sfx,
            camera: &mut self.camera,
        };

        // Render Actors
        for actor in &self.actors {
            actor
                .borrow_mut()
                .render(tick_count, &mut world, game, engine);
        }

        // Render Player
        self.player.render(tick_count, &mut world, game, engine);
    }

    fn get_sfx(&mut self) -> &mut Vec<SoundPlayer> {
        &mut self.sfx
    }
    fn get_music(&mut self) -> &mut Vec<SoundPlayer> {
        &mut self.music
    }

    fn get_current_music(&self) -> Option<usize> {
        self.current_music
    }
    fn get_current_sfx(&self) -> Option<usize> {
        self.current_sfx
    }

    fn set_current_music(&mut self, music: Option<usize>) {
        self.current_music = music;
    }
    fn set_current_sfx(&mut self, sfx: Option<usize>) {
        self.current_sfx = sfx;
    }

    fn get_camera(&self) -> &Camera {
        &self.camera
    }
}
