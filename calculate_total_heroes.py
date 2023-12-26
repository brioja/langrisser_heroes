import json
import sys
from itertools import combinations

def read_json(file_path):
    with open(file_path, 'r') as file:
        return json.load(file)

def count_heroes_by_faction(heroes_data):
    faction_counts = {}
    for factions in heroes_data.values():
        for faction in factions:
            faction_counts[faction] = faction_counts.get(faction, 0) + 1
    return faction_counts

def assign_factions_to_squads(faction_counts):
    sorted_factions = sorted(faction_counts, key=faction_counts.get, reverse=True)
    squads = {}
    for i in range(4):
        squads[f"Squad {i+1}"] = sorted_factions[i*2:(i+1)*2]
    return squads

def calculate_total_heroes(squads, heroes_data):
    total_heroes = 0
    for squad, factions in squads.items():
        heroes_set = set()
        for hero, hero_factions in heroes_data.items():
            if any(faction in factions for faction in hero_factions):
                heroes_set.add(hero)
        total_heroes += len(heroes_set)
    return total_heroes

def main():
    if len(sys.argv) < 2:
        print("Usage: python script.py <path_to_json_file>")
        return

    file_path = sys.argv[1]
    heroes_data = read_json(file_path)
    faction_counts = count_heroes_by_faction(heroes_data)
    squads = assign_factions_to_squads(faction_counts)
    total_heroes = calculate_total_heroes(squads, heroes_data)

    print("Squad Assignments:", squads)
    print("Total number of heroes in all squads:", total_heroes)

if __name__ == "__main__":
    main()
