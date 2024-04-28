from matplotlib.ticker import MaxNLocator
from random import random
import matplotlib.pyplot as plt
import os
import statistics
import matplotlib
matplotlib.rcParams['text.usetex'] = True

# Path for criterion
main_path = './compile_time'

# Get all directories in main_path
directories = os.listdir(main_path)

# Lists for plots
average_mpst = []
average_binary = []
average_crossbeam = []
average_cancel = []
average_cancel_broadcast = []

nb_participants_mpst = []
nb_participants_binary = []
nb_participants_crossbeam = []
nb_participants_cancel = []
nb_participants_cancel_broadcast = []

# # Dictionary for converting from string to int
# str_to_int = {'three': 3, 'four': 4, 'five': 5, 'six': 6, 'seven': 7,
#               'eight': 8, 'nine': 9, 'ten': 10, 'eleven': 11, 'twenty': 20, 'empty': 0}

serie = 'ping_pong'

# For each folder in main_path
for d in directories:

    if ".txt" in d and serie in d:

        file = open(main_path + '/' + d, "r")

        (name, number) = d.split('.txt')[0].split(serie + '_')[1].split('_')

        build_time = []

        for line in file:
            if 'build' in line:
                build_time.append(int(line.split('build; ')[1].split('\n')[0]))

                # If MPST of binary, append to related lists

        if 'mpst' in d:
            average_mpst.append(statistics.mean(build_time))
            nb_participants_mpst.append(int(number))
        elif 'binary' in d:
            average_binary.append(statistics.mean(build_time))
            nb_participants_binary.append(int(number))
        elif 'cancel' in d:
            if 'broadcast' in d:
                average_cancel_broadcast.append(statistics.mean(build_time))
                nb_participants_cancel_broadcast.append(int(number))
            else:
                average_cancel.append(statistics.mean(build_time))
                nb_participants_cancel.append(int(number))
        elif 'crossbeam' in d:
            average_crossbeam.append(statistics.mean(build_time))
            nb_participants_crossbeam.append(int(number))

        file.close()

# Sort the lists in pair
nb_participants_mpst, average_mpst = (list(t) for t in zip(
    *sorted(zip(nb_participants_mpst, average_mpst))))

nb_participants_binary, average_binary = (list(t)
                                          for t in zip(*sorted(zip(nb_participants_binary, average_binary))))

nb_participants_crossbeam, average_crossbeam = (list(t)
                                                for t in zip(*sorted(zip(nb_participants_crossbeam, average_crossbeam))))

nb_participants_cancel, average_cancel = (list(t)
                                          for t in zip(*sorted(zip(nb_participants_cancel, average_cancel))))

nb_participants_cancel_broadcast, average_cancel_broadcast = (list(t)
                                                              for t in zip(*sorted(zip(nb_participants_cancel_broadcast, average_cancel_broadcast))))

# Change size
fig, ax = plt.subplots(figsize=(60, 60))
plt.gcf().subplots_adjust(bottom=0.27, left=0.25)

ax.xaxis.set_major_locator(MaxNLocator(integer=True))
ax.yaxis.set_major_locator(MaxNLocator(integer=True))

# Plot the MPST graph
ax.plot(nb_participants_mpst, average_mpst, label='MPST',
        linestyle='solid', linewidth=10, marker='>', markersize=30)

# Plot the binary graph
ax.plot(nb_participants_binary, average_binary, label='Binary',
        linestyle='solid', linewidth=10, marker='o', markersize=30)

# Plot the crossbeam graph
ax.plot(nb_participants_crossbeam, average_crossbeam, label='Crossbeam',
        linestyle='solid', linewidth=10, marker='d', markersize=30)

if len(average_cancel) > 0:
    # Plot the cancel graph
    ax.plot(nb_participants_cancel, average_cancel, label='Cancel',
            linestyle='solid', linewidth=10, marker='*', markersize=30)

# Label X and Y axis
ax.set_xlabel('\# loops', fontsize=200)
ax.set_ylabel('Time (ms)', fontsize=200)
ax.tick_params(axis='both', which='major', labelsize=190)
# ax.xaxis.set_ticks(np.arange(100, 510, 100))
# ax.set_xlim(3, 10)
# ax.set_ylim(0, 180)
# ax.tick_params(axis='both', which='minor', labelsize=30)

offset_x = matplotlib.transforms.ScaledTranslation(1, -2, fig.dpi_scale_trans)

# apply offset transform to all x ticklabels.
for label in ax.xaxis.get_majorticklabels():
    label.set_transform(label.get_transform() + offset_x)

offset_y = matplotlib.transforms.ScaledTranslation(-1, 1, fig.dpi_scale_trans)

for label in ax.yaxis.get_majorticklabels():
    label.set_transform(label.get_transform() + offset_y)

# Add grid
# ax.grid(True)

# # giving a title to my graph
# plt.title('Compile time needed')

# show a legend on the plot
# ax.legend(bbox_to_anchor=(0.5, 1), loc="lower center", prop={'size': 20})

# Save fig
plt.savefig(main_path + '/graphAverageCompilePingPong.pdf')

# # function to show the plot
# plt.show()
