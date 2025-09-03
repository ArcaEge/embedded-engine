use super::super::sounds::soundtrack;
use super::super::world_actor_abstractions::{
    ConstructableWorld, GameInteractionLayer, WorldTrait,
};
use crate::{
    engine::{
        EngineInteractionLayer, Point,
        alloc::{Box, Vec},
        sound_player::SoundPlayer,
    },
    game::{ActorTrait, WorldInteractionLayer, actors::Player},
};

pub struct MainWorld {
    pub actors: Vec<Box<dyn ActorTrait>>,
    pub player: Player,
    music: Vec<SoundPlayer>,
    sfx: Vec<SoundPlayer>,
    current_music: Option<usize>,
    current_sfx: Option<usize>,
}

impl ConstructableWorld for MainWorld {
    fn create() -> Box<dyn WorldTrait> {
        Box::new(Self {
            actors: Vec::new(),
            player: Player::new(Point { x: 5, y: 5 }),
            music: Vec::new(),
            sfx: Vec::new(),
            current_music: None,
            current_sfx: None,
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
}
