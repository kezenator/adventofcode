#include <iostream>
#include <string>
#include <set>
#include <map>
#include <deque>
#include <boost/algorithm/string.hpp>

const std::string input = R"(#########################################################################################################################################################################################
#.#.....#.........#.....#.......#.#...........#...#.....................#...................#.....#.......#.#...#...................#...#...........#...#...#...#...#...#...#.....#...#.#
#.###.#.#.###.#.#.#.#.#.#########.#.#.###.#.#.#.###.#.#.#.#####.#.###.###.#.###.#.#.#.###.#.#.#.#.#.#.#.#.#.#.#.#.###.###.#.#.#.#.#.#.###.###.#.#.#.#.#.#.#####.#.#.#.#.#.#.#.#.#.#.#.#.#
#.#...#.#...#...#...#.........#...#...#...#.............#.......#.....#.....#...#.#.......#.......#.........#.#.#...#...#...#.....#...#...#.#.#.#..3#.....#.........#.#...#.....#.#...#.#
#.#.#.#.#.#.#.#.#.#.#.###.###.#.#####.#.#.#####.#.###.#####.#.###.#.###.#.###.#####.###.###.#.###.#.#.#.###.#####.###.###.###.#.###.#.#####.#.###.#.#.###.###.#.###.#.###.#.###.###.#.#.#
#.....#.....#...#.....#...#.#.#.#...#...#...#.#...#...#.......#...#.....#...#.....#.......#.#...#.....#...#.#...#.....#.....#.#.#...#.#.......#.......#...#...#.#.#.#.#.#...#.......#.#.#
#.###.#.###.#.###.#.#.#.#.#.#####.#.#.#.#####.#.#.#.#.#.#.#.###.#.#.###.#.#.#.###.###.###.#.###.#.#.#.###.#.#.###.#.#####.#.#.#.#.#.#.#.#####.#.#.#.###.#.#.#.###.#.#.#.###.#.#######.#.#
#...#.#.#5#.#.........#.....#.....#.#.#.#...........#.#.....#.#...#.........#.....#...#.#...#.#.#.#...#.....#...#.#.....#...#...#.#.#...#.#.....#...#...#.....#.......#...#...#.....#...#
#.#.#.###.#.#########.###.#.###.#.#.#.###.#.#######.#.#.#.#.#.#.#######.###.#.#.#.#.###.#.###.#.#.#.###.#.#####.###.###.###.#.#.#.#.#.#.#.###.#####.#######.#.#.#.###.#.#.###.#.###.###.#
#...#.#.......#.#.......#.#...#...#.....#.#.#.....#.#...#...#...#.....#.....#.#...#.#.........#.....#...........#.......#.....#...#.#.....#.......#.#...#...#.#.......#.#.#.#.#.........#
###.#.#.###.#.#.#.#.#.#.#.#.#.#.#######.#.#.#.#.###.#.#.###.#.#.#.#.#.#.#####.#####.#.#.#.###.#.#####.#.#.#.###.#.#.#.###.#.#.#.#.#.#.#.#.#.#.###.###.#.#.#.#.#.#.#.#.#####.###.#######.#
#.#...#.#...#.#.....#...#...#...#.....#.........#.............#.......#.............#.........#.......#...#...#...#.......#.....#.......#...#.....#.....#.........#.#.#...#.#...#.....#6#
#.#.#.#.###.#.#.###.#.#.#.#.#.###.#.#.#.#.#####.#.#.###.#.###.#.#.#######.#.#####.#.#######.#####.#######.#.###.#.#####.#.#.#.#.#.#####.#.#.#.#.#.#######.###.#.###.#.#.#.#.#.#.#.#.#.#.#
#...#.#...#.#...........#...#.......#...#...#.......#...#.#.....#.#.................#...#...#.#.....#.....#.#.......#...#.....#...#.#...#.......#.#.#...#.......#...#.#.#.........#.....#
#####.###.#.###.#####.###.#.#.#.#####.#.###.#.#.#####.#.#.#.#.#.###.#####.###.#.#####.###.#.#.#.#.#.###.###.#######.#.#.#.#.#.#.#.#.###.###.###.###.#.#.#.#.#####.#.#.###.#.#####.###.###
#...#.#.........#.#...#.#...........#.#...#...#.#...#...#...#.......#...#...#...............................#.....#...#...#.....#...#.#.#.#...#.#.......#.....#.....#.#.#...#...#...#...#
###.#.#.#########.#.###.#.#.###.#######.#.###.#.#.###.#.#.###.#####.###.#.#.#####.#.#.#.#.###.###.#.#.#.#.#.#.#######.#.#.#.###.###.#.#.#.#.#.###.###.###.###.#####.#.#.#.#.###.#.#.#.#.#
#.#...#.#.#.#.......#...#.....#.#...#...#.....#.............#.#...#.#...#.#.......#.......#...................#.....#.#.....#.#...#.#...#.#...#...#...#...#.....#.#...#.........#...#...#
#.#.#.###.#.#########.#.#.#.#.#.#.#.#.###.#.###.#.#.#.#.#######.#.#####.#####.#.#.###.###.#.#.#.###.#####.###.#.#.#.#.#.#.###.#####.#####.#.#.#####.#######.###.#.#.#.#.#.#####.###.#.#.#
#...#.#.....#.#.#4#.#...#...#.........#.#.#...#...#.#.#.....#...#.#.....#.....#.......#...#.#.#.......#...#.#...#.....#.#.....#.........#.#...#.........#...#.#...#...............#.#...#
#####.#.#.###.#.#.#.#.#.###.#.#.###.###.#.#.#.#.#.#.###.###.#.#.#.#.#######.###.###.#.###.#.#####.#.###.#.#.###.#####.#.#######.#.#######.###.#.#.###.#.#.###.#.#########.#.###.#.#.###.#
#...#.....#.....#.......#.#...#.#.#.#.....#...#.#.#...........#.#.............#.#.......#.#.#...........#.............#.....#.#.......#...#.....#.#...#.#...#.#.........#.#...#.#...#...#
#.#.#.###.#########.#.#.#.#.#.###.###.###.#######.#.#.###.#.#.#.#.###.#.#.#.#.###.#####.###.#####.#.###.#.#.###.#.#.#.#####.#.#.#.#.###.#.#.#.###.#.###.#.#.#.#.###.###.#.#.#######.#.###
#...#.....#.#.......#...#.#...#.......#.......#...#.#...#...#.#.#...........#...#.......#...#.......#.#.....#.....#...#.......#...#.........#.....#.#.#.#.#...#.#.........#...........#.#
#.#.###.#.#.#.#.#.#.#.#.#.#.#.#.###.#.#.#.#.#.#.#.###.###.###.#.#.#.#####.###.#.#.#########.#.#.###.#.#.#.###.###.#.#.###.#.#.#.#.###.#.#.#.#.#.#.#.#.#.#.#.###.#.###.#.#.#.#####.#.#.#.#
#.#.....#.#...#.#.#.#.#...#.......#.#...#.#.#.#...#...#.......#.......#...#...#.#...#.....#.#...#...#...#.........#...#.#.#.....#.#.....#.....#.............#...#.......#...........#...#
#.#.#.#.#.#.###.###.###.###.#.###.#.#.#.#.#.#.#.#.#.#######.#####.#.#.#####.#.#.###.#.#.#####.#.#####.#.#.#.#.#.#.###.#.###.#.###.#.#.###.###.#.#.#.#.#.###.#.#.#.###.###.#.#.#####.#####
#...#.........#.....#.#.....#.....#.#...#.#...#...........#.#.......#.........#...............#.#...#...#.#...#.....#.#.#.....#...#.......#...#.....#...#...#.....#...#...#...#7#.......#
#.#.#.###.#######.###.#.#####.#.###.#.###.#.#######.#.#.#.#.###.###.#.#.#####.#.#.#.###.#.#.###.#.#.#####.#.#.###.#.#.#.#.#.###.#.#.#.###.#.#.###.#####.#.###.#.###.#.###.#.#.#.#######.#
#...#.#.............#.#...#0..#.#...........#.#.............#...#...#...#.......#.....#...#.......#...#.....#.....#.#.#.#...#.#.#.#...#...#...#.....#.#.#...........#.....#.....#.#.....#
###.#.###.#.#.#.#.#.#.###.#.#.#.###.###.#####.#.#####.###.#.#.#.#.#####.#.#.#.#####.#####.#.#.#.#.#.#####.#.#.#.#.#.#.#.#.#.#.#.#.#.#####.#.###.#####.#.#.###.#######.#.#.#.#####.#.#.#.#
#.#.....#...#.......#.....#.....#...#.....#...........#...#.....#.#.....#.......#.....#.....#.............#.#...#.....#.#.........#...#...#.#.............#...#...#...#.#...#.#.......#.#
#.#.#.#.#.###.###.###.#.#.#.#######.#.#.#.#####.#.###.#.###.#.#.#.#.###.#.###.#.#.###.#.#.#.#########.#.#.#.###.#.#.###.#.#######.#.###.#.#.#.#.#.#.###.#.#.#.###.#.#.#.#####.#.#.###.###
#.....#...#.......#.....#...#.#.....#...#.........#...#.....#.#...#.......#...............#.#.......#.......#.#...........#.....#.......#.#.#.#.#...#.......#.....#.....#...#.......#...#
#.###.###.#.#######.#.#.#.#.#.#.#.#.###.#.#######.#.###.#.#####.###.#.###.###.#####.#.###.###.#.#.#.#####.#.#.#.#########.#.#.#.#####.#.#.#.###.#.###.#.###.#.###.###.#.#.#.#.#.#.#.#.#.#
#.#.#.......#.#.#...#.....#.....#.#...#.....#...#.....#.#.#...........#...#...#...#.....#...#.......#.#.....#.........#...#.#...........#.#.#...#...#......1#.....#...#.#...#...#...#...#
#.#.#.#.#####.#.#.###.#####.#####.#.#.###.###.#.#####.###.#.#.#.#.#####.###.###.#.#.###.###.#######.#.###.#.#.###.#.#.#.###.#.###########.#.#.###.#######.#######.#.###.###.#.#.#.###.#.#
#...#.........#.....#...#.....#.....#...#.....#.......#.#...#...#.#.......#.....#...#.........#.....#...#.#.#...............#.............#.......#.........#.........#...........#.....#
#.#.#.###.#.#.#.###.###.#.#####.#####.#.###.#.###.#.#.#.###.#.#####.#.###.#.#.#.#.#.#.#.###.###.###.###.#.#.#.#.#.#.#####.###.#.#.#######.#.#.#.###.#.###.#.#.#.###.#####.###.###.#.#.###
#.........#.....#.......#....2#.........#.....#.....#.#.......#.#.#.......#.#...#...........#.#.#...#...#.#...#.....#.....#.#.#.#.#.......#...#.#.#.#.........#.#.........#.#...#...#.#.#
#########################################################################################################################################################################################)";

