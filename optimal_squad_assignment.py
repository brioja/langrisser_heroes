import json
import itertools
import sys

def read_json(file_path):
    with open(file_path, 'r') as file:
        return json.load(file)

def get_all_factions(heroes_data):
    factions = set()
    for hero_factions in heroes_data.values():
        factions.update(hero_factions)
    return list(factions)

def calculate_heroes_in_squads(squads, heroes_data):
    heroes_in_squads = set()
    for squad in squads:
        for hero, hero_factions in heroes_data.items():
            if any(faction in squad for faction in hero_factions):
                heroes_in_squads.add(hero)
    return len(heroes_in_squads)

def find_optimal_squad_assignments(heroes_data):
    factions = get_all_factions(heroes_data)
    max_heroes = 0
    best_assignment = None

    total_combinations = sum(1 for _ in itertools.combinations(factions, 8))
    processed_combinations = 0

    for faction_combination in itertools.combinations(factions, 8):
        for squads in itertools.permutations(faction_combination, 8):
            squads = [squads[i:i+2] for i in range(0, 8, 2)]
            num_heroes = calculate_heroes_in_squads(squads, heroes_data)
            if num_heroes > max_heroes:
                max_heroes = num_heroes
                best_assignment = squads

        processed_combinations += 1
        progress_percentage = (processed_combinations / total_combinations) * 100
        print(f"Progress: {progress_percentage:.2f}%", end='\r')

    return best_assignment, max_heroes

def main():
    if len(sys.argv) < 2:
        print("Usage: python script.py <path_to_json_file>")
        sys.exit(1)

    file_path = sys.argv[1]
    heroes_data = read_json(file_path)
    optimal_assignment, max_heroes = find_optimal_squad_assignments(heroes_data)

    print("\nOptimal Squad Assignments:", optimal_assignment)
    print("Maximum number of heroes covered:", max_heroes)

if __name__ == "__main__":
    main()
