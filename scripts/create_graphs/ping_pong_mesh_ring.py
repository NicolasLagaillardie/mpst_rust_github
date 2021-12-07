from matplotlib.ticker import MaxNLocator
import matplotlib.pyplot as plt
import json
import os
import matplotlib
import numpy as np
import statistics
import os.path
matplotlib.rcParams['text.usetex'] = True

# Path for criterion
criterion_path = './target/criterion'

# Path for compile-time
compile_path = 'compile_time'

# Get all directories in criterion_path
criterion_directories = os.listdir(criterion_path)

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
    binary = []
    mpst = []
    crossbeam = []
    cancel = []
    broadcast_cancel = []

    nb_loops_binary = []
    nb_loops_mpst = []
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
                    binary.append(int(test(d))/10**6)
                    nb_loops_binary.append(int(splitted[-1]))
                elif 'MPST' in d and 'baking' in d:
                    if 'broadcast' in d:
                        broadcast_cancel.append(int(test(d))/10**6)
                        nb_loops_broadcast_cancel.append(int(splitted[-1]))
                    elif 'cancel' in d:
                        cancel.append(int(test(d))/10**6)
                        nb_loops_cancel.append(int(splitted[-1]))
                    else:
                        mpst.append(int(test(d))/10**6)
                        nb_loops_mpst.append(int(splitted[-1]))
                elif 'crossbeam' in d and 'cancel' not in d:
                    crossbeam.append(int(test(d))/10**6)
                    nb_loops_crossbeam.append(int(splitted[-1]))
            except:
                print("Missing ", d)

    # Sort the lists in pair
    nb_loops_binary, binary = (list(t)
                               for t in zip(*sorted(zip(nb_loops_binary, binary))))

    nb_loops_mpst, mpst = (list(t)
                           for t in zip(*sorted(zip(nb_loops_mpst, mpst))))

    nb_loops_crossbeam, crossbeam = (list(t) for t in zip(
        *sorted(zip(nb_loops_crossbeam, crossbeam))))

    nb_loops_cancel, cancel = (list(t)
                               for t in zip(*sorted(zip(nb_loops_cancel, cancel))))

    nb_loops_broadcast_cancel, broadcast_cancel = (list(t)
                                                   for t in zip(*sorted(zip(nb_loops_broadcast_cancel, broadcast_cancel))))

    return {'nb_loops_mpst': nb_loops_mpst, 'average_mpst': mpst, 'nb_loops_binary': nb_loops_binary, 'average_binary': binary, 'nb_loops_crossbeam': nb_loops_crossbeam, 'average_crossbeam': crossbeam}


def ping_pong_compile():
    # Path for criterion
    main_path = './compile_time'

    # Get all directories in main_path
    directories = os.listdir(main_path)

    # Lists for plots
    average_mpst = []
    average_binary = []
    average_crossbeam = []
    average_cancel = []
    average_cancel_broadcast = []

    nb_participants_mpst = []
    nb_participants_binary = []
    nb_participants_crossbeam = []
    nb_participants_cancel = []
    nb_participants_cancel_broadcast = []

    # # Dictionary for converting from string to int
    # str_to_int = {'three': 3, 'four': 4, 'five': 5, 'six': 6, 'seven': 7,
    #               'eight': 8, 'nine': 9, 'ten': 10, 'eleven': 11, 'twenty': 20, 'empty': 0}

    serie = 'ping_pong'

    # For each folder in main_path
    for d in directories:

        if ".txt" in d and serie in d:

            file = open(main_path + '/' + d, "r")

            (name, number) = d.split('.txt')[
                0].split(serie + '_')[1].split('_')

            build_time = []

            for line in file:
                if 'build' in line:
                    build_time.append(
                        int(line.split('build; ')[1].split('\n')[0]))

                    # If MPST of binary, append to related lists

            if 'mpst' in d and 'baking' in d:
                average_mpst.append(statistics.mean(build_time))
                nb_participants_mpst.append(int(number))
            elif 'binary' in d:
                average_binary.append(statistics.mean(build_time))
                nb_participants_binary.append(int(number))
            elif 'cancel' in d:
                if 'broadcast' in d:
                    average_cancel_broadcast.append(
                        statistics.mean(build_time))
                    nb_participants_cancel_broadcast.append(int(number))
                else:
                    average_cancel.append(statistics.mean(build_time))
                    nb_participants_cancel.append(int(number))
            elif 'crossbeam' in d:
                average_crossbeam.append(statistics.mean(build_time))
                nb_participants_crossbeam.append(int(number))

            file.close()

    # Sort the lists in pair
    nb_participants_mpst, average_mpst = (list(t) for t in zip(
        *sorted(zip(nb_participants_mpst, average_mpst))))

    nb_participants_binary, average_binary = (list(t)
                                              for t in zip(*sorted(zip(nb_participants_binary, average_binary))))

    nb_participants_crossbeam, average_crossbeam = (list(t)
                                                    for t in zip(*sorted(zip(nb_participants_crossbeam, average_crossbeam))))

    nb_participants_cancel, average_cancel = (list(t)
                                              for t in zip(*sorted(zip(nb_participants_cancel, average_cancel))))

    nb_participants_cancel_broadcast, average_cancel_broadcast = (list(t)
                                                                  for t in zip(*sorted(zip(nb_participants_cancel_broadcast, average_cancel_broadcast))))

    return {'nb_participants_mpst': nb_participants_mpst, 'average_mpst': average_mpst, 'nb_participants_binary': nb_participants_binary, 'average_binary': average_binary, 'nb_participants_crossbeam': nb_participants_crossbeam, 'average_crossbeam': average_crossbeam}


