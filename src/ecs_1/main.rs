use crate::world::World;

mod world;
mod entity;
mod component_storage;



fn main() {
    let mut main_world = world::World::new();
    main_world.spawn();
}



fn movement_system(world: &mut World) {
    for pos_opt in world.position.iter_mut() {
        if let Some(pos) = pos_opt {
            pos.x += 1.0;
            println!("Moved! x is now: {}", pos.x);     
        }
    }
}