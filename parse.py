import sys

def read_trace(file_path):
    dependencies = []
    with open(file_path) as file:
        for line in file:
            # split by white space
            words = line.split()

            # if empty line, pass
            if len(words) == 0:
                continue

            # check if this is an instruction line before finding dependencies
            if words[0].startswith("0x"):
                # if dependencies, store them
                # if not, store an empty value (bc we still need to track this instruction)
                if len(words) > 3:
                    dep = words[3].split(',')
                else:
                    dep = []
            
                dependencies.append(dep)
    
    return dependencies

def run_sim(dep_list, window_width):
    total_instructions = len(dep_list)
    total_executed = 0
    total_fetched = 0
    cycles = 0

    fetch_prev = []
    decode_prev = []
    execute_prev = []

    while total_executed < total_instructions:
        cycles += 1

        # first, fetch all instructions which can be fetched
        capacity = window_width - (len(fetch_prev) + len(decode_prev) + len(execute_prev))
        fetch_now = []
        for i in range(total_fetched, total_fetched + capacity):
            if i >= total_instructions:
                break
            fetch_now.append(dep_list[i])
        total_fetched += capacity

        # then, decode all instructions which can be decode (e.g., the previous fetch stage)
        decode_now = fetch_prev

        # finally, execute all the instructions which can be executed (e.g., instructions in the previous execute stage or the previous decode stage which do not share an dependency with a previous instruction in that group)
        execute_now = execute_prev + decode_prev

        l = len(execute_now)
        # determine if CAN execute
        decide_execute = [True] * l
        for i in range(l):
            for j in range(i):
                if bool(set(execute_now[i]) & set(execute_now[j])):
                    decide_execute[i] = False
                    break
        
        # execute_prev is all the instructions which cannot be executed yet
        execute_prev = [execute_now[i] for i in range(l) if not decide_execute[i]]
        total_executed += l - len(execute_prev)

        # transfer decode and fetch stages for next cycle
        fetch_prev = fetch_now
        decode_prev = decode_now

    return cycles, total_executed
    

def main():
    # check input arguments
    if len(sys.argv) != 3:
        print("Usage: python3 parse.py <file_path> <window width>")
        sys.exit(1)

    # get a list of the dependency lists for each instruction
    dep_list = read_trace(sys.argv[1])

    # complete cycles until all instructions are executed
    num_cycles, num_instructions = run_sim(dep_list, int(sys.argv[2]))

    print(f'Completed {num_instructions} instructions in {num_cycles} cycles!')
    

if __name__ == "__main__":
    main()