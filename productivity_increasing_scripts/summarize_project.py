import os
import sys

def tree_to_string(startpath, exclusions, threshold):
    tree_str = ""
    prefix = ""

    def recurse_folder(folder_path, prefix, level=0):
        nonlocal tree_str
        if level == 0:
            tree_str += os.path.basename(startpath) + "\n"
        entries = os.listdir(folder_path)
        entries_count = len(entries)
        entries.sort()  # Optional: sort entries alphabetically

        if entries_count > threshold and level > 0:
            last_entry = prefix + "└── " + os.path.basename(folder_path) + " [excluded due to size]\n"
            tree_str += last_entry
            return  # Skip folder since it exceeds the threshold

        for i, entry in enumerate(entries):
            full_path = os.path.join(folder_path, entry)
            is_last = i == (entries_count - 1)
            if entry in exclusions or full_path in exclusions:
                tree_str += prefix + ("└── " if is_last else "├── ") + entry + " [excluded]\n"
                continue  # Skip excluded directories
            tree_str += prefix + ("└── " if is_last else "├── ") + entry + "\n"

            if os.path.isdir(full_path):
                new_prefix = prefix + ("    " if is_last else "│   ")
                recurse_folder(full_path, new_prefix, level + 1)

    recurse_folder(startpath, prefix)
    return tree_str

def read_exclusions(file_path):
    exclusions = []
    try:
        with open(file_path, 'r') as file:
            exclusions = [line.strip() for line in file if line.strip()]
    except FileNotFoundError:
        print(f"Exclusions file '{file_path}' not found.")
        sys.exit(1)
    return exclusions

if __name__ == "__main__":
    # Check for command-line argument
    if len(sys.argv) < 4:
        print("Usage: python file_tree.py [path] [exclusions_file] [threshold]")
        sys.exit(1)

    # The first command-line argument is the script's filename;
    # the second is the path; the third is the exclusions file path; the fourth is the entry threshold.
    input_path = sys.argv[1]
    exclusions_file = sys.argv[2]
    entry_threshold = int(sys.argv[3])  # Convert the threshold input to an integer

    # Read exclusion directories from the specified file
    exclusions = read_exclusions(exclusions_file)

    # Normalize the input path and check if it's a directory
    startpath = os.path.abspath(input_path)
    if not os.path.isdir(startpath):
        print(f"The given path '{input_path}' is not a directory or cannot be found.")
        sys.exit(1)

    # Generate and print the visual tree
    visual_tree = tree_to_string(startpath, exclusions, entry_threshold)
    print(visual_tree)

    # Save to a string
    with open('file_tree.txt', 'w') as file:
        file.write(visual_tree)