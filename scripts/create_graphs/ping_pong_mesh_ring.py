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
name_protocols = ['mesh', 'ring', 'ping']


def test(path):
    # Get the wanted data in the JSON file (field -> mean, field -> point_estimate)
    with open(criterion_path + '/' + path + json_path) as json_file:
        data = json.load(json_file)
        return data['mean']['point_estimate']


def ping_pong_bench():
    # Path for criterion
    main_path = './target/criterion'

    # Get all directories in main_path
    directories = os.listdir(main_path)

    # Relative path of the expected file
    path_file = '/base/estimates.json'

    # Lists for plots
    average_binary = []
    average_mpst = []
    average_baking = []
    average_crossbeam = []
    average_cancel = []
    average_broadcast_cancel = []

    nb_loops_binary = []
    nb_loops_mpst = []
    nb_loops_baking = []
    nb_loops_crossbeam = []
    nb_loops_cancel = []
    nb_loops_broadcast_cancel = []

    def test(path):
        # Get the wanted data in the JSON file (field -> mean, field -> point_estimate)
        with open(main_path + '/' + path + path_file) as json_file:
            data = json.load(json_file)
            return data['mean']['point_estimate']

    # For each folder in main_path
    for d in directories:
        # If name looks like the one from what we want
        if 'ping pong' in d:

            # Split the name
            splitted = d.split(' ')

            try:
                if 'binary' in d and 'cancel' not in d:
                    average_binary.append(int(test(d)))
                    nb_loops_binary.append(int(splitted[-1]))
                elif 'MPST' in d in d:
                    if 'baking' in d and 'inline' not in d:
                        average_baking.append(int(test(d)))
                        nb_loops_baking.append(int(splitted[-1]))
                    elif 'broadcast' in d:
                        average_broadcast_cancel.append(int(test(d)))
                        nb_loops_broadcast_cancel.append(int(splitted[-1]))
                    elif 'cancel' in d:
                        average_cancel.append(int(test(d)))
                        nb_loops_cancel.append(int(splitted[-1]))
                    else:
                        average_mpst.append(int(test(d)))
                        nb_loops_mpst.append(int(splitted[-1]))
                elif 'crossbeam' in d and 'cancel' not in d:
                    average_crossbeam.append(int(test(d)))
                    nb_loops_crossbeam.append(int(splitted[-1]))
            except:
                print("Missing ", d)

    # Sort the lists in pair
    if nb_loops_binary and average_binary:
        nb_loops_binary, binary = (list(t)
                                   for t in zip(*sorted(zip(nb_loops_binary, average_binary))))

    if nb_loops_baking and average_baking:
        nb_loops_baking, average_baking = (list(t)
                                           for t in zip(*sorted(zip(nb_loops_baking, average_baking))))

    if nb_loops_mpst and average_mpst:
        nb_loops_mpst, average_mpst = (list(t)
                                       for t in zip(*sorted(zip(nb_loops_mpst, average_mpst))))

    if nb_loops_crossbeam and average_crossbeam:
        nb_loops_crossbeam, average_crossbeam = (list(t) for t in zip(
            *sorted(zip(nb_loops_crossbeam, average_crossbeam))))

    if nb_loops_cancel and average_cancel:
        nb_loops_cancel, average_cancel = (list(t)
                                           for t in zip(*sorted(zip(nb_loops_cancel, average_cancel))))

    if nb_loops_broadcast_cancel and average_broadcast_cancel:
        nb_loops_broadcast_cancel, average_broadcast_cancel = (list(t)
                                                               for t in zip(*sorted(zip(nb_loops_broadcast_cancel, average_broadcast_cancel))))

    return {'nb_loops_baking': nb_loops_baking, 'average_baking': average_baking, 'nb_loops_mpst': nb_loops_mpst, 'average_mpst': average_mpst, 'nb_loops_binary': nb_loops_binary, 'average_binary': average_binary, 'nb_loops_crossbeam': nb_loops_crossbeam, 'average_crossbeam': average_crossbeam}


