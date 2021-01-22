import os
import matplotlib.pyplot as plt

# Path for criterion
main_path = './compile_time'

# Get all directories in main_path
directories = os.listdir(main_path)

# Lists for plots
nb_participants_iterations = [i for i in range(100)]

# Change size
plt.figure(figsize=(20, 10))

# For each folder in main_path
for d in directories:

    if ".txt" in d:

        file = open(main_path + '/' + d, "r")

        name = d.split('.txt')[0].split('long_simple_')[1].replace('_', ' ')

        # Plot the graph
        plt.plot(nb_participants_iterations, [
                 int(line) for line in file], label=name)

        file.close()

# Label X and Y axis
plt.xlabel('Number of iterations')
plt.ylabel('Time (µs)')

# # giving a title to my graph
# plt.title('Compile time needed')

# show a legend on the plot
plt.legend(bbox_to_anchor=(1, 1), loc="upper left", prop={'size': 15})

# # Save fig
plt.savefig(main_path + '/graph_full_compile.pdf')

# # function to show the plot
# plt.show()
