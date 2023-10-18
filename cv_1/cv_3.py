import random
import networkx as nx
from itertools import combinations


def non_edges(graph):
    """Return a list of non-edges in the graph."""
    return [(u, v) for u, v in combinations(graph.nodes(), 2) if not graph.has_edge(u, v)]


def common_neighbors(G, x, y):
    """Return the common neighbors of nodes x and y."""
    nx_neighbors = set(G.neighbors(x))
    ny_neighbors = set(G.neighbors(y))
    return len(nx_neighbors & ny_neighbors)


def jaccard_coefficient(G, x, y):
    """Return the Jaccard coefficient of nodes x and y."""
    nx_neighbors = set(G.neighbors(x))
    ny_neighbors = set(G.neighbors(y))
    intersection = nx_neighbors & ny_neighbors
    union = nx_neighbors | ny_neighbors
    if len(union) == 0:
        return 0
    return len(intersection) / len(union)


def main():
    # K-fold cross-validation
    G = nx.karate_club_graph()
    K = 10
    edges = list(G.edges())
    random.shuffle(edges)
    folds = [edges[i::K] for i in range(K)]

    metrics = {
        'common_neighbors': {
            'precision': [],
            'recall': [],
            'sensitivity': [],
            'specificity': []
        },
        'jaccard': {
            'precision': [],
            'recall': [],
            'sensitivity': [],
            'specificity': []
        }
    }

    for fold in folds:
        G_train = G.copy()
        G_train.remove_edges_from(fold)

        for method in ['common_neighbors', 'jaccard']:
            if method == 'common_neighbors':
                preds = [(e[0], e[1], common_neighbors(G_train, e[0], e[1])) for e in non_edges(G_train)]
            else:
                preds = [(e[0], e[1], jaccard_coefficient(G_train, e[0], e[1])) for e in non_edges(G_train)]

            preds = sorted(preds, key=lambda x: x[2], reverse=True)[:len(fold)]
            pred_edges = [(x[0], x[1]) for x in preds]

            TP = len(set(pred_edges) & set(fold))
            FP = len(pred_edges) - TP
            TN = len(fold) - TP
            FN = len(pred_edges) - TP

            precision = TP / (TP + FP)
            recall = TP / (TP + FN)
            sensitivity = recall
            specificity = TN / (TN + FP)

            metrics[method]['precision'].append(precision)
            metrics[method]['recall'].append(recall)
            metrics[method]['sensitivity'].append(sensitivity)
            metrics[method]['specificity'].append(specificity)

    # Print results
    for method, values in metrics.items():
        print(f"Method: {method}")
        for metric, scores in values.items():
            avg_score = sum(scores) / K
            print(f"{metric}: {avg_score:.4f}")
        print()


if __name__ == '__main__':
    main()