def ping_pong_compile():
    # Path for criterion
    main_path = './compile_time'

    # Get all directories in main_path
    directories = os.listdir(main_path)

    # Lists for plots
    average_mpst = []
    average_baking = []
    average_binary = []
    average_crossbeam = []
    average_cancel = []
    average_cancel_broadcast = []

    nb_loops_mpst = []
    nb_loops_baking = []
    nb_loops_binary = []
    nb_loops_crossbeam = []
    nb_loops_cancel = []
    nb_loops_cancel_broadcast = []

    serie = 'ping_pong'

    # For each folder in main_path
    for d in directories:

        if ".txt" in d and serie in d:

            file = open(main_path + '/' + d, "r")

            number = d.split('.txt')[
                0].split('_')[-1]

            build_time = []

            for line in file:
                if 'build' in line:
                    build_time.append(
                        int(line.split('build; ')[1].split('\n')[0]))

                    # If MPST of binary, append to related lists

            if 'mpst' in d:
                if 'baking' in d and 'inline' not in d:
                    average_baking.append(statistics.mean(build_time))
                    nb_loops_baking.append(int(number))
                else:
                    average_mpst.append(statistics.mean(build_time))
                    nb_loops_mpst.append(int(number))
            elif 'binary' in d:
                average_binary.append(statistics.mean(build_time))
                nb_loops_binary.append(int(number))
            elif 'cancel' in d:
                if 'broadcast' in d:
                    average_cancel_broadcast.append(
                        statistics.mean(build_time))
                    nb_loops_cancel_broadcast.append(int(number))
                else:
                    average_cancel.append(statistics.mean(build_time))
                    nb_loops_cancel.append(int(number))
            elif 'crossbeam' in d:
                average_crossbeam.append(statistics.mean(build_time))
                nb_loops_crossbeam.append(int(number))

            file.close()

    # Sort the lists in pair
    if nb_loops_mpst and average_mpst:
        nb_loops_mpst, average_mpst = (list(t) for t in zip(
            *sorted(zip(nb_loops_mpst, average_mpst))))

    if nb_loops_baking and average_baking:
        nb_loops_baking, average_baking = (list(t) for t in zip(
            *sorted(zip(nb_loops_baking, average_baking))))

    if nb_loops_binary and average_binary:
        nb_loops_binary, average_binary = (list(t)
                                           for t in zip(*sorted(zip(nb_loops_binary, average_binary))))

    if nb_loops_crossbeam and average_crossbeam:
        nb_loops_crossbeam, average_crossbeam = (list(t)
                                                 for t in zip(*sorted(zip(nb_loops_crossbeam, average_crossbeam))))

    if nb_loops_cancel and average_cancel:
        nb_loops_cancel, average_cancel = (list(t)
                                           for t in zip(*sorted(zip(nb_loops_cancel, average_cancel))))

    if nb_loops_cancel_broadcast and average_cancel_broadcast:
        nb_loops_cancel_broadcast, average_cancel_broadcast = (list(t)
                                                               for t in zip(*sorted(zip(nb_loops_cancel_broadcast, average_cancel_broadcast))))

    return {'nb_loops_baking': nb_loops_baking, 'average_baking': average_baking, 'nb_loops_mpst': nb_loops_mpst, 'average_mpst': average_mpst, 'nb_loops_binary': nb_loops_binary, 'average_binary': average_binary, 'nb_loops_crossbeam': nb_loops_crossbeam, 'average_crossbeam': average_crossbeam}


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
    average_binary = []
    average_mpst = []
    average_baking = []
    average_crossbeam = []
    average_cancel = []
    average_broadcast_cancel = []

    nb_participants_mpst = []
    nb_participants_baking = []
    nb_participants_binary = []
    nb_participants_crossbeam = []
    nb_participants_cancel = []
    nb_participants_broadcast_cancel = []

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
                # If MPST of binary, append to related lists
                if 'MPST' in d and str_to_int[splitted[1]] >= 2:
                    if 'baking' in d and 'inline' not in d:
                        average_baking.append(int(test(d)))
                        nb_participants_baking.append(str_to_int[splitted[1]])
                    elif 'broadcast' in d:
                        average_broadcast_cancel.append(int(test(d)))
                        nb_participants_broadcast_cancel.append(
                            str_to_int[splitted[1]])
                    elif 'cancel' in d:
                        average_cancel.append(int(test(d)))
                        nb_participants_cancel.append(str_to_int[splitted[1]])
                    else:
                        average_mpst.append(int(test(d)))
                        nb_participants_mpst.append(str_to_int[splitted[1]])
                elif 'binary' in d and str_to_int[splitted[1]] >= 2 and 'cancel' not in d:
                    average_binary.append(int(test(d)))
                    nb_participants_binary.append(str_to_int[splitted[1]])
                elif 'crossbeam' in d and str_to_int[splitted[1]] >= 2 and 'cancel' not in d:
                    average_crossbeam.append(int(test(d)))
                    nb_participants_crossbeam.append(str_to_int[splitted[1]])
            except:
                print("Missing ", d)

    # Sort the lists in pair
    if nb_participants_mpst and average_mpst:
        nb_participants_mpst, average_mpst = (list(t) for t in zip(
            *sorted(zip(nb_participants_mpst, average_mpst))))

    if nb_participants_baking and average_baking:
        nb_participants_baking, average_baking = (list(t) for t in zip(
            *sorted(zip(nb_participants_baking, average_baking))))

    if nb_participants_binary and average_binary:
        nb_participants_binary, average_binary = (list(t)
                                                  for t in zip(*sorted(zip(nb_participants_binary, average_binary))))

    if nb_participants_crossbeam and average_crossbeam:
        nb_participants_crossbeam, average_crossbeam = (list(t)
                                                        for t in zip(*sorted(zip(nb_participants_crossbeam, average_crossbeam))))

    if nb_participants_cancel and average_cancel:
        nb_participants_cancel, average_cancel = (list(t)
                                                  for t in zip(*sorted(zip(nb_participants_cancel, average_cancel))))

    if nb_participants_broadcast_cancel and average_broadcast_cancel:
        nb_participants_broadcast_cancel, average_broadcast_cancel = (list(t)
                                                                      for t in zip(*sorted(zip(nb_participants_broadcast_cancel, average_broadcast_cancel))))

    return {'nb_participants_baking': nb_participants_baking, 'average_baking': average_baking, 'nb_participants_mpst': nb_participants_mpst, 'average_mpst': average_mpst, 'nb_participants_binary': nb_participants_binary, 'average_binary': average_binary, 'nb_participants_crossbeam': nb_participants_crossbeam, 'average_crossbeam': average_crossbeam}


