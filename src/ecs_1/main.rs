mod world;
mod entity;
mod component_storage;

fn main() {
    let mut main_world = world::World::new();
    main_world.spawn();
}