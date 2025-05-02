import os
import shutil
import argparse

def copy_input_data(benchmarks, input_set, data_repo, exe_repo):
    for bench in benchmarks:
        src = os.path.join(data_repo, bench, f'data-{input_set}')
        dest = os.path.join(exe_repo, bench, 'exe')
        
        if not os.path.exists(src):
            print(f"[ERROR] Input data for {bench} ({input_set}) not found at {src}")
            continue
        
        if not os.path.exists(dest):
            print(f"[ERROR] Executable directory for {bench} not found at {dest}")
            continue
        
        # Copy input data to executable directory
        print(f"[INFO] Copying {input_set} data for {bench}...")
        for item in os.listdir(src):
            s = os.path.join(src, item)
            d = os.path.join(dest, item)
            if os.path.isdir(s):
                shutil.copytree(s, d, dirs_exist_ok=True)
            else:
                shutil.copy2(s, d)
        print(f"[SUCCESS] {input_set} data for {bench} copied successfully.")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Copy SPEC CPU2006 input data to executable directories.")
    parser.add_argument('--benchmarks', nargs='+', required=True, help='List of benchmark names.')
    parser.add_argument('--input-set', choices=['train', 'test', 'ref'], required=True, help='Input data set to copy.')
    parser.add_argument('--data-repo', required=True, help='Path to the data repository (e.g., /m2s-bench-spec2006).')
    parser.add_argument('--exe-repo', required=True, help='Path to the executable repository (e.g., ~/riscv_build/CPU2006).')

    args = parser.parse_args()
    
    copy_input_data(args.benchmarks, args.input_set, os.path.expanduser(args.data_repo), os.path.expanduser(args.exe_repo))
    

