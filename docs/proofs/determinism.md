# Theorem: The Simulation Is Deterministic

## What this proves

This is a **determinism proof**. It shows that running the simulation
twice, from the same source, produces byte-identical output every
time. This is the project's first principle ("Determinism First"): a
run must be reproducible, so that a bug seen once can be seen again,
and so that recorded runs can be replayed exactly.

## Statement

> **Theorem** With fixed initial values and a fixed timestep,
> two executions of the program produce identical output sequences.

## What determinism means, precisely

A program is deterministic when its output depends **only** on its
input, and on nothing else:

$$
\text{output} = f(\text{input})
$$

If the output ever depends on anything outside the input, two runs
with the "same" input can diverge, and determinism is lost. So the
proof reduces to one question: does the output depend on anything
other than the fixed values written in the source?

## The sources of non-determinism

There is a known, finite list of things that make a program's output
depend on something other than its input:

1. **Wall-clock time** (e.g. reading the system clock as `dt`).
2. **Unseeded randomness** (a fresh random seed each run).
3. **Concurrency** (thread interleaving order is not fixed).
4. **External I/O ordering** (reads from network/disk arriving in
   varying order).
5. **Uninitialized memory** (reading whatever garbage was there).
6. **Unordered collection iteration** (e.g. `HashMap` in Rust uses a
   randomized seed per run, so iteration order varies).

If a program touches **none** of these, its output is a pure function
of its fixed inputs, and it is deterministic.

## Proof by exhaustion of the causes

We check the entire code reachable from `main` against the list. The
reachable code is small: `main`, `Train::advance`, `Train::new`,
`Train::apply_braking`, `Train::traveled_distance`, and the getters.

1. **Wall-clock time.** No call reads a clock. `dt` is a fixed
   constant passed in the loop, not measured from real time. ✓ absent
2. **Unseeded randomness.** No `rand` usage anywhere. All values are
   literals in the source. ✓ absent
3. **Concurrency.** No threads, no async, no `spawn`. The loop runs
   sequentially, one tick after another, in a fixed order. ✓ absent
4. **External I/O ordering.** The only I/O is `println!`, which
   *writes* output; it never *reads* anything that could vary. ✓ absent
5. **Uninitialized memory.** Rust forbids reading uninitialized
   values in safe code, and this code is entirely safe (`unsafe`
   appears nowhere). ✓ absent
6. **Unordered collection iteration.** No `HashMap` or other
   unordered collection is used. State is held in plain fields and a
   single `enum`. ✓ absent

All six causes are absent. Therefore the output depends only on the
fixed initial values and on deterministic `f64` arithmetic.

<div style="text-align: right;">

$\blacksquare$

</div>

## The one subtlety: floating point

`f64` arithmetic is deterministic **on a fixed platform with fixed
compiler settings**: IEEE-754 pins down the result of each operation
bit-for-bit, so the same operations in the same order give the same
bits every run. This is what makes the output reproducible here.

The caveat, noted for the future: results are not guaranteed
identical *across different CPU architectures* (e.g. x86 vs ARM),
where intermediate rounding can differ. All current runs are on the
same machine, so the theorem holds. Cross-platform determinism, if
ever required, would need fixed-point arithmetic or a strict
float-control mode. Out of scope for now.

## How this is checked in practice

The proof is a claim about the code as written today. It rests on one
assumption: that no future change introduces any of the six causes.
That assumption is guarded automatically by comparing the hashes of
two runs:

```bash
cargo run | sha256sum
cargo run | sha256sum
```

Identical hashes mean byte-identical output. If someone later adds,
say, a wall-clock `dt` or a `HashMap` over trains, the two hashes
will differ and the check fails, pointing back to this proof.

The proof shows determinism holds now, by reasoning. The hash check
keeps it holding later, by testing. Together they cover both "is it
true?" and "did someone break it?".