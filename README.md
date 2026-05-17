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

**Build & Run:**
```bash
cargo run
```

## Project Structure

```
src/
├── main.rs      # Game loop & Space Defender logic
├── player.rs    # Player ship
├── enemy.rs     # Enemy objects
└── bullet.rs    # Bullet projectiles
```

## Requirements

- Rust (latest stable)
- cargo

## Dependencies

- raylib 5.5
