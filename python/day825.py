import heapq as hq
import math

input = """162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"""
limit = 10

with open("data/2025/full-8-25.txt") as f:
    input = f.read()
limit = 1000

def distance(a: tuple[float, float, float], b: tuple[float, float, float]) -> float:
    return math.sqrt((a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2 + (a[2] - b[2]) ** 2)

graph: dict[tuple[float, float, float], set[tuple[float, float, float]]] = {}
coords: list[tuple[float, float, float]] = []

for line in input.splitlines():
    [x, y, z] = [float(a) for a in line.split(",")]
    coords.append((x, y, z))
    pass

pairs: list[tuple[float, tuple[tuple[float, float, float], tuple[float, float, float]]]] = []
for i in range(0, len(coords)):
    a = coords[i]
    for j in range(i + 1, len(coords)):
        b = coords[j]
        hq.heappush(pairs, (distance(a, b), (a, b)))

def add_next(
        pairs: list[tuple[float, tuple[tuple[float, float, float], tuple[float, float, float]]]],
        circuits: list[set[tuple[float, float, float]]]
        ) -> tuple[tuple[float, float, float], tuple[float, float, float]]:
    _, (a, b) = hq.heappop(pairs)
    [aset] = [c for c in circuits if a in c]
    if b in aset:
        return (a, b)
    [bset] = [c for c in circuits if b in c]
    if len(bset) > len(aset):
        (bset, aset) = (aset, bset)
    circuits.remove(bset)
    for x in bset:
        aset.add(x)
    return (a, b)

circuits: list[set[tuple[float, float, float]]] = [{c} for c in coords]
for _ in range(0, limit):
    _ = add_next(pairs, circuits)

circuit_sizes = [len(c) for c in circuits]
circuit_sizes.sort(reverse=True)
print(circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2])

while True:
    a, b = add_next(pairs, circuits)
    if len(circuits) == 1:
        print(int(a[0] * b[0]))
        break

