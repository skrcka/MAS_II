import random
import networkx as nx
import matplotlib.pyplot as plt

def simulate_influence_spread(
    graph: nx.Graph, *, initial_nodes=None, steps: int = 500, probability: float = 0.5
):
    if initial_nodes is None:
        initial_nodes = random.choice(list(graph.nodes()))
    if not isinstance(initial_nodes, list):
        initial_nodes = [initial_nodes]

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

        yield initial_nodes, influencing_nodes, influenced_nodes

def run_simulation(graph: nx.Graph, initial_nodes, probability: float, steps: int) -> float:
    influenced_nodes = set()
    for _, _, influenced in simulate_influence_spread(
        graph, initial_nodes=initial_nodes, probability=probability, steps=steps
    ):
        influenced_nodes.update(influenced)

    # Return the proportion of influenced nodes
    return len(influenced_nodes) / len(graph.nodes())

def select_initial_nodes(graph: nx.Graph, num_nodes: int = 5) -> list:
    degree_centrality = nx.degree_centrality(graph)
    sorted_nodes = sorted(degree_centrality, key=degree_centrality.get, reverse=True)

    selected_nodes = []
    for node in sorted_nodes:
        if len(selected_nodes) >= num_nodes:
            break
        if not any(node in graph.neighbors(selected_node) for selected_node in selected_nodes):
            selected_nodes.append(node)

    return selected_nodes

def run_multiple_simulations(graph: nx.Graph, num_simulations: int = 10, steps: int = 500, probability: float = 0.01):
    simulation_results = []

    for _ in range(num_simulations):
        initial_nodes = select_initial_nodes(graph, num_nodes=5)
        influenced_rate = run_simulation(graph, initial_nodes, probability, steps)
        simulation_results.append(influenced_rate)

    return simulation_results

def main():
    random.seed(1)
    graph = nx.read_adjlist('socfb-Penn94.txt')

    simulation_results = run_multiple_simulations(graph, num_simulations=10)

    plt.boxplot(simulation_results)
    plt.title('Influence Spread Over Network (10 Simulations)')
    plt.ylabel('Proportion of Influenced Nodes')
    plt.show()

if __name__ == '__main__':
    main()
