use criterion::{black_box, criterion_group, criterion_main, Criterion};
use Simulator::{Comm, Algorithm};

// TODO: Implement benchmarks with changing matrix sizes and processor sizes
pub fn mult_benchmark(c: &mut Criterion) {
    const MATRIX_WIDTH : usize = 100;
    const MATRIX_HEIGHT : usize = 100;
    const PROCESSOR_WIDTH : usize = 10;
    const PROCESSOR_HEIGHT : usize = 10;
    c.bench_function("Hash Mult", |b| b.iter(|| {
      let a = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let b = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let mut p : Algorithm<isize> = Algorithm::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH);
      p.parallel_mult(black_box(a),black_box(b),black_box(Comm::BROADCAST))
    }));
    c.bench_function("FoxOtto Mult", |b| b.iter(|| {
      let a = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let b = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let mut p : Algorithm<isize> = Algorithm::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH);
      p.parallel_mult(black_box(a),black_box(b),black_box(Comm::FOXOTTO))
    }));
    c.bench_function("Cannon Mult", |b| b.iter(|| {
      let a = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let b = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let mut p : Algorithm<isize> = Algorithm::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH);
      p.parallel_mult(black_box(a),black_box(b),black_box(Comm::CANNON))
    }));
}

pub fn square_benchmark(c: &mut Criterion) {
    const MATRIX_WIDTH : usize = 100;
    const MATRIX_HEIGHT : usize = 100;
    const PROCESSOR_WIDTH : usize = 10;
    const PROCESSOR_HEIGHT : usize = 10;
    c.bench_function("Hash Square", |b| b.iter(|| {
      let a = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let iterations = f64::ceil(f64::log2(a.len() as f64)) as usize;
      let mut p : Algorithm<isize> = Algorithm::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH);
      p.parallel_square(black_box(a),black_box(iterations),black_box(Comm::BROADCAST))
    }));
    c.bench_function("FoxOtto Square", |b| b.iter(|| {
      let a = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let iterations = f64::ceil(f64::log2(a.len() as f64)) as usize;
      let mut p : Algorithm<isize> = Algorithm::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH);
      p.parallel_square(black_box(a),black_box(iterations),black_box(Comm::FOXOTTO))
    }));
    c.bench_function("Cannon Square", |b| b.iter(|| {
      let a = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let iterations = f64::ceil(f64::log2(a.len() as f64)) as usize;
      let mut p : Algorithm<isize> = Algorithm::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH);
      p.parallel_square(black_box(a),black_box(iterations),black_box(Comm::CANNON))
    }));
}

criterion_group!(benches, mult_benchmark, square_benchmark);
criterion_main!(benches);
