#!/bin/python3

import numpy as np
from numpy import genfromtxt
import os
import matplotlib.pyplot as plt

filename = "test_results.csv"
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

# Start to build the histogram
first_edge, last_edge = y.min(), y.max()
n_equal_bins = 30
bin_edges = np.linspace(start=first_edge, stop=last_edge,num=n_equal_bins + 1, endpoint=True)
hist, bin_edges = np.histogram(y)

# plot the histogram
n, bins, patches = plt.hist(y, n_equal_bins, facecolor='#0504aa', alpha=0.7,rwidth=0.85)
#n, bins, patches = plt.hist(y, bins='auto', facecolor='blue', alpha=0.5,rwidth=0.85) # auto-define number of bins
plt.xlabel('Value')
plt.ylabel('Frequency')
plt.title('Frame Area Value Histogram')
plt.show()
