extern crate ggez;
extern crate specs;

use ggez::*;
use ggez::graphics::{DrawMode, Point2};

use specs::{Component, VecStorage, World, System, WriteStorage, ReadStorage, RunNow};

struct Systems {
    hello_world: HelloWorld,
    update_pos: UpdatePos,
}

struct MainState {
    world: World,
    systems: Systems,
}

impl MainState {
    fn new(_ctx: &mut Context, world: World, systems: Systems) -> GameResult<MainState> {
        let s = MainState { world, systems };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.systems.hello_world.run_now(&self.world.res);
        self.systems.update_pos.run_now(&self.world.res);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use specs::Join;
        graphics::clear(ctx);
        let positions = self.world.read::<Position>();
        for entity in self.world.entities().join() {
            if let Some(pos) = positions.get(entity) {
                graphics::circle(ctx, DrawMode::Fill, Point2::new(pos.x, pos.y), 100.0, 2.0)?;
            }
        }
        graphics::present(ctx);
        Ok(())
    }
}

// specs stuff

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        use specs::Join;
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * 0.05;
            pos.y += vel.y * 0.05;
        }
    }
}


struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}

pub fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();

    world
        .create_entity()
        .with(Position { x: 4.0, y: 7.0 })
        .build();

    world
        .create_entity()
        .with(Position { x: 0.0, y: 380.0 })
        .with(Velocity { x: 5.0, y: 0.1 })
        .build();

    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();


    let systems = Systems {
        hello_world: HelloWorld,
        update_pos: UpdatePos,
    };
    let state = &mut MainState::new(ctx, world, systems).unwrap();
    event::run(ctx, state).unwrap();
}
