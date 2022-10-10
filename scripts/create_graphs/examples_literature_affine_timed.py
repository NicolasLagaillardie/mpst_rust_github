import json
import os
import numpy as np
import statistics
import os.path
import matplotlib.pyplot as plt
from pathlib import Path

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
                 'simple_voting', 'online_wallet', 'o_auth', 'smtp']

# Expected bench files
bench_files = ['Distributed calculator baking', 'oAuth MPST baking', 'Online wallet baking', 'Simple voting MPST baking', 'SMTP baking', 'Travel MPST baking', 'Three buyers MPST baking',
               'Timed Distributed calculator baking', 'Timed oAuth MPST baking', 'Timed Online wallet baking', 'Timed Simple voting MPST baking', 'Timed SMTP baking', 'Timed Travel MPST baking', 'Timed Three buyers MPST baking']

# Expected bench files
translate = {'Distributed calculator baking': 'distributed_calc', 'oAuth MPST baking': 'o_auth', 'Online wallet baking': 'online_wallet', 'Simple voting MPST baking': 'simple_voting', 'SMTP baking': 'smtp', 'Travel MPST baking': 'three_travel', 'Three buyers MPST baking': 'three_buyers', 'Timed Distributed calculator baking':
             'Timed distributed_calc', 'Timed oAuth MPST baking': 'Timed o_auth', 'Timed Online wallet baking': 'Timed online_wallet', 'Timed Simple voting MPST baking': 'Timed simple_voting', 'Timed SMTP baking': 'Timed smtp', 'Timed Travel MPST baking': 'Timed three_travel', 'Timed Three buyers MPST baking': 'Timed three_buyers'}


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
        print(d, "is in bench_files")
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
                        if 'check' in line:
                            temp_check.append(int(line.split('; ')[1]))
                        elif 'build' in line:
                            temp_build.append(int(line.split('; ')[1]))

                        elif 'release' in line:
                            temp_release.append(int(line.split('; ')[1]))

                result_file = 'benchmarks_main_from_literature_' + \
                    str(index) + '.csv'

                with open(result_folder / result_file, 'a') as report_file:
                    report_file.write(compile_file)
                    report_file.write('; ')
                    print('Done with compilation time of', d)
                    report_file.write(str(statistics.mean(temp_check)))
                    report_file.write('; ')
                    print('Done with checking time of', d)
                    report_file.write(str(statistics.mean(temp_build)))
                    report_file.write('; ')
                    print('Done with building time of', d)
                    report_file.write(str(statistics.mean(temp_release)))
                    report_file.write('; ')
                    print('Done with building --release time of', d)
                    report_file.write(str(bench[compile_file]))
                    report_file.write('; ')
                    print('Done with benchmarking time of', d)
                    report_file.write('\n')

                print(compile_file + " done")
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
                        if 'check' in line:
                            temp_check.append(int(line.split('; ')[1]))
                        elif 'build' in line:
                            temp_build.append(int(line.split('; ')[1]))

                        elif 'release' in line:
                            temp_release.append(int(line.split('; ')[1]))

                result_file = 'benchmarks_main_from_literature_' + \
                    str(index) + '.csv'

                with open(result_folder / result_file, 'a') as report_file:
                    report_file.write(compile_file)
                    report_file.write('; ')
                    print('Done with compilation time of', d)
                    report_file.write(str(statistics.mean(temp_check)))
                    report_file.write('; ')
                    print('Done with checking time of', d)
                    report_file.write(str(statistics.mean(temp_build)))
                    report_file.write('; ')
                    print('Done with building time of', d)
                    report_file.write(str(statistics.mean(temp_release)))
                    report_file.write('; ')
                    print('Done with building --release time of', d)
                    report_file.write(str(bench[compile_file]))
                    report_file.write('; ')
                    print('Done with benchmarking time of', d)
                    report_file.write('\n')

                print(compile_file + " done")
            except:
                print('Issue with ', d)
            break

print("Done")

print("Results: ", bench, bench.__len__())

# running_time_fig = plt.figure()

# running_time = running_time_fig.add_subplot(1, 1, 1)
# running_time.set_title("Running time", pad=10, fontsize=30)
# running_time.xaxis.set_major_locator(MaxNLocator(integer=True))
# running_time.yaxis.set_major_locator(MaxNLocator(integer=True))
# running_time.set_ylabel('Time (ms)', fontsize=30)

# running_time.hist()


# compilation_time_fig = plt.figure()

# compilation_time = compilation_time_fig.add_subplot(1, 1, 1)
# compilation_time.set_title("Running time", pad=10, fontsize=30)
# compilation_time.xaxis.set_major_locator(MaxNLocator(integer=True))
# compilation_time.yaxis.set_major_locator(MaxNLocator(integer=True))
# compilation_time.set_ylabel('Time (ms)', fontsize=30)


# # adjust subplot position
# plt.tight_layout()

# # save the figure
# # plt.savefig(name_graph)

# # show the plot
# plt.show()
