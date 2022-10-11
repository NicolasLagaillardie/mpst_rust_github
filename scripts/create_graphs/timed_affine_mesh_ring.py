from matplotlib.ticker import MaxNLocator
import matplotlib.pyplot as plt
import json
import os
import matplotlib
import numpy as np
import statistics
import os.path
from pathlib import Path
matplotlib.rcParams['text.usetex'] = True

# Path for criterion
criterion_path = './target/criterion'

# Path for compile-time
compile_path = 'compile_time'

# Get all directories in criterion_path
criterion_directories = os.listdir(criterion_path)

# Compile folder
compile_folder = Path(compile_path + '/')

# Result folder
result_folder = Path('results/')

# Relative path of the expected file
json_path = '/base/estimates.json'

# Expected compile files
name_protocols = ['mesh', 'ring']


def test(path):
    # Get the wanted data in the JSON file (field -> mean, field -> point_estimate)
    with open(criterion_path + '/' + path + json_path) as json_file:
        data = json.load(json_file)
        return data['mean']['point_estimate']


def mesh_bench():
    # Path for criterion
    main_path = './target/criterion'

    # Get all directories in main_path
    directories = os.listdir(main_path)

    # Relative path of the expected file
    path_file = '/base/estimates.json'

    # Dictionary for converting from string to int
    str_to_int = {'two': 2, 'three': 3, 'four': 4, 'five': 5, 'six': 6, 'seven': 7,
                  'eight': 8, 'nine': 9, 'ten': 10, 'eleven': 11, 'twenty': 20, 'empty': 0}

    # Lists for plots
    average_baking = []
    average_timed = []

    nb_participants_baking = []
    nb_participants_timed = []

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
        if 'mesh' in d and ' ' + number_of_loops in d:
            # Split the name
            splitted = d.split(' ')

            try:
                if 'baking' in d and 'timed' not in d:
                    average_baking.append(int(test(d))/1000000)
                    nb_participants_baking.append(str_to_int[splitted[1]])
                elif 'baking' in d and 'timed' in d:
                    average_timed.append(int(test(d))/1000000)
                    nb_participants_timed.append(str_to_int[splitted[2]])
            except:
                print("Missing ", d)

    # Sort the lists in pair
    if nb_participants_baking and average_baking:
        nb_participants_baking, average_baking = (list(t) for t in zip(
            *sorted(zip(nb_participants_baking, average_baking))))

    if nb_participants_timed and average_timed:
        nb_participants_timed, average_timed = (list(t) for t in zip(
            *sorted(zip(nb_participants_timed, average_timed))))

    return {
        'nb_participants_baking': nb_participants_baking,
        'average_baking': average_baking,
        'nb_participants_timed': nb_participants_timed,
        'average_timed': average_timed
    }


def mesh_compile():
    # Path for criterion
    main_path = './compile_time'

    # Get all directories in main_path
    directories = os.listdir(main_path)

    # Lists for plots
    average_baking = []
    average_timed = []

    nb_participants_baking = []
    nb_participants_timed = []

    # Dictionary for converting from string to int
    str_to_int = {'two': 2, 'three': 3, 'four': 4, 'five': 5, 'six': 6, 'seven': 7,
                  'eight': 8, 'nine': 9, 'ten': 10, 'eleven': 11, 'twenty': 20, 'empty': 0}

    # For each folder in main_path
    for d in directories:

        if ".txt" in d and 'mesh' in d:

            file = open(main_path + '/' + d, "r")

            name = d.split('.txt')[0].split('mesh_')[1].split('_')[0]

            build_time = []
            try:
                for line in file:
                    if 'build' in line:
                        build_time.append(
                            int(line.split('build; ')[1].split('\n')[0]))

                # If MPST of binary, append to related lists
                if 'baking' in d and 'timed' not in d:
                    average_baking.append(
                        statistics.mean(build_time)/1000000)
                    nb_participants_baking.append(str_to_int[name])
                elif 'baking' in d and 'timed' in d:
                    average_timed.append(
                        statistics.mean(build_time)/1000000)
                    nb_participants_timed.append(str_to_int[name])
            except:
                print('Issue with ', d)

            file.close()

    # Sort the lists in pair
    if nb_participants_baking and average_baking:
        nb_participants_baking, average_baking = (list(t) for t in zip(
            *sorted(zip(nb_participants_baking, average_baking))))

    if nb_participants_timed and average_timed:
        nb_participants_timed, average_timed = (list(t) for t in zip(
            *sorted(zip(nb_participants_timed, average_timed))))

    return {
        'nb_participants_baking': nb_participants_baking,
        'average_baking': average_baking,
        'nb_participants_timed': nb_participants_timed,
        'average_timed': average_timed
    }