def mesh_compile():
    # Path for criterion
    main_path = './compile_time'

    # Get all directories in main_path
    directories = os.listdir(main_path)

    # Lists for plots
    average_mpst = []
    average_baking = []
    average_binary = []
    average_crossbeam = []
    average_cancel = []
    average_cancel_broadcast = []

    nb_participants_mpst = []
    nb_participants_baking = []
    nb_participants_binary = []
    nb_participants_crossbeam = []
    nb_participants_cancel = []
    nb_participants_cancel_broadcast = []

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
                if 'mpst' in d:
                    if 'baking' in d and 'inline' not in d:
                        average_baking.append(statistics.mean(build_time))
                        nb_participants_baking.append(str_to_int[name])
                    else:
                        average_mpst.append(statistics.mean(build_time))
                        nb_participants_mpst.append(str_to_int[name])
                elif 'binary' in d:
                    average_binary.append(statistics.mean(build_time))
                    nb_participants_binary.append(str_to_int[name])
                elif 'cancel' in d:
                    if 'broadcast' in d:
                        average_cancel_broadcast.append(
                            statistics.mean(build_time))
                        nb_participants_cancel_broadcast.append(
                            str_to_int[name])
                    else:
                        average_cancel.append(
                            statistics.mean(build_time))
                        nb_participants_cancel.append(str_to_int[name])
                elif 'crossbeam' in d:
                    average_crossbeam.append(statistics.mean(build_time))
                    nb_participants_crossbeam.append(str_to_int[name])
            except:
                print('Issue with ', d)

            file.close()

    # Sort the lists in pair
    if nb_participants_mpst and average_mpst:
        nb_participants_mpst, average_mpst = (list(t) for t in zip(
            *sorted(zip(nb_participants_mpst, average_mpst))))

    if nb_participants_baking and average_baking:
        nb_participants_baking, average_baking = (list(t) for t in zip(
            *sorted(zip(nb_participants_baking, average_baking))))

    if nb_participants_binary and average_binary:
        nb_participants_binary, average_binary = (list(t)
                                                  for t in zip(*sorted(zip(nb_participants_binary, average_binary))))

    if nb_participants_crossbeam and average_crossbeam:
        nb_participants_crossbeam, average_crossbeam = (list(t)
                                                        for t in zip(*sorted(zip(nb_participants_crossbeam, average_crossbeam))))

    if nb_participants_cancel and average_cancel:
        nb_participants_cancel, average_cancel = (list(t)
                                                  for t in zip(*sorted(zip(nb_participants_cancel, average_cancel))))

    if nb_participants_cancel_broadcast and average_cancel_broadcast:
        nb_participants_cancel_broadcast, average_cancel_broadcast = (list(t)
                                                                      for t in zip(*sorted(zip(nb_participants_cancel_broadcast, average_cancel_broadcast))))

    return {'nb_participants_baking': nb_participants_baking, 'average_baking': average_baking, 'nb_participants_mpst': nb_participants_mpst, 'average_mpst': average_mpst, 'nb_participants_binary': nb_participants_binary, 'average_binary': average_binary, 'nb_participants_crossbeam': nb_participants_crossbeam, 'average_crossbeam': average_crossbeam}


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
    average_binary = []
    average_mpst = []
    average_baking = []
    average_crossbeam = []
    average_cancel = []
    average_broadcast_cancel = []

    nb_participants_mpst = []
    nb_participants_baking = []
    nb_participants_binary = []
    nb_participants_crossbeam = []
    nb_participants_cancel = []
    nb_participants_broadcast_cancel = []

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
                # If MPST of binary, append to related lists
                if 'MPST' in d and str_to_int[splitted[1]] >= 2:
                    if 'baking' in d and 'inline' not in d:
                        average_baking.append(int(test(d)))
                        nb_participants_baking.append(str_to_int[splitted[1]])
                    elif 'broadcast' in d:
                        average_broadcast_cancel.append(int(test(d)))
                        nb_participants_broadcast_cancel.append(
                            str_to_int[splitted[1]])
                    elif 'cancel' in d:
                        average_cancel.append(int(test(d)))
                        nb_participants_cancel.append(str_to_int[splitted[1]])
                    else:
                        average_mpst.append(int(test(d)))
                        nb_participants_mpst.append(str_to_int[splitted[1]])
                elif 'binary' in d and str_to_int[splitted[1]] >= 2 and 'cancel' not in d:
                    average_binary.append(int(test(d)))
                    nb_participants_binary.append(str_to_int[splitted[1]])
                elif 'crossbeam' in d and str_to_int[splitted[1]] >= 2 and 'cancel' not in d:
                    average_crossbeam.append(int(test(d)))
                    nb_participants_crossbeam.append(str_to_int[splitted[1]])
            except:
                print("Missing ", d)

    # Sort the lists in pair
    if nb_participants_mpst and average_mpst:
        nb_participants_mpst, average_mpst = (list(t) for t in zip(
            *sorted(zip(nb_participants_mpst, average_mpst))))

    if nb_participants_baking and average_baking:
        nb_participants_baking, average_baking = (list(t) for t in zip(
            *sorted(zip(nb_participants_baking, average_baking))))

    if nb_participants_binary and average_binary:
        nb_participants_binary, average_binary = (list(t)
                                                  for t in zip(*sorted(zip(nb_participants_binary, average_binary))))

    if nb_participants_crossbeam and average_crossbeam:
        nb_participants_crossbeam, average_crossbeam = (list(t)
                                                        for t in zip(*sorted(zip(nb_participants_crossbeam, average_crossbeam))))

    if nb_participants_cancel and average_cancel:
        nb_participants_cancel, average_cancel = (list(t)
                                                  for t in zip(*sorted(zip(nb_participants_cancel, average_cancel))))

    if nb_participants_broadcast_cancel and average_broadcast_cancel:
        nb_participants_broadcast_cancel, average_broadcast_cancel = (list(t)
                                                                      for t in zip(*sorted(zip(nb_participants_broadcast_cancel, average_broadcast_cancel))))

    return {'nb_participants_baking': nb_participants_baking, 'average_baking': average_baking, 'nb_participants_mpst': nb_participants_mpst, 'average_mpst': average_mpst, 'nb_participants_binary': nb_participants_binary, 'average_binary': average_binary, 'nb_participants_crossbeam': nb_participants_crossbeam, 'average_crossbeam': average_crossbeam}