using Position = std::pair<int, int>;
using PositionAndDistance = std::pair<Position, int>;

void get_locations(std::set<char> &locations, std::map<char, Position> &positions)
{
    std::vector<std::string> lines;
    boost::split(lines, input, boost::is_any_of("\n"));

    for (int y = 0; y < lines.size(); ++y)
    {
        const std::string &line = lines[y];

        for (int x = 0; x < line.size(); ++x)
        {
            char ch = line[x];

            if ((ch >= '0' && (ch <= '9')))
            {
                locations.insert(ch);
                positions[ch] = Position(y, x);
            }
        }
    }
}

std::vector<Position> next_positions(const Position &pos, std::vector<std::string> &lines)
{
    std::vector<Position> result;

    if (pos.first > 0)
        result.push_back(Position(pos.first - 1, pos.second));
    if (pos.first < (lines.size() - 1))
        result.push_back(Position(pos.first + 1, pos.second));

    if (pos.second > 0)
        result.push_back(Position(pos.first, pos.second - 1));
    if (pos.second < (lines[pos.first].size() - 1))
        result.push_back(Position(pos.first, pos.second + 1));

    return result;
}

int shortest_path_between(const char start_loc, const char end_loc, std::map<char, Position> &positions)
{
    std::vector<std::string> lines;
    boost::split(lines, input, boost::is_any_of("\n"));

    const int width = lines[0].size();
    const int height = lines.size();

    std::map<Position, int> visited_shortest_paths;

    std::deque<PositionAndDistance> positions_to_test;
    positions_to_test.push_back(PositionAndDistance(positions[start_loc], 0));

    int max_distance = std::numeric_limits<int>::max();

    //std::cout << "Finding shortest distance from " << start_loc << " to " << end_loc << std::endl;

    while (!positions_to_test.empty())
    {
        const Position cur_pos = positions_to_test.front().first;
        const int cur_distance = positions_to_test.front().second;

        positions_to_test.pop_front();

        char ch = lines[cur_pos.first][cur_pos.second];

        //std::cout << "   Testing (" << cur_pos.first << ", " << cur_pos.second << "), distance = " << cur_distance
        //    << ", ch = " << ch << ", max_distance = " << max_distance << std::endl;

        if (ch == '#')
        {
            // It's a wall - ignore
        }
        else if (cur_distance > max_distance)
        {
            // Already traveled too far
        }
        else if ((visited_shortest_paths.find(cur_pos) != visited_shortest_paths.end())
            && (visited_shortest_paths[cur_pos] <= cur_distance))
        {
            // Already been here via a shorter path
        }
        else
        {
            visited_shortest_paths[cur_pos] = cur_distance;

            if (ch == end_loc)
            {
                // We've got to the end - and we can't have got
                // here via a shorter path - so accept this as
                // the best answer found so far

                max_distance = cur_distance;
            }
            else
            {
                // We've made it to a new location - now we need to
                // test all the surrounding locations

                std::vector<Position> next = next_positions(cur_pos, lines);

                for (const Position &next_pos : next)
                {
                    positions_to_test.push_back(PositionAndDistance(next_pos, cur_distance + 1));
                }
            }
        }
    }

    return max_distance;
}

