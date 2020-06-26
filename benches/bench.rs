use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};

const NUM_PEERS: usize = 20;

fn write_notification(msg: Vec<u8>) {
    black_box(msg);
}

fn distribute(peers: Vec<()>, msg: Vec<u8>) {
    for _peer in peers {
        write_notification(msg.clone());
    }
}

fn distribute_clone_trick(peers: Vec<()>, msg: Vec<u8>) {
    for _peer in peers.iter().skip(1) {
        write_notification(msg.clone());
    }

    if let Some(_peer) = peers.first() {
        write_notification(msg);
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let KB: usize = 1024;

    let mut group = c.benchmark_group("distribute");
    for size in [KB, 10 * KB, 100 * KB, 1000 * KB].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::new("distribute_clone_trick", size), size, |b, size| {
            let msg = vec![0; *size];

            b.iter(|| distribute_clone_trick(vec![(); NUM_PEERS], msg.clone()));
        });

        group.bench_with_input(BenchmarkId::new("distribute", size), size, |b, size| {
            let msg = vec![0; *size];

            b.iter(|| distribute(vec![(); NUM_PEERS], msg.clone()));
        });
    }

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
