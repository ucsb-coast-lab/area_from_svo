#!/usr/bin/python3

import numpy as np
from numpy import genfromtxt
import os
import matplotlib.pyplot as plt

filename = "results.csv"
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
n_equal_bins = 300
bin_edges = np.linspace(start=first_edge, stop=25000,num=n_equal_bins + 1, endpoint=True)
hist, bin_edges = np.histogram(y)

# plot the histogram
#n, bins, patches = plt.hist(y, n_equal_bins, facecolor='#0504aa', alpha=0.7,rwidth=0.85)
#n, bins, patches = plt.hist(y[y>250], bins='auto', facecolor='blue', alpha=0.5,rwidth=0.85) # auto-define number of bins
plt.subplot(1,2,1)
n, bins, patches = plt.hist(y[y>250], bins=1000, facecolor='blue', alpha=0.5,rwidth=0.85) # auto-define number of bins
plt.xlabel('Value')
plt.ylabel('Frequency')
plt.xlim(0,40000)
plt.title('Frame Area Value Histogram')

plt.subplot(1,2,2)
plt.bar(x, y, width=0.8, bottom=None, align='center', data=None)
plt.plot([0,max(x)],[12500,12500],'r')
plt.scatter(x,y,marker='.',color='black',alpha=0.6,s=1)
plt.ylim(0,25000)
plt.xlabel('Frame')
plt.ylabel('Area [mm^2]')
plt.title('Frame vs Area')

plt.show()