int path_distance(const std::string &path, std::map<std::pair<char, char>, int> &distances)
{
    int result = 0;

    for (int i = 1; i < path.size(); ++i)
    {
        result += distances[std::make_pair(path[i - 1], path[i])];
    }

    return result;
}

int distance_to_visit_all(std::set<char> &locations, std::map<std::pair<char, char>, int> &distances, bool return_to_start)
{
    // First, find the distance of a naieve path so that we
    // have an upper limit to the search

    std::string naieve_path;
    for (char ch : locations)
        naieve_path.push_back(ch);

    if (return_to_start)
        naieve_path.push_back('0');

    int max_distance = path_distance(naieve_path, distances);

    std::cout << "Naieve distance \"" << naieve_path << "\" = " << max_distance << std::endl;

    // Now, keep recursively searching looking for shorter paths
    // and limiting the search space by the naieve distance

    using PathAndDistance = std::pair<std::string, int>;

    std::deque<PathAndDistance> to_search;
    to_search.push_back(PathAndDistance("0", 0));

    while (!to_search.empty())
    {
        const std::string cur_path = to_search.front().first;
        const int cur_distance = to_search.front().second;

        to_search.pop_front();

        if (cur_distance >= max_distance)
        {
            // Already too long
        }
        else
        {
            std::set<char> visited_locations;
            for (char ch : cur_path)
                visited_locations.insert(ch);

            if ((visited_locations == locations)
                && (!return_to_start
                    || (cur_path.back() == '0')))
            {
                // We've found a shorter path

                std::cout << "Shorter path \"" << cur_path << "\" = " << cur_distance << std::endl;

                max_distance = cur_distance;
            }
            else
            {
                // Keep searching

                for (char ch : locations)
                {
                    if (ch != cur_path.back())
                    {
                        const std::string new_path = cur_path + ch;
                        const int new_distance = cur_distance + distances[std::make_pair(cur_path.back(), ch)];

                        to_search.push_back(PathAndDistance(new_path, new_distance));
                    }
                }
            }
        }
    }

    return max_distance;
}

