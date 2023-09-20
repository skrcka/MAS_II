import pandas as pd
import matplotlib.pyplot as plt


def main():
    # create a dataframe from the distributions
    df = pd.read_csv('distributions.txt', names=['distribution', 'count'], sep=' ', header=None)
    print(df.head())
    print(max(df['distribution']) + 1)
    print(max(df['count']) + 1)

    # plot the distributions
    plt.figure(figsize=(10, 6))
    plt.bar(df['distribution'], df['count'], color='skyblue', edgecolor='black')
    plt.xlabel('Distribution')
    plt.ylabel('Count')
    plt.xlim(0, 100)
    plt.ylim(0, df['count'].max() + 1)
    plt.title('Distribution Counts')
    plt.grid(axis='y', which='both', linestyle='--', linewidth=0.5)
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.show()

if __name__ == '__main__':
    main()
