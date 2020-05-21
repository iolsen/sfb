# TODO

- hex
  - make it 0-based
  - screen<->hex coords
  - distance: number of hexes in between + 1. same hex is 0.
- graphics
  - drawing ships
    - proper placement in hex
    - rotate
    - move
    - multiple ship types, non-ships
  - make the default window size choice smarter
  - hex grid
    - make it a tile map
    - hex numbers
    - scale better on resize
    - zoom/pan
  - imgui-rs PoC
- other
  - organize and clean up main.rs
  - separate stuff into crates such that you could have a server between players

# Resources

## Hex geometry

- https://www.redblobgames.com/grids/hexagons/
- http://www-cs-students.stanford.edu/~amitp/Articles/Hexagon1.html
- https://github.com/jbochi/duelo
- https://s3.amazonaws.com/jbochi/layout.html

## Graphics

- https://github.com/hecrj/iced
- http://shipschematics.net

## Other

`rusty-tags vi`