def ring_compile():
    # Path for criterion
    main_path = './compile_time'

    # Get all directories in main_path
    directories = os.listdir(main_path)

    # Lists for plots
    average_mpst = []
    average_baking = []
    average_binary = []
    average_crossbeam = []
    average_cancel = []
    average_cancel_broadcast = []

    nb_participants_mpst = []
    nb_participants_baking = []
    nb_participants_binary = []
    nb_participants_crossbeam = []
    nb_participants_cancel = []
    nb_participants_cancel_broadcast = []

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
                if 'mpst' in d:
                    if 'baking' in d and 'inline' not in d:
                        average_mpst.append(statistics.mean(build_time))
                        nb_participants_mpst.append(str_to_int[name])
                    else:
                        average_baking.append(
                            statistics.mean(build_time))
                        nb_participants_baking.append(str_to_int[name])
                elif 'binary' in d:
                    average_binary.append(statistics.mean(build_time))
                    nb_participants_binary.append(str_to_int[name])
                elif 'cancel' in d:
                    if 'broadcast' in d:
                        average_cancel_broadcast.append(
                            statistics.mean(build_time))
                        nb_participants_cancel_broadcast.append(
                            str_to_int[name])
                    else:
                        average_cancel.append(
                            statistics.mean(build_time))
                        nb_participants_cancel.append(str_to_int[name])
                elif 'crossbeam' in d:
                    average_crossbeam.append(statistics.mean(build_time))
                    nb_participants_crossbeam.append(str_to_int[name])
            except:
                print('Issue with ', d)

            file.close()

    # Sort the lists in pair
    if nb_participants_mpst and average_mpst:
        nb_participants_mpst, average_mpst = (list(t) for t in zip(
            *sorted(zip(nb_participants_mpst, average_mpst))))

    if nb_participants_baking and average_baking:
        nb_participants_baking, average_baking = (list(t) for t in zip(
            *sorted(zip(nb_participants_baking, average_baking))))

    if nb_participants_binary and average_binary:
        nb_participants_binary, average_binary = (list(t)
                                                  for t in zip(*sorted(zip(nb_participants_binary, average_binary))))

    if nb_participants_crossbeam and average_crossbeam:
        nb_participants_crossbeam, average_crossbeam = (list(t)
                                                        for t in zip(*sorted(zip(nb_participants_crossbeam, average_crossbeam))))

    if nb_participants_cancel and average_cancel:
        nb_participants_cancel, average_cancel = (list(t)
                                                  for t in zip(*sorted(zip(nb_participants_cancel, average_cancel))))

    if nb_participants_cancel_broadcast and average_cancel_broadcast:
        nb_participants_cancel_broadcast, average_cancel_broadcast = (list(t)
                                                                      for t in zip(*sorted(zip(nb_participants_cancel_broadcast, average_cancel_broadcast))))

    return {'nb_participants_baking': nb_participants_baking, 'average_baking': average_baking, 'nb_participants_mpst': nb_participants_mpst, 'average_mpst': average_mpst, 'nb_participants_binary': nb_participants_binary, 'average_binary': average_binary, 'nb_participants_crossbeam': nb_participants_crossbeam, 'average_crossbeam': average_crossbeam}


