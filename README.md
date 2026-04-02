# CrabScat

`crabscat` is a Rust library for fitting analytical form factors to SAXS data.

The project is being built in small vertical slices so the code stays easy to
understand while still leaving room for more advanced fitting routines,
parallel execution, and richer I/O later.

## Current Layout

```text
crabscat/
├── Cargo.toml
├── README.md
├── examples/
│   └── sphere_fit.rs
├── src/
│   ├── lib.rs
│   ├── error.rs
│   ├── data/
│   │   ├── mod.rs
│   │   └── profile.rs
│   ├── fitting/
│   │   ├── least_squares.rs
│   │   ├── mod.rs
│   │   └── result.rs
│   ├── form_factors/
│   │   ├── cylinder.rs
│   │   ├── mod.rs
│   │   └── sphere.rs
│   ├── io/
│   │   ├── dat.rs
│   │   └── mod.rs
│   ├── models/
│   │   ├── mod.rs
│   │   └── single_particle.rs
│   └── main.rs
└── tests/
    └── sphere_fit.rs
```

## What Each Part Is For

- `src/lib.rs`
  Re-exports the main public API so users can start with a small set of names.

- `src/data/`
  Holds in-memory SAXS data structures. Right now that is mainly `Profile`,
  which stores `q`, intensity, and optional uncertainty values.

- `src/form_factors/`
  Holds the mathematical kernels for analytical shapes such as `Sphere` and,
  later, `Cylinder`, ellipsoids, shells, and more.

- `src/models/`
  Holds higher-level scattering models built on top of form factors. A model
  can combine scale terms, backgrounds, structure factors, smearing, and other
  physics without pushing all of that complexity into the raw form-factor code.

- `src/fitting/`
  Holds objective functions and later optimizers. This keeps fitting logic
  separate from model evaluation logic.

- `src/io/`
  Holds file readers and writers. Parsing and data loading stay out of the
  numerical core so the math code remains easy to test.

- `examples/`
  Small runnable end-to-end examples. These are especially useful while the
  library is still growing.

- `tests/`
  Regression tests for each vertical slice.

## Why The Code Starts With A Scalar Form-Factor API

The current `FormFactor` trait uses a scalar method as the core mathematical
operation:

```rust
pub trait FormFactor {
    fn intensity_at(&self, q: f64) -> f64;

    fn evaluate(&self, q: &[f64]) -> Vec<f64> {
        q.iter().map(|&value| self.intensity_at(value)).collect()
    }
}
```

At first glance it can feel more natural to write everything as
`&[f64] -> Vec<f64>`, and that is a perfectly good user-facing API. The reason
for keeping the scalar method underneath is that it gives a cleaner foundation.

### Why keep `intensity_at(q)` as the core method?

- The math is naturally pointwise.
  For many analytical form factors, the actual formula is defined for one
  `q` value at a time.

- Special cases are easier to handle.
  For example, the `q = 0` limit for a sphere is easier to reason about and
  test in a scalar function than inside a larger batch routine.

- The function becomes easier to test.
  It is very convenient to write small unit tests like "what happens at
  `q = 0`?" or "does the amplitude stay finite near zero?"

- It stays composable for later algorithms.
  Orientation averaging, resolution smearing, quadrature, adaptive
  integration, and some diagnostic routines often need to evaluate the model
  at one point at a time inside a larger loop.

- It avoids repeating the same looping code in every form factor.
  The trait can provide one shared default implementation of the batch method.

### Why also keep `evaluate(&[f64]) -> Vec<f64>`?

Because it is convenient and ergonomic.

The scalar method is the core implementation, but users of the library usually
want to evaluate many `q` values at once. The default `evaluate` method gives
that batch API immediately, without forcing every form factor to reimplement
the same `iter().map().collect()` loop.

That means the design supports both styles:

- scalar evaluation for the mathematical core
- slice-based evaluation for everyday use

## Why This Is Good For Parallelization Later

Using a scalar core does not block parallelization. In fact, it usually makes
parallelization easier.

The important idea is:

- `intensity_at(q)` should be a pure calculation
- `evaluate(q_values)` can decide how to loop over many points

Today the default `evaluate` method is serial:

```rust
fn evaluate(&self, q: &[f64]) -> Vec<f64> {
    q.iter().map(|&value| self.intensity_at(value)).collect()
}
```

Later, if the crate adds optional `rayon` support, the batch method can switch
to parallel iteration while still reusing the same scalar kernel:

```rust
use rayon::prelude::*;

fn evaluate(&self, q: &[f64]) -> Vec<f64> {
    q.par_iter()
        .map(|&value| self.intensity_at(value))
        .collect()
}
```

That gives a very nice separation of concerns:

- the scalar method defines the physics and mathematics
- the batch method defines how work is scheduled

This is one of the main reasons the scalar core is a strong long-term choice.

## A Likely Future Shape With Rayon

When parallel evaluation becomes useful, a good next step is to add it as an
optional Cargo feature rather than making it mandatory from the start.

For example, the project could later grow toward something like:

```toml
[features]
default = []
parallel = ["dep:rayon"]

[dependencies]
rayon = { version = "1", optional = true }
```

And then:

```rust
#[cfg(feature = "parallel")]
use rayon::prelude::*;

pub trait FormFactor {
    fn intensity_at(&self, q: f64) -> f64;

    #[cfg(not(feature = "parallel"))]
    fn evaluate(&self, q: &[f64]) -> Vec<f64> {
        q.iter().map(|&value| self.intensity_at(value)).collect()
    }

    #[cfg(feature = "parallel")]
    fn evaluate(&self, q: &[f64]) -> Vec<f64>
    where
        Self: Sync,
    {
        q.par_iter()
            .map(|&value| self.intensity_at(value))
            .collect()
    }
}
```

The important point is that parallelization happens at the slice-processing
layer, not by changing the mathematical core of each form factor.

## Design Goals For The Project

- Start simple.
  Keep the code easy to read while learning Rust.

- Grow in vertical slices.
  Add one complete path at a time: data type, model, objective function,
  example, and test.

- Keep modules focused.
  Data, models, fitting, and I/O should not get tangled together early.

- Keep numerical kernels pure.
  Pure functions are easier to test, reuse, and parallelize.

- Delay abstraction until it pays for itself.
  Start concrete, then extract traits or more advanced structures when a
  second or third implementation makes the pattern clear.

## Near-Term Growth Path

A reasonable next sequence for the project is:

1. Add another form factor such as a real cylinder implementation.
2. Add a small optimizer in `src/fitting/`, such as a grid search or a simple
   local minimizer.
3. Expand `src/io/` with more robust SAXS file parsing.
4. Add more tests around edge cases and parameter validation.
5. Add an optional `parallel` feature with `rayon` once model evaluation
   becomes expensive enough for it to matter.

## Running The Current Example

```bash
cargo run --example sphere_fit
```

## Running Tests

```bash
cargo test
```
