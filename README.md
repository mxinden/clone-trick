Small benchmark testing whether the following *manual loop unrolling* improves
performance.

```rust
// Simple version
fn distribute(peers: Vec<()>, msg: Vec<u8>) {
    for _peer in peers {
        write_notification(msg.clone());
    }
}

// Optimized version
fn distribute_clone_trick(peers: Vec<()>, msg: Vec<u8>) {
    for _peer in peers.iter().skip(1) {
        write_notification(msg.clone());
    }

    if let Some(_peer) = peers.first() {
        write_notification(msg);
    }
}
```

Run `cargo bench` to produce report in `target/criterion/report/index.html`.
