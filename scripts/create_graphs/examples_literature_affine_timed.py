import json
import os
import numpy as np
import statistics
import os.path
import matplotlib.pyplot as plt
from pathlib import Path
import pandas as pd
from pandas import Series, DataFrame
from matplotlib.ticker import MaxNLocator
import matplotlib.ticker as mticker

# Path for criterion
criterion_path = './target/criterion'

# Path for compile-time
compile_path = 'compile_time'

# Get all directories in criterion_path
criterion_directories = os.listdir(criterion_path)

# Get all directories in criterion_path
compile_directories = os.listdir(compile_path)

# Compile folder
compile_folder = Path(compile_path + '/')

# Result folder
result_folder = Path('results/')

# Relative path of the expected file
json_path = '/base/estimates.json'

# Expected compile files
compile_files = ['three_buyers', 'distributed_calc', 'three_travel',
                 'simple_voting', 'online_wallet', 'o_auth', 'smtp', 'remote_data', 'servo']

# Expected bench files
bench_files = ['Distributed calculator baking', 'oAuth MPST baking', 'Online wallet baking', 'Simple voting MPST baking', 'SMTP baking', 'Travel MPST baking', 'Three buyers MPST baking',
               'Timed Remote data', 'Timed Servo', 'Timed Distributed calculator baking', 'Timed oAuth MPST baking', 'Timed Online wallet baking', 'Timed Simple voting MPST baking', 'Timed SMTP baking', 'Timed Travel MPST baking', 'Timed Three buyers MPST baking']

# Expected bench files
translate = {'Distributed calculator baking': 'distributed_calc', 'oAuth MPST baking': 'o_auth', 'Online wallet baking': 'online_wallet', 'Simple voting MPST baking': 'simple_voting', 'SMTP baking': 'smtp', 'Travel MPST baking': 'three_travel', 'Three buyers MPST baking': 'three_buyers', 'Timed Distributed calculator baking': 'distributed_calc_timed',
             'Timed Servo': 'servo_timed', 'Timed Remote data': 'remote_data_timed', 'Timed oAuth MPST baking': 'o_auth_timed', 'Timed Online wallet baking': 'online_wallet_timed', 'Timed Simple voting MPST baking': 'simple_voting_timed', 'Timed SMTP baking': 'smtp_timed', 'Timed Travel MPST baking': 'three_travel_timed', 'Timed Three buyers MPST baking': 'three_buyers_timed'}

# Indexung for bar lists
index_bench = {'Distributed calculator baking': 0, 'Online wallet baking': 1, 'SMTP baking': 2, 'Simple voting MPST baking': 3, 'Three buyers MPST baking': 4, 'Timed Distributed calculator baking': 0, 'Timed Online wallet baking': 1, 'Timed Remote data': 7,
               'Timed SMTP baking': 2, 'Timed Servo': 8, 'Timed Simple voting MPST baking': 3, 'Timed Three buyers MPST baking': 4, 'Timed Travel MPST baking': 5, 'Timed oAuth MPST baking': 6, 'Travel MPST baking': 5, 'oAuth MPST baking': 6}
index_compile = {'distributed_calc': 0, 'online_wallet': 1, 'smtp': 2, 'simple_voting': 3, 'three_buyers': 4, 'distributed_calc_timed': 0, 'online_wallet_timed': 1,
                 'remote_data_timed': 7, 'smtp_timed': 2, 'servo_timed': 8, 'simple_voting_timed': 3, 'three_buyers_timed': 4, 'three_travel_timed': 5, 'o_auth_timed': 6, 'three_travel': 5, 'o_auth': 6}

bar_compilation_baking = [0] * 7
bar_running_baking = [0] * 7

bar_compilation_timed = [0] * 9
bar_running_timed = [0] * 9


def test(path):
    # Get the wanted data in the JSON file (field -> mean, field -> point_estimate)
    with open(criterion_path + '/' + path + json_path) as json_file:
        data = json.load(json_file)
        return data['mean']['point_estimate']


# Lists for plots
bench = {}
compilation = {}

# For each folder in criterion_path
for d in criterion_directories:
    if d in bench_files:
        # print(d, "is in bench_files")
        bench[translate[d]] = int(test(d))
    else:
        print(d, "is not in bench_files")

# Get index of new file
index = 0
while os.path.isfile('results/benchmarks_main_from_literature_' + str(index) + '.csv'):
    index += 1