# Get index of new file
index_mesh = 0
while os.path.isfile('results/mesh_' + str(index_mesh) + '.csv'):
    index_mesh += 1
index_ring = 0
while os.path.isfile('results/ring_' + str(index_ring) + '.csv'):
    index_ring += 1
index_ping_pong = 0
while os.path.isfile('results/ping_pong_' + str(index_ping_pong) + '.csv'):
    index_ping_pong += 1

# Get the dictionaries with the results
ping_pong_bench_lists = ping_pong_bench()
ping_pong_compile_lists = ping_pong_compile()
mesh_bench_lists = mesh_bench()
mesh_compile_lists = mesh_compile()
ring_bench_lists = ring_bench()
ring_compile_lists = ring_compile()

# Create the correct str for the new files
result_mesh_file = 'mesh_' + str(index_mesh) + '.csv'
result_ring_file = 'ring_' + str(index_ring) + '.csv'
result_ping_pong_file = 'ping_pong_' + str(index_ping_pong) + '.csv'

# Open the file in the folder and append each block of type of protocol, in the order: crossbeam, binary, mpst and ampst
# If they don't exist, skip
with open(result_folder / result_mesh_file, 'a') as report_file:
    for i, val in enumerate(mesh_bench_lists['nb_participants_crossbeam']):
        report_file.write('crossbeam')
        report_file.write('; ')
        report_file.write(str(val))
        report_file.write('; ')
        if i < len(mesh_bench_lists['average_crossbeam']):
            report_file.write(str(mesh_bench_lists['average_crossbeam'][i]))
        report_file.write('; ')
        if i < len(mesh_compile_lists['average_crossbeam']):
            report_file.write(str(mesh_compile_lists['average_crossbeam'][i]))
        report_file.write('\n')
    report_file.write('\n')
    for i, val in enumerate(mesh_bench_lists['nb_participants_binary']):
        report_file.write('binary')
        report_file.write('; ')
        report_file.write(str(val))
        report_file.write('; ')
        if i < len(mesh_bench_lists['average_binary']):
            report_file.write(str(mesh_bench_lists['average_binary'][i]))
        report_file.write('; ')
        if i < len(mesh_compile_lists['average_binary']):
            report_file.write(str(mesh_compile_lists['average_binary'][i]))
        report_file.write('\n')
    report_file.write('\n')
    for i, val in enumerate(mesh_bench_lists['nb_participants_mpst']):
        report_file.write('mpst')
        report_file.write('; ')
        report_file.write(str(val))
        report_file.write('; ')
        if i < len(mesh_bench_lists['average_mpst']):
            report_file.write(str(mesh_bench_lists['average_mpst'][i]))
        report_file.write('; ')
        if i < len(mesh_compile_lists['average_mpst']):
            report_file.write(str(mesh_compile_lists['average_mpst'][i]))
        report_file.write('\n')
    report_file.write('\n')
    for i, val in enumerate(mesh_bench_lists['nb_participants_baking']):
        report_file.write('ampst')
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


