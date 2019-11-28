#include <iostream>
#include <string>
#include <sstream>
#include <vector>
#include <set>
#include <boost/algorithm/string.hpp>

const std::string input = "L5, R1, R4, L5, L4, R3, R1, L1, R4, R5, L1, L3, R4, L2, L4, R2, L4, L1, R3, R1, R1, L1, R1, L5, R5, R2, L5, R2, R1, L2, L4, L4, R191, R2, R5, R1, L1, L2, R5, L2, L3, R4, L1, L1, R1, R50, L1, R1, R76, R5, R4, R2, L5, L3, L5, R2, R1, L1, R2, L3, R4, R2, L1, L1, R4, L1, L1, R185, R1, L5, L4, L5, L3, R2, R3, R1, L5, R1, L3, L2, L2, R5, L1, L1, L3, R1, R4, L2, L1, L1, L3, L4, R5, L2, R3, R5, R1, L4, R5, L3, R3, R3, R1, R1, R5, R2, L2, R5, L5, L4, R4, R3, R5, R1, L3, R1, L2, L2, R3, R4, L1, R4, L1, R4, R3, L1, L4, L1, L5, L2, R2, L1, R1, L5, L3, R4, L1, R5, L5, L5, L1, L3, R1, R5, L2, L4, L5, L1, L1, L2, R5, R5, L4, R3, L2, L1, L3, L4, L5, L5, L2, R4, R3, L5, R4, R2, R1, L5";

using IntPair = std::pair<int, int>;

IntPair turn(char ch, IntPair dir)
{
    int angle_sine = 0;

    if (ch == 'L')
        angle_sine = 1;
    else
        angle_sine = -1;

    return std::make_pair(-dir.second * angle_sine, dir.first * angle_sine);
}

IntPair move(IntPair from, IntPair direction, int distance)
{
    return std::make_pair(
        from.first + distance * direction.first,
        from.second + distance * direction.second);
}

int calc_distance(IntPair pos)
{
    return std::abs(pos.first) + std::abs(pos.second);
}

std::string to_string(IntPair pair)
{
    std::stringstream stream;
    stream << '[' << pair.first << ',' << pair.second << ']';
    return stream.str();
}

int main(int argc, char *argv[])
{
    std::vector<std::string> instructions;
    boost::split(instructions, input, boost::is_any_of(", "), boost::token_compress_on);

    IntPair pos{ 0, 0 };
    IntPair direction{ 0, 1 };

    std::set<IntPair> visited;
    visited.insert(pos);
    int visited_twice = false;
    int distance_to_first_visited_twice = 0;

    for (const std::string &instruction : instructions)
    {
        int distance = atoi(instruction.substr(1).c_str());

        IntPair new_dir = turn(instruction[0], direction);

        IntPair new_pos = pos;
        for (int i = 0; i < distance; ++i)
        {
            new_pos = move(new_pos, new_dir, 1);

            if (!visited.insert(new_pos).second
                && !visited_twice)
            {
                distance_to_first_visited_twice = calc_distance(new_pos);
                visited_twice = true;
            }
        }

        //std::cout << instruction << " : "
        //    << to_string(pos) << " by " << to_string(direction) << " dis " << distance
        //    << " => " << to_string(new_pos) << " by " << to_string(new_dir)
        //    << std::endl;

        pos = new_pos;
        direction = new_dir;
    }

    std::cout << "Answer #1 : " << calc_distance(pos) << std::endl;
    std::cout << "Answer #2 : " << distance_to_first_visited_twice << std::endl;

    return 0;
}