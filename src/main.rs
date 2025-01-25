use bevy::prelude::*;
use caw::prelude::*;

struct AudioState {
    player: PlayerAsyncStereo,
    sig: Stereo<SigBoxed<f32>, SigBoxed<f32>>,
}

impl AudioState {
    fn new() -> Self {
        let player = Player::new()
            .unwrap()
            .into_async_stereo(Default::default())
            .unwrap();
        let sig = Stereo::new_fn(|| {
            super_saw(60.)
                .build()
                .filter(low_pass::default(2000.).resonance(0.5))
                .filter(chorus())
                .boxed()
        });
        Self { player, sig }
    }

    fn tick(&mut self) {
        self.player.play_signal(&mut self.sig);
    }
}

fn setup_caw_player(world: &mut World) {
    world.insert_non_send_resource(AudioState::new());
}

fn caw_tick(mut audio_state: NonSendMut<AudioState>) {
    audio_state.tick();
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_caw_player)
        .add_systems(Update, caw_tick)
        .run();
}
