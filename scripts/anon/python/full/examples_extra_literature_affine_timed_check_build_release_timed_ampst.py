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
criterion_path = './save/examples'

# Path for compile-time
compile_path = 'compile_time'

# Get all directories in criterion_path
criterion_directories = os.listdir(criterion_path)

# Get all directories in criterion_path
compile_directories = os.listdir(compile_path)

# Compile folder
compile_folder = Path(compile_path + '/')

# Relative path of the expected file
json_path = '/base/estimates.json'

# Expected compile files
compile_files = [
    'calculator',
    'online_wallet',
    'smtp',
    'simple_voting',
    'three_buyers',
    'travel_agency',
    'o_auth',
    'http',
    'remote_data',
    'servo',
    'gravity_android',
    'pinetime_heart_rate',
    'proximity_based_car_key',
]

# Expected bench files
bench_files = [
    'Calculator',
    'Online wallet',
    'SMTP',
    'Simple voting',
    'Three buyers',
    'Travel agency',
    'oAuth',
    'HTTP',
    'Remote data',
    'Servo',
    'Circuit breaker',
    'Gravity Android',
    'Heart Rate',
    'Car-Key',
]

# Indexing for bar lists
index_compile = {}

for index, elt in enumerate(compile_files):
    # index_compile[elt] = index
    index_compile[elt + '_timed'] = index
    index_compile[elt + '_ampst'] = index

# Translate from compile files to bench files and vice_versa
translate = {}
reverse_translate = {}

for key, value in index_compile.items():
    if 'timed' in key:
        translate['Timed ' + bench_files[value]] = key
        reverse_translate[key] = 'Timed ' + bench_files[value]
    elif 'ampst' in key:
        translate[bench_files[value] + ' AMPST'] = key
        reverse_translate[key] = bench_files[value] + ' AMPST'
    else:
        # translate[bench_files[value]] = key
        # reverse_translate[key] = bench_files[value]
        print("Issue with " + key + ' ' + value + ' while building translate and reverse_translate')
        # exit()

# Lists with metrics
bar_build_timed = [0] * len(compile_files)
bar_build_ampst = [0] * len(compile_files)

bar_run_timed = [0] * len(compile_files)
bar_run_ampst = [0] * len(compile_files)

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
        # exit()

# For each protocol to be studied
for protocol, index in index_compile.items():

    name_file = protocol + '.txt'

    if name_file in compile_directories:
        try:
            with open(compile_folder / name_file, 'r') as f:
                lines = [line.rstrip() for line in f]
                temp_build = []
                for line in lines:
                    if 'build' in line:
                        temp_build.append(int(line.split('; ')[1]))

            if 'timed' in name_file:
                bar_build_timed[index] = statistics.mean(temp_build)/10**6
                bar_run_timed[index] = bench[protocol]/10**6
            elif 'ampst' in name_file:
                bar_build_ampst[index] = statistics.mean(temp_build)/10**6
                bar_run_ampst[index] = bench[protocol]/10**6
            else:
                print('Issue with ', name_file)
                # exit()

        except:
            print('Issue with ', protocol)
            # # exit()
    else:
        print(protocol + " not in compiled files")
        # # exit()

range_compile_files = [i for i in range(len(compile_files))]
ticklabels_compile_files = [chr(i + 97) for i in range(len(compile_files))]
# ticklabels_compile_files = bench_files

width = 0.3

# Setting up the figure
fig, (axsLeft, axsRight) = plt.subplots(1, 2, figsize=(32, 6))

# Plotting compilation time
axsLeft.bar(np.arange(len(bar_build_ampst)) - width/2, bar_build_ampst, width=width, color='#d62728')
axsLeft.bar(np.arange(len(bar_build_timed)) + width/2, bar_build_timed, width=width, color='#9467bd')

axsLeft.set_title('Build time', pad=10, fontsize=30)
axsLeft.xaxis.set_major_locator(MaxNLocator(integer=True))
axsLeft.yaxis.set_major_locator(MaxNLocator(integer=True))
axsLeft.tick_params(axis='both', which='major', labelsize=20)
axsLeft.set_ylabel('Time (s)', fontsize=30)
axsLeft.xaxis.set_ticks(range_compile_files)
axsLeft.xaxis.set_ticklabels(ticklabels_compile_files, ha='right')

# Plotting compilation time
axsRight.bar(np.arange(len(bar_run_ampst)) - width/2, bar_run_ampst, width=width, color='#d62728')
axsRight.bar(np.arange(len(bar_run_timed)) + width/2, bar_run_timed, width=width, color='#9467bd')

axsRight.set_title('Runtime', pad=10, fontsize=30)
axsRight.xaxis.set_major_locator(MaxNLocator(integer=True))
axsRight.yaxis.set_major_locator(MaxNLocator(integer=True))
axsRight.tick_params(axis='both', which='major', labelsize=20)
axsRight.set_ylabel('Time (ms)', fontsize=30)
axsRight.xaxis.set_ticks(range_compile_files)
axsRight.xaxis.set_ticklabels(ticklabels_compile_files, ha='right')

plt.legend(
    ['Multicrusty', 'Anon'],
    loc='upper left',
    fancybox=True,
    shadow=True,
    ncol=5,
    fontsize=30,
)

# adjust subplot position
plt.tight_layout()

# create the name for the new figure
index_graphs = 0
while os.path.isfile('graphs_bench/examples/merge_build_run_' + str(index_graphs) + '.pdf'):
    index_graphs += 1

name_graph = './graphs_bench/examples/merge_build_run_' + str(index_graphs) + '.pdf'

# save the figure
plt.savefig(name_graph)
