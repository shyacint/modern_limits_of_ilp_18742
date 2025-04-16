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

    # ---------- REGISTER RENAMING STRUCTURES ----------
    rename_table = {}     # Maps logical register -> current physical name
    next_phys_id = 0      # Assign new physical reg IDs as we go

    def rename_instruction(instr_deps):
        """
        instr_deps: list of strings (like ["a0","a1","a2"]) with the last one as dest if length>1
        Returns a new list of strings (the renamed dependencies).
        We'll rename them all, but each new definition of the last item gets a fresh physical reg.
        """
        nonlocal next_phys_id
        renamed = []

        if len(instr_deps) == 0:
            return renamed  # no registers to rename

        # Identify source regs (all except last if length>1)
        num = len(instr_deps)
        srcs = instr_deps[:-1] if num > 1 else instr_deps[:]  # if 1 or 0 items, everything is "src"

        # Rename each source
        for s in srcs:
            if s in rename_table:
                renamed.append(rename_table[s])
            else:
                # If never seen, map it to some default physical reg
                # but we won't call it a 'new definition' because it's a read
                renamed.append(rename_table.setdefault(s, f"p{next_phys_id}"))
                next_phys_id += 1

        # If there's at least 1 register, treat the last as destination
        if num > 1:
            dest_reg = instr_deps[-1]
            # create new physical name
            phys = f"p{next_phys_id}"
            next_phys_id += 1
            rename_table[dest_reg] = phys
            renamed.append(phys)
        elif num == 1:
            # then no distinct "dest"
            # we treat that 1 register as read only
            # we've already appended it above
            pass

        return renamed
#end RR
    while total_executed < total_instructions:
        cycles += 1

        # 1. FETCH
        capacity = window_width - (len(fetch_prev) + len(decode_prev) + len(execute_prev))
        if capacity < 0:
            capacity = 0
        fetch_now = []
        for i in range(total_fetched, total_fetched + capacity):
            if i >= total_instructions:
                break
            fetch_now.append(dep_list[i])
        total_fetched += capacity

        # 2. DECODE
        # the code says "decode_now = fetch_prev" => instructions from the previous fetch stage
        # we ADD rename logic here
        decode_now = []
        for instr in fetch_prev:
            renamed_deps = rename_instruction(instr)
            decode_now.append(renamed_deps)

        # 3. EXECUTE
        # execute_all = combine old stalls + newly decoded instructions
        execute_now = execute_prev + decode_prev

        l = len(execute_now)
        decide_execute = [True] * l
        for i in range(l):
            for j in range(i):
                # check intersection
                if bool(set(execute_now[i]) & set(execute_now[j])):
                    decide_execute[i] = False
                    break

        # whatever couldn't execute => remains in execute_prev
        execute_prev = [execute_now[i] for i in range(l) if not decide_execute[i]]
        total_executed += (l - len(execute_prev))

        # move pipeline forward
        fetch_prev = fetch_now
        decode_prev = decode_now

    return cycles, total_executed

def main():
    if len(sys.argv) != 3:
        print("Usage: python3 parse.py <file_path> <window width>")
        sys.exit(1)

    dep_list = read_trace(sys.argv[1])
    width = int(sys.argv[2])
    num_cycles, num_instructions = run_sim(dep_list, width)

    print(f'For width {width}: Completed {num_instructions} instructions in {num_cycles} cycles!')

if __name__ == "__main__":
    main()