def ping_pong():
    ping_pong_bench()
    ping_pong_compile()


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
    binary = []
    mpst = []
    crossbeam = []
    cancel = []
    broadcast_cancel = []

    nb_participants_mpst = []
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
                if 'MPST' in d and 'baking' in d and str_to_int[splitted[1]] >= 2:
                    if 'broadcast' in d:
                        broadcast_cancel.append(int(test(d))/10**6)
                        nb_participants_broadcast_cancel.append(
                            str_to_int[splitted[1]])
                    elif 'cancel' in d:
                        cancel.append(int(test(d))/10**6)
                        nb_participants_cancel.append(str_to_int[splitted[1]])
                    else:
                        mpst.append(int(test(d))/10**6)
                        nb_participants_mpst.append(str_to_int[splitted[1]])
                elif 'binary' in d and str_to_int[splitted[1]] >= 2 and 'cancel' not in d:
                    binary.append(int(test(d))/10**6)
                    nb_participants_binary.append(str_to_int[splitted[1]])
                elif 'crossbeam' in d and str_to_int[splitted[1]] >= 2 and 'cancel' not in d:
                    crossbeam.append(int(test(d))/10**6)
                    nb_participants_crossbeam.append(str_to_int[splitted[1]])
            except:
                print("Missing ", d)

    # Sort the lists in pair
    nb_participants_mpst, mpst = (list(t) for t in zip(
        *sorted(zip(nb_participants_mpst, mpst))))

    nb_participants_binary, binary = (list(t)
                                      for t in zip(*sorted(zip(nb_participants_binary, binary))))

    nb_participants_crossbeam, crossbeam = (list(t)
                                            for t in zip(*sorted(zip(nb_participants_crossbeam, crossbeam))))

    if len(cancel) > 0:
        nb_participants_cancel, cancel = (list(t)
                                          for t in zip(*sorted(zip(nb_participants_cancel, cancel))))

    if len(broadcast_cancel) > 0:
        nb_participants_broadcast_cancel, broadcast_cancel = (list(t)
                                                              for t in zip(*sorted(zip(nb_participants_broadcast_cancel, broadcast_cancel))))

    return {'nb_participants_mpst': nb_participants_mpst, 'average_mpst': mpst, 'nb_participants_binary': nb_participants_binary, 'average_binary': binary, 'nb_participants_crossbeam': nb_participants_crossbeam, 'average_crossbeam': crossbeam}


