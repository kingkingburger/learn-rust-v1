use crate::world::World;

mod world;
mod entity;
mod component_storage;



fn main() {
    let mut main_world = world::World::new();
    
    let entity = main_world.spawn();

    let initial_pos = component_storage::Position {x: 0.0, y: 0.0};
    main_world.add_position(entity, initial_pos);

    println!("시스템 실행 전");
    movement_system(&mut main_world);

    movement_system(&mut main_world);
}



fn movement_system(world: &mut World) {
    for pos_opt in world.position.iter_mut() {
        if let Some(pos) = pos_opt {
            pos.x += 1.0;
            println!("Moved! x is now: {}", pos.x);     
        }
    }
}