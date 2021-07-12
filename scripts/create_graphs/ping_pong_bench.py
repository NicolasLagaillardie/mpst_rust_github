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
            if 'binary' in d and 'cancel' not in d:
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
            elif 'crossbeam' in d and 'cancel' not in d:
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
fig, ax = plt.subplots(figsize=(60, 60))
plt.gcf().subplots_adjust(bottom=0.27, left=0.25)

ax.xaxis.set_major_locator(MaxNLocator(integer=True))
ax.yaxis.set_major_locator(MaxNLocator(integer=True))

# Plot the MPST graph
ax.plot(nb_loops_mpst, mpst, label='MPST',
        linestyle='solid', linewidth=5, marker='>', markersize=20)

# Plot the binary graph
ax.plot(nb_loops_binary, binary, label='Binary',
        linestyle='solid', linewidth=5, marker='o', markersize=20)

# Plot the crossbeam graph
ax.plot(nb_loops_crossbeam, crossbeam, label='Crossbeam',
        linestyle='solid', linewidth=5, marker='d', markersize=20)

if len(cancel) > 0:
    # Plot the cancel graph
    ax.plot(nb_loops_cancel, cancel, label='AMPST',
            linestyle='solid', linewidth=5, marker='*', markersize=20)

# if len(broadcast_cancel) > 0:
#     # Plot the broadcast cancel graph
#     ax.plot(nb_loops_broadcast_cancel, broadcast_cancel,
#             label='Broadcast cancel', linestyle='dotted', linewidth=5)

# Label X and Y axis
ax.set_xlabel('\# iterations', fontsize=600)
ax.set_ylabel('Time (ms)', fontsize=500)
ax.tick_params(axis='both', which='major', labelsize=500)
ax.xaxis.set_ticks(np.arange(0, 510, 250))
ax.yaxis.set_ticks(np.arange(0, 25, 8))
ax.set_xlim(0, 510)
ax.set_ylim(0, 25)
# ax.tick_params(axis='both', which='minor', labelsize=30)

offset_x = matplotlib.transforms.ScaledTranslation(0, -2, fig.dpi_scale_trans)

# apply offset transform to all x ticklabels.
for label in ax.xaxis.get_majorticklabels():
    label.set_transform(label.get_transform() + offset_x)

offset_y = matplotlib.transforms.ScaledTranslation(-1, 0, fig.dpi_scale_trans)

for label in ax.yaxis.get_majorticklabels():
    label.set_transform(label.get_transform() + offset_y)

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
# ax.grid(which='both')


# ax.grid(which='minor', alpha=0.2)
# ax.grid(which='major', alpha=0.5)

# # giving a title to my graph
# plt.title('MPST vs binary along number of participants for ' +
#           number_of_loops + ' loops')

# show a legend on the plot
# ax.legend(bbox_to_anchor=(0.25, 0.80), loc="center", prop={'size': 150}, markerscale=3)
# ax.legend(bbox_to_anchor=(0.5, 1.08), loc="center",
#           prop={'size': 200}, markerscale=5, ncol=4)

# Save fig
plt.savefig('./graphs_bench/graphPingPong.pdf')

# # function to show the plot
# plt.show()
