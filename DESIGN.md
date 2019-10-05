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
1.  4 directional movement, no gravity __DONE__
2.  first encounter with non solid tiles __DONE__
3.  first encounter with solid tiles __DONE__
4.  gravity + first music trigger __DONE (gravity)__
5.  first encounter with non textured enemy
6.  jump
7.  player textures (single frame)
8.  bg + player animation + second music trigger
9.  enemy textures
10. spikes
11. enemy animations
12. parallax bgs?
13. jetpack jump + hover
14. first wave of environmental decoration
15. sprint + third music trigger
16. second wave of environmental decoration
17. dash
18. third wave of environmental decoration