def ring_bench():
    # Path for criterion
    main_path_criterion = './target/criterion'

    # Get all directories_criterion in main_path_criterion
    directories_criterion = os.listdir(main_path_criterion)

    # Relative path of the expected file
    path_file = '/base/estimates.json'

    # Dictionary for converting from string to int
    str_to_int = {'two': 2, 'three': 3, 'four': 4, 'five': 5, 'six': 6, 'seven': 7,
                  'eight': 8, 'nine': 9, 'ten': 10, 'eleven': 11, 'twenty': 20, 'empty': 0}

    # Lists for plots
    average_baking = []
    average_timed = []

    nb_participants_baking = []
    nb_participants_timed = []

    # Number of loops in the recursion
    number_of_loops = '100'

    def test(path):
        # Get the wanted data in the JSON file (field -> mean, field -> point_estimate)
        with open(main_path_criterion + '/' + path + path_file) as json_file:
            data = json.load(json_file)
            return data['mean']['point_estimate']

    # For each folder in main_path_criterion
    for d in directories_criterion:
        # If name looks like the one from what we want
        if 'ring' in d and ' ' + number_of_loops in d:
            # Split the name
            splitted = d.split(' ')

            try:
                if 'baking' in d and 'timed' not in d:
                    average_baking.append(int(test(d))/1000000)
                    nb_participants_baking.append(str_to_int[splitted[1]])
                elif 'baking' in d and 'timed' in d:
                    average_timed.append(int(test(d))/1000000)
                    nb_participants_timed.append(str_to_int[splitted[2]])
            except:
                print("Missing ", d)

    # Sort the lists in pair
    if nb_participants_baking and average_baking:
        nb_participants_baking, average_baking = (list(t) for t in zip(
            *sorted(zip(nb_participants_baking, average_baking))))

    if nb_participants_timed and average_timed:
        nb_participants_timed, average_timed = (list(t) for t in zip(
            *sorted(zip(nb_participants_timed, average_timed))))

    return {
        'nb_participants_baking': nb_participants_baking,
        'average_baking': average_baking,
        'nb_participants_timed': nb_participants_timed,
        'average_timed': average_timed
    }


def ring_compile():
    # Path for criterion
    main_path = './compile_time'

    # Get all directories in main_path
    directories = os.listdir(main_path)

    # Lists for plots
    average_baking = []
    average_timed = []

    nb_participants_baking = []
    nb_participants_timed = []

    # Dictionary for converting from string to int
    str_to_int = {'two': 2, 'three': 3, 'four': 4, 'five': 5, 'six': 6, 'seven': 7,
                  'eight': 8, 'nine': 9, 'ten': 10, 'eleven': 11, 'twenty': 20, 'empty': 0}

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
                if 'baking' in d and 'timed' not in d:
                    average_baking.append(
                        statistics.mean(build_time)/1000000)
                    nb_participants_baking.append(str_to_int[name])
                elif 'baking' in d and 'timed' in d:
                    average_timed.append(
                        statistics.mean(build_time)/1000000)
                    nb_participants_timed.append(str_to_int[name])
            except:
                print('Issue with ', d)

            file.close()

    # Sort the lists in pair
    if nb_participants_baking and average_baking:
        nb_participants_baking, average_baking = (list(t) for t in zip(
            *sorted(zip(nb_participants_baking, average_baking))))

    if nb_participants_timed and average_timed:
        nb_participants_timed, average_timed = (list(t) for t in zip(
            *sorted(zip(nb_participants_timed, average_timed))))

    return {
        'nb_participants_baking': nb_participants_baking,
        'average_baking': average_baking,
        'nb_participants_timed': nb_participants_timed,
        'average_timed': average_timed
    }


# Get index of new file
index_mesh = 0
while os.path.isfile('results/mesh_' + str(index_mesh) + '.csv'):
    index_mesh += 1
index_ring = 0
while os.path.isfile('results/ring_' + str(index_ring) + '.csv'):
    index_ring += 1

# Get the dictionaries with the results
mesh_bench_lists = mesh_bench()
mesh_compile_lists = mesh_compile()
ring_bench_lists = ring_bench()
ring_compile_lists = ring_compile()

# Create the correct str for the new files
result_mesh_file = 'mesh_' + str(index_mesh) + '.csv'
result_ring_file = 'ring_' + str(index_ring) + '.csv'

