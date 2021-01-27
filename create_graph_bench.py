import json
import os
import matplotlib
matplotlib.rcParams['text.usetex'] = True
import matplotlib.pyplot as plt

# Path for criterion
main_path = './target/criterion'

# Get all directories in main_path
directories = os.listdir(main_path)

# Relative path of the expected file
path_file = '/base/estimates.json'

# Dictionary for converting from string to int
str_to_int = {'three': 3, 'four': 4, 'five': 5, 'six': 6,
              'seven': 7, 'eight': 8, 'nine': 9, 'ten': 10, 'eleven': 11, 'empty': 0}

# Lists for plots
binary = []
mpst = []
nb_participants_mpst = []
nb_participants_binary = []

# Number of loops in the recursion
number_of_loops = '0'


def test(path):
    # Get the wanted data in the JSON file (field -> mean, field -> point_estimate)
    with open(main_path + '/' + path + path_file) as json_file:
        data = json.load(json_file)
        return data['mean']['point_estimate']


# For each folder in main_path
for d in directories:
    # If name looks like the one from what we want
    if ('MPST' in d or 'binary' in d) and ' ' + number_of_loops in d:
        # Split the name
        splitted = d.split(' ')

        # If MPST of binary, append to related lists
        if 'MPST' in d:
            mpst.append(test(d))
            nb_participants_mpst.append(str_to_int[splitted[1]])
        else:
            binary.append(test(d))
            nb_participants_binary.append(str_to_int[splitted[1]])

# Sort the lists in pair
nb_participants_mpst, mpst = (list(t) for t in zip(
    *sorted(zip(nb_participants_mpst, mpst))))

nb_participants_binary, binary = (list(t)
                                  for t in zip(*sorted(zip(nb_participants_binary, binary))))

# Change size
plt.figure(figsize=(20, 10))

# Plot the MPST graph
plt.plot(nb_participants_mpst, mpst, label='MPST',
         linestyle='solid', linewidth=5)

# Plot the binary graph
plt.plot(nb_participants_binary, binary, label='Binary',
         linestyle='dashed', linewidth=5)

# Label X and Y axis
plt.xlabel('Number of participants', fontsize=30)
plt.ylabel('Time (ns)', fontsize=30)
plt.xticks(fontsize=30)
plt.yticks(fontsize=30)

# Add grid
plt.grid(True)

# # giving a title to my graph
# plt.title('MPST vs binary along number of participants for ' +
#           number_of_loops + ' loops')

# show a legend on the plot
plt.legend(bbox_to_anchor=(1, 1), loc="upper left", prop={'size': 20})

# Save fig
plt.savefig('./graphs_bench/graph_'+number_of_loops+'.pdf')

# # function to show the plot
# plt.show()