int main(int argc, char *argv[])
{
    // Find what locations we need to visit and the
    // position of each of these in the map

    std::set<char> locations;
    std::map<char, Position> positions;
    
    get_locations(locations, positions);

    // Now, work out the shortest distance
    // between each pair of locations - this
    // is a breath first search through the map
    // but no back-tracking is allowed.

    std::map<std::pair<char, char>, int> distances;

    for (char start_loc : locations)
    {
        for (char end_loc : locations)
        {
            if (end_loc > start_loc)
            {
                int distance = shortest_path_between(start_loc, end_loc, positions);

                distances[std::make_pair(start_loc, end_loc)] = distance;
                distances[std::make_pair(end_loc, start_loc)] = distance;

                //std::cout << "distance " << start_loc << " => " << end_loc << " = " << distance << std::endl;
            }
        }
    }

    // Finally, now we can run a brute-force
    // traveling salesman algorithm to find the
    // shortest path that visits each location.
    // At this point we've ditched the complex
    // map data so a brute force approach is OK.

    int answer1 = distance_to_visit_all(locations, distances, false);

    std::cout << "Answer #1: " << answer1 << std::endl;

    int answer2 = distance_to_visit_all(locations, distances, true);

    std::cout << "Answer #2: " << answer1 << std::endl;
}
