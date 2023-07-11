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
    'three_buyers',
    'distributed_calc',
    'three_travel',
    'simple_voting',
    'online_wallet',
    'o_auth',
    'smtp',
    'remote_data',
    'servo',
    'http'
]

# Expected bench files
bench_files = [
    'Distributed calculator baking',
    'oAuth MPST baking',
    'Online wallet baking',
    'Simple voting MPST baking',
    'SMTP baking',
    'Travel MPST baking',
    'Three buyers MPST baking',
    'Timed Remote data',
    'Timed Servo',
    'Timed HTTP',
    'HTTP baking',
    'Servo baking',
    'Remote data baking',
    'Timed Distributed calculator baking',
    'Timed oAuth MPST baking',
    'Timed Online wallet baking',
    'Timed Simple voting MPST baking',
    'Timed SMTP baking',
    'Timed Travel MPST baking',
    'Timed Three buyers MPST baking'
]

# Expected bench files
translate = {
    'Distributed calculator baking': 'distributed_calc',
    'oAuth MPST baking': 'o_auth',
    'Online wallet baking': 'online_wallet',
    'Simple voting MPST baking': 'simple_voting',
    'SMTP baking': 'smtp',
    'Travel MPST baking': 'three_travel',
    'Three buyers MPST baking': 'three_buyers',
    'Timed Distributed calculator baking': 'distributed_calc_timed',
    'Timed Servo': 'servo_timed',
    'HTTP baking': 'http',
    'Timed HTTP': 'http_timed',
    'Servo baking': 'servo',
    'Remote data baking': 'remote_data',
    'Timed Remote data': 'remote_data_timed',
    'Timed oAuth MPST baking': 'o_auth_timed',
    'Timed Online wallet baking': 'online_wallet_timed',
    'Timed Simple voting MPST baking': 'simple_voting_timed',
    'Timed SMTP baking': 'smtp_timed',
    'Timed Travel MPST baking': 'three_travel_timed',
    'Timed Three buyers MPST baking': 'three_buyers_timed'
}

# Indexing for bar lists
index_compile = {
    'distributed_calc': 0,
    'online_wallet': 1,
    'smtp': 2,
    'simple_voting': 3,
    'three_buyers': 4,
    'distributed_calc_timed': 0,
    'online_wallet_timed': 1,
    'remote_data_timed': 8,
    'smtp_timed': 2,
    'http': 7,
    'http_timed': 7,
    'servo_timed': 9,
    'servo': 9,
    'remote_data': 8,
    'simple_voting_timed': 3,
    'three_buyers_timed': 4,
    'three_travel_timed': 5,
    'o_auth_timed': 6,
    'three_travel': 5,
    'o_auth': 6
}

bar_compilation_baking = [0] * 10

bar_compilation_timed = [0] * 10


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
        if compile_file in d and 'timed' not in d and 'checking' not in d and 'transport' not in d:
            try:
                with open(compile_folder / d, 'r') as f:
                    lines = [line.rstrip() for line in f]
                    temp_check = []
                    temp_build = []
                    temp_release = []
                    for line in lines:
                        if 'build' in line:
                            temp_build.append(int(line.split('; ')[1]))

                result_file = 'benchmarks_main_from_literature_' + str(index) + '.csv'

                bar_compilation_baking[index_compile[compile_file]] = statistics.mean(temp_build)/1000000

                with open(result_folder / result_file, 'a') as report_file:
                    report_file.write(compile_file)
                    report_file.write('; ')
                    report_file.write(str(statistics.mean(temp_build)/1000000))
                    report_file.write('; ')
                    report_file.write(str(bench[compile_file]/1000000))
                    report_file.write('; ')
                    report_file.write('\n')

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

                result_file = 'benchmarks_main_from_literature_' + str(index) + '.csv'

                bar_compilation_timed[index_compile[compile_file + '_timed']] = statistics.mean(temp_build)/1000000

                with open(result_folder / result_file, 'a') as report_file:
                    report_file.write(compile_file + '_timed')
                    report_file.write('; ')
                    report_file.write(str(statistics.mean(temp_build)/1000000))
                    report_file.write('; ')
                    report_file.write(str(bench[compile_file + '_timed']/1000000))
                    report_file.write('; ')
                    report_file.write('\n')

            except:
                print('Issue with ', d)
            break
    else:
        print("Issue with ", d)


sorted_translate = dict(sorted(translate.items()))

# Setting up the figure
fig, ax = plt.subplots(figsize=(16, 6))

width = 0.3

# Plotting compilation time
ax.bar(np.arange(len(bar_compilation_baking)) - width/2, bar_compilation_baking, width=width, color='#d62728')
ax.bar(np.arange(len(bar_compilation_timed)) + width/2, bar_compilation_timed, width=width, color='#9467bd')

ax.set_title("Compilation time", pad=10, fontsize=30)
ax.xaxis.set_major_locator(MaxNLocator(integer=True))
ax.yaxis.set_major_locator(MaxNLocator(integer=True))
ax.tick_params(axis='both', which='major', labelsize=20)
ax.set_ylabel('Time (s)', fontsize=30)
ax.xaxis.set_ticks([0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
ax.xaxis.set_ticklabels(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'])

plt.legend(
    ['AMPST', 'ATMP'],
    loc='best',
    # bbox_to_anchor=(-0.5, -0.3),
    fancybox=True,
    shadow=True,
    ncol=1,
    fontsize=20
)

# adjust subplot position
plt.tight_layout()

# create the name for the new figure
index_graphs = 0
while os.path.isfile('graphs_bench/examples/graphs_examples_compile_' + str(index_graphs) + '.pdf'):
    index_graphs += 1

name_graph = './graphs_bench/examples/graphs_examples_compile_' + str(index_graphs) + '.pdf'

# save the figure
plt.savefig(name_graph)

# show the plot
plt.show()
