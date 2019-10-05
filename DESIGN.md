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
