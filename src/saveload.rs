use super::ecs::*;
use ron;
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{DeserializeComponents, SerializeComponents, U64Marker, U64MarkerAllocator};
use std::fs::File;

pub fn save(world: &mut World) {
    //SaveWorld(format!("{}/save", env!("CARGO_MANIFEST_DIR"))).run_now(&world.res);
    SaveWorld("./save".to_owned()).run_now(&world.res);
}

pub fn load(world: &mut World) {
    //LoadWorld(format!("{}/save", env!("CARGO_MANIFEST_DIR"))).run_now(&world.res);
    SaveWorld("./save".to_owned()).run_now(&world.res);
}

struct SaveWorld(String);

impl<'a> System<'a> for SaveWorld {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, U64Marker>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Tile>,
    );

    fn run(&mut self, (entities, markers, positions, sprites): Self::SystemData) {
        let mut serializer = ron::ser::Serializer::new(Some(Default::default()), true);
        SerializeComponents::<NoError, U64Marker>::serialize(
            &(positions, sprites),
            &entities,
            &markers,
            &mut serializer,
        ).unwrap();
        let save = serializer.into_output_string();
        info!("{}", &save);
        use std::io::Write;
        File::create(&self.0)
            .expect("Could not create save file.")
            .write_all(save.as_bytes())
            .expect("Could not write save file.");
    }
}

struct LoadWorld(String);

impl<'a> System<'a> for LoadWorld {
    type SystemData = (
        Entities<'a>,
        Write<'a, U64MarkerAllocator>,
        WriteStorage<'a, U64Marker>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Tile>,
    );

    fn run(
        &mut self,
        (entities, mut allocator, mut markers, positions, sprites): Self::SystemData,
    ) {
        let save = {
            let mut file = match File::open(&self.0) {
                Ok(file) => file,
                Err(e) => {
                    panic!("Could not open save file: {} ({})", self.0, e);
                }
            };
            let mut save = Vec::new();
            use std::io::Read;
            file.read_to_end(&mut save).expect("Could not read file.");
            save
        };

        let mut deserializer = ron::de::Deserializer::from_bytes(&save).unwrap();

        DeserializeComponents::<NoError, U64Marker>::deserialize(
            &mut (positions, sprites),
            &entities,
            &mut markers,
            &mut allocator,
            &mut deserializer,
        ).unwrap();
    }
}
