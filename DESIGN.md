# LD45 Design
## Features (conceptually)
- Left/right movement
- Simple jump
- Improved jump (short press kills velocity)
- Hover (hold jump in-air)
- Walls without collision
- Walls with collision

## Tile Properties
- `solid` (`boolean`)  
  Marks a tile as solid (player will only collide with them once `AddCollisions` feature was triggered).
- `spike` (`boolean`)  
  Makes the tile a spike, which will reset the level when touched by the player.

## Object Properties
### Type: "Player"
No properties.

### Type: "Feature"
- `feature_type` (`string`)  
  One of:  
  - `AddCollisions`  
    Makes the player collide with solid walls.
  - `AddGravity1`  
    Gives the player the _first_ gravity and jump strength(s). (_before_ jetpack)
  - `AddGravity1`  
    Gives the player the _second_ gravity and jump strength(s). (_after_ jetpack)
  - `AddJump`  
    Gives the player the ability to _jump_ and to _wall jump_.
  - `AddSingleSprite`  
    Gives the player a single sprite.
  - `AddAnimatedSprite`  
    Gives the player its full set of animations (idle, walking, jumping, etc.).
  - `AddEnemySprite`  
    Adds full animations to all enemies.
  - `AddRun`  
    Gives the player the ability to move faster using the run button.
  - `SetSong1`  
    Play first song.
  - `SetSong2`  
    Play second song.

### Type: "Enemy"
- `enemy_type` (`string`)  
  One of:  
  - `Ground`
  - `Flying`
- `pace_distance_x` (`float`) __optional__  
  How far this pacing enemy should walk, before turning around.  
  Specified distance is the distance from the placed object, to either the left or right sides.  
  If omitted, this enemy will _not pace horizontally_.
- `pace_distance_y` (`float`) __optional__  
  Same as `pace_distance_x`, but for the `y` direction (vertically).  
  If omitted, this enemy will _not pace vertically_.

## Sequential Features
1.  4 directional movement, no gravity __DONE__
2.  first encounter with non solid tiles __DONE__
3.  first encounter with solid tiles __DONE__
4.  gravity + first music trigger __DONE__
5.  first encounter with non textured enemy __DONE (not non-textured, rather single-sprite-textured)__
6.  jump and wall jump __DONE (jump)__
7.  player textures (single frame) __DONE__
8.  bg + player animation + second music trigger __DONE (player animation, music trigger)__
9.  enemy textures __DONE__
10. spikes __DONE__
11. enemy animations __DONE (happens with 'enemy textures')__
12. parallax bgs?
13. jetpack jump + hover
14. first wave of environmental decoration
15. sprint + third music trigger
16. second wave of environmental decoration
17. dash
18. third wave of environmental decoration
