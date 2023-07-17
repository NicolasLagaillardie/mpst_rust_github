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
compile_files = [
    'fib',
    'simple_voting',
    'calculator',
    'three_buyers',
    'travel_agency',
    'o_auth',
    'online_wallet',
    'smtp',
    'servo',
    'logging',
    'video_stream',
    'remote_data',
    'circuit_breaker',
]

# Expected bench files
bench_files = [
    'Fibo 20',
    'Simple voting',
    'Calculator',
    'Three buyers',
    'Travel agency',
    'oAuth',
    'Online wallet',
    'SMTP',
    'Servo',
    'Logging',
    'Video stream',
    'Remote data',
    'Circuit breaker',
]

# Indexing for bar lists
index_compile = {}

for index, elt in enumerate(compile_files):
    index_compile[elt] = index
    index_compile[elt + '_timed'] = index

# Translate from compile files to bench files and vice_versa
translate = {}
reverse_translate = {}

for key, value in index_compile.items():
    if 'timed' in key:
        translate['Timed ' + bench_files[value]] = key
        reverse_translate[key] = 'Timed ' + bench_files[value]
    else:
        translate[bench_files[value]] = key
        reverse_translate[key] = bench_files[value]

# Lists with metrics
bar_check_baking = [0] * len(compile_files)
bar_check_timed = [0] * len(compile_files)

bar_build_baking = [0] * len(compile_files)
bar_build_timed = [0] * len(compile_files)

bar_release_baking = [0] * len(compile_files)
bar_release_timed = [0] * len(compile_files)

bar_run_baking = [0] * len(compile_files)
bar_run_timed = [0] * len(compile_files)

def test(path):
    # Get the wanted data in the JSON file (field -> mean, field -> point_estimate)
    with open(criterion_path + '/' + path + json_path) as json_file:
        data = json.load(json_file)
        return data['mean']['point_estimate']

# List for runtime plots
bench = {}

# For each folder in criterion_path
for key, value in translate.items():
    if key in criterion_directories:
        bench[value] = int(test(key))
    else:
        print(key, "is missing")
        exit()

# Get index of new csv result file
index = 0
while os.path.isfile('results/benchmarks_main_from_literature_' + str(index) + '.csv'):
    index += 1

result_file = 'benchmarks_main_from_literature_' + str(index) + '.csv'

# Add name of columns
with open(result_folder / result_file, 'a') as report_file:
    report_file.write('Name of the protocol')
    report_file.write('; ')
    report_file.write('Check time')
    report_file.write('; ')
    report_file.write('Build time')
    report_file.write('; ')
    report_file.write('Release time')
    report_file.write('; ')
    report_file.write('Run time')
    report_file.write('; ')
    report_file.write('\n')

# For each folder in compile_path
for protocol, index in index_compile.items():

    name_file = protocol + '.txt'

    if name_file in compile_directories:
        try:
            with open(compile_folder / name_file, 'r') as f:
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

            if 'timed' not in name_file:
                bar_check_baking[index] = statistics.mean(temp_check)/10**6
                bar_build_baking[index] = statistics.mean(temp_build)/10**6
                bar_release_baking[index] = statistics.mean(temp_release)/10**6
                bar_run_baking[index] = bench[protocol]/10**6
            else:
                bar_check_timed[index] = statistics.mean(temp_check)/10**6
                bar_build_timed[index] = statistics.mean(temp_build)/10**6
                bar_release_timed[index] = statistics.mean(temp_release)/10**6
                bar_run_timed[index] = bench[protocol]/10**6

            with open(result_folder / result_file, 'a') as report_file:
                report_file.write(reverse_translate[protocol])
                report_file.write('; ')
                report_file.write(str(statistics.mean(temp_check)/10**6))
                report_file.write('; ')
                report_file.write(str(statistics.mean(temp_build)/10**6))
                report_file.write('; ')
                report_file.write(str(statistics.mean(temp_release)/10**6))
                report_file.write('; ')
                report_file.write(str(bench[protocol]/10**6))
                report_file.write('; ')
                report_file.write('\n')

        except:
            print('Issue with ', protocol)
            exit()
    else:
        print(protocol + " not in compiled files")
        exit()

range_compile_files = [i for i in range(len(compile_files))]
ticklabels_compile_files = [chr(i + 97) for i in range(len(compile_files))]

def create_and_save_fig(name_file, title_graph, list_baking, list_timed, scale):
    # Setting up the figure
    fig, ax = plt.subplots(figsize=(16, 6))

    width = 0.3

    # Plotting compilation time
    ax.bar(np.arange(len(list_baking)) - width/2, list_baking, width=width, color='#d62728')
    ax.bar(np.arange(len(list_timed)) + width/2, list_timed, width=width, color='#9467bd')

    ax.set_title(title_graph, pad=10, fontsize=30)
    ax.xaxis.set_major_locator(MaxNLocator(integer=True))
    ax.yaxis.set_major_locator(MaxNLocator(integer=True))
    ax.tick_params(axis='both', which='major', labelsize=20)
    ax.set_ylabel(scale, fontsize=30)
    ax.xaxis.set_ticks(range_compile_files)
    ax.xaxis.set_ticklabels(ticklabels_compile_files)

    plt.legend(
        ['AMPST', 'ATMP'],
        loc='lower right',
        fancybox=True,
        shadow=True,
        ncol=1,
        fontsize=20
    )

    # adjust subplot position
    plt.tight_layout()

    # create the name for the new figure
    index_graphs = 0
    while os.path.isfile('graphs_bench/examples/' + name_file + str(index_graphs) + '.pdf'):
        index_graphs += 1

    name_graph = './graphs_bench/examples/' + name_file + str(index_graphs) + '.pdf'

    # save the figure
    plt.savefig(name_graph)

create_and_save_fig('graphs_examples_check_', 'Check time', bar_check_baking, bar_check_timed, 'Time (s)')
create_and_save_fig('graphs_examples_build_', 'Build time', bar_build_baking, bar_build_timed, 'Time (s)')
create_and_save_fig('graphs_examples_release_', 'Release time', bar_release_baking, bar_release_timed, 'Time (s)')
create_and_save_fig('graphs_examples_run_', 'Runtime', bar_run_baking, bar_run_timed, 'Time (ms)')
