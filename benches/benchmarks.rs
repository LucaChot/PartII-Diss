use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use Simulator::{MatMul, Hash, FoxOtto, Cannon};

// TODO: Implement benchmarks with changing matrix sizes and processor sizes
pub fn mult_benchmark(c: &mut Criterion) {
    const MATRIX_WIDTH : usize = 125;
    const MATRIX_HEIGHT : usize = 125;
    const PROCESSOR_WIDTH : usize = 10;
    const PROCESSOR_HEIGHT : usize = 10;
    c.bench_function("Hash Mult", |b| b.iter(|| {
      let a = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let b = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let mut p : MatMul<isize> = MatMul::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH);
      p.parallel_mult::<Hash>(black_box(a),black_box(b))
    }));
    c.bench_function("FoxOtto Mult", |b| b.iter(|| {
      let a = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let b = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let mut p : MatMul<isize> = MatMul::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH);
      p.parallel_mult::<FoxOtto>(black_box(a),black_box(b))
    }));
    c.bench_function("Cannon Mult", |b| b.iter(|| {
      let a = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let b = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let mut p : MatMul<isize> = MatMul::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH);
      p.parallel_mult::<Cannon>(black_box(a),black_box(b))
    }));
}

pub fn square_benchmark(c: &mut Criterion) {
    const MATRIX_WIDTH : usize = 125;
    const MATRIX_HEIGHT : usize = 125;
    const PROCESSOR_WIDTH : usize = 10;
    const PROCESSOR_HEIGHT : usize = 10;
    c.bench_function("Hash Square", |b| b.iter(|| {
      let a = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let iterations = f64::ceil(f64::log2(a.len() as f64)) as usize;
      let mut p : MatMul<isize> = MatMul::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH);
      p.parallel_square::<Hash>(black_box(a),black_box(iterations))
    }));
    c.bench_function("FoxOtto Square", |b| b.iter(|| {
      let a = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let iterations = f64::ceil(f64::log2(a.len() as f64)) as usize;
      let mut p : MatMul<isize> = MatMul::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH);
      p.parallel_square::<FoxOtto>(black_box(a),black_box(iterations))
    }));
    c.bench_function("Cannon Square", |b| b.iter(|| {
      let a = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
      let iterations = f64::ceil(f64::log2(a.len() as f64)) as usize;
      let mut p : MatMul<isize> = MatMul::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH);
      p.parallel_square::<Cannon>(black_box(a),black_box(iterations))
    }));
}

pub fn bench_matrices(c : &mut Criterion) {
  let mut group = c.benchmark_group("Matrices");
  const PROCESSOR_WIDTH : usize = 10;
  const PROCESSOR_HEIGHT : usize = 10;
  for i in 1..11 {
    let matrix_side = 20 * i;
    group.bench_with_input(BenchmarkId::new("HASH", matrix_side), &matrix_side, 
      |b, matrix_side| b.iter(|| {
        let a = vec![vec![2; *matrix_side]; *matrix_side];
        let iterations = f64::ceil(f64::log2(a.len() as f64)) as usize;
        let mut p : MatMul<isize> = MatMul::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH);
        p.parallel_square::<Hash>(black_box(a),black_box(iterations))
      }));
    group.bench_with_input(BenchmarkId::new("FOXOTTO", matrix_side), &matrix_side, 
      |b, matrix_side| b.iter(|| {
        let a = vec![vec![2; *matrix_side]; *matrix_side];
        let iterations = f64::ceil(f64::log2(a.len() as f64)) as usize;
        let mut p : MatMul<isize> = MatMul::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH);
        p.parallel_square::<FoxOtto>(black_box(a),black_box(iterations))
      }));
    group.bench_with_input(BenchmarkId::new("CANNON", matrix_side), &matrix_side, 
      |b, matrix_side| b.iter(|| {
        let a = vec![vec![2; *matrix_side]; *matrix_side];
        let iterations = f64::ceil(f64::log2(a.len() as f64)) as usize;
        let mut p : MatMul<isize> = MatMul::new(PROCESSOR_HEIGHT,PROCESSOR_WIDTH);
        p.parallel_square::<Cannon>(black_box(a),black_box(iterations))
      }));
  }
  group.finish()
}

pub fn bench_processors(c : &mut Criterion) {
  let mut group = c.benchmark_group("Processors");
  const MATRIX_WIDTH : usize = 50;
  const MATRIX_HEIGHT : usize = 50;
  for processor_side in 3..11 {
    group.bench_with_input(BenchmarkId::new("HASH", processor_side), &processor_side, 
      |b, processor_side| b.iter(|| {
        let a = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
        let iterations = f64::ceil(f64::log2(a.len() as f64)) as usize;
        let mut p : MatMul<isize> = MatMul::new(*processor_side, *processor_side);
        p.parallel_square::<Hash>(black_box(a),black_box(iterations))
      }));
    group.bench_with_input(BenchmarkId::new("FOXOTTO", processor_side), &processor_side, 
      |b, processor_side| b.iter(|| {
        let a = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
        let iterations = f64::ceil(f64::log2(a.len() as f64)) as usize;
        let mut p : MatMul<isize> = MatMul::new(*processor_side, *processor_side);
        p.parallel_square::<FoxOtto>(black_box(a),black_box(iterations))
      }));
    group.bench_with_input(BenchmarkId::new("CANNON", processor_side), &processor_side, 
      |b, processor_side| b.iter(|| {
        let a = vec![vec![2; MATRIX_WIDTH]; MATRIX_HEIGHT];
        let iterations = f64::ceil(f64::log2(a.len() as f64)) as usize;
        let mut p : MatMul<isize> = MatMul::new(*processor_side, *processor_side);
        p.parallel_square::<Cannon>(black_box(a),black_box(iterations))
      }));
  }
  group.finish()

}
criterion_group!(benches, mult_benchmark, square_benchmark, bench_matrices, bench_processors);
criterion_main!(benches);
