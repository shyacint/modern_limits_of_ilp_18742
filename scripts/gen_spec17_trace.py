import subprocess
import os
import argparse
import gzip
import shutil
import time

# --- CONFIGURATION ---
SPEC_DIR = "/home/shyacinthe/speccpu2017-clean"
DATA_DIR = os.path.join(SPEC_DIR, "benchspec/CPU")
QEMU_EXECUTABLE = "/home/shyacinthe/qemu/build-latest/qemu-riscv64"

SPEC_BENCHMARK_TO_QEMU_COMMAND_MAP = {
    "502.gcc_r": [
        ("cpugcc_r_base.riscv", ["200.c", "-O3", "-finline-limit=50000", "-o", "200.opts-O3_-finline-limit_50000.s"], "gcc_200_trace.log", "gcc_200_output.txt"),
        ("cpugcc_r_base.riscv", ["scilab.c", "-O3", "-finline-limit=50000", "-o", "scilab.opts-O3_-finline-limit_50000.s"], "gcc_scilab_trace.log", "gcc_scilab_output.txt"),
        ("cpugcc_r_base.riscv", ["train01.c", "-O3", "-finline-limit=50000", "-o", "train01.opts-O3_-finline-limit_50000.s"], "gcc_train01_trace.log", "gcc_train01_output.txt"),
    ],
    "505.mcf_r": [("mcf_r_base.riscv", ["inp.in"], "mcf_trace.log", "mcf_output.txt")],
    "520.omnetpp_r": [("omnetpp_r_base.riscv", ["-c", "General", "-r", "0"], "omnetpp_trace.log", "omnetpp_output.txt")],
    "531.deepsjeng_r": [("deepsjeng_r_base.riscv", ["train.txt"], "deepsjeng_trace.log", "deepsjeng_output.txt")],
    "511.povray_r": [("povray_r_base.riscv", ["SPEC-benchmark-train.ini"], "povray_trace.log", "povray_output.txt")],
    "519.lbm_r": [("lbm_r_base.riscv", ["300", "reference.dat", "0", "1", "100_100_130_cf_b.of"], "lbm_trace.log", "lbm_output.txt")],
}

SUPPORTED_BENCHMARKS = " ".join(SPEC_BENCHMARK_TO_QEMU_COMMAND_MAP.keys())

def resolve_input_path(arg, benchmark, run_dir):
    local_path = os.path.join(run_dir, arg)
    data_path = os.path.join(DATA_DIR, benchmark, "data", "train", "input", arg)
    return arg if os.path.exists(local_path) else data_path if os.path.exists(data_path) else arg

def run_qemu_trace(benchmark, output_dir, compress=False, timeout=None):
    if benchmark not in SPEC_BENCHMARK_TO_QEMU_COMMAND_MAP:
        print(f"[ERROR] Unknown benchmark: {benchmark}")
        print(f"[INFO] Supported benchmarks: {SUPPORTED_BENCHMARKS}")
        return

    run_dir_base = os.path.join(SPEC_DIR, "benchspec/CPU", benchmark, "run")
    run_dirs = [d for d in os.listdir(run_dir_base) if "train" in d]
    if not run_dirs:
        print(f"[ERROR] No train input run directory found for {benchmark}")
        return
    run_dir = os.path.join(run_dir_base, run_dirs[0])

    benchmark_outdir = os.path.join(output_dir, benchmark)
    os.makedirs(benchmark_outdir, exist_ok=True)

    print(f"[INFO] Using run directory: {run_dir}")
    print(f"[INFO] Saving output files to: {benchmark_outdir}")

    for binary, args, trace_name, out_name in SPEC_BENCHMARK_TO_QEMU_COMMAND_MAP[benchmark]:
        binary_path = os.path.join(run_dir, binary)
        resolved_args = [resolve_input_path(arg, benchmark, run_dir) for arg in args]

        trace_path = os.path.join(benchmark_outdir, trace_name)
        out_path = os.path.join(benchmark_outdir, out_name)

        qemu_cmd = [QEMU_EXECUTABLE, "-d", "in_asm,exec,nochain", binary_path] + resolved_args

        print(f"[INFO] Running QEMU command: {' '.join(qemu_cmd)}")
        print(f"       Trace → {trace_path}")
        print(f"       Output → {out_path}")

        start_time = time.time()
        try:
            with open(trace_path, "w") as trace_log, open(out_path, "w") as stdout_log:
                subprocess.run(qemu_cmd, cwd=run_dir, stdout=stdout_log, stderr=trace_log, timeout=timeout)
            elapsed = time.time() - start_time
            print(f"[INFO] Completed in {elapsed:.2f} seconds")
        except subprocess.TimeoutExpired:
            elapsed = time.time() - start_time
            print(f"[WARN] Timeout expired after {elapsed:.2f} seconds")

        if os.path.exists(out_path) and os.path.getsize(out_path) == 0:
            print(f"[WARN] Output file '{out_path}' is empty. Deleting...")
            os.remove(out_path)

        if compress and os.path.exists(trace_path):
            compressed_path = f"{trace_path}.gz"
            with open(trace_path, 'rb') as f_in, gzip.open(compressed_path, 'wb') as f_out:
                shutil.copyfileobj(f_in, f_out)
            os.remove(trace_path)
            print(f"[INFO] Compressed trace → {compressed_path}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run SPEC CPU2017 benchmarks under QEMU and generate traces.")
    parser.add_argument("benchmark", nargs="?", help=str(SUPPORTED_BENCHMARKS))
    parser.add_argument("--all", action="store_true", help="Run all supported benchmarks")
    parser.add_argument("--outdir", type=str, default=os.getcwd(), help="Directory to save output logs")
    parser.add_argument("--compress", action="store_true", help="Gzip compress trace files after run")
    parser.add_argument("--timeout", type=int, default=5, help="Timeout for each benchmark (in minutes)")

    args = parser.parse_args()

    timeout_seconds = args.timeout * 60

    if args.all:
        for benchmark in SPEC_BENCHMARK_TO_QEMU_COMMAND_MAP.keys():
            run_qemu_trace(benchmark, args.outdir, args.compress, timeout_seconds)
    elif args.benchmark:
        run_qemu_trace(args.benchmark, args.outdir, args.compress, timeout_seconds)
    else:
        print("[ERROR] Please provide a benchmark name or use --all")
        parser.print_help()
