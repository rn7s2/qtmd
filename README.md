# qtmd（qù tā mā de）

`tqdm`, what a weird name! Oh wait, I have a better name for it: `qtmd`!

"qtmd" is the phonetic abbreviation of the Chinese "去他妈的" (qù tā mā de), which translates to "<span style="opacity: 0;">go f\*\*k yourself</span>". The next time you meet your Chinese friends, greet them with "qù tā mā de", and you will definitely leave a lasting impression on them.

## Usage

Wrap anything that implements the [Iterator](https://doc.rust-lang.org/core/iter/trait.Iterator.html) trait with qtmd

```rust
use qtmd::qtmd;
for _ in qtmd(0..10000) {
    ...
}
```

Expose trait to allow method chaining

```rust
use qtmd::{Iter, Style};
for _ in (0..).take(10000).qtmd().style(Style::Balloon) {
    ...
}
```

## Well...

Enough jokes, don't use this crate in your projects. And never say "qù tā mā de", it's rude.

For original crate: [tqdm](https://crates.io/crates/tqdm).
