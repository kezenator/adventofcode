#include <iostream>
#include <string>
#include <map>
#include <vector>
#include <cassert>

static constexpr uint64_t INPUT = 1358;

using Location = std::pair<uint64_t, uint64_t>;

bool is_wall(const Location &location)
{
    uint64_t sum = (location.first * location.first)
        + (3 * location.first)
        + (2 * location.first * location.second)
        + location.second
        + (location.second * location.second);

    sum += INPUT;

    uint64_t num_1_bits = __builtin_popcount(sum);

    if (num_1_bits & 1)
        return true; // wall
    else
        return false; // space
}

class PathSet
{
public:
    PathSet()
        : m_shortest_paths()
    {
    }

    bool empty() const
    {
        return m_shortest_paths.empty();
    }

    void add(const Location &location, uint64_t length)
    {
        auto it = m_shortest_paths.find(location);

        if (it == m_shortest_paths.end())
            m_shortest_paths[location] = length;
        else if (length < it->second)
            it->second = length;
    }

    uint64_t get(const Location &location) const
    {
        auto it = m_shortest_paths.find(location);
        assert(it != m_shortest_paths.end());
        return it->second;
    }

    void remove_next(Location &location, uint64_t &length)
    {
        assert(!m_shortest_paths.empty());
        auto it = m_shortest_paths.begin();

        location = it->first;
        length = it->second;

        m_shortest_paths.erase(it);
    }

    bool contains_better(const Location &location, const uint64_t length)
    {
        auto it = m_shortest_paths.find(location);
        if (it == m_shortest_paths.end())
            return false;
        else if (it->second >= length)
            return false;
        else
            return true;
    }

    uint64_t size()
    {
        return m_shortest_paths.size();
    }

private:
    // Compare items closest to (0, 0) as less
    // this ensures we don't keep searching out further and further

    struct custom_less
    {
        bool operator()(const Location &a, const Location &b) const
        {
            uint64_t suma = a.first + a.second;
            uint64_t sumb = b.first + b.second;

            if (suma < sumb)
                return true;
            else if (suma > sumb)
                return false;

            // They are the same distance from (0, 0) -
            // just use the standard less operator to compare

            return a < b;
        }
    };

    std::map<Location, uint64_t, custom_less> m_shortest_paths;
};

std::vector<Location> next_locations(const Location &from)
{
    std::vector<Location> result;

    if (from.first > 0)
    {
        Location next = Location(from.first - 1, from.second);
        if (!is_wall(next))
            result.push_back(next);
    }

    if (from.second> 0)
    {
        Location next = Location(from.first, from.second - 1);
        if (!is_wall(next))
            result.push_back(next);
    }

    {
        Location next = Location(from.first + 1, from.second);
        if (!is_wall(next))
            result.push_back(next);
    }

    {
        Location next = Location(from.first, from.second + 1);
        if (!is_wall(next))
            result.push_back(next);
    }

    return result;
}

uint64_t shortest_path(const Location &from, const Location &to)
{
    uint64_t max_length = 100 * (to.first + to.second);

    PathSet already_tested;
    PathSet to_test;

    to_test.add(from, 0);

    while (!to_test.empty())
    {
        Location test_location;
        uint64_t test_length;

        to_test.remove_next(test_location, test_length);
        already_tested.add(test_location, test_length);

        //std::cout << "Testing (" << test_location.first << ", " << test_location.second << "), length = " << test_length << std::endl;

        for (const Location new_location : next_locations(test_location))
        {
            const uint64_t new_length = test_length + 1;

            if (new_length > max_length)
            {
                // Don't search paths this long
                continue;
            }

            if (already_tested.contains_better(new_location, new_length)
                || to_test.contains_better(new_location, new_length))
            {
                // Already found a better path here
                continue;
            }

            to_test.add(new_location, new_length);
        }
    }

    return already_tested.get(to);
}

uint64_t unique_locations(const Location &from, uint64_t max_length)
{
    PathSet already_tested;
    PathSet to_test;

    to_test.add(from, 0);

    while (!to_test.empty())
    {
        Location test_location;
        uint64_t test_length;

        to_test.remove_next(test_location, test_length);
        already_tested.add(test_location, test_length);

        //std::cout << "Testing (" << test_location.first << ", " << test_location.second << "), length = " << test_length << std::endl;

        for (const Location new_location : next_locations(test_location))
        {
            const uint64_t new_length = test_length + 1;

            if (new_length > max_length)
            {
                // Don't search paths this long
                continue;
            }

            if (already_tested.contains_better(new_location, new_length)
                || to_test.contains_better(new_location, new_length))
            {
                // Already found a better path here
                continue;
            }

            to_test.add(new_location, new_length);
        }
    }

    return already_tested.size();
}

int main(int argc, char *argv[])
{
    uint64_t answer1 = shortest_path(Location(1, 1), Location(31, 39));
    uint64_t answer2 = unique_locations(Location(1, 1), 50);

    std::cout << "Answer #1: " << answer1 << std::endl;
    std::cout << "Answer #2: " << answer2 << std::endl;
}
