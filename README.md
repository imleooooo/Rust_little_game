# Rust Little Game

A collection of small games built with Rust and [raylib](https://www.raylib.com/).

## Games

### Space Defender

A classic space shooter game.

**Controls:**
- `←` `→` - Move left/right
- `SPACE` - Shoot
- `R` - Restart (after game over)

**Gameplay:**
- Destroy incoming enemies to score points
- Each enemy destroyed = 10 points
- Difficulty increases as your score grows
- Game over if an enemy reaches the bottom or collides with you

### Angry Birds

Physics-based puzzle game using Rapier2D physics engine.

**Controls:**
- **Mouse drag** - Aim slingshot
- **Mouse release** - Launch bird
- `SPACE` - Use bird ability
- `ENTER` - Next level (after completing)
- `R` - Restart level

**Bird Types:**
- **Red** - Boost upward
- **Blue** - Split into 3 birds
- **Yellow** - Speed boost
- **Black** - Explosion damage
- **White** - Drop eggs

**Block Types:**
- **Wood** (Brown) - Medium durability
- **Stone** (Gray) - High durability
- **Ice** (Light Blue) - Low durability

**Scoring:**
- Destroy block = 10 points
- Kill pig = 50 points
- 3 Stars: Use ≤ 2 birds
- 2 Stars: Use ≤ 3 birds
- 1 Star: Complete level

**Build & Run:**
```bash
cargo run
```

## Project Structure

```
src/
├── main.rs                    # Menu and game selection
├── space_defender/            # Space Defender game
│   ├── mod.rs
│   ├── player.rs
│   ├── enemy.rs
│   └── bullet.rs
└── angry_birds/               # Angry Birds game
    ├── mod.rs                # Main game loop
    ├── physics.rs            # Rapier2D wrapper
    ├── bird.rs              # Bird types & abilities
    ├── block.rs             # Block types
    ├── pig.rs               # Pig enemies
    ├── slingshot.rs         # Slingshot mechanics
    └── level.rs             # Level data (5 levels)
```

## Requirements

- Rust (latest stable)
- cargo

## Dependencies

- raylib 5.5
- rapier2d 0.22
- nalgebra 0.33
