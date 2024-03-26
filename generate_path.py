# Made this during the holidays while bored, hence the sequence break
import random
from typing import Dict, List, Tuple
from pprint import pprint


def generate_map():
    connections: Dict[Tuple[int, int], List[Tuple[int, int]]] = {}
    for i in range(4):
        current_tile = (random.randint(0, 5), 0)
        for row in range(11):
            new_tile = (
                random.randint(
                    max(current_tile[0] - 1, 0), min(current_tile[0] + 1, 5)
                ),
                current_tile[1] + 1,
            )
            add_connection(connections, current_tile, new_tile)
            current_tile = new_tile
    return connections


def add_connection(
    connections: Dict[Tuple[int, int], List[Tuple[int, int]]],
    from_tile: Tuple[int, int],
    to_tile: Tuple[int, int],
) -> None:
    if from_tile in connections:
        connections[from_tile].append(to_tile)
    else:
        connections[from_tile] = [to_tile]
