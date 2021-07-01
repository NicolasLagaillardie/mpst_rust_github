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
binary = []
mpst = []
crossbeam = []
cancel = []
broadcast_cancel = []

nb_loops_binary = []
nb_loops_mpst = []
nb_loops_crossbeam = []
nb_loops_cancel = []
nb_loops_broadcast_cancel = []


def test(path):
    # Get the wanted data in the JSON file (field -> mean, field -> point_estimate)
    with open(main_path + '/' + path + path_file) as json_file:
        data = json.load(json_file)
        return data['mean']['point_estimate']


# For each folder in main_path
for d in directories:
    # If name looks like the one from what we want
    if 'ping pong' in d:

        # Split the name
        splitted = d.split(' ')

        try:
            if 'binary' in d:
                binary.append(int(test(d))/10**6)
                nb_loops_binary.append(int(splitted[-1]))
            elif 'MPST' in d:
                if 'broadcast' in d:
                    broadcast_cancel.append(int(test(d))/10**6)
                    nb_loops_broadcast_cancel.append(int(splitted[-1]))
                elif 'cancel' in d:
                    cancel.append(int(test(d))/10**6)
                    nb_loops_cancel.append(int(splitted[-1]))
                else:
                    mpst.append(int(test(d))/10**6)
                    nb_loops_mpst.append(int(splitted[-1]))
            elif 'crossbeam' in d:
                crossbeam.append(int(test(d))/10**6)
                nb_loops_crossbeam.append(int(splitted[-1]))
        except:
            print("Missing ", d)

# Sort the lists in pair
nb_loops_binary, binary = (list(t)
                           for t in zip(*sorted(zip(nb_loops_binary, binary))))

nb_loops_mpst, mpst = (list(t) for t in zip(*sorted(zip(nb_loops_mpst, mpst))))

nb_loops_crossbeam, crossbeam = (list(t) for t in zip(
    *sorted(zip(nb_loops_crossbeam, crossbeam))))

nb_loops_cancel, cancel = (list(t)
                           for t in zip(*sorted(zip(nb_loops_cancel, cancel))))

nb_loops_broadcast_cancel, broadcast_cancel = (list(t)
                                               for t in zip(*sorted(zip(nb_loops_broadcast_cancel, broadcast_cancel))))

# Change size
ax = plt.figure(figsize=(30, 15)).gca()

ax.xaxis.set_major_locator(MaxNLocator(integer=True))
ax.yaxis.set_major_locator(MaxNLocator(integer=True))

# Plot the MPST graph
ax.plot(nb_loops_mpst, mpst, label='MPST',
        linestyle='solid', linewidth=5)

# Plot the binary graph
ax.plot(nb_loops_binary, binary, label='Binary',
        linestyle='dashed', linewidth=5)

# Plot the crossbeam graph
ax.plot(nb_loops_crossbeam, crossbeam, label='Crossbeam',
        linestyle='dashdot', linewidth=5)

if len(cancel) > 0:
    # Plot the cancel graph
    ax.plot(nb_loops_cancel, cancel, label='Cancel',
            linestyle='dotted', linewidth=5)

# if len(broadcast_cancel) > 0:
#     # Plot the broadcast cancel graph
#     ax.plot(nb_loops_broadcast_cancel, broadcast_cancel,
#             label='Broadcast cancel', linestyle='dotted', linewidth=5)

# Label X and Y axis
ax.set_xlabel('Number of loops', fontsize=30)
ax.set_ylabel('Time (ms)', fontsize=30)
ax.tick_params(axis='both', which='major', labelsize=30)
# ax.tick_params(axis='both', which='minor', labelsize=30)

# maxi1 = max(ping_pong)
# maxi2 = max(binary)
# maxi3 = max(crossbeam)
# maxi = max(maxi1, maxi2, maxi3)

# mini1 = min(ping_pong)
# mini2 = min(binary)
# mini3 = min(crossbeam)
# mini = min(mini1, mini2, mini3)

# # Major ticks every 20, minor ticks every 5
# major_ticks = np.arange(mini, maxi+0.1, 0.2)
# minor_ticks = np.arange(mini, maxi+0.1, 0.05)

# # ax.set_xticks(major_ticks)
# # ax.set_xticks(minor_ticks, minor=True)
# ax.set_yticks(major_ticks)
# ax.set_yticks(minor_ticks, minor=True)

# # Add grid
ax.grid(which='both')


# ax.grid(which='minor', alpha=0.2)
# ax.grid(which='major', alpha=0.5)

# # giving a title to my graph
# plt.title('MPST vs binary along number of participants for ' +
#           number_of_loops + ' loops')

# show a legend on the plot
# ax.legend(bbox_to_anchor=(0.5, 1), loc="lower center", prop={'size': 20})

# Save fig
plt.savefig('./graphs_bench/graph_ping_pong.pdf')

# # function to show the plot
# plt.show()
