# DEVLOG - Week 1 (2026-08-29)

## Ownership

We are already familiar with primitive and reference data types from
other languages. This distinction roughly exists in Rust too, but one
of its strongest aspects is that it offers C-level deterministic
behavior while giving a Kotlin-level garbage-collector experience, and
this all comes from the ownership mechanism.

In languages like C, we could define a reference type and point as many
local variables as we wanted at that address, but this led to dangling
pointers and use-after-frees. Rust prevents this by limiting an object to
a single owner, so not everyone gets to act as if they own it. Anyone
else who wants to use it has to borrow it from the owner, and the owner
decides how: sometimes read-only (immutable borrow), sometimes read and
write (mutable borrow). An object can have many immutable readers at the
same time, or exactly one mutable writer, never both at once. The owner
can also transfer ownership entirely (move).

Ownership behaves a lot like a lobby host in online games. If the host
closes the lobby, everyone gets kicked and their presence is gone: the
owner goes out of scope, so the value is dropped. But if the host hands
leadership to someone else before leaving, the lobby stays open, because
the new leader is still inside: ownership moved, the old variable is
invalid, but the value lives on. A mutable borrow is like filling out a
form on a website that writes to a database. You get permission to make
one specific change, but you can't reach other tables, and you never
become the owner.

The primitive-vs-reference split from other languages maps onto Rust's
`Copy` trait. Small stack values like `f64` are copied, so they behave
like primitives, while heap types like `String` are moved: ownership
transfers instead of a garbage collector cleaning up later.

## Learned

- Enums carry data per variant (algebraic data types), the same idea as
  OCaml variants and Kotlin sealed classes.
- `match` for state-dependent behavior, and why it must be exhaustive:
  every variant has to be handled.
- The `Copy` trait let me `match self.state` by value and drop the `&`
  that was blocking a `&mut self` call inside the match.
- Proof vs test: a proof shows a property holds for all inputs; a test
  guards that property against future changes.

## Broke

- I was already familiar with pattern matching from OCaml, but I didn't
  know that `match` takes ownership. So when I matched on the train
  directly, the train just vanished after the match block ended. This
  confused me for a while, but after some research I understood what was
  happening and fixed it.