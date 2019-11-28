#include <iostream>
#include <map>
#include <utility>
#include <cassert>

constexpr int PUZZLE_INPUT = 347991;

std::pair<int, int> location_of(int square)
{
    if (square <= 1)
        return std::make_pair(0, 0);

    int layer = 1;
    int start_square = 1;

    while (true)
    {
        int new_layer = layer + 1;
        int layer_size = 2 * new_layer - 1;
        int squares_in_layer = (layer_size * layer_size) - start_square;
        assert((squares_in_layer % 4) == 0);

        int first_square = start_square + 1;
        int last_square = start_square + squares_in_layer;

        //std::cout << new_layer << " " << start_square << " " << squares_in_layer << " " << first_square << " " << last_square << std::endl;

        if (square <= last_square)
        {
            int offset = square - first_square;
            int per_side = squares_in_layer / 4;

            int side = offset / per_side;
            int distance = offset % per_side;

            if (side == 0)
                return std::make_pair(layer, -layer + distance + 1);
            else if (side == 1)
                return std::make_pair(layer - distance - 1, layer);
            else if (side == 2)
                return std::make_pair(-layer, layer - distance - 1);
            else
                return std::make_pair(-layer + distance + 1, -layer);
        }

        layer += 1;
        start_square += squares_in_layer;
    }
}

int distance_from(int square)
{
    auto location = location_of(square);
    return std::abs(location.first) + std::abs(location.second);
}

bool are_neighbours(const std::pair<int, int> &a, const std::pair<int, int> &b)
{
    int diffx = std::abs(a.first - b.first);
    int diffy = std::abs(a.second - b.second);

    return ((diffx <= 1) && (diffy <= 1));
}

int part2(int larger_than)
{
    std::map<int, int> store;
    store[1] = 1;

    int square = 2;
    while (true)
    {
        auto location = location_of(square);

        int sum = 0;

        for (int other_sq = 1; other_sq < square; ++other_sq)
        {
            if (are_neighbours(location, location_of(other_sq)))
            {
                sum += store[other_sq];
            }
        }

        if (sum > larger_than)
            return sum;

        //std::cout << "store " << square << " = " << sum << std::endl;
        store[square] = sum;

        square += 1;
    }
}

int main(int /*argc*/, const char */*argv*/[])
{
    assert(distance_from(1) == 0);
    assert(distance_from(12) == 3);
    assert(distance_from(23) == 2);
    assert(distance_from(1024) == 31);

    assert(part2(24) == 25);
    assert(part2(60) == 122);
    assert(part2(748) == 806);

    std::cout << "Answer #1: " << distance_from(PUZZLE_INPUT) << std::endl;
    std::cout << "Answer #2: " << part2(PUZZLE_INPUT) << std::endl;
    return 0;
}