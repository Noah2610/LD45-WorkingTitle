# LD45 Design
## Features (conceptually)
- Left/right movement
- Simple jump
- Improved jump (short press kills velocity)
- Hover (hold jump in-air)
- Walls without collision
- Walls with collision

## Features (ingame)
In _Tiled_, objects of type `Feature` need to have the property `feature_type`.  
`feature_type`'s value has to be one of the following:

- `AddCollisions`  
  Makes the player collide with solid walls.

## Sequential Features
- 4 directional movement, non gravital
- first encounter with non solid tiles
- first encounter with solid tiles
- gravity + first music trigger
- first encounter with non textured enemy
- jump
- player textures
- bg + player animation + second music trigger
- enemy textures 
- spikes
- enemy animations
- parallax bgs? 
- jetpack jump + float
- first wave of environmental decoration 
- sprint + third music trigger
- second wave of environmental decoration
- dash
- third wave of environmental decoration
