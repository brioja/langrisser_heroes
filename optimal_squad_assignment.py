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

def calculate_total_heroes(squads, heroes_data):
    total_heroes = 0
    for squad in squads:
        for hero, hero_factions in heroes_data.items():
            if any(faction in squad for faction in hero_factions):
                total_heroes += 1
    return total_heroes

def find_optimal_squad_assignments(heroes_data, num_squads):
    factions = get_all_factions(heroes_data)
    max_heroes = 0
    best_assignment = None

    total_combinations = sum(1 for _ in itertools.combinations(factions, num_squads * 2))
    processed_combinations = 0

    for faction_combination in itertools.combinations(factions, num_squads * 2):
        for squads in itertools.permutations(faction_combination, num_squads * 2):
            squads = [squads[i:i+2] for i in range(0, num_squads * 2, 2)]
            num_heroes = calculate_total_heroes(squads, heroes_data)
            if num_heroes > max_heroes:
                max_heroes = num_heroes
                best_assignment = squads

        processed_combinations += 1
        progress_percentage = (processed_combinations / total_combinations) * 100
        print(f"Progress: {progress_percentage:.2f}%", end='\r')

    return best_assignment, max_heroes

def main():
    if len(sys.argv) < 3:
        print("Usage: python script.py <path_to_json_file> <number_of_squads>")
        sys.exit(1)

    file_path = sys.argv[1]
    num_squads = int(sys.argv[2])

    heroes_data = read_json(file_path)
    optimal_assignment, max_heroes = find_optimal_squad_assignments(heroes_data, num_squads)

    print("\nOptimal Squad Assignments:", optimal_assignment)
    print("Total number of heroes (including duplicates):", max_heroes)

if __name__ == "__main__":
    main()

