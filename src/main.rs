use bevy::prelude::*;
use caw::prelude::*;
use std::collections::VecDeque;

const MAX_NUM_SAMPLES: usize = 2_000;

struct AudioState {
    player: PlayerAsyncStereo,
    sig: StereoPair<SigBoxed<f32>>,
}

impl AudioState {
    fn new() -> Self {
        let player = Player::new()
            .unwrap()
            .into_async_stereo(Default::default())
            .unwrap();
        let sig = Stereo::new_fn_channel(|_channel| {
            super_saw(60.)
                .build()
                .filter(low_pass::default(2000.).resonance(0.5))
                .filter(chorus())
                .filter(compressor().threshold(0.1).ratio(0.25).scale(1.))
                .boxed()
        });
        Self { player, sig }
    }

    fn tick(&mut self, scope_state: &mut ScopeState) {
        self.player.play_signal_callback(&mut self.sig, |samples| {
            for (&x, &y) in samples.left.iter().zip(samples.right.iter()) {
                scope_state.samples.push_back(Vec2::new(x, y));
            }
            while scope_state.samples.len() > MAX_NUM_SAMPLES {
                scope_state.samples.pop_front();
            }
        });
    }
}

#[derive(Resource)]
struct ScopeState {
    samples: VecDeque<Vec2>,
}

impl ScopeState {
    fn new() -> Self {
        Self {
            samples: VecDeque::new(),
        }
    }
}

fn setup_caw_player(world: &mut World) {
    world.insert_non_send_resource(AudioState::new());
    world.insert_resource(ScopeState::new());
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn caw_tick(mut audio_state: NonSendMut<AudioState>, mut scope_state: ResMut<ScopeState>) {
    audio_state.tick(&mut scope_state);
}

fn render_scope(scope_state: Res<ScopeState>, window: Query<&Window>, mut gizmos: Gizmos) {
    let color = Vec3::new(0., 1., 0.);
    let mut current_color = Vec3::ZERO;
    let color_step = color / scope_state.samples.len() as f32;
    let scale = window.single().width();
    let mut samples_iter = scope_state.samples.iter().map(|sample| sample * scale);
    let mut prev = if let Some(first) = samples_iter.next() {
        first
    } else {
        return;
    };
    for sample in samples_iter {
        current_color += color_step;
        gizmos.line_2d(
            prev,
            sample,
            Color::srgb(current_color.x, current_color.y, current_color.z),
        );
        prev = sample;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_caw_player, setup))
        .insert_resource(ClearColor(Color::srgb(0., 0., 0.)))
        .add_systems(Update, (caw_tick, render_scope))
        .run();
}
