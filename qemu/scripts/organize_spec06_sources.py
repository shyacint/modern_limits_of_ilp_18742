import os
import shutil

# Set this to your SPEC CPU2006 directory
spec_dir = os.path.expanduser('~/speccpu2006')

# List all benchmark directories (skip non-benchmark files)
for item in os.listdir(spec_dir):
    item_path = os.path.join(spec_dir, item)

    # Only process benchmark directories like 400.perlbench
    if os.path.isdir(item_path) and item[0].isdigit():
        src_dir = os.path.join(item_path, 'src')

        # Create src/ directory if it doesn't exist
        if not os.path.exists(src_dir):
            os.makedirs(src_dir)
            print(f'Created src/ in {item}')

        # Move all files (but not the src/ folder itself) into src/
        for f in os.listdir(item_path):
            f_path = os.path.join(item_path, f)
            if f != 'src':  # Skip src directory
                dest = os.path.join(src_dir, f)
                shutil.move(f_path, dest)
                print(f'Moved {f} -> {src_dir}')

print("✅ Done organizing SPEC sources.")
