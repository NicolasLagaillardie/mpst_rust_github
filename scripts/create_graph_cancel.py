from matplotlib.ticker import MaxNLocator
import matplotlib.pyplot as plt
import json
import os
import matplotlib
import numpy as np
matplotlib.rcParams['text.usetex'] = True

# Path for criterion
main_path = './target/criterion'

# Get all directories in main_path
directories = os.listdir(main_path)

# Relative path of the expected file
path_file = '/base/estimates.json'

# Lists for plots
cancel = []
protocol = []

def test(path):
    # Get the wanted data in the JSON file (field -> mean, field -> point_estimate)
    with open(main_path + '/' + path + path_file) as json_file:
        data = json.load(json_file)
        return data['mean']['point_estimate']

# For each folder in main_path
for d in directories:
    # If name looks like the one from what we want
    if 'Cancel' in d:
        # Split the name
        splitted = d.split(' ')[0]
        splitted = splitted.split('_')

        cancel.append(int(test(d)/10**6))
        protocol.append(int(splitted[1]))

# Sort the lists in pair
protocol, cancel = (list(t) for t in zip(*sorted(zip(protocol, cancel))))

# Change size
ax = plt.figure(figsize=(30, 15)).gca()

ax.xaxis.set_major_locator(MaxNLocator(integer=True))
ax.yaxis.set_major_locator(MaxNLocator(integer=True))

# Plot the MPST graph
ax.plot(protocol, cancel, label='Cancel', linestyle='solid', linewidth=3)

# Label X and Y axis
ax.set_xlabel('Protocol used', fontsize=30)
ax.set_ylabel('Time (ms)', fontsize=30)
ax.tick_params(axis='both', which='major', labelsize=30)
# ax.tick_params(axis='both', which='minor', labelsize=30)

maxi = max(cancel)
mini = min(cancel)

# Major ticks every 20, minor ticks every 5
major_ticks = np.arange(mini, maxi+1, 5)
minor_ticks = np.arange(mini, maxi+0.5, 1)

# ax.set_xticks(major_ticks)
# ax.set_xticks(minor_ticks, minor=True)
ax.set_yticks(major_ticks)
ax.set_yticks(minor_ticks, minor=True)

# Add grid
ax.grid(which='both')

# show a legend on the plot
ax.legend(bbox_to_anchor=(1, 1), loc="upper left", prop={'size': 20})

# Save fig
plt.savefig('./graphs_bench/graph_cancel.pdf')

# # function to show the plot
# plt.show()
