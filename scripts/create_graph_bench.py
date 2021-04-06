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

# Dictionary for converting from string to int
str_to_int = {'three': 3, 'four': 4, 'five': 5, 'six': 6, 'seven': 7,
              'eight': 8, 'nine': 9, 'ten': 10, 'eleven': 11, 'twenty': 20, 'empty': 0}

# Lists for plots
binary = []
mpst = []
crossbeam = []
nb_participants_mpst = []
nb_participants_binary = []
nb_participants_crossbeam = []

# # Number of loops in the recursion
# number_of_loops = '100'


# def test(path):
#     # Get the wanted data in the JSON file (field -> mean, field -> point_estimate)
#     with open(main_path + '/' + path + path_file) as json_file:
#         data = json.load(json_file)
#         return data['mean']['point_estimate']


# # For each folder in main_path
# for d in directories:
#     # If name looks like the one from what we want
#     if ('MPST' in d or 'binary' in d or 'crossbeam' in d) and ' ' + number_of_loops in d:
#         # Split the name
#         splitted = d.split(' ')

#         # If MPST of binary, append to related lists
#         if 'MPST' in d and str_to_int[splitted[1]] >= 3:
#             mpst.append(int(test(d))/10**9)
#             nb_participants_mpst.append(str_to_int[splitted[1]])
#         elif 'binary' in d and str_to_int[splitted[1]] >= 3:
#             binary.append(int(test(d))/10**9)
#             nb_participants_binary.append(str_to_int[splitted[1]])
#         elif 'crossbeam' in d and str_to_int[splitted[1]] >= 3:
#             crossbeam.append(int(test(d))/10**9)
#             nb_participants_crossbeam.append(str_to_int[splitted[1]])

# # Sort the lists in pair
# nb_participants_mpst, mpst = (list(t) for t in zip(
#     *sorted(zip(nb_participants_mpst, mpst))))

# nb_participants_binary, binary = (list(t)
#                                   for t in zip(*sorted(zip(nb_participants_binary, binary))))

# nb_participants_crossbeam, crossbeam = (list(t)
#                                         for t in zip(*sorted(zip(nb_participants_crossbeam, crossbeam))))

# # Change size
# ax = plt.figure(figsize=(30, 15)).gca()

# ax.xaxis.set_major_locator(MaxNLocator(integer=True))
# ax.yaxis.set_major_locator(MaxNLocator(integer=True))

# # Plot the MPST graph
# ax.plot(nb_participants_mpst, mpst, label='MPST',
#         linestyle='solid', linewidth=3)

# # Plot the binary graph
# ax.plot(nb_participants_binary, binary, label='Binary',
#         linestyle='dashed', linewidth=3)

# # Plot the crossbeam graph
# ax.plot(nb_participants_crossbeam, crossbeam, label='Crossbeam',
#         linestyle='-.', linewidth=3)

# # Label X and Y axis
# ax.set_xlabel('Number of participants', fontsize=30)
# ax.set_ylabel('Time (s)', fontsize=30)
# ax.tick_params(axis='both', which='major', labelsize=30)
# # ax.tick_params(axis='both', which='minor', labelsize=30)

# maxi1 = max(mpst)
# maxi2 = max(binary)
# maxi3 = max(crossbeam)
# maxi = max(maxi1, maxi2, maxi3)

# mini1 = min(mpst)
# mini2 = min(binary)
# mini3 = min(crossbeam)
# mini = min(mini1, mini2, mini3)

# # Major ticks every 20, minor ticks every 5
# # major_ticks = np.arange(mini, maxi+2, 10)
# # minor_ticks = np.arange(mini, maxi+2, 2)
# major_ticks = np.arange(mini, maxi, 0.)
# minor_ticks = np.arange(mini, maxi, 2)

# # ax.set_xticks(major_ticks)
# # ax.set_xticks(minor_ticks, minor=True)
# ax.set_yticks(major_ticks)
# ax.set_yticks(minor_ticks, minor=True)

