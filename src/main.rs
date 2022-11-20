use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

//screen size
pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

use std::f32::consts::PI;

//assets
#[derive(Resource)]
pub struct GameAssets {
    bullet_scene: Handle<Scene>
}

#[derive(Reflect,Component,Default)]
#[reflect(Component)]
pub struct Tower{
    shooting_timer: Timer,  
}

#[derive(Reflect,Component,Default)]
#[reflect(Component)]
pub struct Lifetime{
    timer: Timer,  
}

fn main() {
    //create the window
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(asset_loading)
        .add_system(tower_shooting)
        .add_system(bullet_despawn)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "Rusty Towers".to_string(),
                resizable: false,
                ..Default::default()
            },
            ..default()
        }))
        .register_type::<Tower>()
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

//generate a 3d camera
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_basic_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
   
    //create a flat plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.1, 0.4, 0.7).into()),
        ..default()
    })
    .insert(Name::new("Plane"));

    //create a cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.67, 0.94, 0.42).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
    .insert(Name::new("Cube"))
    .insert(Tower{
        shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    })
    .insert(Name::new("Light"));
}

fn tower_shooting(mut commands: Commands, 
     mut towers:  Query<&mut Tower>,
     bullet_assets: Res<GameAssets>,
     time: Res<Time>,
){
    for mut tower in towers.iter_mut(){
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let spawn_transform = Transform::from_xyz(0.0, 0.7, 0.6).with_rotation(Quat::from_rotation_y(-PI / 2.0));

            commands.spawn(SceneBundle{
                scene: bullet_assets.bullet_scene.clone(),
                transform: spawn_transform,
                ..Default::default()
            })
            .insert(Lifetime{
                timer: Timer::from_seconds(0.5, TimerMode::Once),
            })
            .insert(Name::new("Bullet"));
        }
    }
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn asset_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: asset_server.load("Bullet.glb#Scene0"),
    });
}