with open('results/ring_' + str(index_ring) + '.csv', 'a') as report_file:
    for i, val in enumerate(ring_bench_lists['nb_participants_crossbeam']):
        report_file.write('crossbeam')
        report_file.write('; ')
        report_file.write(str(val))
        report_file.write('; ')
        if i < len(ring_bench_lists['average_crossbeam']):
            report_file.write(str(ring_bench_lists['average_crossbeam'][i]))
        report_file.write('; ')
        if i < len(ring_compile_lists['average_crossbeam']):
            report_file.write(str(ring_compile_lists['average_crossbeam'][i]))
        report_file.write('\n')
    report_file.write('\n')
    for i, val in enumerate(ring_bench_lists['nb_participants_binary']):
        report_file.write('binary')
        report_file.write('; ')
        report_file.write(str(val))
        report_file.write('; ')
        if i < len(ring_bench_lists['average_binary']):
            report_file.write(str(ring_bench_lists['average_binary'][i]))
        report_file.write('; ')
        if i < len(ring_compile_lists['average_binary']):
            report_file.write(str(ring_compile_lists['average_binary'][i]))
        report_file.write('\n')
    report_file.write('\n')
    for i, val in enumerate(ring_bench_lists['nb_participants_mpst']):
        report_file.write('mpst')
        report_file.write('; ')
        report_file.write(str(val))
        report_file.write('; ')
        if i < len(ring_bench_lists['average_mpst']):
            report_file.write(str(ring_bench_lists['average_mpst'][i]))
        report_file.write('; ')
        if i < len(ring_compile_lists['average_mpst']):
            report_file.write(str(ring_compile_lists['average_mpst'][i]))
        report_file.write('\n')
    report_file.write('\n')
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

with open('results/ping_pong_' + str(index_ping_pong) + '.csv', 'a') as report_file:
    for i, val in enumerate(ping_pong_bench_lists['nb_loops_crossbeam']):
        report_file.write('crossbeam')
        report_file.write('; ')
        report_file.write(str(val))
        report_file.write('; ')
        if i < len(ping_pong_bench_lists['average_crossbeam']):
            report_file.write(
                str(ping_pong_bench_lists['average_crossbeam'][i]))
        report_file.write('; ')
        if i < len(ping_pong_compile_lists['average_crossbeam']):
            report_file.write(
                str(ping_pong_compile_lists['average_crossbeam'][i]))
        report_file.write('\n')
    report_file.write('\n')
    for i, val in enumerate(ping_pong_bench_lists['nb_loops_binary']):
        report_file.write('binary')
        report_file.write('; ')
        report_file.write(str(val))
        report_file.write('; ')
        if i < len(ping_pong_bench_lists['average_binary']):
            report_file.write(str(ping_pong_bench_lists['average_binary'][i]))
        report_file.write('; ')
        if i < len(ping_pong_compile_lists['average_binary']):
            report_file.write(
                str(ping_pong_compile_lists['average_binary'][i]))
        report_file.write('\n')
    report_file.write('\n')
    for i, val in enumerate(ping_pong_bench_lists['nb_loops_mpst']):
        report_file.write('mpst')
        report_file.write('; ')
        report_file.write(str(val))
        report_file.write('; ')
        if i < len(ping_pong_bench_lists['average_mpst']):
            report_file.write(str(ping_pong_bench_lists['average_mpst'][i]))
        report_file.write('; ')
        if i < len(ping_pong_compile_lists['average_mpst']):
            report_file.write(str(ping_pong_compile_lists['average_mpst'][i]))
        report_file.write('\n')
    report_file.write('\n')
    for i, val in enumerate(ping_pong_bench_lists['nb_loops_baking']):
        report_file.write('ampst')
        report_file.write('; ')
        report_file.write(str(val))
        report_file.write('; ')
        if i < len(ping_pong_compile_lists['average_baking']):
            report_file.write(
                str(ping_pong_compile_lists['average_baking'][i]))
        report_file.write('; ')
        if i < len(ping_pong_compile_lists['average_baking']):
            report_file.write(
                str(ping_pong_compile_lists['average_baking'][i]))
        report_file.write('\n')
    report_file.write('\n')


# Create figures
all_graphs = plt.figure()
ax_mesh_compile = all_graphs.add_subplot(3, 2, 1)
ax_mesh_bench = all_graphs.add_subplot(3, 2, 2)
ax_ring_compile = all_graphs.add_subplot(3, 2, 3)
ax_ring_bench = all_graphs.add_subplot(3, 2, 4)
# ax_ping_pong_compile = all_graphs.add_subplot(3,2,5)
ax_ping_pong_bench = all_graphs.add_subplot(3, 2, 6)

# adjust subplot position
plt.subplots_adjust(hspace=1)

