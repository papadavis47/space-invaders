# Space Invaders

A small, asset-free clone of the classic Space Invaders arcade game, built
with Rust and Bevy as a learning project.

## Current gameplay

- Move and fire from a player ship
- Fight a five-row formation of moving invaders
- Avoid timed enemy fire
- Track score and three lives
- Win by clearing the formation or lose when the invaders reach you
- Restart after the game ends

## Run the game

Install a current Rust toolchain and the
[Bevy platform dependencies](https://bevyengine.org/learn/quick-start/getting-started/setup/),
then run:

```bash
git clone https://github.com/papadavis47/space-invaders.git
cd space-invaders
cargo run
```

The first build can take a few minutes while Bevy compiles.

## Controls

| Action | Controls |
| --- | --- |
| Move left | Left Arrow or A |
| Move right | Right Arrow or D |
| Fire | Space |
| Restart | Enter |

## Development

Before completing a change, run:

```bash
cargo fmt --check
cargo check
cargo test
```

The next milestone will establish a `GamePlugin`, explicit gameplay system
ordering, fair player-hit recovery, and initial automated tests. See
[AGENTS.md](AGENTS.md) for the current project state and full roadmap.
