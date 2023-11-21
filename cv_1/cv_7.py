import random
import networkx as nx
import matplotlib.pyplot as plt


def simulate_influence_spread(
    graph: nx.Graph, *, initial_nodes: list[str] | str | None = None, steps: int = 500, probability: int = 0.5
) -> list[str]:
    if initial_nodes is None:
        initial_nodes = random.choice(list(graph.nodes()))
    if not isinstance(initial_nodes, list):
        initial_nodes = [initial_nodes]

    inicial_nodes = set(initial_nodes)
    influencing_nodes = set(initial_nodes)
    influenced_nodes = set()

    for _ in range(steps):
        new_influences = set()
        for node in influencing_nodes:
            for neighbor in graph.neighbors(node):
                if neighbor in influenced_nodes:
                    continue
                if random.random() < probability:
                    new_influences.add(neighbor)
        influenced_nodes.update(influencing_nodes)
        influencing_nodes = new_influences

        yield inicial_nodes, influencing_nodes, influenced_nodes


def main() -> None:
    graphs = []
    graphs.append(nx.les_miserables_graph())
    graphs.append(nx.read_adjlist('socfb-Penn94.txt'))
    print('Graphs loaded')

    for i, G in enumerate(graphs):
        for step, (inicial, influencing, influenced) in enumerate(simulate_influence_spread(G, steps=10)):
            plt.figure()
            node_colors = []
            for node in G.nodes():
                if node in influencing:
                    node_colors.append('red')
                elif node in inicial:
                    node_colors.append('orange')
                elif node in influenced:
                    node_colors.append('green')
                else:
                    node_colors.append('blue')
            nx.draw(
                G,
                pos=nx.spring_layout(G, seed=1),
                with_labels=True,
                font_size=5,
                node_color=node_colors,
            )
            plt.savefig(f"visualization_{i}/step_{step}.png")  # Saves the figure to a file
            plt.close()
            #plt.title(f"Step {step}")
            #plt.show()


if __name__ == '__main__':
    main()