# # Add grid
# ax.grid(which='both')


# Number of loops in the recursion
number_of_loops = '100'


def test(path):
    # Get the wanted data in the JSON file (field -> mean, field -> point_estimate)
    with open(main_path + '/' + path + path_file) as json_file:
        data = json.load(json_file)
        return data['mean']['point_estimate']


# For each folder in main_path
for d in directories:
    # If name looks like the one from what we want
    if ('MPST' in d or 'binary' in d or 'crossbeam' in d) and ' ' + number_of_loops in d:
        # Split the name
        splitted = d.split(' ')

        # If MPST of binary, append to related lists
        if 'MPST' in d and str_to_int[splitted[1]] >= 3:
            mpst.append(int(test(d))/10**9)
            nb_participants_mpst.append(str_to_int[splitted[1]])
        elif 'binary' in d and str_to_int[splitted[1]] >= 3:
            binary.append(int(test(d))/10**9)
            nb_participants_binary.append(str_to_int[splitted[1]])
        elif 'crossbeam' in d and str_to_int[splitted[1]] >= 3:
            crossbeam.append(int(test(d))/10**9)
            nb_participants_crossbeam.append(str_to_int[splitted[1]])

# Sort the lists in pair
nb_participants_mpst, mpst = (list(t) for t in zip(
    *sorted(zip(nb_participants_mpst, mpst))))

nb_participants_binary, binary = (list(t)
                                  for t in zip(*sorted(zip(nb_participants_binary, binary))))

nb_participants_crossbeam, crossbeam = (list(t)
                                        for t in zip(*sorted(zip(nb_participants_crossbeam, crossbeam))))

# Change size
ax = plt.figure(figsize=(30, 15)).gca()

ax.xaxis.set_major_locator(MaxNLocator(integer=True))
ax.yaxis.set_major_locator(MaxNLocator(integer=True))

# Plot the MPST graph
ax.plot(nb_participants_mpst, mpst, label='MPST',
        linestyle='solid', linewidth=3)

# Plot the binary graph
ax.plot(nb_participants_binary, binary, label='Binary',
        linestyle='dashed', linewidth=3)

# Plot the crossbeam graph
ax.plot(nb_participants_crossbeam, crossbeam, label='Crossbeam',
        linestyle='-.', linewidth=3)

# Label X and Y axis
ax.set_xlabel('Number of participants', fontsize=30)
ax.set_ylabel('Time (s)', fontsize=30)
ax.tick_params(axis='both', which='major', labelsize=30)
# ax.tick_params(axis='both', which='minor', labelsize=30)

maxi1 = max(mpst)
maxi2 = max(binary)
maxi3 = max(crossbeam)
maxi = max(maxi1, maxi2, maxi3)

mini1 = min(mpst)
mini2 = min(binary)
mini3 = min(crossbeam)
mini = min(mini1, mini2, mini3)

# Major ticks every 20, minor ticks every 5
major_ticks = np.arange(mini, maxi+0.1, 0.2)
minor_ticks = np.arange(mini, maxi+0.1, 0.05)

# ax.set_xticks(major_ticks)
# ax.set_xticks(minor_ticks, minor=True)
ax.set_yticks(major_ticks)
ax.set_yticks(minor_ticks, minor=True)

# Add grid
ax.grid(which='both')


# ax.grid(which='minor', alpha=0.2)
# ax.grid(which='major', alpha=0.5)

# # giving a title to my graph
# plt.title('MPST vs binary along number of participants for ' +
#           number_of_loops + ' loops')

# show a legend on the plot
ax.legend(bbox_to_anchor=(1, 1), loc="upper left", prop={'size': 20})

# Save fig
plt.savefig('./graphs_bench/graph'+number_of_loops+'.pdf')

# # function to show the plot
# plt.show()
