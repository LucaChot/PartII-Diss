use criterion::{black_box, criterion_group, criterion_main, Criterion};
use project::{Comm, Processor};

pub fn criterion_benchmark(c: &mut Criterion) {
    const MATRIX_WIDTH : usize = 100;
    const MATRIX_HEIGHT : usize = 100;
    const PROCESSOR_WIDTH : usize = 10;
    const PROCESSOR_HEIGHT : usize = 10;
    c.bench_function("Hash", |b| b.iter(|| {
      let a = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let b = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let c = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let mut p : Processor<isize> = Processor::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH);
      p.parralel_mult(black_box(a),black_box(b),black_box(Comm::BROADCAST))
    }));
    c.bench_function("FoxOtto", |b| b.iter(|| {
      let a = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let b = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let c = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let mut p : Processor<isize> = Processor::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH);
      p.parralel_mult(black_box(a),black_box(b),black_box(Comm::FOXOTTO))
    }));
    c.bench_function("Cannon", |b| b.iter(|| {
      let a = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let b = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let c = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let mut p : Processor<isize> = Processor::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH);
      p.parralel_mult(black_box(a),black_box(b),black_box(Comm::CANNON))
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
