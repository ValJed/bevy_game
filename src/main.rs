use bevy::prelude::*;
use bevy::time::Timer;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .insert_resource(PlayerInput::new())
            .add_systems(Update, movement_input)
            .add_systems(Update, move_character)
            .add_systems(Update, player_jump)
            .add_systems(Update, update_char_sprite);
    }
}

#[derive(Component)]
struct Player;

#[derive(Resource, Debug)]
struct PlayerInput {
    left: bool,
    right: bool,
    jump: bool,
}

impl PlayerInput {
    fn new() -> PlayerInput {
        PlayerInput {
            left: false,
            right: false,
            jump: false,
        }
    }
}

#[derive(Component, Debug)]
struct Jump(f32);

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component)]
struct SpriteTimer(Timer);

const FALL_SPEED: f32 = 98.0;
const JUMP_HEIGHT: f32 = 100.0;

fn main() {
    App::new().add_plugins((DefaultPlugins, MainPlugin)).run();
}

fn window_collision(mut characters: Query<&mut Transform, &Player>) {}

fn movement_input(mut player_input: ResMut<PlayerInput>, input: Res<Input<KeyCode>>) {
    player_input.right = input.pressed(KeyCode::D);
    player_input.left = input.pressed(KeyCode::Q);
    player_input.jump = input.pressed(KeyCode::Space);
}

fn update_char_sprite(
    mut query: Query<(
        &mut TextureAtlasSprite,
        &AnimationIndices,
        &mut SpriteTimer,
        With<Player>,
    )>,
    player_input: Res<PlayerInput>,
    time: Res<Time>,
) {
    let Ok((mut sprite, indices, mut timer, _)) = query.get_single_mut() else {
        return;
    };

    if player_input.right {
        if timer.0.tick(time.delta()).just_finished() {
            if sprite.index == indices.last {
                sprite.index = 0;
            } else {
                sprite.index += 1;
            }
        }
    }

    if player_input.left {
        if timer.0.tick(time.delta()).just_finished() {
            if sprite.index == indices.last {
                sprite.index = 0;
            } else {
                sprite.index += 1;
            }
        }
    }
}

fn player_jump(
    mut query: Query<(Entity, &mut Transform, &mut Jump), With<Player>>,
    mut commands: Commands,
    player_input: Res<PlayerInput>,
    time: Res<Time>,
) {
    let Ok((player, mut transform, mut jump)) = query.get_single_mut() else {
        return;
    };

    if !player_input.jump && jump.0 == 0. {
        return;
    }

    let jump_power = time.delta_seconds() * FALL_SPEED * 2.0;

    if jump.0 <= JUMP_HEIGHT {
        transform.translation.y += jump_power;
        jump.0 += jump_power;
    } else {
        jump.0 = 0.0;
    }
}

fn player_fall(mut query: Query<(Entity, &mut Transform, &mut Jump), With<Player>>) {}

fn move_character(
    mut characters: Query<(&mut Transform, &mut TextureAtlasSprite, &mut Jump, &Player)>,
    player_input: Res<PlayerInput>,
    time: Res<Time>,
) {
    for (mut transform, mut sprite, mut jump, _) in &mut characters {
        if player_input.right {
            transform.translation.x += 100.0 * time.delta_seconds();
            transform.rotation = Quat::default();
        }

        if player_input.left {
            transform.translation.x -= 100.0 * time.delta_seconds();
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        }

        if !player_input.left && !player_input.right {
            sprite.index = 0;
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("characters/frog/run.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32., 32.), 12, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 1, last: 11 };

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_scale(Vec3::splat(1.0)), // Set the scale of the sprite
                ..default()
            },
            animation_indices,
            Player {},
            Jump(0.0),
        ))
        .insert(SpriteTimer(Timer::from_seconds(0.05, TimerMode::Repeating)));
}
