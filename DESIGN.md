# LD45 Tiled Documentation
Tiled maps must have a tile size of __16x16px__.

## Tile Properties
- `solid` (`boolean`)  
  Marks a tile as solid (player will only collide with them once `AddCollisions` feature was triggered).
- `spike` (`boolean`)  
  Makes the tile a spike, which will reset the level when touched by the player.
- `always_loaded` (`boolean`)  
  Makes this tile always be loaded. Use with caution.

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
  - `AddGravity2`  
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
  - `AddDash`  
    Gives the player the ability to dash in-air, by pressing down any pair of movement keys,  
    and then pressing the jump button again.  
    They can dash once, after which the player must stand on solid ground again to recharge the dash.  
    The dash is _8-directional_.
  - `SetSongN`  
    Set the currently playing song to the `N`th song.  
    `N` should be replaced with an integer, starting at `0` for the first song, etc.  
    Will panic if `N` is not an integer.  
    If `N` is an integer, but the song at the given index doesn't exist, then no song will be played.

### Type: "Enemy"
- `enemy_type` (`string`)  
  One of:  
  - `Ground`
  - `Flying`
- `pace_distance_x` (`float`) __optional__  
  How far this pacing enemy should walk, before turning around.  
  Specified distance is the distance from the placed object, to either the left or right sides.  
  The number's sign specifies the initial direction, in which this enemy will move.  
  So if the value is _larger than (or equal to) 0.0_, then the enemy will initially move _right_;  
  if it is _less than 0.0_, the enemy will initially move _left_.
  If omitted, this enemy will _not pace horizontally_.
- `pace_distance_y` (`float`) __optional__  
  Same as `pace_distance_x`, but for the `y` direction (vertically).  
  If the value is _larger than (or equal to) 0.0_, then the enemy will initially move _up_;  
  if it is _less than 0.0_, the enemy will initially move _down_.
  If omitted, this enemy will _not pace vertically_.
- `always_loaded` (`boolean`)  
  Makes this tile always be loaded. Use with caution.

### Type: "Background"
Backgrounds will automatically start following the camera,  
once the camera's left edge passes the background's left edge.

- `image` (`string`)  
  The filename of the background image to use.  
  The background image must be placed under `resources/spritesheets/bg`.  
  Accompanying the image file, you must also create a config `.ron` file,  
  in the same directory.  
  See the example `.ron` file `resources/spritesheets/bg/background_example.ron`  
  for more information on how to create the `.ron` file.
- `always_loaded` (`boolean`)  
  Makes this tile always be loaded. Use with caution.  
  (shouldn't be necessary for background)

### Type: "Checkpoint"
No properties.

### Type: "Indicator"
Indicators are images, which only become visible after the associated feature was triggered.

- `image` (`string`)  
  The filename of the image to use.  
  The image file must be placed under `resources/spritesheets/indicators`.  
  Accompanying the image file, you must also create a config `.ron` file,  
  in the same directory.  
  See the example `.ron` file `resources/spritesheets/bg/background_example.ron`  
  for more information on how to create the `.ron` file.
- `feature_trigger` (`string`)  
  The `Feature`, which will make this indicator visible when triggered.  
  The value has the same format as the `Feature`'s `feature_type`.
- `always_loaded` (`boolean`)  
  Makes this tile always be loaded. Use with caution.  
  (shouldn't be necessary for indicators)

# Sequential Features
1.  4 directional movement, no gravity __DONE__
2.  first encounter with non solid tiles __DONE__
3.  first encounter with solid tiles __DONE__
4.  gravity + first music trigger __DONE__
5.  first encounter with non textured enemy __DONE (not non-textured, rather single-sprite-textured)__
6.  jump and wall jump __DONE__
7.  player textures (single frame) __DONE__
8.  bg + player animation + second music trigger __DONE__
9.  enemy textures __DONE__
10. spikes __DONE__
11. enemy animations __DONE (happens with 'enemy textures')__
12. parallax bgs?
13. jetpack jump + hover __DONE (settings need to be tweaked for this)__
14. first wave of environmental decoration
15. run button + third music trigger __DONE__
16. second wave of environmental decoration
17. dash __DONE__
18. third wave of environmental decoration