def mesh_compile():
    # Path for criterion
    main_path = './compile_time'

    # Get all directories in main_path
    directories = os.listdir(main_path)

    # Lists for plots
    average_mpst = []
    average_binary = []
    average_crossbeam = []
    average_cancel = []
    average_cancel_broadcast = []

    nb_participants_mpst = []
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
                if 'mpst' in d and 'baking' in d:
                    average_mpst.append(statistics.mean(build_time)/10**6)
                    nb_participants_mpst.append(str_to_int[name])
                elif 'binary' in d:
                    average_binary.append(statistics.mean(build_time)/10**6)
                    nb_participants_binary.append(str_to_int[name])
                elif 'cancel' in d:
                    if 'broadcast' in d:
                        average_cancel_broadcast.append(
                            statistics.mean(build_time)/10**6)
                        nb_participants_cancel_broadcast.append(
                            str_to_int[name])
                    else:
                        average_cancel.append(
                            statistics.mean(build_time)/10**6)
                        nb_participants_cancel.append(str_to_int[name])
                elif 'crossbeam' in d:
                    average_crossbeam.append(statistics.mean(build_time)/10**6)
                    nb_participants_crossbeam.append(str_to_int[name])
            except:
                print('Issue with ', d)

            file.close()

    # Sort the lists in pair
    nb_participants_mpst, average_mpst = (list(t) for t in zip(
        *sorted(zip(nb_participants_mpst, average_mpst))))

    nb_participants_binary, average_binary = (list(t)
                                              for t in zip(*sorted(zip(nb_participants_binary, average_binary))))

    nb_participants_crossbeam, average_crossbeam = (list(t)
                                                    for t in zip(*sorted(zip(nb_participants_crossbeam, average_crossbeam))))

    nb_participants_cancel, average_cancel = (list(t)
                                              for t in zip(*sorted(zip(nb_participants_cancel, average_cancel))))

    nb_participants_cancel_broadcast, average_cancel_broadcast = (list(t)
                                                                  for t in zip(*sorted(zip(nb_participants_cancel_broadcast, average_cancel_broadcast))))

    return {'nb_participants_mpst': nb_participants_mpst, 'average_mpst': average_mpst, 'nb_participants_binary': nb_participants_binary, 'average_binary': average_binary, 'nb_participants_crossbeam': nb_participants_crossbeam, 'average_crossbeam': average_crossbeam}


def mesh():
    mesh_bench()
    mesh_compile()


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
    binary = []
    mpst = []
    crossbeam = []
    cancel = []
    broadcast_cancel = []

    nb_participants_mpst = []
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
                if 'MPST' in d and 'baking' in d and str_to_int[splitted[1]] >= 2:
                    if 'broadcast' in d:
                        broadcast_cancel.append(int(test(d))/10**6)
                        nb_participants_broadcast_cancel.append(
                            str_to_int[splitted[1]])
                    elif 'cancel' in d:
                        cancel.append(int(test(d))/10**6)
                        nb_participants_cancel.append(str_to_int[splitted[1]])
                    else:
                        mpst.append(int(test(d))/10**6)
                        nb_participants_mpst.append(str_to_int[splitted[1]])
                elif 'binary' in d and str_to_int[splitted[1]] >= 2 and 'cancel' not in d:
                    binary.append(int(test(d))/10**6)
                    nb_participants_binary.append(str_to_int[splitted[1]])
                elif 'crossbeam' in d and str_to_int[splitted[1]] >= 2 and 'cancel' not in d:
                    crossbeam.append(int(test(d))/10**6)
                    nb_participants_crossbeam.append(str_to_int[splitted[1]])
            except:
                print("Missing ", d)

    # Sort the lists in pair
    nb_participants_mpst, mpst = (list(t) for t in zip(
        *sorted(zip(nb_participants_mpst, mpst))))

    nb_participants_binary, binary = (list(t)
                                      for t in zip(*sorted(zip(nb_participants_binary, binary))))

    nb_participants_crossbeam, crossbeam = (list(t)
                                            for t in zip(*sorted(zip(nb_participants_crossbeam, crossbeam))))

    if len(cancel) > 0:
        nb_participants_cancel, cancel = (list(t)
                                          for t in zip(*sorted(zip(nb_participants_cancel, cancel))))

    if len(broadcast_cancel) > 0:
        nb_participants_broadcast_cancel, broadcast_cancel = (list(t)
                                                              for t in zip(*sorted(zip(nb_participants_broadcast_cancel, broadcast_cancel))))

    return {'nb_participants_mpst': nb_participants_mpst, 'average_mpst': mpst, 'nb_participants_binary': nb_participants_binary, 'average_binary': binary, 'nb_participants_crossbeam': nb_participants_crossbeam, 'average_crossbeam': crossbeam}