# For each folder in compile_path
for d in compile_directories:
    for compile_file in compile_files:
        # If name looks like the one from what we want
        if compile_file in d and 'timed' not in d:
            try:
                with open(compile_folder / d, 'r') as f:
                    lines = [line.rstrip() for line in f]
                    temp_check = []
                    temp_build = []
                    temp_release = []
                    for line in lines:
                        if 'build' in line:
                            temp_build.append(int(line.split('; ')[1]))

                result_file = 'benchmarks_main_from_literature_' + \
                    str(index) + '.csv'

                print("compile_file baking: ", compile_file)
                print("index_compile", index_compile[compile_file])
                bar_compilation_baking[index_compile[compile_file]] = statistics.mean(
                    temp_build)/1000000
                # print("done with bar_compilation_baking")

                with open(result_folder / result_file, 'a') as report_file:
                    report_file.write(compile_file)
                    report_file.write('; ')
                    # print('Done with compilation time of', d)
                    report_file.write(str(statistics.mean(temp_build)/1000000))
                    report_file.write('; ')
                    # print('Done with building time of', d)
                    report_file.write(str(bench[compile_file]/1000000))
                    bar_running_baking[index_compile[compile_file]
                                       ] = bench[compile_file]/1000000
                    report_file.write('; ')
                    # print('Done with benchmarking time of', d)
                    report_file.write('\n')

                # print(compile_file + " done")
            except:
                print('Issue with ', d)
            break
        elif compile_file in d and 'timed' in d:
            try:
                with open(compile_folder / d, 'r') as f:
                    lines = [line.rstrip() for line in f]
                    temp_check = []
                    temp_build = []
                    temp_release = []
                    for line in lines:
                        if 'build' in line:
                            temp_build.append(int(line.split('; ')[1]))

                result_file = 'benchmarks_main_from_literature_' + \
                    str(index) + '.csv'

                print("compile_file timed: ", compile_file + '_timed')
                print("index_compile", index_compile[compile_file + '_timed'])
                bar_compilation_timed[index_compile[compile_file + '_timed']] = statistics.mean(
                    temp_build)/1000000
                # print("done with bar_compilation_timed")

                with open(result_folder / result_file, 'a') as report_file:
                    report_file.write(compile_file + '_timed')
                    report_file.write('; ')
                    # print('Done with compilation time of', d)
                    report_file.write(str(statistics.mean(temp_build)/1000000))
                    report_file.write('; ')
                    # print('Done with building time of', d)
                    report_file.write(str(bench[compile_file + '_timed']/1000000))
                    bar_running_timed[index_compile[compile_file + '_timed']
                                      ] = bench[compile_file + '_timed']/1000000
                    report_file.write('; ')
                    # print('Done with benchmarking time of', d)
                    report_file.write('\n')

                # print(compile_file + " done")
            except:
                print('Issue with ', d)
            break

# print("Done")

sorted_translate = dict(sorted(translate.items()))

# print("bar_compilation_timed", bar_compilation_timed)
# print("bar_compilation_baking", bar_compilation_baking)
# print("bar_running_baking", bar_running_baking)
# print("bar_running_timed", bar_running_timed)
# print("benches", bench)

# Setting up the figure
fig = plt.figure(figsize=(20, 5))

width = 0.3

# Plotting compilation time
compilation_time = fig.add_subplot(1, 2, 1)

compilation_time.bar(np.arange(len(bar_compilation_baking)) -
                     width/2, bar_compilation_baking, width=width)
compilation_time.bar(np.arange(len(bar_compilation_timed)) +
                     width/2, bar_compilation_timed, width=width)

compilation_time.set_title("Compilation time", pad=10, fontsize=30)
compilation_time.xaxis.set_major_locator(MaxNLocator(integer=True))
compilation_time.yaxis.set_major_locator(MaxNLocator(integer=True))
compilation_time.tick_params(axis='both', which='major', labelsize=20)
compilation_time.set_ylabel('Time (s)', fontsize=30)
compilation_time.xaxis.set_ticks([0, 1, 2, 3, 4, 5, 6, 7, 8])
compilation_time.xaxis.set_ticklabels(
    ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'])

# Setting the number of ticks
compilation_time.yaxis.set_ticks(np.arange(0, int(max(
    max(bar_compilation_timed), max(bar_compilation_baking)))+5, 10), labelsize=20)
compilation_time.tick_params(labelsize=20)

# Plotting running time
running_time = fig.add_subplot(1, 2, 2)

running_time.bar(np.arange(len(bar_running_baking)) -
                 width/2, bar_running_baking, width=width)
running_time.bar(np.arange(len(bar_running_timed)) +
                 width/2, bar_running_timed, width=width)

running_time.set_title("Running time", pad=10, fontsize=30)
running_time.xaxis.set_major_locator(MaxNLocator(integer=True))
running_time.yaxis.set_major_locator(MaxNLocator(integer=True))
running_time.tick_params(axis='both', which='major', labelsize=20)
running_time.set_ylabel('Time (ms)', fontsize=30)
running_time.xaxis.set_ticks([0, 1, 2, 3, 4, 5, 6, 7, 8])
running_time.xaxis.set_ticklabels(
    ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'])

# Setting the number of ticks
running_time.yaxis.set_ticks(np.arange(0, int(
    max(max(bar_running_timed), max(bar_running_baking)))+1.5, 0.5), labelsize=20)
running_time.tick_params(labelsize=20)

plt.legend(
    ['AMPST','ATMP'],
    loc='best',
    # bbox_to_anchor=(-0.5, -0.3),
    fancybox=True,
    shadow=True,
    ncol=1,
    fontsize=20)

# adjust subplot position
plt.tight_layout()

# create the name for the new figure
index_graphs = 0
while os.path.isfile('results/graphs_examples_' + str(index_graphs) + '.pdf'):
    index_graphs += 1

name_graph = './results/graphs_examples_' + str(index_graphs) + '.pdf'

# save the figure
plt.savefig(name_graph)

# show the plot
plt.show()
