# Procedural map generation

This is a simple demo to visualize different map generation algorithms.

> :warning: This is a work in progress. I'm still working on the algorithms and the UI. The code is full of bugs and the algorithms are not optimized.

## Algorithms

- [x] Random walls
    - Make the boundaries of the map walls and fill the rest with random walls.
- [x] Random rooms and corridors
    - Fill the map with walls and place rooms randomly. Then connect the rooms with corridors.
- [ ] Random rooms and corridors with BSP
    - Fill the map with walls and place rooms randomly. Then connect the rooms with corridors. The rooms are placed using a binary space partitioning algorithm.
- [ ] Random rooms and corridors with BSP interior
    - Fill the map with walls and place rooms randomly. The rooms are placed using a binary space partitioning algorithm. The entire map is filled with rooms.
- [ ] Cellular automata
    - Fill the map with random walls. Then apply the cellular automata algorithm to smooth the map.
- [ ] Drunkard's walk
    - Fill the map with walls, pick a random point and start walking. The drunkard will leave a trail of walls behind him.
- [ ] Voronoi Hive
    - Subdivide the map into regions and place walls between them.
