use crate::bevy::window::{PresentMode, WindowPosition};
use rand::prelude::*;
use rusty_engine::prelude::*;
use std::f32::consts::{FRAC_2_PI, PI};

// struct Player {}

struct GameState {
    // player: Player,
    score: u32,
    high_score: u32,
    enemy_index: u32,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            // player: Player {},
            score: 0,
            high_score: 0,
            enemy_index: 1,
            spawn_timer: Timer::from_seconds(2.0, true),
        }
    }
}

fn main() {
    let mut game = Game::new();

    game.window_settings(WindowDescriptor {
        title: "Game1".to_string(),
        width: 1400.0,
        height: 768.0,
        ..Default::default()
    });

    game.show_colliders = true;
    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.01);

    let player = game.add_sprite("player", SpritePreset::RacingCarRed);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = EAST;
    player.scale = 1.0;
    player.layer = 1.0;
    player.collision = true;

    let car = game.add_sprite("car", SpritePreset::RacingCarYellow);
    car.translation = Vec2::new(300.0, 0.0);
    car.rotation = WEST;
    car.scale = 1.0;
    car.layer = 1.0;
    car.collision = true;

    let score = game.add_text("score", "Score: 0");
    score.translation = Vec2::new(520.0, 320.0);

    let high_score = game.add_text("high_score", "High Score: 0");
    high_score.translation = Vec2::new(-520.0, 300.0);

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    window_dimensions_text_positioning(engine, game_state);
    collide_to_score_points(engine, game_state);
    handle_keyboard_input(engine, game_state);
    handle_mouse_input(engine, game_state);
    press_r_to_reset_score(engine, game_state);
    press_q_to_quit(engine, game_state);
    spawn_new_stuff(engine, game_state);
}

fn window_dimensions_text_positioning(engine: &mut Engine, game_state: &mut GameState) {
    let offset_x = ((engine.time_since_startup_f64 * 1.0).cos() * 0.3) as f32;
    let offset_y = ((engine.time_since_startup_f64 * 5.0).cos() * 1.2) as f32;

    let score = engine.texts.get_mut("score").unwrap();
    score.translation.x = engine.window_dimensions.x / 2.0 - 80.0 - offset_x;
    score.translation.y = engine.window_dimensions.y / 2.0 - 30.0 + offset_y;

    let high_score = engine.texts.get_mut("high_score").unwrap();
    high_score.translation.x = engine.window_dimensions.x / -2.0 + 110.0 - offset_x;
    high_score.translation.y = engine.window_dimensions.y / 2.0 - 30.0 + offset_y;
}

fn press_q_to_quit(engine: &mut Engine, game_state: &mut GameState) {
    if engine.keyboard_state.just_pressed(KeyCode::Q) {
        engine.should_exit = true;
    }
}

fn spawn_new_stuff(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let label = format!("enemy_{}", game_state.enemy_index);
        let enemy = engine.add_sprite(&label, SpritePreset::RollingBallRed);
        enemy.translation = Vec2::new(
            thread_rng().gen_range(-550.0..550.0),
            thread_rng().gen_range(-325.0..325.0),
        );
        enemy.collision = true;
        game_state.enemy_index += 1;
    }
}

fn press_r_to_reset_score(engine: &mut Engine, game_state: &mut GameState) {
    if engine.keyboard_state.just_pressed(KeyCode::R) {
        game_state.score = 0;
        engine.texts.get_mut("score").unwrap().value = format!("Score: {}", game_state.score)
    }
}

fn collide_to_score_points(engine: &mut Engine, game_state: &mut GameState) {
    for event in engine.collision_events.drain(..) {
        if event.state == CollisionState::Begin {
            if event.pair.one_starts_with("player") {
                game_state.score += 1;
                let score = engine.texts.get_mut("score").unwrap();
                score.value = format!("Score: {}", game_state.score);

                if game_state.high_score < game_state.score {
                    game_state.high_score = game_state.score;
                    let high_score = engine.texts.get_mut("high_score").unwrap();
                    high_score.value = format!("High Score: {}", game_state.high_score);
                }

                for label in [event.pair.0, event.pair.1] {
                    if label != "player" {
                        engine.sprites.remove(&label);
                    }
                }

                engine.audio_manager.play_sfx(SfxPreset::Minimize1, 0.1)
            }
        }
    }
}

fn handle_mouse_input(engine: &mut Engine, game_state: &mut GameState) {
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(mouse_location) = engine.mouse_state.location() {
            let label = format!("enemy_{}", game_state.enemy_index);

            // let player = engine.sprites.get_mut("player").unwrap();
            let enemy = engine.add_sprite(&label, SpritePreset::RollingBallRed);
            enemy.translation = mouse_location;
            enemy.collision = true;
            // enemy.rotation = mouse_location - player.translation;
            game_state.enemy_index += 1;
        }
    }
}

fn handle_keyboard_input(engine: &mut Engine, game_state: &mut GameState) {
    const MOVEMENT_SPEED: f32 = 100.0;
    // const ROTATION_SPEED: f32 = 2.0 * PI;

    let player = engine.sprites.get_mut("player").unwrap();

    if engine.keyboard_state.pressed(KeyCode::W) {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine.keyboard_state.pressed(KeyCode::S) {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine.keyboard_state.pressed(KeyCode::A) {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine.keyboard_state.pressed(KeyCode::D) {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }
}
