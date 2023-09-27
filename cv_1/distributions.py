import pandas as pd
import matplotlib.pyplot as plt


def main():
    df = pd.read_csv('distributions.txt', names=['distribution', 'count'], sep=' ', header=None)

    plt.figure(figsize=(10, 6))
    plt.loglog(df['distribution'], df['count'], color='skyblue', marker='o', markerfacecolor='black', markersize=3)
    plt.xlabel('Distribution')
    plt.ylabel('Count')
    plt.ylim(0, df['count'].max() + 1)
    plt.title('Degree distribution')
    plt.grid(axis='y', which='both', linestyle='--', linewidth=0.5)
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.savefig('distributions.png')

if __name__ == '__main__':
    main()
