// -*- mode: rust; -*-
//
// This file is part of x25519-dalek-ng.
// Copyright (c) 2017-2019 isis agora lovecruft
// Copyright (c) 2019 DebugSteven
// See LICENSE for licensing information.
//
// Authors:
// - isis agora lovecruft <isis@patternsinthevoid.net>
// - DebugSteven <debugsteven@gmail.com>

//! Benchmark the Diffie-Hellman operation.

#[macro_use]
extern crate criterion;
extern crate curve25519_dalek_ng;
extern crate rand_core;
extern crate x25519_dalek_ng;

use criterion::Criterion;

use rand::rngs::OsRng;

use x25519_dalek_ng::EphemeralSecret;
use x25519_dalek_ng::PublicKey;

fn bench_diffie_hellman(c: &mut Criterion) {
    let bob_secret = EphemeralSecret::new(&mut OsRng);
    let bob_public = PublicKey::from(&bob_secret);

    c.bench_function("diffie_hellman", move |b| {
        b.iter_with_setup(
            || EphemeralSecret::new(&mut OsRng),
            |alice_secret| alice_secret.diffie_hellman(&bob_public),
        )
    });
}

criterion_group! {
    name = x25519_benches;
    config = Criterion::default();
    targets =
        bench_diffie_hellman,
}
criterion_main! {
    x25519_benches,
}
