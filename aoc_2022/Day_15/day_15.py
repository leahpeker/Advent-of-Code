import numpy as np
import sys
import re
from collections import deque, defaultdict
np.set_printoptions(threshold=sys.maxsize, linewidth=3000)


class Sensor:
    def __init__(self, line, lowest_x, lowest_y):
        self.s_x, self.s_y, self.b_x, self.b_y = line
        self.normalize_for_grid(lowest_x, lowest_y)
        self.manhattan_distance = abs(self.s_x - self.b_x) + abs(self.s_y - self.b_y)
        self.tuple_dict = {(self.s_x, self.s_y): set()}

    def normalize_for_grid(self, lowest_x, lowest_y):
        self.s_x -= lowest_x
        self.b_x -= lowest_x
        self.s_y -= lowest_y
        self.b_y -= lowest_y
    def look_everywhere(self, grid_height, grid_width):
        self.look(grid_height, grid_width, 1, 1)
        self.look(grid_height, grid_width, 1, -1)
        self.look(grid_height, grid_width, -1, 1)
        self.look(grid_height, grid_width, -1, -1)

    def look(self, grid_height, grid_width, x_direction, y_direction):
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
                    self.tuple_dict[(self.s_x, self.s_y)].add((current_x, y_coord))
            starting_y -= y_direction
            starting_x += x_direction

class Map:
    def __init__(self, input_lines):
        self.lines = input_lines
        self.x_dim = 0
        self.y_dim = 0
        self.lowest_x = 0
        self.lowest_y = 0
        self.sensor_queue = deque()
        self.parse_and_pull_info()
        self.map = self.build_map()
        self.layout_sensors_and_beacons()
        self.mark_the_map()
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
        print(self.x_dim, self.y_dim)

    def build_map(self):
        return np.full((self.y_dim, self.x_dim), '.')

    def layout_sensors_and_beacons(self):
        for line in self.lines:
            sensor = Sensor(line, self.lowest_x, self.lowest_y)
            self.map[sensor.s_y, sensor.s_x] = 'S'
            self.map[sensor.b_y, sensor.b_x] = 'B'
            self.sensor_queue.append(sensor)

    def mark_the_map(self):
        while self.sensor_queue:
            current_sensor = self.sensor_queue.pop()
            current_sensor.look_everywhere(self.y_dim, self.x_dim)
            for x, y in current_sensor.tuple_dict[current_sensor.s_x, current_sensor.s_y]:
                if self.map[y, x] == '.':
                    self.map[y, x] = '#'

    def count_where_its_not(self, line_number):
        count = 0
        line_number = line_number - self.lowest_y
        for x_coord in range(self.x_dim):
            if self.map[line_number, x_coord] == '#':
                count += 1
        print(count)


with open('test.txt') as file:
    lines = [line.strip() for line in file.readlines()]

map = Map(lines)
map.count_where_its_not(10)
print(map.map)

#[(0, 10), (1, 10), (2, 10), (3, 10), (4, 10), (5, 10), (6, 10), (7, 10), (8, 10), (9, 10), (10, 10), (11, 10), (12, 10), (13, 10), (14, 10), (15, 10), (16, 10), (17, 10), (18, 10), (19, 10), (20, 10), (21, 10), (22, 10), (23, 10), (24, 10), (25, 10), (26, 10)]