# Open the file in the folder and append each block of type of protocol, in the order: crossbeam, binary, mpst and ampst
# If they don't exist, skip
with open(result_folder / result_mesh_file, 'a') as report_file:
    for i, val in enumerate(mesh_bench_lists['nb_participants_baking']):
        report_file.write('AMPST')
        report_file.write('; ')
        report_file.write(str(val))
        report_file.write('; ')
        if i < len(mesh_bench_lists['average_baking']):
            report_file.write(str(mesh_bench_lists['average_baking'][i]))
        report_file.write('; ')
        if i < len(mesh_compile_lists['average_baking']):
            report_file.write(str(mesh_compile_lists['average_baking'][i]))
        report_file.write('\n')
    report_file.write('\n')
    for i, val in enumerate(mesh_bench_lists['nb_participants_timed']):
        report_file.write('ATMP')
        report_file.write('; ')
        report_file.write(str(val))
        report_file.write('; ')
        if i < len(mesh_bench_lists['average_timed']):
            report_file.write(str(mesh_bench_lists['average_timed'][i]))
        report_file.write('; ')
        if i < len(mesh_compile_lists['average_timed']):
            report_file.write(str(mesh_compile_lists['average_timed'][i]))
        report_file.write('\n')
    report_file.write('\n')


with open('results/ring_' + str(index_ring) + '.csv', 'a') as report_file:
    for i, val in enumerate(ring_bench_lists['nb_participants_baking']):
        report_file.write('ampst')
        report_file.write('; ')
        report_file.write(str(val))
        report_file.write('; ')
        if i < len(ring_bench_lists['average_baking']):
            report_file.write(str(ring_bench_lists['average_baking'][i]))
        report_file.write('; ')
        if i < len(ring_compile_lists['average_baking']):
            report_file.write(str(ring_compile_lists['average_baking'][i]))
        report_file.write('\n')
    report_file.write('\n')
    for i, val in enumerate(ring_bench_lists['nb_participants_timed']):
        report_file.write('aatmpst')
        report_file.write('; ')
        report_file.write(str(val))
        report_file.write('; ')
        if i < len(ring_bench_lists['average_timed']):
            report_file.write(str(ring_bench_lists['average_timed'][i]))
        report_file.write('; ')
        if i < len(ring_compile_lists['average_baking']):
            report_file.write(str(ring_compile_lists['average_timed'][i]))
        report_file.write('\n')
    report_file.write('\n')

# Create the figure
all_graphs = plt.figure(figsize=(20, 5))

# Create the graphs
ax_mesh_compile = all_graphs.add_subplot(1, 4, 1)
ax_ring_compile = all_graphs.add_subplot(1, 4, 2)
ax_mesh_bench = all_graphs.add_subplot(1, 4, 3)
ax_ring_bench = all_graphs.add_subplot(1, 4, 4)

# Set title
ax_mesh_compile.set_title("Mesh compilation time", pad=10, fontsize=30)
ax_mesh_bench.set_title("Mesh bench time", pad=10, fontsize=30)
ax_ring_compile.set_title("Ring compilation time", pad=10, fontsize=30)
ax_ring_bench.set_title("Ring bench time", pad=10, fontsize=30)

# Set axis to integers
# Mesh compile
ax_mesh_compile.xaxis.set_major_locator(MaxNLocator(integer=True))
ax_mesh_compile.yaxis.set_major_locator(MaxNLocator(integer=True))
# Mesh bench
ax_mesh_bench.xaxis.set_major_locator(MaxNLocator(integer=True))
ax_mesh_bench.yaxis.set_major_locator(MaxNLocator(integer=True))
# Ring compile
ax_ring_compile.xaxis.set_major_locator(MaxNLocator(integer=True))
ax_ring_compile.yaxis.set_major_locator(MaxNLocator(integer=True))
# Ring bench
ax_ring_bench.xaxis.set_major_locator(MaxNLocator(integer=True))
ax_ring_bench.yaxis.set_major_locator(MaxNLocator(integer=True))

# Set the axis titles
ax_mesh_compile.set_xlabel('\# roles', fontsize=30)
ax_mesh_bench.set_xlabel('\# roles', fontsize=30)
ax_ring_compile.set_xlabel('\# roles', fontsize=30)
ax_ring_bench.set_xlabel('\# roles', fontsize=30)

# Set ticks parameters to major/both
ax_mesh_compile.tick_params(axis='both', which='major', labelsize=20)
ax_mesh_compile.yaxis.set_ticks(
    np.arange(
        int(min(min(mesh_compile_lists['average_baking']), min(
            mesh_compile_lists['average_timed']))/10)*10-5,
        int(max(max(mesh_compile_lists['average_baking']), max(
            mesh_compile_lists['average_timed']))/10)*10+15,
        10))
