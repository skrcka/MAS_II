import random
import networkx as nx
import matplotlib.pyplot as plt


def simulate_influence_spread(
    graph, *, initial_nodes: list[str] | str | None = None, steps: int = 500, probability: int = 0.5
):
    if initial_nodes is None:
        initial_nodes = random.choice(list(graph.nodes()))
    if not isinstance(initial_nodes, list):
        initial_nodes = [initial_nodes]

    influenced_nodes = set(initial_nodes)

    for _ in range(steps):
        new_influences = set()
        for node in influenced_nodes:
            for neighbor in graph.neighbors(node):
                if random.random() < probability:
                    new_influences.add(neighbor)

        influenced_nodes.update(new_influences)
        yield influenced_nodes


def main():
    G = nx.les_miserables_graph()

    for step, influenced in enumerate(simulate_influence_spread(G, steps=10)):
        plt.figure()
        nx.draw(
            G,
            with_labels=True,
            font_size=5,
            node_color=['red' if node in influenced else 'green' for node in G.nodes()],
        )
        #plt.savefig(f"step_{step}.png")  # Saves the figure to a file
        #plt.close()
        plt.title(f"Step {step}")
        plt.show()


if __name__ == '__main__':
    main()
