from collections import defaultdict
import statistics


def load_data(filename: str) -> list[tuple[int, int, int, float]]:
    with open(filename, 'r') as f:
        lines = f.readlines()
    edges = [(int(line.split()[0]), int(line.split()[1]), int(line.split()[2]), float(line.split()[3])) for line in lines]
    return edges


def construct_multilayer_network(edges: list[tuple[int, int, int, float]]) -> dict[int, dict[int, set[int]]]:
    multilayer_network = defaultdict(lambda: defaultdict(set))
    for layerID, node1, node2, weight in edges:
        multilayer_network[layerID][node1].add(node2)
        multilayer_network[layerID][node2].add(node1)  # assuming the network is undirected
    return multilayer_network


def degree_centrality(a: int, L: list[int], multilayer_network: dict[int, dict[int, set[int]]]) -> int:
    count = 0
    for l in L:
        for node in multilayer_network[l]:
            if a in multilayer_network[l][node]:
                count += 1
    return count


def degree_deviation(a: int, multilayer_network: dict[int, dict[int, set[int]]]) -> float:
    degrees = [len(multilayer_network[layer][a]) for layer in multilayer_network if a in multilayer_network[layer]]
    return statistics.stdev(degrees) if len(degrees) > 1 else 0


def neighbors(a: int, L: list[int], multilayer_network: dict[int, dict[int, set[int]]]) -> set[int]:
    neigh_set = set()
    for l in L:
        if a in multilayer_network[l]:
            neigh_set.update(multilayer_network[l][a])
    return neigh_set


def neighborhood_centrality(a: int, L: list[int], multilayer_network: dict[int, dict[int, set[int]]]) -> int:
    return len(neighbors(a, L, multilayer_network))


def connective_redundancy(a: int, L: list[int], multilayer_network: dict[int, dict[int, set[int]]]) -> float:
    return 1 - (neighborhood_centrality(a, L, multilayer_network) / degree_centrality(a, L, multilayer_network))


def exclusive_neighborhood(a: int, L: list[int], multilayer_network: dict[int, dict[int, set[int]]]) -> set[int]:
    total_layers = set(multilayer_network.keys())
    other_layers = total_layers - set(L)
    return neighbors(a, L, multilayer_network) - neighbors(a, other_layers, multilayer_network)


def main():
    edges = load_data('lazega.edges')
    multilayer_network = construct_multilayer_network(edges)
    a = 1
    L = [1, 2]
    print("Degree Centrality:", degree_centrality(a, L, multilayer_network))
    print("Degree Deviation:", degree_deviation(a, multilayer_network))
    print("Neighborhood Centrality:", neighborhood_centrality(a, L, multilayer_network))
    print("Connective Redundancy:", connective_redundancy(a, L, multilayer_network))
    print("Exclusive Neighborhood:", exclusive_neighborhood(a, L, multilayer_network))


if __name__ == '__main__':
    main()
