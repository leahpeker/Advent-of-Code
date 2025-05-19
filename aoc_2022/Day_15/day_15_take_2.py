### try again considering that we ONLY care about spots on a given line ###
import re


class Sensor:
    def __init__(self, line, lowest_x, lowest_y):
        self.s_x, self.s_y, self.b_x, self.b_y = line
        self.normalize_for_grid(lowest_x, lowest_y)
        self.manhattan_distance = abs(self.s_x - self.b_x) + abs(self.s_y - self.b_y)
        # self.tuple_dict = {(self.s_x, self.s_y): set()}
        self.tuple_set = set()

    def normalize_for_grid(self, lowest_x, lowest_y):
        self.s_x -= lowest_x
        self.b_x -= lowest_x
        self.s_y -= lowest_y
        self.b_y -= lowest_y
    def look_everywhere(self, grid_height, grid_width, search_line, tuple_set):
        if self.s_y == search_line:
            tuple_set.add((self.s_x, self.s_y))
        self.look(grid_height, grid_width, 1, 1, search_line, tuple_set)
        self.look(grid_height, grid_width, 1, -1, search_line, tuple_set)
        self.look(grid_height, grid_width, -1, 1, search_line, tuple_set)
        self.look(grid_height, grid_width, -1, -1, search_line, tuple_set)

    def look(self, grid_height, grid_width, x_direction, y_direction, search_line, tuple_set):
        starting_y = self.s_y + (self.manhattan_distance) * y_direction
        starting_x = self.s_x
        while abs(starting_y - self.s_y) <= self.manhattan_distance and abs(starting_x - self.s_x) <= self.manhattan_distance:
            current_x = starting_x
            current_y = starting_y
            while current_y < 0 or current_y >= grid_height:
                current_y -= y_direction
            while current_x < 0 or current_x >= grid_width:
                current_x -= x_direction
            for y_coord in range(self.s_y, current_y + y_direction, y_direction):
                if y_coord == search_line:
                    # self.tuple_dict[(self.s_x, self.s_y)].add((current_x, y_coord))
                    tuple_set.add((current_x, y_coord))
                    # print('hi', len(tuple_set), sorted(tuple_set))
            # print(search_line)
            starting_y -= y_direction
            starting_x += x_direction

    def beacon_on_search_line(self, search_line, beacon_set):
        if self.b_y == search_line:
            beacon_set.add((self.b_x, self.b_y))

class Map:
    def __init__(self, input_lines, search_line):
        self.lines = input_lines
        self.x_dim = 0
        self.y_dim = 0
        self.lowest_x = 0
        self.lowest_y = 0
        self.search_line = search_line - self.lowest_y
        # self.sensor_queue = deque()
        self.parse_and_pull_info()

    def parse_and_pull_info(self):
        parsed_lines = []
        x_set = set()
        y_set = set()
        for line in self.lines:
            line = re.compile('=(-?[0-9]*)').findall(line)
            int_numbers = []
            for i, number in enumerate(line):
                int_number = int(number)
                if i == 0 or i == 2:
                    x_set.add(int_number)
                else:
                    y_set.add(int_number)
                int_numbers.append(int_number)
            parsed_lines.append(int_numbers)
        self.x_dim = max(x_set) - min(x_set) + 1
        self.y_dim = max(y_set) - min(y_set) + 1
        self.lowest_x = min(x_set)
        self.lowest_y = min(y_set)
        self.lines = parsed_lines
        # print(self.x_dim, self.y_dim)

    def initialize_sensors(self):
        set_of_non_beacons = set()
        beacon_set = set()
        for line in self.lines:
            sensor = Sensor(line, self.lowest_x, self.lowest_y)
            sensor.look_everywhere(self.y_dim, self.x_dim, self.search_line, set_of_non_beacons)
            sensor.beacon_on_search_line(self.search_line, beacon_set)
        # print(len(set_of_non_beacons) - len(beacon_set))

with open('input.txt') as file:
    lines = [line.strip() for line in file.readlines()]

input_line = 2000000
test_line = 10
map = Map(lines, input_line)
map.initialize_sensors()