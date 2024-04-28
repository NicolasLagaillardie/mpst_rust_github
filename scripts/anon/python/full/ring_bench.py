from matplotlib.ticker import MaxNLocator
import matplotlib.pyplot as plt
import json
import os
import matplotlib
import numpy as np
matplotlib.rcParams['text.usetex'] = True

# Path for criterion
main_path_criterion = './save/ring'

# Get all directories_criterion in main_path_criterion
directories_criterion = os.listdir(main_path_criterion)

# Relative path of the expected file
path_file = '/base/estimates.json'

# Dictionary for converting from string to int
str_to_int = {
    'two': 2,
    'three': 3,
    'four': 4,
    'five': 5,
    'six': 6,
    'seven': 7,
    'eight': 8,
}

# Lists for plots
ampst = []
atmp = []

nb_participants_ampst = []
nb_participants_atmp = []

# Number of loops in the recursion
number_of_loops = '100'

def test(path):
    # Get the wanted data in the JSON file (field -> mean, field -> point_estimate)
    with open(main_path_criterion + '/' + path + path_file) as json_file:
        data = json.load(json_file)
        return data['mean']['point_estimate']

# For each folder in main_path_criterion
for d in directories_criterion:
    # If name looks like the one from what we want
    if 'ring' in d and ' ' + number_of_loops in d and 'inline' not in d:
        # Split the name
        splitted = d.split(' ')

        try:
            # If MPST of binary, append to related lists
            if 'protocol' not in d:
                if 'ATMP' in d and str_to_int[splitted[1]] >= 2:
                    atmp.append(int(test(d))/10**6)
                    nb_participants_atmp.append(str_to_int[splitted[1]])
                elif 'AMPST' in d and str_to_int[splitted[1]] >= 2:
                    ampst.append(int(test(d))/10**6)
                    nb_participants_ampst.append(str_to_int[splitted[1]])
        except:
            print("Missing ", d)

# Sort the lists in pair
if nb_participants_ampst and ampst:
    nb_participants_ampst, ampst = (list(t) for t in zip(*sorted(zip(nb_participants_ampst, ampst))))

if nb_participants_atmp and atmp:
    nb_participants_atmp, atmp = (list(t) for t in zip(*sorted(zip(nb_participants_atmp, atmp))))

# Change size
# ax = plt.figure(figsize=(50, 50)).gca()
fig, ax = plt.subplots(figsize=(60, 60))
plt.gcf().subplots_adjust(bottom=0.27, left=0.13)

ax.xaxis.set_major_locator(MaxNLocator(integer=True))
ax.yaxis.set_major_locator(MaxNLocator(integer=True))

# Plot the AMPST graph
ax.plot(nb_participants_ampst, ampst, label='MultiCrusty', linestyle='solid', linewidth=20, marker='*', markersize=70, color='#d62728')

# Plot the ATMP graph
ax.plot(nb_participants_atmp, atmp, label='Anon', linestyle='solid', linewidth=20, marker='v', markersize=70, color='#9467bd')

min_ampst_atmp = int(min(min(ampst), min(atmp)))
max_ampst_atmp = int(max(max(ampst), max(atmp))) + 1.1

# Label X and Y axis
ax.set_xlabel('\# roles', fontsize=300)
# ax.set_ylabel('Time (ms)', fontsize=300)
ax.tick_params(axis='both', which='major', labelsize=300)
ax.xaxis.set_ticks(np.arange(2, 11, 3))
ax.yaxis.set_ticks(np.arange(min_ampst_atmp, max_ampst_atmp, 1))
ax.set_xlim(2, 8)
ax.set_ylim(min_ampst_atmp, max_ampst_atmp)

offset_x = matplotlib.transforms.ScaledTranslation(0, -2, fig.dpi_scale_trans)

# apply offset transform to all x ticklabels.
for label in ax.xaxis.get_majorticklabels():
    label.set_transform(label.get_transform() + offset_x)

offset_y = matplotlib.transforms.ScaledTranslation(-1, 0, fig.dpi_scale_trans)

for label in ax.yaxis.get_majorticklabels():
    label.set_transform(label.get_transform() + offset_y)

# Tight layout
plt.tight_layout()

# create the name for the new figure
index_graphs = 0
while os.path.isfile('graphs_bench/ring/runtime_' + number_of_loops + '_' + str(index_graphs) + '.pdf'):
    index_graphs += 1

name_graph = './graphs_bench/ring/runtime_' + number_of_loops + '_' + str(index_graphs) + '.pdf'

# Save fig
plt.savefig(name_graph)

# # function to show the plot
# plt.show()
