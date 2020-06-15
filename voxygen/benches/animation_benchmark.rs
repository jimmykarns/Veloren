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

    /* let mut rhai_engine = quadruped_small::idle::AnimationRhaiEngine::new();
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

    let leon_ast = quadruped_small::idle::LeonAst::new();
    c.bench_function("leon idle animation", |b| {
        b.iter(|| {
            <&quadruped_small::idle::LeonIdleAnimation>::update_skeleton(
                black_box(&skeleton),
                black_box((global_time, &leon_ast)),
                black_box(anim_time),
                black_box(&mut rate),
                black_box(&attr),
            )
        })
    });

    let mut mun_runtime = quadruped_small::idle::MunRuntime::new();
    c.bench_function("mun idle animation", |b| {
        b.iter(|| {
            <&quadruped_small::idle::MunIdleAnimation>::update_skeleton(
                black_box(&skeleton),
                black_box((global_time, &mut mun_runtime)),
                black_box(anim_time),
                black_box(&mut rate),
                black_box(&attr),
            )
        })
    }); */

    let idle_lib = quadruped_small::idle::IdleLib::new();
    c.bench_function("libloading idle animation", |b| {
        b.iter(|| {
            <&quadruped_small::idle::LibIdleAnimation>::update_skeleton(
                black_box(&skeleton),
                black_box((global_time, &idle_lib)),
                black_box(anim_time),
                black_box(&mut rate),
                black_box(&attr),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
