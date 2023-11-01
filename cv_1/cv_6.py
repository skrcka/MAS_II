from collections import defaultdict
import igraph as ig


def visualize_communities(flattened_network):
    nodes = list(flattened_network.keys())
    node_to_int = {node: i for i, node in enumerate(nodes)}
    edges_as_int = [(node_to_int[node1], node_to_int[node2]) for node1, links in flattened_network.items() for node2 in links]
    edges_as_int = list(set(edges_as_int))
    edges_as_int = [(node1, node2) for node1, node2 in edges_as_int if node1 < node2]

    #g = ig.Graph(edges=[(node1, node2) for node1, links in flattened_network.items() for node2 in links])
    g = ig.Graph(len(nodes), edges_as_int)
    g.vs['name'] = nodes

    louvain_clusters = g.community_multilevel()

    palette = ig.RainbowPalette(n=len(louvain_clusters))
    g.vs['color'] = [palette.get(cluster) for cluster in louvain_clusters.membership]

    layout = g.layout('kk')
    ig.plot(g, target='communities.png', layout=layout, vertex_label=g.vs["name"], vertex_size=10, edge_arrow_size=0.5, vertex_color=g.vs['color'])


def detect_communities_igraph(flattened_network):
    nodes = list(flattened_network.keys())
    node_to_int = {node: i for i, node in enumerate(nodes)}
    edges_as_int = [(node_to_int[node1], node_to_int[node2]) for node1, links in flattened_network.items() for node2 in links]

    g = ig.Graph(len(nodes), edges_as_int)
    g.vs['name'] = nodes

    louvain_clusters = g.community_multilevel()

    partition = {nodes[i]: louvain_clusters.membership[i] for i in range(len(nodes))}

    return partition


def visualize_multilayer_network(multilayer_network):
    for layer_id, layer_data in multilayer_network.items():
        nodes = list(layer_data.keys())
        node_to_int = {node: i for i, node in enumerate(nodes)}
        edges_as_int = [(node_to_int[node1], node_to_int[node2]) for node1, links in layer_data.items() for node2 in links]
        edges_as_int = list(set(edges_as_int))
        edges_as_int = [(node1, node2) for node1, node2 in edges_as_int if node1 < node2]

        g = ig.Graph(len(nodes), edges_as_int)
        g.vs['name'] = nodes
        ig.plot(g, target=f'layer_{layer_id}.png', vertex_label=g.vs["name"], layout=g.layout('kk'))


def flatten_multilayer_network(multilayer_network):
    flattened_network = defaultdict(set)
    for layer_data in multilayer_network.values():
        for node1, links in layer_data.items():
            for node2 in links:
                flattened_network[node1].add(node2)
    return flattened_network


def load_data(filename: str) -> list[tuple[str, str, str]]:
    with open(filename, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    edges = [tuple(line.split(',')) for line in lines]
    return edges


def construct_multilayer_network(edges: list[tuple[str, str, str]]) -> dict[str, dict[str, set[str]]]:
    multilayer_network = defaultdict(lambda: defaultdict(set))
    layers = set()
    for node1, node2, layer_id in edges:
        layer_id = layer_id.strip()
        node1 = int(node1)
        node2 = int(node2)
        multilayer_network[layer_id][node1].add(node2)
        multilayer_network[layer_id][node2].add(node1)
    return multilayer_network


def main():
    edges = load_data('aucs.edges')
    multilayer_network = construct_multilayer_network(edges)
    visualize_multilayer_network(multilayer_network)

    flattened_network = flatten_multilayer_network(multilayer_network)
    communities = detect_communities_igraph(flattened_network)
    print(communities)
    visualize_communities(flattened_network)


if __name__ == '__main__':
    main()
