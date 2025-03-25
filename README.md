Initial sketch for the trait was

```
pub trait IndexSettings<T: DerefMut<Target = H266MergersPadSettings>> {
    fn settings_at(&mut self, idx: usize) -> Option<T>;

    fn n_pads(&self) -> usize;
}
```

But when we try to implement it for a target of `MutexGuard` the compiler rebuffs us.

The first attempt fails with

```
error[E0726]: implicit elided lifetime not allowed here
  --> first/src/lib.rs:36:20
   |
36 | impl IndexSettings<MutexGuard<H266MergersPadSettings>> for Vec<H266MergersPad> {
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected lifetime parameter
   |
```

The second fails with

```
error: `impl` item signature doesn't match `trait` item signature
  --> second/src/lib.rs:37:5
   |
21 |     fn settings_at(&mut self, idx: usize) -> Option<T>;
   |     --------------------------------------------------- expected `fn(&'1 mut Vec<H266MergersPad>, usize) -> Option<MutexGuard<'2, H266MergersPadSettings>>`
...
37 |     fn settings_at(&mut self, idx: usize) -> Option<MutexGuard<H266MergersPadSettings>...
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found `fn(&'1 mut Vec<H266MergersPad>, usize) -> Option<MutexGuard<'1, H266MergersPadSettings>>`
   |
   = note: expected signature `fn(&'1 mut Vec<H266MergersPad>, usize) -> Option<MutexGuard<'2, H266MergersPadSettings>>`
              found signature `fn(&'1 mut Vec<H266MergersPad>, usize) -> Option<MutexGuard<'1, H266MergersPadSettings>>`
help: the lifetime requirements from the `impl` do not correspond to the requirements in the `trait`
  --> second/src/lib.rs:21:53
```

The third fails with
```
error: lifetime may not live long enough
  --> third/src/lib.rs:61:28
   |
51 | pub fn foo<'a, T: DerefMut<Target = H266MergersPadSettings>>(
   |            -- lifetime `'a` defined here
...
60 |         .map(|i| {
   |              --- lifetime `'1` represents this closure's body
61 |             let settings = pads.settings_at(i).unwrap();
   |                            ^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'a`
   |
   = note: closure implements `FnMut`, so references to captured variables can't escape the closure

error[E0597]: `pads` does not live long enough
  --> third/src/lib.rs:55:24
   |
51 | pub fn foo<'a, T: DerefMut<Target = H266MergersPadSettings>>(
   |            -- lifetime `'a` defined here
52 |     mut pads: impl IndexSettings<'a, T> + 'a,
   |     -------- binding `pads` declared here
...
55 |         let settings = pads.settings_at(i).unwrap();
   |                        ^^^^---------------
   |                        |
   |                        borrowed value does not live long enough
   |                        argument requires that `pads` is borrowed for `'a`
...
65 | }
   | - `pads` dropped here while still borrowed
```

The fourth fails with

```
error[E0207]: the lifetime parameter `'a` is not constrained by the impl trait, self type, or predicates
  --> fourth/src/lib.rs:28:6
   |
28 | impl<'a> IndexSettings for &mut [H266MergersPadSettings] {
   |      ^^ unconstrained lifetime parameter
```

The fifth fails with

```
error: missing required bound on `Target`
  --> fifth/src/lib.rs:21:5
   |
21 |     type Target<'a>: DerefMut<Target = H266MergersPadSettings>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-
   |                                                               |
   |                                                               help: add the required where clause: `where Self: 'a`
   |
   = note: this bound is currently required to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information

error: lifetime may not live long enough
  --> fifth/src/lib.rs:63:28
   |
55 | pub fn foo<'a, T: DerefMut<Target = H266MergersPadSettings>>(mut pads: impl Ind...
   |            -- lifetime `'a` defined here
...
62 |         .map(|i| {
   |              --- lifetime `'1` represents this closure's body
63 |             let settings = pads.settings_at(i).unwrap();
   |                            ^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'a`
   |
   = note: closure implements `FnMut`, so references to captured variables can't escape the closure
```

That hint about `where Self: 'a` turns out to be the solution.

So the final trait is
```
pub trait IndexSettings {
    type Target<'a>: DerefMut<Target = H266MergersPadSettings>
    where
        Self: 'a; // where clauses on associated types are unstable until rust 1.65 and nonexistent before 1.61
    fn settings_at(&mut self, idx: usize) -> Option<Self::Target<'_>>;

    fn n_pads(&self) -> usize;
}
```

This syntax involving where clauses in an associated type was unstable until Rust 1.65, and nonexistent before 1.61
