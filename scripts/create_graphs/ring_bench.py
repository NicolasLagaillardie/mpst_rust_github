from matplotlib.ticker import MaxNLocator
import matplotlib.pyplot as plt
import json
import os
import matplotlib
import numpy as np
matplotlib.rcParams['text.usetex'] = True

# Path for criterion
main_path_criterion = './target/criterion'

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
    'nine': 9,
    'ten': 10,
    'eleven': 11,
    'twenty': 20,
    'empty': 0
}

# Lists for plots
crossbeam = []
binary = []
mpst = []
ampst = []
atmp = []
cancel = []
broadcast_cancel = []

nb_participants_crossbeam = []
nb_participants_binary = []
nb_participants_mpst = []
nb_participants_ampst = []
nb_participants_atmp = []
nb_participants_cancel = []
nb_participants_broadcast_cancel = []

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
            if 'ATMP' in d and str_to_int[splitted[1]] >= 2:
                atmp.append(int(test(d))/10**6)
                nb_participants_atmp.append(str_to_int[splitted[1]])
            elif 'AMPST' in d and str_to_int[splitted[1]] >= 2:
                ampst.append(int(test(d))/10**6)
                nb_participants_ampst.append(str_to_int[splitted[1]])
            elif 'MPST' in d and str_to_int[splitted[1]] >= 2:
                if 'broadcast' in d:
                    broadcast_cancel.append(int(test(d))/10**6)
                    nb_participants_broadcast_cancel.append(
                        str_to_int[splitted[1]])
                elif 'cancel' in d:
                    cancel.append(int(test(d))/10**6)
                    nb_participants_cancel.append(str_to_int[splitted[1]])
                elif 'baking' in d:
                    mpst.append(int(test(d))/10**6)
                    nb_participants_mpst.append(str_to_int[splitted[1]])
            elif 'binary' in d and str_to_int[splitted[1]] >= 2 and 'cancel' not in d and 'baking' in d:
                binary.append(int(test(d))/10**6)
                nb_participants_binary.append(str_to_int[splitted[1]])
            elif 'crossbeam' in d and str_to_int[splitted[1]] >= 2 and 'cancel' not in d and 'baking' in d:
                crossbeam.append(int(test(d))/10**6)
                nb_participants_crossbeam.append(str_to_int[splitted[1]])
        except:
            print("Missing ", d)

# Sort the lists in pair
if nb_participants_crossbeam and crossbeam:
    nb_participants_crossbeam, crossbeam = (list(t) for t in zip(*sorted(zip(nb_participants_crossbeam, crossbeam))))

if nb_participants_binary and binary:
    nb_participants_binary, binary = (list(t) for t in zip(*sorted(zip(nb_participants_binary, binary))))

if nb_participants_mpst and mpst:
    nb_participants_mpst, mpst = (list(t) for t in zip(*sorted(zip(nb_participants_mpst, mpst))))

if nb_participants_ampst and ampst:
    nb_participants_ampst, ampst = (list(t) for t in zip(*sorted(zip(nb_participants_ampst, ampst))))

if nb_participants_atmp and atmp:
    nb_participants_atmp, atmp = (list(t) for t in zip(*sorted(zip(nb_participants_atmp, atmp))))

if nb_participants_cancel and cancel:
    nb_participants_cancel, cancel = (list(t) for t in zip(*sorted(zip(nb_participants_cancel, cancel))))

if nb_participants_broadcast_cancel and broadcast_cancel:
    nb_participants_broadcast_cancel, broadcast_cancel = (list(t) for t in zip(*sorted(zip(nb_participants_broadcast_cancel, broadcast_cancel))))

# Change size
# ax = plt.figure(figsize=(50, 50)).gca()
fig, ax = plt.subplots(figsize=(60, 60))
plt.gcf().subplots_adjust(bottom=0.27, left=0.13)

ax.xaxis.set_major_locator(MaxNLocator(integer=True))
ax.yaxis.set_major_locator(MaxNLocator(integer=True))

# # Plot the Crossbeam graph
# ax.plot(nb_participants_crossbeam, crossbeam, label='Crossbeam', linestyle='solid', linewidth=20, marker='P', markersize=70, color='#1f77b4')

# # Plot the binary graph
# ax.plot(nb_participants_binary, binary, label='Binary', linestyle='solid', linewidth=20, marker='o', markersize=70, color='#ff7f0e')

# # Plot the MPST graph
# ax.plot(nb_participants_mpst, mpst, label='MPST', linestyle='solid', linewidth=20, marker='^', markersize=70, color='#2ca02c')

# Plot the AMPST graph
ax.plot(nb_participants_ampst, ampst, label='AMPST', linestyle='solid', linewidth=20, marker='*', markersize=70, color='#d62728')

# Plot the ATMP graph
ax.plot(nb_participants_atmp, atmp, label='ATMP', linestyle='solid', linewidth=20, marker='v', markersize=70, color='#9467bd')

# if len(cancel) > 0:
#     # Plot the cancel graph
#     ax.plot(nb_participants_cancel, cancel, label='Cancel',
#             linestyle='solid', linewidth=20, marker='*', markersize=150)

# if len(broadcast_cancel) > 0:
#     # Plot the broadcast cancel graph
#     ax.plot(nb_loops_broadcast_cancel, broadcast_cancel,
#             label='Broadcast cancel', linestyle='dotted', linewidth=5)

# Label X and Y axis
ax.set_xlabel('\# roles', fontsize=300)
# ax.set_ylabel('Time (ms)', fontsize=300)
ax.tick_params(axis='both', which='major', labelsize=300)
ax.xaxis.set_ticks(np.arange(2, 11, 3))
ax.yaxis.set_ticks(np.arange(0, 40, 3))
ax.set_xlim(2, 8)
ax.set_ylim(0, 9)

# maxi1 = max(mpst)
# maxi2 = max(binary)
# maxi3 = max(crossbeam)
# maxi = max(maxi1, maxi2, maxi3)

# mini1 = min(mpst)
# mini2 = min(binary)
# mini3 = min(crossbeam)
# mini = min(mini1, mini2, mini3)

offset_x = matplotlib.transforms.ScaledTranslation(0, -2, fig.dpi_scale_trans)

# apply offset transform to all x ticklabels.
for label in ax.xaxis.get_majorticklabels():
    label.set_transform(label.get_transform() + offset_x)

offset_y = matplotlib.transforms.ScaledTranslation(-1, 0, fig.dpi_scale_trans)

for label in ax.yaxis.get_majorticklabels():
    label.set_transform(label.get_transform() + offset_y)

# # Major ticks every 20, minor ticks every 5
# major_ticks = np.arange(mini, maxi+0.1, 0.2)
# minor_ticks = np.arange(mini, maxi+0.1, 0.05)

# # ax.set_xticks(major_ticks)
# # ax.set_xticks(minor_ticks, minor=True)
# ax.set_yticks(major_ticks)
# ax.set_yticks(minor_ticks, minor=True)

# # Add grid
# ax.grid(which='both')

# ax.grid(which='minor', alpha=0.2)
# ax.grid(which='major', alpha=0.5)

# # giving a title to my graph
# plt.title('MPST vs binary along number of participants for ' +
#           number_of_loops + ' loops')

# show a legend on the plot
# ax.legend(bbox_to_anchor=(0.5, 1), loc="lower center", prop={'size': 20})

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
