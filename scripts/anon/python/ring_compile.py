from matplotlib.ticker import MaxNLocator
import matplotlib.pyplot as plt
import os
import statistics
import matplotlib
import numpy as np
matplotlib.rcParams['text.usetex'] = True

# Path for criterion
main_path = './compile_time'

# Get all directories in main_path
directories = os.listdir(main_path)

# Lists for plots
ampst = []
atmp = []

nb_participants_ampst = []
nb_participants_atmp = []

# Dictionary for converting from string to int
str_to_int = {
    'two': 2,
    'three': 3,
    'four': 4,
}

# Number of loops in the recursion
number_of_loops = '100'

# For each folder in main_path
for d in directories:

    if ".txt" in d and 'ring' in d:

        file = open(main_path + '/' + d, "r")

        name = d.split('.txt')[0].split('ring_')[1].split('_')[0]

        build_time = []
        try:
            for line in file:
                if 'build' in line:
                    build_time.append(
                        int(line.split('build; ')[1].split('\n')[0]))

            # If MPST of binary, append to related lists
            if 'protocol' not in d:
                if 'baking_ampst' in d:
                    ampst.append(statistics.mean(build_time)/10**6)
                    nb_participants_ampst.append(str_to_int[name])
                elif 'baking_atmp' in d:
                    atmp.append(statistics.mean(build_time)/10**6)
                    nb_participants_atmp.append(str_to_int[name])
        except:
            print('Issue with ', d)

        file.close()

# Sort the lists in pair
if nb_participants_ampst and ampst:
    nb_participants_ampst, ampst = (list(t) for t in zip(*sorted(zip(nb_participants_ampst, ampst))))

if nb_participants_atmp and atmp:
    nb_participants_atmp, atmp = (list(t) for t in zip(*sorted(zip(nb_participants_atmp, atmp))))

# Change size
fig, ax = plt.subplots(figsize=(60, 60))
plt.gcf().subplots_adjust(bottom=0.27, left=0.13)

ax.xaxis.set_major_locator(MaxNLocator(integer=True))
ax.yaxis.set_major_locator(MaxNLocator(integer=True))

# Plot the AMPST graph
ax.plot(nb_participants_ampst, ampst, label='MultiCrusty', linestyle='solid', linewidth=20, marker='*', markersize=70, color='#d62728')

# Plot the ATMP graph
ax.plot(nb_participants_atmp, atmp, label='Anon', linestyle='solid', linewidth=20, marker='v', markersize=70, color='#9467bd')

# Label X and Y axis
ax.set_xlabel('\# roles', fontsize=300)
# ax.set_ylabel('Time (s)', fontsize=200)
ax.tick_params(axis='both', which='major', labelsize=300)
ax.xaxis.set_ticks(np.arange(2, 11, 3))
ax.yaxis.set_ticks(np.arange(18, 25, 1))
ax.set_xlim(2, 8)
ax.set_ylim(18, 21)

offset_x = matplotlib.transforms.ScaledTranslation(0, -2, fig.dpi_scale_trans)

# apply offset transform to all x ticklabels.
for label in ax.xaxis.get_majorticklabels():
    label.set_transform(label.get_transform() + offset_x)

offset_y = matplotlib.transforms.ScaledTranslation(-1, 0, fig.dpi_scale_trans)

for label in ax.yaxis.get_majorticklabels():
    label.set_transform(label.get_transform() + offset_y)

# Add grid
# ax.grid(True)

# # giving a title to my graph
# plt.title('Compile time needed')

# show a legend on the plot
# ax.legend(bbox_to_anchor=(0.5, 1), loc="lower center", prop={'size': 20})

# Tight layout
plt.tight_layout()

# create the name for the new figure
index_graphs = 0
while os.path.isfile('graphs_bench/ring/compile_time_' + number_of_loops + '_' + str(index_graphs) + '.pdf'):
    index_graphs += 1

name_graph = './graphs_bench/ring/compile_time_' + number_of_loops + '_' + str(index_graphs) + '.pdf'

# Save fig
plt.savefig(name_graph)

# # function to show the plot
# plt.show()
