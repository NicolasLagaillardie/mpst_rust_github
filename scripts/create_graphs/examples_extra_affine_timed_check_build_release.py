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
    'servo',
    'logging',
    'video_stream',
    'remote_data',
    'circuit_breaker',
]

# Expected bench files
bench_files = [
    'Servo',
    'Logging',
    'Video stream',
    'Remote data',
    'Circuit breaker',
]

# Indexing for bar lists
index_compile = {}

for index, elt in enumerate(compile_files):
    # index_compile[elt] = index
    index_compile[elt + '_timed'] = index
    index_compile[elt + '_binary'] = index
    index_compile[elt + '_crossbeam'] = index
    index_compile[elt + '_mpst'] = index
    index_compile[elt + '_ampst'] = index

# Translate from compile files to bench files and vice_versa
translate = {}
reverse_translate = {}

for key, value in index_compile.items():
    if 'timed' in key:
        translate['Timed ' + bench_files[value]] = key
        reverse_translate[key] = 'Timed ' + bench_files[value]
    elif 'binary' in key:
        translate[bench_files[value] + ' binary'] = key
        reverse_translate[key] = bench_files[value] + ' binary'
    elif 'crossbeam' in key:
        translate[bench_files[value] + ' crossbeam'] = key
        reverse_translate[key] = bench_files[value] + ' crossbeam'
    elif 'ampst' in key:
        translate[bench_files[value] + ' AMPST'] = key
        reverse_translate[key] = bench_files[value] + ' AMPST'
    elif 'mpst' in key:
        translate[bench_files[value] + ' MPST'] = key
        reverse_translate[key] = bench_files[value] + ' MPST'
    else:
        # translate[bench_files[value]] = key
        # reverse_translate[key] = bench_files[value]
        print("Issue with " + key + ' ' + value + ' while building translate and reverse_translate')
        # exit()

# Lists with metrics
bar_check_ampst = [0] * len(compile_files)
bar_check_timed = [0] * len(compile_files)
bar_check_binary = [0] * len(compile_files)
bar_check_crossbeam = [0] * len(compile_files)
bar_check_mpst = [0] * len(compile_files)

bar_build_ampst = [0] * len(compile_files)
bar_build_timed = [0] * len(compile_files)
bar_build_binary = [0] * len(compile_files)
bar_build_crossbeam = [0] * len(compile_files)
bar_build_mpst = [0] * len(compile_files)

bar_release_ampst = [0] * len(compile_files)
bar_release_timed = [0] * len(compile_files)
bar_release_binary = [0] * len(compile_files)
bar_release_crossbeam = [0] * len(compile_files)
bar_release_mpst = [0] * len(compile_files)

bar_run_ampst = [0] * len(compile_files)
bar_run_timed = [0] * len(compile_files)
bar_run_binary = [0] * len(compile_files)
bar_run_crossbeam = [0] * len(compile_files)
bar_run_mpst = [0] * len(compile_files)

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
        # # exit()

# Get index of new csv result file
index = 0
while os.path.isfile('results/benchmarks_examples_extra_' + str(index) + '.csv'):
    index += 1

result_file = 'benchmarks_examples_extra_' + str(index) + '.csv'

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

# For each protocol to be studied
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

            if 'timed' in name_file:
                bar_check_timed[index] = statistics.mean(temp_check)/10**6
                bar_build_timed[index] = statistics.mean(temp_build)/10**6
                bar_release_timed[index] = statistics.mean(temp_release)/10**6
                bar_run_timed[index] = bench[protocol]/10**6
            elif 'binary' in name_file:
                bar_check_binary[index] = statistics.mean(temp_check)/10**6
                bar_build_binary[index] = statistics.mean(temp_build)/10**6
                bar_release_binary[index] = statistics.mean(temp_release)/10**6
                bar_run_binary[index] = bench[protocol]/10**6
            elif 'crossbeam' in name_file:
                bar_check_crossbeam[index] = statistics.mean(temp_check)/10**6
                bar_build_crossbeam[index] = statistics.mean(temp_build)/10**6
                bar_release_crossbeam[index] = statistics.mean(temp_release)/10**6
                bar_run_crossbeam[index] = bench[protocol]/10**6
            elif 'ampst' in name_file:
                bar_check_ampst[index] = statistics.mean(temp_check)/10**6
                bar_build_ampst[index] = statistics.mean(temp_build)/10**6
                bar_release_ampst[index] = statistics.mean(temp_release)/10**6
                bar_run_ampst[index] = bench[protocol]/10**6
            elif 'mpst' in name_file:
                bar_check_mpst[index] = statistics.mean(temp_check)/10**6
                bar_build_mpst[index] = statistics.mean(temp_build)/10**6
                bar_release_mpst[index] = statistics.mean(temp_release)/10**6
                bar_run_mpst[index] = bench[protocol]/10**6
            else:
                print('Issue with ', name_file)
                # exit()

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
            # # exit()
    else:
        print(protocol + " not in compiled files")
        # # exit()

range_compile_files = [i for i in range(len(compile_files))]
# ticklabels_compile_files = [chr(i + 97) for i in range(len(compile_files))]
ticklabels_compile_files = bench_files

width = 0.3

def create_and_save_fig(name_file, title_graph, list_ampst, list_timed, list_mpst, list_binary, list_crossbeam, scale, localisation):
    # Setting up the figure
    fig, ax = plt.subplots(figsize=(16, 6))

    # Plotting compilation time
    ax.bar(np.arange(len(list_crossbeam)) - 4*width/5, list_crossbeam, width=2*width/5, color='#ff7f0e')
    ax.bar(np.arange(len(list_binary)) - 2*width/5, list_binary, width=2*width/5, color='#1f77b4')
    ax.bar(np.arange(len(list_mpst)), list_mpst, width=2*width/5, color='#2ca02c')
    ax.bar(np.arange(len(list_ampst)) + 2*width/5, list_ampst, width=2*width/5, color='#d62728')
    ax.bar(np.arange(len(list_timed)) + 4*width/5, list_timed, width=2*width/5, color='#9467bd')

    ax.set_title(title_graph, pad=10, fontsize=30)
    ax.xaxis.set_major_locator(MaxNLocator(integer=True))
    ax.yaxis.set_major_locator(MaxNLocator(integer=True))
    ax.tick_params(axis='both', which='major', labelsize=20)
    ax.set_ylabel(scale, fontsize=30)
    ax.xaxis.set_ticks(range_compile_files)
    ax.xaxis.set_ticklabels(ticklabels_compile_files, rotation=45, ha='right')

    plt.legend(
        ['Crossbeam', 'Binary', 'MPST', 'AMPST', 'ATMP'],
        loc=localisation,
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

create_and_save_fig('extra_check_', 'Check time', bar_check_ampst, bar_check_timed, bar_check_mpst, bar_check_binary, bar_check_crossbeam, 'Time (s)', 'lower right')
create_and_save_fig('extra_build_', 'Build time', bar_build_ampst, bar_build_timed, bar_build_mpst, bar_build_binary, bar_build_crossbeam, 'Time (s)', 'lower right')
create_and_save_fig('extra_release_', 'Release time', bar_release_ampst, bar_release_timed, bar_release_mpst, bar_release_binary, bar_release_crossbeam, 'Time (s)', 'lower right')
create_and_save_fig('extra_run_', 'Runtime', bar_run_ampst, bar_run_timed, bar_run_mpst, bar_run_binary, bar_run_crossbeam, 'Time (ms)', 'upper left')