# Set title
ax_mesh_compile.set_title("Mesh compilation time")
ax_mesh_bench.set_title("Mesh bench time")
ax_ring_compile.set_title("Ring compilation time")
ax_ring_bench.set_title("Ring bench time")
# ax_ping_pong_compile.set_title("Ping pong compilation time")
ax_ping_pong_bench.set_title("Ping pong bench time")

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
# # Ping_pong compile
# ax_ping_pong_compile.xaxis.set_major_locator(MaxNLocator(integer=True))
# ax_ping_pong_compile.yaxis.set_major_locator(MaxNLocator(integer=True))
# Ping_pong bench
ax_ping_pong_bench.xaxis.set_major_locator(MaxNLocator(integer=True))
ax_ping_pong_bench.yaxis.set_major_locator(MaxNLocator(integer=True))

# Set the axis titles
ax_mesh_compile.set_xlabel('\# roles')
ax_mesh_bench.set_xlabel('\# roles')
ax_ring_compile.set_xlabel('\# roles')
ax_ring_bench.set_xlabel('\# roles')
# ax_ping_pong_compile.set_xlabel('\# loops')
ax_ping_pong_bench.set_xlabel('\# loops')

# Set ticks parameters to major/both
ax_mesh_compile.tick_params(axis='both', which='major')
ax_mesh_bench.tick_params(axis='both', which='major')
ax_ring_compile.tick_params(axis='both', which='major')
ax_ring_bench.tick_params(axis='both', which='major')
# ax_ping_pong_compile.tick_params(axis='both', which='major')
ax_ping_pong_bench.tick_params(axis='both', which='major')

# Plot the graphs
# Mesh compile
# Crossbeam
if len(mesh_compile_lists['nb_participants_crossbeam']) > 0:
    ax_mesh_compile.plot(mesh_compile_lists['nb_participants_crossbeam'], mesh_compile_lists['average_crossbeam'], label='Crossbeam',
                         linestyle='solid', marker='>')

# Binary
if len(mesh_compile_lists['nb_participants_binary']) > 0:
    ax_mesh_compile.plot(mesh_compile_lists['nb_participants_binary'], mesh_compile_lists['average_binary'], label='Binary',
                         linestyle='solid', marker='o')

# MPST
if len(mesh_compile_lists['nb_participants_mpst']) > 0:
    ax_mesh_compile.plot(mesh_compile_lists['nb_participants_mpst'], mesh_compile_lists['average_mpst'], label='MPST',
                         linestyle='solid', marker='d')

# AMPST
if len(mesh_compile_lists['nb_participants_baking']) > 0:
    ax_mesh_compile.plot(mesh_compile_lists['nb_participants_baking'], mesh_compile_lists['average_baking'], label='AMPST',
                         linestyle='solid', marker='*')
# Mesh bench
# Crossbeam
if len(mesh_bench_lists['nb_participants_crossbeam']) > 0:
    ax_mesh_bench.plot(mesh_bench_lists['nb_participants_crossbeam'], mesh_bench_lists['average_crossbeam'], label='Crossbeam',
                       linestyle='solid', marker='>')

# Binary
if len(mesh_bench_lists['nb_participants_binary']) > 0:
    ax_mesh_bench.plot(mesh_bench_lists['nb_participants_binary'], mesh_bench_lists['average_binary'], label='Binary',
                       linestyle='solid', marker='o')

# MPST
if len(mesh_bench_lists['nb_participants_mpst']) > 0:
    ax_mesh_bench.plot(mesh_bench_lists['nb_participants_mpst'], mesh_bench_lists['average_mpst'], label='MPST',
                       linestyle='solid', marker='d')

# AMPST
if len(mesh_bench_lists['nb_participants_baking']) > 0:
    ax_mesh_bench.plot(mesh_bench_lists['nb_participants_baking'], mesh_bench_lists['average_baking'], label='AMPST',
                       linestyle='solid', marker='*')

# Ring compile
# Crossbeam
if len(ring_compile_lists['nb_participants_crossbeam']) > 0:
    ax_ring_compile.plot(ring_compile_lists['nb_participants_crossbeam'], ring_compile_lists['average_crossbeam'], label='Crossbeam',
                         linestyle='solid', marker='>')

# Binary
if len(ring_compile_lists['nb_participants_binary']) > 0:
    ax_ring_compile.plot(ring_compile_lists['nb_participants_binary'], ring_compile_lists['average_binary'], label='Binary',
                         linestyle='solid', marker='o')

# MPST
if len(ring_compile_lists['nb_participants_mpst']) > 0:
    ax_ring_compile.plot(ring_compile_lists['nb_participants_mpst'], ring_compile_lists['average_mpst'], label='MPST',
                         linestyle='solid', marker='d')