ax_mesh_bench.tick_params(axis='both', which='major', labelsize=20)
ax_mesh_bench.yaxis.set_ticks(
    np.arange(
        int(min(min(mesh_bench_lists['average_baking']), min(
            mesh_bench_lists['average_timed']))/10)*10-5,
        int(max(max(mesh_bench_lists['average_baking']), max(
            mesh_bench_lists['average_timed']))/10)*10+16,
        4))
ax_ring_compile.tick_params(axis='both', which='major', labelsize=20)
ax_ring_compile.yaxis.set_ticks(
    np.arange(
        int(min(min(ring_compile_lists['average_baking']), min(
            ring_compile_lists['average_timed']))/10)*10-5,
        int(max(max(ring_compile_lists['average_baking']), max(
            ring_compile_lists['average_timed']))/10)*10+15,
        2))
ax_ring_bench.tick_params(axis='both', which='major', labelsize=20)
ax_ring_bench.yaxis.set_ticks(
    np.arange(
        int(min(min(ring_bench_lists['average_baking']), min(
            ring_bench_lists['average_timed']))/10)*10-5,
        int(max(max(ring_bench_lists['average_baking']), max(
            ring_bench_lists['average_timed']))/10)*10+3,
        2))

# Plot the graphs
# Mesh compile
# AMPST
if len(mesh_compile_lists['nb_participants_baking']) > 0:
    ax_mesh_compile.plot(
        mesh_compile_lists['nb_participants_baking'],
        mesh_compile_lists['average_baking'],
        label='AMPST',
        linestyle='solid',
        marker='o'
    )
# ATMP
if len(mesh_compile_lists['nb_participants_timed']) > 0:
    ax_mesh_compile.plot(
        mesh_compile_lists['nb_participants_timed'],
        mesh_compile_lists['average_timed'],
        label='ATMP',
        linestyle='solid',
        marker='*'
    )
# Mesh bench
# AMPST
if len(mesh_bench_lists['nb_participants_baking']) > 0:
    ax_mesh_bench.plot(
        mesh_bench_lists['nb_participants_baking'],
        mesh_bench_lists['average_baking'],
        label='AMPST',
        linestyle='solid',
        marker='o'
    )

# ATMP
if len(mesh_bench_lists['nb_participants_timed']) > 0:
    ax_mesh_bench.plot(
        mesh_bench_lists['nb_participants_timed'],
        mesh_bench_lists['average_timed'],
        label='ATMP',
        linestyle='solid',
        marker='*'
    )

# Ring compile
# AMPST
if len(ring_compile_lists['nb_participants_baking']) > 0:
    ax_ring_compile.plot(
        ring_compile_lists['nb_participants_baking'],
        ring_compile_lists['average_baking'],
        label='AMPST',
        linestyle='solid',
        marker='o'
    )

# ATMP
if len(ring_compile_lists['nb_participants_timed']) > 0:
    ax_ring_compile.plot(
        ring_compile_lists['nb_participants_timed'],
        ring_compile_lists['average_timed'],
        label='ATMP',
        linestyle='solid',
        marker='*'
    )

# Ring bench
# AMPST
if len(ring_bench_lists['nb_participants_baking']) > 0:
    ax_ring_bench.plot(
        ring_bench_lists['nb_participants_baking'],
        ring_bench_lists['average_baking'],
        label='AMPST',
        linestyle='solid',
        marker='o'
    )

# ATMP
if len(ring_bench_lists['nb_participants_timed']) > 0:
    ax_ring_bench.plot(
        ring_bench_lists['nb_participants_timed'],
        ring_bench_lists['average_timed'],
        label='ATMP',
        linestyle='solid',
        marker='*'
    )

# create the name for the new figure
index_graphs = 0
while os.path.isfile('results/graphs_mesh_ring_' + str(index_graphs) + '.pdf'):
    index_graphs += 1

name_graph = './results/graphs_mesh_ring_' + str(index_graphs) + '.pdf'

# Put a legend below current axis
ax_mesh_compile.set_ylabel('Time (s)', fontsize=30)
# ax_mesh_compile.legend(
#     loc='upper center',
#     bbox_to_anchor=(0.5, -0.3),
#     fancybox=True,
#     shadow=True,
#     ncol=2,
#     fontsize=20)
ax_mesh_bench.set_ylabel('Time (ms)', fontsize=30)

# adjust subplot position
# plt.subplots_adjust(hspace=1)
plt.tight_layout()

# save the figure
plt.savefig(name_graph)

# show the plot
plt.show()
