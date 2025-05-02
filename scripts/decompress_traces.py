import gzip
import shutil
import os

def decompress_gz_in_dir(directory):
    for root, _, files in os.walk(directory):  # Recursively walk through the directory
        for filename in files:
            if filename.endswith(".gz"):
                gz_path = os.path.join(root, filename)
                output_path = os.path.join(root, filename[:-3])  # remove .gz extension

                print(f"[INFO] Decompressing: {gz_path} → {output_path}")

                with gzip.open(gz_path, 'rb') as f_in, open(output_path, 'wb') as f_out:
                    shutil.copyfileobj(f_in, f_out)

                # Optionally, delete the .gz file
                # os.remove(gz_path)

if __name__ == "__main__":
    decompress_gz_in_dir("../qemu_dynamic_traces/final_spec06/")  # Replace with your directory path
