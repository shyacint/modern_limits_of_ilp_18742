# modern_limits_of_ilp_18742
[CMU 18-742] Semester Research Project exploring the upper bounds of ILP in modern workloads with an emphasis ML inference and SPEC benchmarks

parse.py (as of March 27):
* USAGE: python3 parse.py <file_path> <window_width>
* reads an instruction trace and gathers a list of dependencies for each instruction
* iterates through dependency list until all instructions are "executed":
    * **fetches** the next *n* instructions which can fit in the instruction window
    * **decodes** the instructions which were fetched in the previous cycle
    * **executes** instructions which were decoded or waiting for execution in the previous cycle IF there are not preceding instructions in the window with the same dependencies
* prints number of cycles this simulation took
* TODO: 
    * add register & memory renaming
    * determine if different hazards (RAW, WAR, etc.) should be approached differently
    * more robust test cases for script accuracy
