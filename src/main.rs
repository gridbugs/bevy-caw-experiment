use bevy::{input::keyboard::KeyboardInput, prelude::*, window::PrimaryWindow};
use caw::prelude::*;
use std::collections::VecDeque;

const MAX_NUM_SAMPLES: usize = 2_000;

struct AudioState {
    player: PlayerOwned,
    mouse_x: FrameSigVar<f32>,
    mouse_y: FrameSigVar<f32>,
    keyboard: Keyboard<FrameSig<FrameSigVar<bool>>>,
}

impl AudioState {
    fn new() -> Self {
        let keyboard = Keyboard::new(|_| frame_sig_var(false));
        let mouse_x = frame_sig_var(0.5);
        let mouse_y = frame_sig_var(0.5);
        let sig = Stereo::new_fn_channel(|_channel| {
            let MonoVoice {
                note,
                key_down_gate,
                key_press_trig,
                ..
            } = keyboard
                .clone()
                .opinionated_key_events(Note::B0)
                .mono_voice();
            let env = adsr_linear_01(key_down_gate)
                .key_press_trig(key_press_trig)
                .release_s(10.)
                .build()
                .exp_01(1.0);
            super_saw(note.freq_hz().filter(low_pass::butterworth(10.)))
                .build()
                .filter(low_pass::default(env * mouse_y.clone() * 8000.).resonance(0.2))
                .filter(
                    chorus()
                        .num_voices(1)
                        .lfo_rate_hz(0.05)
                        .delay_s(mouse_x.clone() * 0.02),
                )
                .filter(compressor().threshold(0.1).ratio(0.1).scale(5.))
                .filter(reverb::default())
                .filter(high_pass_butterworth(1.))
                .boxed()
        });
        let player = Player::new()
            .unwrap()
            .into_owned_stereo(
                sig,
                ConfigSync {
                    system_latency_s: 0.0167,
                },
            )
            .unwrap();

        Self {
            player,
            mouse_x: mouse_x.0,
            mouse_y: mouse_y.0,
            keyboard,
        }
    }

    fn tick(&mut self, scope_state: &mut ScopeState) {
        self.player.with_latest_data(|data| {
            for chunks in data.chunks_exact(2) {
                let x = chunks[0];
                let y = chunks[1];
                scope_state.samples.push_back(Vec2::new(x, y));
            }
        });
        while scope_state.samples.len() > MAX_NUM_SAMPLES {
            scope_state.samples.pop_front();
        }
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

fn caw_tick(
    mut audio_state: NonSendMut<AudioState>,
    mut scope_state: ResMut<ScopeState>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single();
    if let Some(position) = window.cursor_position() {
        let window_size = window.size();
        let position_01 = position / window_size;
        audio_state.mouse_x.set(position_01.x);
        audio_state.mouse_y.set(position_01.y);
    }
    audio_state.tick(&mut scope_state);
}

fn caw_update_keyboard(
    audio_state: NonSendMut<AudioState>,
    mut evr_kbd: EventReader<KeyboardInput>,
) {
    use bevy::input::ButtonState;
    for ev in evr_kbd.read() {
        let key_state = match ev.key_code {
            KeyCode::KeyA => &audio_state.keyboard.a,
            KeyCode::KeyB => &audio_state.keyboard.b,
            KeyCode::KeyC => &audio_state.keyboard.c,
            KeyCode::KeyD => &audio_state.keyboard.d,
            KeyCode::KeyE => &audio_state.keyboard.e,
            KeyCode::KeyF => &audio_state.keyboard.f,
            KeyCode::KeyG => &audio_state.keyboard.g,
            KeyCode::KeyH => &audio_state.keyboard.h,
            KeyCode::KeyI => &audio_state.keyboard.i,
            KeyCode::KeyJ => &audio_state.keyboard.j,
            KeyCode::KeyK => &audio_state.keyboard.k,
            KeyCode::KeyL => &audio_state.keyboard.l,
            KeyCode::KeyM => &audio_state.keyboard.m,
            KeyCode::KeyN => &audio_state.keyboard.n,
            KeyCode::KeyO => &audio_state.keyboard.o,
            KeyCode::KeyP => &audio_state.keyboard.p,
            KeyCode::KeyQ => &audio_state.keyboard.q,
            KeyCode::KeyR => &audio_state.keyboard.r,
            KeyCode::KeyS => &audio_state.keyboard.s,
            KeyCode::KeyT => &audio_state.keyboard.t,
            KeyCode::KeyU => &audio_state.keyboard.u,
            KeyCode::KeyV => &audio_state.keyboard.v,
            KeyCode::KeyW => &audio_state.keyboard.w,
            KeyCode::KeyX => &audio_state.keyboard.x,
            KeyCode::KeyY => &audio_state.keyboard.y,
            KeyCode::KeyZ => &audio_state.keyboard.z,
            KeyCode::Digit0 => &audio_state.keyboard.n0,
            KeyCode::Digit1 => &audio_state.keyboard.n1,
            KeyCode::Digit2 => &audio_state.keyboard.n2,
            KeyCode::Digit3 => &audio_state.keyboard.n3,
            KeyCode::Digit4 => &audio_state.keyboard.n4,
            KeyCode::Digit5 => &audio_state.keyboard.n5,
            KeyCode::Digit6 => &audio_state.keyboard.n6,
            KeyCode::Digit7 => &audio_state.keyboard.n7,
            KeyCode::Digit8 => &audio_state.keyboard.n8,
            KeyCode::Digit9 => &audio_state.keyboard.n9,
            KeyCode::BracketLeft => &audio_state.keyboard.left_bracket,
            KeyCode::BracketRight => &audio_state.keyboard.right_bracket,
            KeyCode::Semicolon => &audio_state.keyboard.semicolon,
            KeyCode::Quote => &audio_state.keyboard.apostrophe,
            KeyCode::Comma => &audio_state.keyboard.comma,
            KeyCode::Period => &audio_state.keyboard.period,
            KeyCode::Minus => &audio_state.keyboard.minus,
            KeyCode::Equal => &audio_state.keyboard.equals,
            KeyCode::Slash => &audio_state.keyboard.slash,
            KeyCode::Space => &audio_state.keyboard.space,
            KeyCode::Backspace => &audio_state.keyboard.backspace,
            KeyCode::Backslash => &audio_state.keyboard.backslash,
            _ => continue,
        };
        let pressed = match ev.state {
            ButtonState::Pressed => true,
            ButtonState::Released => false,
        };
        key_state.0.set(pressed);
    }
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
        .add_systems(FixedFirst, caw_tick)
        .add_systems(Update, (caw_update_keyboard, render_scope))
        .run();
}
