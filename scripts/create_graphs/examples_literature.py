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
compile_files = ['three_buyers', 'distributed_calc', 'travel_three', 'simple_voting', 'online_wallet',
                 'fib', 'video_stream', 'o_auth', 'logging_baking', 'circuit_breaker_baking', 'smtp']

# Expected bench files
bench_files = ['Circuit breaker baking', 'Distributed calculator baking', 'Fibo MPST baking', 'Logging baking', 'oAuth MPST baking',
               'Online wallet baking', 'Simple voting MPST baking', 'Video stream baking', 'SMTP baking', 'Travel MPST baking', 'Three buyers MPST baking']

# Expected bench files
translate = {'Circuit breaker baking': 'circuit_breaker_baking', 'Distributed calculator baking': 'distributed_calc', 'Fibo MPST baking': 'fib', 'Logging baking': 'logging_baking', 'oAuth MPST baking': 'o_auth',
             'Online wallet baking': 'online_wallet', 'Simple voting MPST baking': 'simple_voting', 'Video stream baking': 'video_stream', 'SMTP baking': 'smtp', 'Travel MPST baking': 'travel_three', 'Three buyers MPST baking': 'three_buyers'}



def test(path):
    # Get the wanted data in the JSON file (field -> mean, field -> point_estimate)
    with open(criterion_path + '/' + path + json_path) as json_file:
        data = json.load(json_file)
        return data['mean']['point_estimate']

# Lists for plots
bench = {}

# For each folder in criterion_path
for d in criterion_directories:
    # If name looks like the one from what we want
    for bench_file in bench_files:
        if bench_file in d:
            bench[translate[bench_file]] = int(test(d))
            break

# Get index of new file
index = 0
while os.path.isfile('results/benchmarks_main_from_literature_' + index + '.csv'):
    index += 1

# For each folder in compile_path
for d in compile_path:
    for compile_file in compile_files:
        # If name looks like the one from what we want
        if compile_file in d:
            try:
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

                with open('results/benchmarks_main_from_literature_' + index + '.csv', 'a') as report_file:
                    report_file.write(compile_file)
                    report_file.write('; ')
                    report_file.write(statistics.mean(temp_check))
                    report_file.write('; ')
                    report_file.write(statistics.mean(temp_build))
                    report_file.write('; ')
                    report_file.write(statistics.mean(temp_release))
                    report_file.write('; ')
                    report_file.write(bench[compile_file])
                    report_file.write('\n')
            except:
                print('Issue with ', d)
            break
