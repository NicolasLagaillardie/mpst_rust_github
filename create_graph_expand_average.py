import os
import matplotlib.pyplot as plt
import statistics

# Path for criterion
main_path = './expand'

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
            average_mpst.append(sum(1 for line in file))
            nb_participants_mpst.append(str_to_int[name])
        else:
            average_binary.append(sum(1 for line in file))
            nb_participants_binary.append(str_to_int[name])

        file.close()

# Sort the lists in pair
nb_participants_mpst, average_mpst = (list(t) for t in zip(
    *sorted(zip(nb_participants_mpst, average_mpst))))

nb_participants_binary, average_binary = (list(t)
                                          for t in zip(*sorted(zip(nb_participants_binary, average_binary))))

# Plot the graph
plt.plot(nb_participants_mpst, average_mpst, label="MPST", linestyle='solid')
plt.plot(nb_participants_binary, average_binary,
         label="binary", linestyle='dashed')

# Label X and Y axis
plt.xlabel('Number of participants')
plt.ylabel('Number of lines')

# # giving a title to my graph
# plt.title('Number of lines')

# show a legend on the plot
plt.legend(bbox_to_anchor=(1, 1), loc="upper left", prop={'size': 15})

# Save fig
plt.savefig(main_path + '/graph_average_line.pdf')

# # function to show the plot
# plt.show()
