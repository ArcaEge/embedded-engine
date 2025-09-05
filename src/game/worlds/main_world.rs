use super::super::{
    ActorTrait, Camera, WorldInteractionLayer,
    actors::Player,
    sounds::soundtrack,
    world_actor_abstractions::{ConstructableWorld, GameInteractionLayer, WorldTrait},
};
use crate::engine::{
    EngineInteractionLayer, PreciseOffset, PrecisePoint, Spritesheet,
    alloc::{Box, Vec},
    sound_player::SoundPlayer,
};

pub struct MainWorld {
    pub actors: Vec<Box<dyn ActorTrait>>,
    pub player: Player,
    music: Vec<SoundPlayer>,
    sfx: Vec<SoundPlayer>,
    current_music: Option<usize>,
    current_sfx: Option<usize>,
    camera: Camera,
}

impl ConstructableWorld for MainWorld {
    fn create(spritesheet: &Spritesheet) -> Box<dyn WorldTrait> {
        Box::new(Self {
            actors: Vec::new(),
            player: Player::create(PrecisePoint { x: 5.0, y: 5.0 }, spritesheet),
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
        })
    }
}

impl WorldTrait for MainWorld {
    fn init(&mut self, _game: &mut GameInteractionLayer, _engine: &mut EngineInteractionLayer) {
        self.music.push(SoundPlayer::new(soundtrack()));
        self.set_current_music(Some(0));
        self.music.get_mut(0).unwrap().repeat = true;
    }

    fn tick(
        &mut self,
        tick_count: u64,
        game: &mut GameInteractionLayer,
        engine: &mut EngineInteractionLayer,
    ) {
        let mut world = WorldInteractionLayer {
            music: &mut self.music,
            sfx: &mut self.sfx,
            current_music: &mut self.current_music,
            current_sfx: &mut self.current_sfx,
            camera: &mut self.camera,
        };

        // Tick Player
        self.player.tick(tick_count, &mut world, game, engine);

        // Tick Actors
        for actor in &mut self.actors {
            actor.as_mut().tick(tick_count, &mut world, game, engine);
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
            music: &mut self.music,
            sfx: &mut self.sfx,
            current_music: &mut self.current_music,
            current_sfx: &mut self.current_sfx,
            camera: &mut self.camera,
        };

        // Render Actors
        for actor in &mut self.actors {
            actor.as_mut().render(tick_count, &mut world, game, engine);
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
