use criterion::{black_box, criterion_group, criterion_main, Criterion};
use veloren_voxygen::anim::{quadruped_small, Animation};

pub fn criterion_benchmark(c: &mut Criterion) {
    let skeleton = quadruped_small::QuadrupedSmallSkeleton::new();
    let attr = quadruped_small::SkeletonAttr::default();
    let global_time = 4.0;
    let anim_time = 2.0;
    let mut rate = 1.0;

    c.bench_function("rust idle animation", |b| {
        b.iter(|| {
            quadruped_small::idle::IdleAnimation::update_skeleton(
                black_box(&skeleton),
                black_box(global_time),
                black_box(anim_time),
                black_box(&mut rate),
                black_box(&attr),
            )
        })
    });

    let mut rhai_engine = quadruped_small::idle::AnimationRhaiEngine::new();
    c.bench_function("rhai idle animation", |b| {
        b.iter(|| {
            <&quadruped_small::idle::RhaiIdleAnimation>::update_skeleton(
                black_box(&skeleton),
                black_box((global_time, &mut rhai_engine)),
                black_box(anim_time),
                black_box(&mut rate),
                black_box(&attr),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
