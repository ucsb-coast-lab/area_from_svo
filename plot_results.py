#!/usr/bin/python3

import numpy as np
from numpy import genfromtxt
import os
import matplotlib.pyplot as plt
import matplotlib.mlab as mlab
from scipy.stats import norm

# We're going to open the results csv file, and write all the valid results to another
# data file
def import_data_from_csv(filename):
    # filename = "results.csv"
    fltrd_fname = "filtered_"+filename;
    print(fltrd_fname)
    # filter the .csv file in case of error messages, only accepts if there are two columns
    with open(filename, "r") as f:
        lines = f.readlines()
    #os.remove(filename)
    with open(fltrd_fname, "w") as f:
        for line in lines:
            items = line.split(',')
            if len(items) == 2:
                f.write(line)

    my_data = genfromtxt(fltrd_fname, delimiter=',')
    x = my_data[:,0]
    y = my_data[:,1]
    return x,y

def build_plots(x,y):
    # Start to build the histogram
    #first_edge, last_edge = y.min(), y.max()
    #n_equal_bins = 300
    #bin_edges = np.linspace(start=first_edge, stop=25000,num=n_equal_bins + 1, endpoint=True)
    #hist, bin_edges = np.histogram(y)

    # plot the histogram
    #n, bins, patches = plt.hist(y, n_equal_bins, facecolor='#0504aa', alpha=0.7,rwidth=0.85)
    #n, bins, patches = plt.hist(y[y>250], bins='auto', facecolor='blue', alpha=0.5,rwidth=0.85) # auto-define number of bins
    plt.subplot(1,2,1)
    n, bins, patches = plt.hist(y[y>250], bins='auto', facecolor='blue', alpha=0.5,rwidth=0.85) # auto-define number of bins
    (mu, sigma) = norm.fit(y)
    curve = norm.pdf(bins,mu,sigma)

    plt.plot(bins, curve*bins[round(len(bins)/2)], 'b--', linewidth=2)
    plt.xlabel('Value')
    plt.ylabel('Frequency')
    #plt.xlim(0,40000)
    plt.title('Distribution of Estimated Target Area   $\mu={:.3}$, $\sigma={:.3}$'.format(mu,sigma))
    plt.ylabel('Binned Frames')
    plt.xlabel('Estimated Target Area (mm^2)')

    plt.subplot(1,2,2)
    plt.bar(x, y, width=0.8, bottom=None, align='center', data=None)
    plt.scatter(x,y,marker='.',color='black',alpha=0.6,s=1)
    #plt.ylim(0,25000)
    plt.axhline(y=150000, color='b', linestyle='--')
    plt.ylabel('Estimated Target Area (mm^2)')
    plt.xlabel("Frame Number")
    plt.title('Estimated Frame Target Area')
    plt.show()

def select_subset_from_total_data(frames,x,y):
    subset_y = np.zeros([len(frames)]);
    subset_x = np.zeros([len(frames)]);

    i = 0;
    for frame in frames:
        print("Find the values in frame ",frame);
        for val in range(0,len(x)):
            if x[val] == frame:
                subset_x[i] = frame;
                subset_y[i] = y[val];
                print("x = ",frame,"y = ",y[val]);
                break;
        i = i + 1;

    return subset_x,subset_y


def main():
    x,y = import_data_from_csv('results.csv')
    build_plots(x,y);
    frames = [308,310,311,312,323,325,326,327,328,329,330,331,345,346,347,348,349,350,351,352,353,354,354,355,356,357,358,359,360,361,362,363,364,365,366,436,666,672,694,695,696,697,698,699,700,701,702,703,704,705,726,727,728,729,730,731,763,773,774,775,776,777,778,779,780,781,782,783,784,785,786,787,788,789,790,791,792,793,794,795,796,797,798,799,800,801,802,803,804,805,806,807,808,809,810,811,812,813,814,815,816,817,818,819,820,821,822,823,824,825,826,827];
    #frames = [308,310,311];
    subset_x,subset_y =  select_subset_from_total_data(frames,x,y);
    build_plots(subset_x,subset_y)


if __name__ == "__main__":
    main()
