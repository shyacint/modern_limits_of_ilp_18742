import os

def main():
    for w in [8,16,32,64,128,256]:
        os.system(f'python3 parse.py qemu_dynamic_traces/saxpy_trace {w}')

if __name__ == "__main__":
    main()