def ring_compile():
    # Path for criterion
    main_path = './compile_time'

    # Get all directories in main_path
    directories = os.listdir(main_path)

    # Lists for plots
    average_mpst = []
    average_binary = []
    average_crossbeam = []
    average_cancel = []
    average_cancel_broadcast = []

    nb_participants_mpst = []
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
                if 'mpst' in d and 'baking' in d:
                    average_mpst.append(statistics.mean(build_time)/10**6)
                    nb_participants_mpst.append(str_to_int[name])
                elif 'binary' in d:
                    average_binary.append(statistics.mean(build_time)/10**6)
                    nb_participants_binary.append(str_to_int[name])
                elif 'cancel' in d:
                    if 'broadcast' in d:
                        average_cancel_broadcast.append(
                            statistics.mean(build_time)/10**6)
                        nb_participants_cancel_broadcast.append(
                            str_to_int[name])
                    else:
                        average_cancel.append(
                            statistics.mean(build_time)/10**6)
                        nb_participants_cancel.append(str_to_int[name])
                elif 'crossbeam' in d:
                    average_crossbeam.append(statistics.mean(build_time)/10**6)
                    nb_participants_crossbeam.append(str_to_int[name])
            except:
                print('Issue with ', d)

            file.close()

    # Sort the lists in pair
    nb_participants_mpst, average_mpst = (list(t) for t in zip(
        *sorted(zip(nb_participants_mpst, average_mpst))))

    nb_participants_binary, average_binary = (list(t)
                                              for t in zip(*sorted(zip(nb_participants_binary, average_binary))))

    nb_participants_crossbeam, average_crossbeam = (list(t)
                                                    for t in zip(*sorted(zip(nb_participants_crossbeam, average_crossbeam))))

    nb_participants_cancel, average_cancel = (list(t)
                                              for t in zip(*sorted(zip(nb_participants_cancel, average_cancel))))

    nb_participants_cancel_broadcast, average_cancel_broadcast = (list(t)
                                                                  for t in zip(*sorted(zip(nb_participants_cancel_broadcast, average_cancel_broadcast))))

    return {'nb_participants_mpst': nb_participants_mpst, 'average_mpst': average_mpst, 'nb_participants_binary': nb_participants_binary, 'average_binary': average_binary, 'nb_participants_crossbeam': nb_participants_crossbeam, 'average_crossbeam': average_crossbeam}


def ring():
    ring_bench()
    ring_compile()


# Get index of new file
index_mesh = 0
while os.path.isfile('results/mesh_' + index_mesh + '.csv'):
    index_mesh += 1
index_ring = 0
while os.path.isfile('results/ring_' + index_ring + '.csv'):
    index_ring += 1
index_ping_pong = 0
while os.path.isfile('results/ping_pong_' + index_ping_pong + '.csv'):
    index_ping_pong += 1

# For each folder in compile_path
for d in compile_path:
    for name_protocol in name_protocols:
        # If name looks like the one from what we want
        if name_protocol in d:
            with open(d) as f:
                lines = [line.rstrip() for line in f]
                temp_check = []
                temp_build = []
                temp_release = []
                for line in lines:
                    if 'check' in line:
                        temp_check.append(int(line.split('; ')[1]))
                    elif 'build' in line:
                        temp_build.append(int(line.split('; ')[1]))
                    elif 'release' in line:
                        temp_release.append(int(line.split('; ')[1]))

            if 'mesh' in d:
                with open('results/mesh_' + index_mesh + '.csv', 'a') as report_file:
                    report_file.write(name_protocol)
                    report_file.write('; ')
                    report_file.write(statistics.mean(temp_check))
                    report_file.write('; ')
                    report_file.write(statistics.mean(temp_build))
                    report_file.write('; ')
                    report_file.write(statistics.mean(temp_release))
                    report_file.write('; ')
                    report_file.write(bench_crossbeam[compile_file])
                    report_file.write('; ')
                    report_file.write(bench_binary[compile_file])
                    report_file.write('; ')
                    report_file.write(bench_mpst[compile_file])
                    report_file.write('\n')
            elif 'ring' in d:
                with open('results/ring_' + index_ring + '.csv', 'a') as report_file:
                    report_file.write(name_protocol)
                    report_file.write('; ')
                    report_file.write(statistics.mean(temp_check))
                    report_file.write('; ')
                    report_file.write(statistics.mean(temp_build))
                    report_file.write('; ')
                    report_file.write(statistics.mean(temp_release))
                    report_file.write('; ')
                    report_file.write(bench_crossbeam[compile_file])
                    report_file.write('; ')
                    report_file.write(bench_binary[compile_file])
                    report_file.write('; ')
                    report_file.write(bench_mpst[compile_file])
                    report_file.write('\n')
            elif 'ping' in d:
                with open('results/ping_pong_' + index_ping_pong + '.csv', 'a') as report_file:
                    report_file.write(name_protocol)
                    report_file.write('; ')
                    report_file.write(statistics.mean(temp_check))
                    report_file.write('; ')
                    report_file.write(statistics.mean(temp_build))
                    report_file.write('; ')
                    report_file.write(statistics.mean(temp_release))
                    report_file.write('; ')
                    report_file.write(bench_crossbeam[compile_file])
                    report_file.write('; ')
                    report_file.write(bench_binary[compile_file])
                    report_file.write('; ')
                    report_file.write(bench_mpst[compile_file])
                    report_file.write('\n')
            break
