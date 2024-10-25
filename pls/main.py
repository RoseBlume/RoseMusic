import os
import json
import re

def parse_pls_file(file_path):
    with open(file_path, 'r') as file:
        lines = file.readlines()
    
    data = {}
    for line in lines:
        if line.startswith('File'):
            data['src'] = line.split('=')[1].strip()
        elif line.startswith('Title'):
            title_match = re.search(r'\) (.+)', line)
            if title_match:
                data['title'] = title_match.group(1).strip()
    
    data['image'] = 'covers/default.png'
    return data

def convert_pls_to_json(directory):
    pls_files = [f for f in os.listdir(directory) if f.endswith('.pls')]
    json_data = []

    for pls_file in pls_files:
        file_path = os.path.join(directory, pls_file)
        parsed_data = parse_pls_file(file_path)
        json_data.append(parsed_data)
    
    return json_data

def save_json(data, output_file):
    with open(output_file, 'w') as file:
        json.dump(data, file, indent=4)

if __name__ == "__main__":
    directory = '..\pls'  # Replace with the path to your directory
    output_file = 'output.json'
    
    json_data = convert_pls_to_json(directory)
    save_json(json_data, output_file)
    print(f"JSON data saved to {output_file}")