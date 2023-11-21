mod particles;


use bevy::{prelude::{*, Color}, sprite::{MaterialMesh2dBundle, collide_aabb::{collide, Collision}, Material2d, Mesh2dHandle}, ecs::query};


#[derive(Component)]
struct Circle;
#[derive(Component)]
struct Collider;
#[derive(Event, Default)]
struct CollisionEvent;
#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider
}

#[derive(Component)]
struct Particle;

#[derive(Bundle)]
struct ParticleBundle<M:Material2d> {
    particle: Particle,
    mesh: MaterialMesh2dBundle<M>,
    velocity: Velocity
}

impl<M: Material2d> ParticleBundle<M> {
    fn new(pos: Vec2, mesh: Mesh2dHandle, material: Handle<M>) -> Self {
        ParticleBundle {
            particle: Particle {},
            mesh: MaterialMesh2dBundle {
                mesh,
                material,
                transform: Transform::from_translation(pos.extend(0.0)).with_scale(Vec3::new(PARTICLE_RADIUS * 2., PARTICLE_RADIUS * 2., 0.0)),
                ..default()
            },
            velocity: Velocity(Vec2::new(0.0, 0.0))
        }
    }
}


const LEFT_WALL: f32 = -300.;
const RIGHT_WALL: f32 = 300.;
const BOTTOM_WALL: f32 = -250.;
const TOP_WALL: f32 = 250.;

const WALL_THICKNESS: f32 = 5.;

enum WallLocation {
    Left, Right, Bottom, Top
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match &self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.0),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.0),
            WallLocation::Top => Vec2::new(0.0, TOP_WALL),
            WallLocation::Bottom => Vec2::new(0.0, BOTTOM_WALL)
        }
    }
    fn size(&self) -> Vec2 {
        let bounds_height = TOP_WALL - BOTTOM_WALL;
        let bounds_width = RIGHT_WALL - LEFT_WALL;
        
        match self {
            WallLocation::Right | WallLocation::Left => {
                Vec2::new(WALL_THICKNESS, bounds_height + WALL_THICKNESS)
            },
            WallLocation::Top | WallLocation::Bottom => {
                Vec2::new(bounds_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.5, 0.5, 0.5),
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}




fn distance_between(p1: Vec2, p2: Vec2) -> f32 {
    ((p2.y - p1.y).powi(2) + (p2.x - p1.x).powi(2)).abs().sqrt()
}

fn find_influence(transform: Transform, point: Vec2) -> f32 {
    let distance = distance_between(transform.translation.truncate(), point);
    let clamped = f64::max(0.0, (SMOOTHING_RADIUS - distance) as f64);
    (clamped * clamped * clamped) as f32
}


const PARTICLE_MASS: f32 = 1.0;

fn find_density(point: Vec2, particles: &Query<(&mut Transform, &mut Velocity)>) -> f32 {
    let mut total_mass = 0.0;
    
    for (transform, _velocity) in particles.iter() {
        total_mass += PARTICLE_MASS * find_influence(*transform, point);
    }
    println!("{}", total_mass / (std::f32::consts::PI * SMOOTHING_RADIUS * SMOOTHING_RADIUS));
    0.0
}

// fn find_steepest_gradient(particle_position: Vec2, particle_density: f32) -> Vec2 {
//     let mut biggest_diff = particle_density;
    
    // for i in (-1..1) {
    //     for j in (-1..1) {
    //         let new_density = 
    //     }
    // }
// }









fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {primary_window: Some(Window {title: "Fluid Simulation".into(), ..default()}), ..default()}))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (
            apply_velocity,
            gravity,
            wall_collisions,
        ).chain())
        .run()
}

const GRAVITY_STRENGTH: f32 = 1.0;
const PARTICLE_NUMBER: u64 = 36;
const PARTICLE_RADIUS: f32 = 5.;
const SPACING: f32 = 0.5;
const BOUNCE: f32 = 0.4;

const SMOOTHING_RADIUS: f32 = 40.;

fn apply_velocity(mut objects: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in objects.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}


fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    
    
    commands.spawn(Camera2dBundle::default());
    let cols: u64 = (PARTICLE_NUMBER as f64).sqrt().floor() as u64;
    let rows = PARTICLE_NUMBER / cols;
    
    let x_start = 0.5 * (PARTICLE_RADIUS * 2.) * cols as f32;
    let y_start = 0.5 * (PARTICLE_RADIUS * 2.) * rows as f32;

    for i in 1..PARTICLE_NUMBER + 1 {
        let x_pos: f32 = (-x_start + ((i % cols) * (PARTICLE_RADIUS * 2.) as u64) as f32) * (SPACING + 1.);
        let y_pos: f32 = (y_start - (i.div_ceil(cols) * (PARTICLE_RADIUS * 2.) as u64) as f32) * (SPACING + 1.);

        commands.spawn(ParticleBundle::new(Vec2::new(x_pos, y_pos), meshes.add(shape::Circle::default().into()).into(),materials.add(ColorMaterial::from(Color::ALICE_BLUE))));
    }

    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
}


fn gravity(mut particles: Query<(&mut Velocity, &Transform)>, time: Res<Time>, mut commands: Commands) {
    for (mut velocity, transform) in particles.iter_mut() {
        // velocity.y -= GRAVITY_STRENGTH * 200. * time.delta_seconds();
    }
}

fn wall_collisions(walls: Query<(&Transform, &Collider), Without<Velocity>>, time: Res<Time>, mut particles: Query<(&mut Transform, &mut Velocity )>) {
    for wall in walls.iter() {
        let density = find_density(Vec2::new(0.0, 0.0), &particles);
        for (mut transform, mut velocity) in particles.iter_mut() {
            let collision = collide(transform.translation, transform.scale.truncate(), wall.0.translation, wall.0.scale.truncate());
            if let Some(collision) = collision {
                

                let mut reflect_x = false;
                let mut reflect_y = false;
                
                match collision {
                    Collision::Left => reflect_x = velocity.x < 0.0,
                    Collision::Right => reflect_x = velocity.x > 0.0,
                    Collision::Top => reflect_y = velocity.y > 0.0,
                    Collision::Bottom => reflect_y = velocity.y < 0.0,
                    Collision::Inside => {}
                }

                if reflect_y {
                    transform.translation.y += WALL_THICKNESS;
                    velocity.y *= -1. * BOUNCE;
                } if reflect_x {
                    transform.translation.x += WALL_THICKNESS;
                    velocity.x *= -1. * BOUNCE;
                }
            }
        }
    }
}


