import os
import statistics
import matplotlib
matplotlib.rcParams['text.usetex'] = True
import matplotlib.pyplot as plt

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
str_to_int = {'three': 3, 'four': 4, 'five': 5, 'six': 6,
              'seven': 7, 'eight': 8, 'nine': 9, 'ten': 10, 'eleven': 11, 'empty': 0}

# Change size
plt.figure(figsize=(20, 10))

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

# Plot the graph
plt.plot(nb_participants_mpst, average_mpst,
         label="MPST", linestyle='solid', linewidth=5)
plt.plot(nb_participants_binary, average_binary,
         label="Binary", linestyle='dashed', linewidth=5)

# Label X and Y axis
plt.xlabel('Number of participants', fontsize=30)
plt.ylabel('Time (µs)', fontsize=30)
plt.xticks(fontsize=30)
plt.yticks(fontsize=30)

# Add grid
plt.grid(True)

# # giving a title to my graph
# plt.title('Compile time needed')

# show a legend on the plot
plt.legend(bbox_to_anchor=(1, 1), loc="upper left", prop={'size': 20})

# Save fig
plt.savefig(main_path + '/graph_average_compile.pdf')

# # function to show the plot
# plt.show()
