#[macro_use]
extern crate criterion;


use criterion::Criterion;


extern crate CoGeoRs;


use CoGeoRs::algorithms::convex_hull::*;
use CoGeoRs::Point2D;

fn criterion_benchmark(c: &mut Criterion) {
    use rand::Rng;


    let mut jm_points = Vec::new();
    jm_points.push(Point2D::new(0.5,0.5));

    for _ in 0..10000{
        jm_points.push( Point2D::new(rand::thread_rng().gen(),rand::thread_rng().gen()));
    }

    let mut mt_points = jm_points.clone();
    let mut gs_points = jm_points.clone();

    c.bench_function("jarvis march", move |b| b.iter(|| jarvis_march(&mut jm_points.clone())));
    c.bench_function("monotone chain", move |b| b.iter(|| monotone_chain(&mut mt_points.clone())));
    c.bench_function("graham scan", move |b| b.iter(|| jarvis_march(&mut gs_points.clone())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);