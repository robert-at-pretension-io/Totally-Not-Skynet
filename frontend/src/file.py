import os
import re

# set your project directory and output CSS file
PROJECT_DIRECTORY = '.'
OUTPUT_CSS_FILE = 'OUTPUT_CSS_FILE.css'

# regular expression to match <style> tag and contents
STYLE_REGEX = r'<style[^>]*>(.*?)</style>'

# open output file
with open(OUTPUT_CSS_FILE, 'w') as output:
    # recursively walk through the project directory
    for dirpath, dirnames, filenames in os.walk(PROJECT_DIRECTORY):
        for filename in filenames:
            if filename.endswith('.svelte'):
                # construct the full file path
                file_path = os.path.join(dirpath, filename)

                # read the svelte file
                with open(file_path, 'r') as svelte_file:
                    content = svelte_file.read()

                # search for <style> tag and contents
                match = re.search(STYLE_REGEX, content,
                                  re.DOTALL | re.IGNORECASE)

                if match:
                    # write the CSS content to the output file
                    css_content = match.group(1).strip()
                    output.write(f'/* {file_path} */\n{css_content}\n\n')
