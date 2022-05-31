use bevy::{
    math::{const_vec2, const_vec3},
    prelude::*,
};
use bevy_rapier2d::prelude::*;

const WORM_SIZE: f32 = 50.0;
const WORM_SPEED: f32 = 5.0;
const WORM_COLOR: Color = Color::rgb(2.0, 0.5, 0.0);

const TILE_SIZE: Vec2 = const_vec2!([120.0, 20.0]);
const TILE_COLOR: Color = Color::rgb(0.3, 0.0, 1.0);

#[derive(Component)]
struct TileMap;

#[derive(Component)]
struct Worm;

#[derive(Component)]
struct WormState {
    is_alive: bool,
    id: u16,
}

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugin(RapierDebugRenderPlugin::default()) // <- Uncomment to see lines and stuff (debugmode)
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(modify_body_translation)
        .add_system_to_stage(CoreStage::PostUpdate, display_events)
        .run();
}

fn display_events(
    //worm_state_query: Query<&WormState>,
    worm_query: Query<(Entity, &WormState)>,
    //tile_query: Query<(Entity, &TileMap), With<TileMap>>,
    mut contact_events: EventReader<CollisionEvent>,
) {

    for contact_event in contact_events.iter() {
        for (entity, worm) in worm_query.iter() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &entity || h2 == &entity {
                    println!("worm ded {}", worm.id);
                }
            }
        }
    }
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_physics(mut commands: Commands) {

    let worm_instance = WormState{is_alive: true, id: 1};
    let worm_instance2 = WormState{is_alive: true, id: 2};

    // Worm
    commands.spawn()
    .insert(Worm)
    .insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: WORM_COLOR,
            custom_size: Some(Vec2::new(WORM_SIZE, WORM_SIZE)),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(worm_instance)
    .insert(LockedAxes::TRANSLATION_LOCKED)
    .insert(Collider::cuboid(WORM_SIZE / 2.0, WORM_SIZE / 2.0))
    .insert(RigidBody::Dynamic)
    .insert(Transform::from_xyz(2.0, 0.0, 0.0))
    .insert(GravityScale(0.0))
    .insert(ActiveEvents::COLLISION_EVENTS);

    // Worm 2
    commands.spawn()
    .insert(Worm)
    .insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: WORM_COLOR,
            custom_size: Some(Vec2::new(WORM_SIZE, WORM_SIZE)),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(worm_instance2)
    .insert(LockedAxes::TRANSLATION_LOCKED)
    .insert(Collider::cuboid(WORM_SIZE / 2.0, WORM_SIZE / 2.0))
    .insert(RigidBody::Dynamic)
    .insert(Transform::from_xyz(2.0, 100.0, 0.0))
    .insert(GravityScale(0.0))
    .insert(ActiveEvents::COLLISION_EVENTS);

    // Tile
    commands.spawn()
    .insert(TileMap)
    .insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: TILE_COLOR,
            custom_size: Some(TILE_SIZE),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Collider::cuboid(TILE_SIZE.x / 2.0, TILE_SIZE.y / 2.0))
    .insert(Transform::from_xyz(20.0, 200.0, 0.0))
    .insert(ColliderMassProperties::Density(2.0))
    .insert(GravityScale(0.0));
}

fn modify_body_translation(
    keyboard_input: Res<Input<KeyCode>>,
    mut worm_query: Query<&mut Transform, With<Worm>>,
    worm_state_query: Query<&WormState>,

) {
    let mut direction_y = 0.0;
    let mut direction_x = 0.0;

    for worm_state in worm_state_query.iter() {
        if worm_state.is_alive {
            if keyboard_input.pressed(KeyCode::Up) {
                direction_y += WORM_SPEED;
            }
        
            if keyboard_input.pressed(KeyCode::Down) {
                direction_y -= WORM_SPEED;
            }
        
            if keyboard_input.pressed(KeyCode::Right) {
                direction_x += WORM_SPEED;
            }
        
            if keyboard_input.pressed(KeyCode::Left) {
                direction_x -= WORM_SPEED;
            }   
        }
    }
    
    for mut position in worm_query.iter_mut() {
        position.translation.y += direction_y;
        position.translation.x += direction_x;
    }
}
