use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, move_character);
    }
}

fn main() {
    App::new().add_plugins((DefaultPlugins, MainPlugin)).run();
}

fn move_character(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    println!("characters: {:?}", characters);
    for (mut transform, _) in &mut characters {
        println!("transform: {:?}", transform);
        if input.pressed(KeyCode::Z) {
            transform.translation.y += 100.0 * time.delta_seconds()
        }

        if input.pressed(KeyCode::S) {
            transform.translation.y -= 100.0 * time.delta_seconds()
        }

        if input.pressed(KeyCode::D) {
            transform.translation.x += 100.0 * time.delta_seconds()
        }

        if input.pressed(KeyCode::Q) {
            transform.translation.y -= 100.0 * time.delta_seconds()
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(30., 30.)).into())
            .into(),
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        material: materials.add(ColorMaterial::from(Color::GOLD)),
        ..default()
    });
}
