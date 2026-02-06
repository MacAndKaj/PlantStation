import matplotlib.pyplot as plt
import csv
import argparse

if __name__ == "__main__":
    # argparse.
    parser = argparse.ArgumentParser(prog="PS Plotter")
    parser.add_argument('filepath')
    args = parser.parse_args()
    humidity=[]
    with open(args.filepath) as csvfile:
        filereader = csv.reader(csvfile, delimiter=',')
        for row in filereader:
            humidity.append(int(row[1]))

    plt.plot(humidity, '--')
    plt.ylabel('Humidity')
    plt.show()
