# Space Invaders Project Guide

## Project purpose

This is a small clone of the classic Space Invaders arcade game, built as a
learning project for Rust and Bevy. Favor changes that make Bevy's ECS,
scheduling, state, and resource patterns easy to understand. Keep each
milestone playable and avoid adding abstractions before the game needs them.

## Technology

- Rust 2024 edition
- Bevy 0.16
- One desktop binary
- No external art, font, audio, or random-number dependencies yet

## Current state

The first asset-free gameplay prototype is complete and lives in
`src/main.rs`.

Implemented behavior:

- A fixed 900 x 700 game window and 2D camera
- Player movement with Left/Right or A/D
- Player firing with Space, limited to one player bullet at a time
- A five-row, ten-column invader formation
- Formation edge detection, direction changes, downward movement, and
  increasing horizontal speed
- Timed enemy firing
- Axis-aligned bullet collision detection
- Score tracking and a three-life HUD
- Win when all invaders are destroyed
- Loss when lives reach zero or an invader reaches the player
- Game-over screen and restart with Enter

All gameplay components, resources, states, setup, systems, and UI currently
share `src/main.rs`. This was intentional for the first learning milestone, but
the file is now large enough that the next milestone should establish one
clear gameplay boundary.

The repository is currently on `main`, tracking `origin/main`. The latest
change fixed a Bevy B0002 runtime conflict in `check_game_over`.

## Current verification

Run these checks before considering a change complete:

```bash
cargo fmt --check
cargo check
cargo test
```

Run `cargo run` for gameplay changes and verify the behavior interactively.
The project currently compiles and its test harness passes, but it has zero
automated tests.

## Known limitations

- Gameplay systems are registered as one tuple without explicit semantic
  ordering.
- Collision despawns use deferred `Commands`, so outcome evaluation can observe
  entities that were marked for despawn earlier in the same frame.
- Multiple enemy bullets can remove multiple lives in one frame, with no
  recovery or invulnerability period.
- Enemy shooters are selected from every surviving invader rather than only
  the lowest invader in each column.
- Enemy selection is based on elapsed time rather than a real random source.
- Clearing the formation ends the game; there is no wave progression.
- There are no defensive bunkers, mystery ship, pause/title screen, sprites,
  animation, audio, or effects.
- Collision and game-rule behavior has no automated coverage.

## Next milestone: gameplay foundation

Create a focused branch such as `feature/gameplay-foundation`. Do not combine
this milestone with an art or audio pass.

### 1. Establish a small plugin boundary

- Keep `src/main.rs` responsible for constructing the Bevy app, configuring the
  window, and adding plugins.
- Introduce one `GamePlugin`, initially in `src/game.rs`, which owns gameplay
  resources, states, components, and systems.
- Do not immediately split gameplay into many files. Extract player, invader,
  collision, or UI modules later when those boundaries carry enough behavior
  to justify themselves.

### 2. Make gameplay order explicit

Define and configure named system sets for the semantic frame order:

1. Input
2. Movement and firing
3. Collision detection and resolution
4. Outcome evaluation
5. HUD/presentation updates

Ensure deferred despawns are applied before game-outcome rules query surviving
entities. The result of a frame must not depend on incidental scheduler order.

### 3. Add a fair life-loss flow

- One damage event should consume one life.
- Remove or neutralize bullets that would cause repeated immediate damage.
- Add a short recovery/invulnerability period with simple visual feedback,
  such as blinking.
- Preserve score and remaining invaders during recovery.
- Transition to game over only when the final life is consumed.

### 4. Add the first automated tests

At minimum, cover:

- Overlapping and non-overlapping rectangles, including edge behavior
- Formation boundary/turn calculations if extracted into pure functions
- Win/loss rule precedence
- Life loss saturating at zero and consuming only one life per resolved hit

Prefer small pure rule functions where that improves clarity. Do not attempt to
unit-test every ECS system merely to increase test count.

### Acceptance criteria

- The existing controls and game loop still work.
- Gameplay is owned by a `GamePlugin`.
- System ordering expresses the intended frame sequence.
- Destroying the final invader produces the correct result after deferred
  commands are resolved.
- A cluster of enemy bullets cannot erase several lives in one frame.
- Recovery behavior is visible and temporary.
- `cargo fmt --check`, `cargo check`, and `cargo test` pass.
- The changed flow is played manually with `cargo run`.

## Roadmap after the foundation

### Milestone 2: defensive bunkers

- Build bunkers from small destructible sections.
- Let player and enemy bullets damage them.
- Reuse the collision-resolution model rather than adding bunker-only
  collision loops.
- Verify that openings emerge naturally as sections are destroyed.

### Milestone 3: waves and difficulty

- Track and display the wave number.
- Spawn a new formation after a cleared wave.
- Increase formation speed and enemy firing frequency in controlled steps.
- Select enemy shooters from the lowest surviving invader in each column.
- Decide whether the game is endless or ends after a documented number of
  waves.

### Milestone 4: classic mechanics

- Add a mystery/UFO ship and bonus scoring.
- Give invader rows distinct score values.
- Add a title screen and pause state.
- Revisit firing cadence and movement values through playtesting.

### Milestone 5: presentation

- Replace colored rectangles with coherent pixel-art sprites.
- Add invader animation and player/invader explosion feedback.
- Add sound effects and music.
- Add restrained effects such as flashes, particles, or screen shake without
  obscuring gameplay.

### Milestone 6: packaging

- Add a player-facing README with controls, screenshots, and build
  instructions.
- Produce and test a release build.
- Optionally add a WebAssembly target and document browser deployment.

## Working approach

- Keep `main` stable and put one milestone on one feature branch.
- Prefer behavior-focused commits that are easy to review and playtest.
- Fix gameplay rules at their shared source rather than patching individual
  examples.
- Preserve the simple asset-free version until the underlying gameplay is
  dependable.
- Explain new Bevy concepts in code structure and names; add comments only when
  they clarify a non-obvious rule or engine behavior.
- Update this file when a milestone changes the documented current state or
  invalidates a roadmap item.
