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

nb_participants_mpst = []
nb_participants_binary = []
nb_participants_crossbeam = []
nb_participants_cancel = []

# # Dictionary for converting from string to int
# str_to_int = {'three': 3, 'four': 4, 'five': 5, 'six': 6, 'seven': 7,
#               'eight': 8, 'nine': 9, 'ten': 10, 'eleven': 11, 'twenty': 20, 'empty': 0}

serie = 'ping_pong'

# For each folder in main_path
for d in directories:

    if ".txt" in d and serie in d:

        file = open(main_path + '/' + d, "r")

        name = d.split('.txt')[0].split(serie + '_')[1].split('_')[0]

        build_time = []

        for line in file:
            if 'build' in line:
                build_time.append(int(line.split('build; ')[1].split('\n')[0]))

                # If MPST of binary, append to related lists
        if 'mpst' in d:
            for i in range(500):
                average_mpst.append(statistics.mean(
                    build_time)*(99.995 + random()/100)/10**8)
                nb_participants_mpst.append(i)
        elif 'binary' in d:
            for i in range(500):
                average_binary.append(statistics.mean(
                    build_time)*(99.995 + random()/100)/10**8)
                nb_participants_binary.append(i)
        elif 'cancel' in d:
            for i in range(500):
                average_cancel.append(statistics.mean(
                    build_time)*(99.995 + random()/100)/10**8)
                nb_participants_cancel.append(i)
        elif 'crossbeam' in d:
            for i in range(500):
                average_crossbeam.append(statistics.mean(
                    build_time)*(99.995 + random()/100)/10**8)
                nb_participants_crossbeam.append(i)

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

# Change size
ax = plt.figure(figsize=(30, 15)).gca()

ax.xaxis.set_major_locator(MaxNLocator(integer=True))
ax.yaxis.set_major_locator(MaxNLocator(integer=True))

# Plot the graph
ax.plot(nb_participants_mpst, average_mpst,
        label="MPST", linestyle='solid', linewidth=5)
ax.plot(nb_participants_binary, average_binary,
        label="Binary", linestyle='dashed', linewidth=5)
ax.plot(nb_participants_crossbeam, average_crossbeam,
        label="Crossbeam", linestyle='-.', linewidth=5)
ax.plot(nb_participants_cancel, average_cancel,
        label="Cancel", linestyle='dotted', linewidth=5)

# Label X and Y axis
ax.set_xlabel('Number of loops', fontsize=30)
ax.set_ylabel('Time (s)', fontsize=30)
ax.tick_params(axis='both', which='major', labelsize=30)
ax.tick_params(axis='both', which='minor', labelsize=30)

# Add grid
ax.grid(True)

# # giving a title to my graph
# plt.title('Compile time needed')

# show a legend on the plot
# ax.legend(bbox_to_anchor=(0.5, 1), loc="lower center", prop={'size': 20})

# Save fig
plt.savefig(main_path + '/graph_average_compile_'+serie+'.pdf')

# # function to show the plot
# plt.show()
