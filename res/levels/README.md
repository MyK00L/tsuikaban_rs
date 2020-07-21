# Tsuikaban-levels
M's Tsuikaban's levels

## Attribution

The levels hereby provided are designed by:
- [MyK00L](https://github.com/myk00l) (0-16)

## Level file specification
```
4 12
############
#..........#
######...#D#
############
1 1
2
3 1 255 0 0 1
5 1 0 0 255 1
```
- The first line contains two integers: the `height` and the `width` of the board
- The next `height` lines contain the map:
  - `#` is used to represent a wall tile
  - `.` is used to represent a floor tile
  - `D` is used to represent a door tile
- The next line contains the initial position (`x`, `y`) of the player
- The following line contains the number of coloured `Cubes` in the board
- Each following line contains:
  - `x y` coordinates of the cube
  - `r g b` components of the cube
  - `n`, the value of the cube
