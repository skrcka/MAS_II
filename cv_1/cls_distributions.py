import pandas as pd
import matplotlib.pyplot as plt


def main():
    df = pd.read_csv('cls_distributions.txt', names=['distribution', 'count'], sep=' ', header=None)

    plt.figure(figsize=(10, 6))
    plt.loglog(df['distribution'], df['count'], color='skyblue', marker='o', linestyle='None')
    plt.xlabel('Distribution')
    plt.ylabel('Count')
    plt.ylim(0, df['count'].max() + 1)
    plt.title('Clustering effect distribution')
    plt.grid(axis='y', which='both', linestyle='--', linewidth=0.5)
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.savefig('cls_distributions.png')

if __name__ == '__main__':
    main()
