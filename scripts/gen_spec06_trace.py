import subprocess
import os
import argparse
import gzip
import shutil

# --- CONFIGURATION ---
SPEC_DIR = os.path.expanduser("~/riscv_build/CPU2006")

SPEC_BENCHMARK_TO_QEMU_COMMAND_MAP = {
    "403.gcc": [
        ("gcc", ["cccp.i", "-o", "cccp.s"], "gcc_trace.log", "gcc_output.txt"),
    ],
    "429.mcf": [
        ("mcf", ["inp.in"], "mcf_trace.log", "mcf_output.txt"),
    ],
    "458.sjeng": [
        ("sjeng", ["train.txt"], "sjeng_trace.log", "sjeng_output.txt"),
    ],
    "471.omnetpp": [
        ("omnetpp", ["-c", "General", "-r", "0"], "omnetpp_trace.log", "omnetpp_output.txt"),
    ],
    "453.povray": [
        ("povray", ["SPEC-benchmark-train.ini"], "povray_trace.log", "povray_output.txt"),
    ],
    "470.lbm": [
        ("lbm", ["300", "lbm.in", "0", "1", "100_100_130_cf_b.of"], "lbm_trace.log", "lbm_output.txt"),
    ],
}


SUPPORTED_BENCHMARKS = " ".join(SPEC_BENCHMARK_TO_QEMU_COMMAND_MAP.keys())

def run_qemu_trace(benchmark, output_dir, compress=False):
    if benchmark not in SPEC_BENCHMARK_TO_QEMU_COMMAND_MAP:
        print(f"[ERROR] Unknown benchmark: {benchmark}")
        print(f"[INFO] Supported benchmarks: {SUPPORTED_BENCHMARKS}")
        return

    exe_dir = os.path.join(SPEC_DIR, benchmark, "exe")
    if not os.path.exists(exe_dir):
        print(f"[ERROR] Executable directory not found: {exe_dir}")
        return

    benchmark_outdir = os.path.join(output_dir, benchmark)
    os.makedirs(benchmark_outdir, exist_ok=True)

    print(f"[INFO] Using executable directory: {exe_dir}")
    print(f"[INFO] Saving output files to: {benchmark_outdir}")

    for binary, args, trace_name, out_name in SPEC_BENCHMARK_TO_QEMU_COMMAND_MAP[benchmark]:
        binary_path = os.path.join(exe_dir, binary)
        trace_path = os.path.join(benchmark_outdir, trace_name)
        out_path = os.path.join(benchmark_outdir, out_name)

        qemu_cmd = ["qemu-riscv64", "-d", "in_asm,cpu", binary_path] + args

        print(f"[INFO] Running QEMU command:")
        print(f"       {' '.join(qemu_cmd)}")
        print(f"       Trace → {trace_path}")
        print(f"       Output → {out_path}")

        with open(trace_path, "w") as trace_log, open(out_path, "w") as stdout_log:
            subprocess.run(qemu_cmd, stdout=stdout_log, stderr=trace_log)

        if os.path.exists(out_path) and os.path.getsize(out_path) == 0:
            print(f"[WARN] Output file '{out_path}' is empty. Deleting...")
            os.remove(out_path)

        if compress and os.path.exists(trace_path):
            compressed_path = f"{trace_path}.gz"
            with open(trace_path, 'rb') as f_in, gzip.open(compressed_path, 'wb') as f_out:
                shutil.copyfileobj(f_in, f_out)
            os.remove(trace_path)
            print(f"[INFO] Compressed trace → {compressed_path}")

# --- ENTRY POINT ---

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run SPEC CPU2006 benchmarks under QEMU and generate traces.")
    parser.add_argument("benchmark", nargs="?", help=str(SUPPORTED_BENCHMARKS))
    parser.add_argument("--all", action="store_true", help="Run all supported benchmarks")
    parser.add_argument("--outdir", type=str, default=os.getcwd(), help="Directory to save output logs")
    parser.add_argument("--compress", action="store_true", help="Gzip compress trace files after run")

    args = parser.parse_args()

    if args.all:
        for benchmark in SPEC_BENCHMARK_TO_QEMU_COMMAND_MAP.keys():
            run_qemu_trace(benchmark, args.outdir, args.compress)
    elif args.benchmark:
        run_qemu_trace(args.benchmark, args.outdir, args.compress)
    else:
        print("[ERROR] Please provide a benchmark name or use --all")
        parser.print_help()
