from matplotlib.ticker import MaxNLocator
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
nb_participants_mpst = []
nb_participants_binary = []

# Dictionary for converting from string to int
str_to_int = {'three': 3, 'four': 4, 'five': 5, 'six': 6, 'seven': 7,
              'eight': 8, 'nine': 9, 'ten': 10, 'eleven': 11, 'twenty': 20, 'empty': 0}

# For each folder in main_path
for d in directories:

    if ".txt" in d:

        file = open(main_path + '/' + d, "r")

        name = d.split('.txt')[0].split('long_simple_')[1].split('_')[0]

        # If MPST of binary, append to related lists
        if 'mpst' in d:
            average_mpst.append(statistics.mean([
                int(line) for line in file]))
            nb_participants_mpst.append(str_to_int[name])
        else:
            average_binary.append(statistics.mean([
                int(line) for line in file]))
            nb_participants_binary.append(str_to_int[name])

        file.close()

# Sort the lists in pair
nb_participants_mpst, average_mpst = (list(t) for t in zip(
    *sorted(zip(nb_participants_mpst, average_mpst))))

nb_participants_binary, average_binary = (list(t)
                                          for t in zip(*sorted(zip(nb_participants_binary, average_binary))))

# Change size
ax = plt.figure(figsize=(30, 15)).gca()

ax.xaxis.set_major_locator(MaxNLocator(integer=True))
ax.yaxis.set_major_locator(MaxNLocator(integer=True))

# Plot the graph
ax.plot(nb_participants_mpst, average_mpst,
        label="MPST", linestyle='solid', linewidth=5)
ax.plot(nb_participants_binary, average_binary,
        label="Binary", linestyle='dashed', linewidth=5)

# Label X and Y axis
ax.set_xlabel('Number of participants', fontsize=30)
ax.set_ylabel('Time (µs)', fontsize=30)
ax.tick_params(axis='both', which='major', labelsize=30)
ax.tick_params(axis='both', which='minor', labelsize=30)

# Add grid
ax.grid(True)

# # giving a title to my graph
# plt.title('Compile time needed')

# show a legend on the plot
ax.legend(bbox_to_anchor=(1, 1), loc="upper left", prop={'size': 20})

# Save fig
plt.savefig(main_path + '/graphAverageCompile.pdf')

# # function to show the plot
# plt.show()
