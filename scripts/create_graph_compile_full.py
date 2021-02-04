from matplotlib.ticker import MaxNLocator
import matplotlib.pyplot as plt
import os
import matplotlib
matplotlib.rcParams['text.usetex'] = True

# Path for criterion
main_path = './compile_time'

# Get all directories in main_path
directories = os.listdir(main_path)

# Lists for plots
nb_participants_iterations = [i for i in range(100)]

# Change size
ax = plt.figure(figsize=(30, 15)).gca()

ax.xaxis.set_major_locator(MaxNLocator(integer=True))
ax.yaxis.set_major_locator(MaxNLocator(integer=True))

# For each folder in main_path
for d in directories:

    if ".txt" in d:

        file = open(main_path + '/' + d, "r")

        name = d.split('.txt')[0].split('long_simple_')[1].replace('_', ' ')

        # Plot the graph
        ax.plot(nb_participants_iterations, [
            int(line) for line in file], label=name, linewidth=5)

        file.close()

# Label X and Y axis
ax.set_xlabel('Number of iterations', fontsize=30)
ax.set_ylabel('Time (µs)', fontsize=30)
ax.tick_params(axis='both', which='major', labelsize=30)
ax.tick_params(axis='both', which='minor', labelsize=30)

# Add grid
ax.grid(True)

# # giving a title to my graph
# plt.title('Compile time needed')

# show a legend on the plot
ax.legend(bbox_to_anchor=(1, 1), loc="upper left", prop={'size': 20})

# # Save fig
plt.savefig(main_path + '/graphFullCompile.pdf')

# # function to show the plot
# plt.show()