# AMPST
if len(ring_compile_lists['nb_participants_baking']) > 0:
    ax_ring_compile.plot(ring_compile_lists['nb_participants_baking'], ring_compile_lists['average_baking'], label='AMPST',
                         linestyle='solid', marker='*')

# Ring bench
# Crossbeam
if len(ring_bench_lists['nb_participants_crossbeam']) > 0:
    ax_ring_bench.plot(ring_bench_lists['nb_participants_crossbeam'], ring_bench_lists['average_crossbeam'], label='Crossbeam',
                       linestyle='solid', marker='>')

# Binary
if len(ring_bench_lists['nb_participants_binary']) > 0:
    ax_ring_bench.plot(ring_bench_lists['nb_participants_binary'], ring_bench_lists['average_binary'], label='Binary',
                       linestyle='solid', marker='o')

# MPST
if len(ring_bench_lists['nb_participants_mpst']) > 0:
    ax_ring_bench.plot(ring_bench_lists['nb_participants_mpst'], ring_bench_lists['average_mpst'], label='MPST',
                       linestyle='solid', marker='d')

# AMPST
if len(ring_bench_lists['nb_participants_baking']) > 0:
    ax_ring_bench.plot(ring_bench_lists['nb_participants_baking'], ring_bench_lists['average_baking'], label='AMPST',
                       linestyle='solid', marker='*')

# # Ping_pong compile
# # Crossbeam
# if len(ping_pong_compile_lists['nb_loops_crossbeam']) > 0:
#     ax_ping_pong_compile.plot(ping_pong_compile_lists['nb_loops_crossbeam'], ping_pong_compile_lists['average_crossbeam'], label='Crossbeam',
#                               linestyle='solid', marker='>')

# # Binary
# if len(ping_pong_compile_lists['nb_loops_binary']) > 0:
#     ax_ping_pong_compile.plot(ping_pong_compile_lists['nb_loops_binary'], ping_pong_compile_lists['average_binary'], label='Binary',
#                               linestyle='solid', marker='o')

# # MPST
# if len(ping_pong_compile_lists['nb_loops_mpst']) > 0:
#     ax_ping_pong_compile.plot(ping_pong_compile_lists['nb_loops_mpst'], ping_pong_compile_lists['average_mpst'], label='MPST',
#                               linestyle='solid', marker='d')

# # AMPST
# if len(ping_pong_compile_lists['nb_loops_baking']) > 0:
#     ax_ping_pong_compile.plot(ping_pong_compile_lists['nb_loops_baking'], ping_pong_compile_lists['average_baking'], label='AMPST',
#                               linestyle='solid', marker='*')

# Ping_pong bench
# Crossbeam
if len(ping_pong_bench_lists['nb_loops_crossbeam']) > 0:
    ax_ping_pong_bench.plot(ping_pong_bench_lists['nb_loops_crossbeam'], ping_pong_bench_lists['average_crossbeam'], label='Crossbeam',
                            linestyle='solid', marker='>')

# Binary
if len(ping_pong_bench_lists['nb_loops_binary']) > 0:
    ax_ping_pong_bench.plot(ping_pong_bench_lists['nb_loops_binary'], ping_pong_bench_lists['average_binary'], label='Binary',
                            linestyle='solid', marker='o')

# MPST
if len(ping_pong_bench_lists['nb_loops_mpst']) > 0:
    ax_ping_pong_bench.plot(ping_pong_bench_lists['nb_loops_mpst'], ping_pong_bench_lists['average_mpst'], label='MPST',
                            linestyle='solid', marker='d')

# AMPST
if len(ping_pong_bench_lists['nb_loops_baking']) > 0:
    ax_ping_pong_bench.plot(ping_pong_bench_lists['nb_loops_baking'], ping_pong_bench_lists['average_baking'], label='AMPST',
                            linestyle='solid', marker='*')

# Shrink current axis's height by 10% on the bottom
box = ax_ping_pong_bench.get_position()
ax_ping_pong_bench.set_position(
    [box.x0, box.y0 + box.height * 0.1, box.width, box.height * 0.9])

# Put a legend below current axis
ax_ping_pong_bench.legend(loc='upper center', bbox_to_anchor=(
    0.5, -0.5), fancybox=True, shadow=True, ncol=5)

# create the name for the new figure
index_graphs = 0
while os.path.isfile('results/graphs_' + str(index_graphs) + '.pdf'):
    index_graphs += 1

name_graph = './results/graphs_' + str(index_graphs) + '.pdf'

# save the figure
plt.savefig(name_graph)

# show the plot
plt.show()
