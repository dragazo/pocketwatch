# About

`pocketwatch` is a simple Rust CLI timer utility.
It has the ability to read the current local time, count up (stopwatch), or count down (timer) with an audible alarm.

# Installation

`pocketwatch` can be installed through `cargo`.

```sh
cargo install pocketwatch
```

# Usage

Read the current local time:

```sh
pocketwatch read
```

Start a stopwatch

```sh
pocketwatch count-up
```

Start a timer - duration specified as `((h:)m:)s`

```sh
pocketwatch count-down 10
pocketwatch count-down 2:30
pocketwatch count-down 1:30:00
```

# Credits

The [alarm sound](https://freesound.org/people/FoolBoyMedia/sounds/352653/) was created by [FoolBoyMedia](https://freesound.org/people/FoolBoyMedia/).
