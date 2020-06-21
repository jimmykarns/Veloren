use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

use specs::{WorldExt, World};
use specs::join::Join;
use veloren_common::comp::Player;
use authc::Uuid;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("entities_find_player");

    for players in [100, 1000, 10000, 100000, 1000000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(players), players, |b, &players| {
            let ecs = init_ecs(players);
            let target_player_name = format!("test_player_{}", players / 2);
            b.iter(|| {
                find_player_by_name(&ecs, &target_player_name);
            });
        });
    }
}

fn init_ecs(player_count: u32) -> World {
    let mut ecs = specs::World::new();
    ecs.register::<Player>();

    // Insert player_count players into the ECS Player storage
    let players: Vec<Player> = (0..player_count)
        .map(|i| Player::new(format!("test_player_{}", i), None, None, Uuid::parse_str("936DA01F-9ABD-4D9D-80C7-02AF85C822A8").unwrap()))
        .collect();
    for player in players {
        ecs.insert(player);
    }

    ecs
}

fn find_player_by_name(ecs: &World, name: &str) {
    let entities = ecs.entities();
    let players = ecs.read_storage::<Player>();
    let player = (&entities, &players)
        .join()
        .find(|(_, player)| player.alias == name)
        .map(|(entity, _)| entity);

    black_box(player);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);