use bevy::prelude::*;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, move_character);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

fn main() {
    App::new().add_plugins((DefaultPlugins, MainPlugin)).run();
}

fn window_collision(mut characters: Query<&mut Transform, &Player>) {}

fn move_character(
    mut characters: Query<(
        &mut Transform,
        &mut TextureAtlasSprite,
        &AnimationIndices,
        &Player,
    )>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut sprite, indices, _) in &mut characters {
        if input.pressed(KeyCode::Z) {
            transform.translation.y += 100.0 * time.delta_seconds();
        }

        if input.pressed(KeyCode::S) {
            transform.translation.y -= 100.0 * time.delta_seconds();
        }

        if input.pressed(KeyCode::D) {
            transform.translation.x += 100.0 * time.delta_seconds();
            if sprite.index == indices.last {
                sprite.index = 0;
            } else {
                sprite.index += 1;
            }
        }

        if input.pressed(KeyCode::Q) {
            transform.translation.x -= 100.0 * time.delta_seconds();
            sprite.index = 1;
        }

        if !input.pressed(KeyCode::Z)
            && !input.pressed(KeyCode::S)
            && !input.pressed(KeyCode::D)
            && !input.pressed(KeyCode::Q)
        {
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

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(1.0)), // Set the scale of the sprite
            ..default()
        },
        animation_indices,
        Player {},
    ));
}
