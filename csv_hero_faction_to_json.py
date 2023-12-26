import csv
import json
import sys

def csv_to_json(csv_file, json_file):
    # Read the CSV and add the data to a dictionary
    data = {}
    with open(csv_file, mode='r', encoding='utf-8') as file:
        csv_reader = csv.DictReader(file)
        for row in csv_reader:
            hero_name = row['Hero']
            factions = [faction for faction, value in row.items() if value and faction != 'Hero']
            data[hero_name] = factions

    # Write the data to a JSON file
    with open(json_file, mode='w', encoding='utf-8') as file:
        json.dump(data, file, indent=4)

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python script.py <csv_file>")
        sys.exit(1)

    csv_file = sys.argv[1]
    json_file = csv_file.split('.')[0] + '.json'
    csv_to_json(csv_file, json_file)
