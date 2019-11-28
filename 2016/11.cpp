#include <iostream>
#include <string>
#include <vector>
#include <sstream>
#include <map>
#include <boost/optional.hpp>

static constexpr size_t NUM_FLOORS = 4;
static constexpr uint64_t MAX_MOVES = 100;

// Example input
//static constexpr size_t NUM_ITEMS = 4;
//static std::string const NAMES[] = { "HG", "HM", "LG", "LM" };
//enum class ITEM
//{
//    HYDROGEN_RTG,
//    HYDROGEN_CHIP,
//    LITHIUM_RTG,
//    LITHIUM_CHIP,
//};

// Real input
static constexpr size_t NUM_ITEMS = 14;
static std::string const NAMES[] = { "PolG", "PolM", "ThuG", "ThuM", "ProG", "ProM", "RutG", "RutM", "CobG", "CobM", "EleG", "EleM", "DilG", "DilM" };
enum class ITEM
{
    polonium_generator,
    polonium_chip,
    thulium_generator,
    thulium_chip,
    promethium_generator,
    promethium_chip,
    ruthenium_generator,
    ruthenium_chip,
    cobalt_generator,
    cobalt_chip,
    elerium_generator,
    elerium_chip,
    dilithium_generator,
    dilithium_chip,
};

bool is_chip(ITEM item)
{
    return (int(item) & 1) == 1;
}

bool is_rtg(ITEM item)
{
    return (int(item) & 1) == 0;
}

bool are_paired(ITEM item_a, ITEM item_b)
{
    return (int(item_a) & ~1) == (int(item_b) & ~1);
}

class State
{
    uint64_t m_value;

public:
    State()
        : m_value(0)
    {
    }

    explicit State(int part)
        : m_value(0)
    {
        set_elevator(1);

        // Example start state
        //set_item(ITEM::HYDROGEN_RTG, 2);
        //set_item(ITEM::HYDROGEN_CHIP, 1);
        //set_item(ITEM::LITHIUM_RTG, 3);
        //set_item(ITEM::LITHIUM_CHIP, 1);

        // Real start state
        set_item(ITEM::polonium_generator, 1);
        set_item(ITEM::polonium_chip, 2);
        set_item(ITEM::thulium_generator, 1);
        set_item(ITEM::thulium_chip, 1);
        set_item(ITEM::promethium_generator, 1);
        set_item(ITEM::promethium_chip, 2);
        set_item(ITEM::ruthenium_generator, 1);
        set_item(ITEM::ruthenium_chip, 1);
        set_item(ITEM::cobalt_generator, 1);
        set_item(ITEM::cobalt_chip, 1);
        
        if (part == 1)
        {
            set_item(ITEM::elerium_generator, 4);
            set_item(ITEM::elerium_chip, 4);
            set_item(ITEM::dilithium_generator, 4);
            set_item(ITEM::dilithium_chip, 4);
        }
        else
        {
            set_item(ITEM::elerium_generator, 1);
            set_item(ITEM::elerium_chip, 1);
            set_item(ITEM::dilithium_generator, 1);
            set_item(ITEM::dilithium_chip, 1);
        }
    }

    State(const State &) = default;
    State &operator =(const State &) = default;
    ~State() = default;

    static State get_complete_state()
    {
        State result;
        result.set_elevator(NUM_FLOORS);
        for (int item_index = 0; item_index < NUM_ITEMS; ++item_index)
            result.set_item(ITEM(item_index), NUM_FLOORS);
        return result;
    }

    int get_elevator() const
    {
        return get_index(0);
    }

    int get_item(ITEM item) const
    {
        return get_index(int(item) + 1);
    }

    void set_elevator(int floor)
    {
        set_index(0, floor);
    }

    void set_item(ITEM item, int floor)
    {
        set_index(int(item) + 1, floor);
    }

    bool operator <(const State &other) const
    {
        return m_value < other.m_value;
    }

    bool is_complete_state() const
    {
        if (get_elevator() != NUM_FLOORS)
            return false;

        for (int item_index = 0; item_index < NUM_ITEMS; ++item_index)
        {
            if (get_item(ITEM(item_index)) != NUM_FLOORS)
                return false;
        }

        return true;
    }

    std::vector<ITEM> items_on_floor(int floor) const
    {
        std::vector<ITEM> result;
        for (int item_index = 0; item_index < NUM_ITEMS; ++item_index)
        {
            ITEM item = ITEM(item_index);

            if (get_item(item) == floor)
                result.push_back(item);
        }
        return result;
    }

    bool are_any_chips_fried() const
    {
        for (int chip_index = 0; chip_index < NUM_ITEMS; ++chip_index)
        {
            ITEM chip = ITEM(chip_index);

            if (is_chip(chip))
            {
                const int chip_floor = get_item(chip);

                bool has_matching_rtg = false;
                bool has_other_rtg = false;

                for (int rtg_index = 0; rtg_index < NUM_ITEMS; ++rtg_index)
                {
                    ITEM rtg = ITEM(rtg_index);

                    if (is_rtg(rtg)
                        && (get_item(rtg) == chip_floor))
                    {
                        if (are_paired(chip, rtg))
                            has_matching_rtg = true;
                        else
                            has_other_rtg = true;
                    }
                }

                if (has_other_rtg
                    && !has_matching_rtg)
                {
                    // There is another RTG on the same floor as this chip,
                    // and this chip's matching RTG isn't on this floor so
                    // the chip shielding is not working - it's FRIED!!!

                    return true;
                }
            }
        }

        return false;
    }

    State reduce() const
    {
        // CRITICAL optimization - copied from redit
        // It doesn't matter which pairs are on which floors -
        // only the combination of pairs and floors.
        // As such, we can reduce a state to an equivalent
        // state

        State result(*this);

        std::vector<std::pair<int, int>> pair_floors;

        for (int item_index = 0; item_index < NUM_ITEMS; item_index += 2)
        {
            ITEM gen = ITEM(item_index);
            ITEM chip = ITEM(item_index + 1);

            pair_floors.push_back(std::make_pair(get_item(gen), get_item(chip)));
        }

        std::sort(pair_floors.begin(), pair_floors.end());

        for (int index = 0; index < pair_floors.size(); ++index)
        {
            ITEM gen = ITEM(2 * index);
            ITEM chip = ITEM((2 * index) + 1);

            result.set_item(gen, pair_floors[index].first);
            result.set_item(chip, pair_floors[index].second);
        }

        return result;
    }

    void print() const
    {
        std::vector<std::stringstream> streams(NUM_FLOORS + 1);

        for (int floor = 1; floor <= NUM_FLOORS; ++floor)
        {
            streams[floor] << "F" << floor << " ";
            if (get_elevator() == floor)
                streams[floor] << "E ";
            else
                streams[floor] << ". ";
        }

        for (int item_index = 0; item_index < NUM_ITEMS; ++item_index)
        {
            ITEM item = ITEM(item_index);

            for (int floor = 1; floor <= NUM_FLOORS; ++floor)
            {
                if (get_item(item) == floor)
                    streams[floor] << NAMES[item_index] << " ";
                else
                    streams[floor] << '.' << std::string(NAMES[item_index].size(), ' ');
            }
        }

        for (int floor = NUM_FLOORS; floor >= 1; --floor)
        {
            std::cout << streams[floor].str() << std::endl;
        }
    }

private:
    void set_index(int index, int floor)
    {
        uint64_t mask = uint64_t(0x7) << (index * 3);
        uint64_t value = uint64_t(floor) << (index * 3);

        m_value &= ~mask;
        m_value |= value;
    }

    uint64_t get_index(int index) const
    {
        return (m_value >> (index * 3)) & 0x7;
    }
};

class StatePathLengths
{
public:
    StatePathLengths() = delete;
    ~StatePathLengths() = default;

    explicit StatePathLengths(std::string &&name)
        : m_name(std::move(name))
        , m_state_lengths()
    {
    }

    bool is_better(const State &state, uint64_t length) const
    {
        auto it = m_state_lengths.find(state);

        return (it == m_state_lengths.end())
            || (length < it->second);
    }

    void add(const State &state, uint64_t length)
    {
        auto it = m_state_lengths.find(state);

        if (it == m_state_lengths.end())
            m_state_lengths[state] = length;
        else if (length < it->second)
            it->second = length;
    }

    bool empty() const
    {
        return m_state_lengths.empty();
    }
    
    void remove_next(State &state, uint64_t &length)
    {
        auto it = m_state_lengths.begin();

        state = it->first;
        length = it->second;

        m_state_lengths.erase(it);
    }

    uint64_t get_length(const State &state)
    {
        auto it = m_state_lengths.find(state);
        assert(it != m_state_lengths.end());

        return it->second;
    }

private:
    std::string m_name;
    std::map<State, uint64_t> m_state_lengths;
};

uint64_t solve(int part)
{
    StatePathLengths paths_found("paths_found");
    StatePathLengths paths_to_test("paths_to_test");
    uint64_t max_moves_to_search = MAX_MOVES;

    paths_to_test.add(State(part), 0);

    while (!paths_to_test.empty())
    {
        // Get the next state to test and
        // add it to the paths that we've found

        State cur_state;
        uint64_t cur_moves;

        paths_to_test.remove_next(cur_state, cur_moves);

        paths_found.add(cur_state, cur_moves);

        // Don't do any more testing if we've already moved
        // too many moves

        if (cur_moves >= max_moves_to_search)
            continue;

        // See which floor we're on and which floors we
        // can move to from here

        const int cur_floor = cur_state.get_elevator();

        std::vector<int> new_floors;
        if (cur_floor > 1)
            new_floors.push_back(cur_floor - 1);
        if (cur_floor < NUM_FLOORS)
            new_floors.push_back(cur_floor + 1);

        // See which items are on this floor which we can move

        std::vector<ITEM> items_on_floor = cur_state.items_on_floor(cur_floor);

        // Now, create all of the new states that we can get to

        std::vector<State> new_states;

        for (int new_floor : new_floors)
        {
            // Moving one item

            for (int i = 0; i < items_on_floor.size(); ++i)
            {
                State new_state = cur_state;
                new_state.set_elevator(new_floor);
                new_state.set_item(items_on_floor[i], new_floor);

                new_states.push_back(new_state);
            }

            // Moving two items

            for (int i = 0; i < items_on_floor.size(); ++i)
            {
                for (int j = i + 1; j < items_on_floor.size(); ++j)
                {
                    State new_state = cur_state;
                    new_state.set_elevator(new_floor);
                    new_state.set_item(items_on_floor[i], new_floor);
                    new_state.set_item(items_on_floor[j], new_floor);

                    new_states.push_back(new_state);
                }
            }
        }

        const uint64_t new_moves = cur_moves + 1;

        for (const State potentially_duplicate_new_state : new_states)
        {
            // CRITICAL OPTIMIZATION!!!
            const State new_state = potentially_duplicate_new_state.reduce();

            if (!paths_to_test.is_better(new_state, new_moves)
                || !paths_found.is_better(new_state, new_moves))
            {
                // We've already found a quicker path to this state -
                // don't re-test

                continue;
            }

            if (new_state.are_any_chips_fried())
            {
                // This move has fried at least one chip - it's
                // a dead end - don't continue

                continue;
            }

            if (new_state.is_complete_state())
            {
                // This is the complete state - save it if we've taken
                // the shortest path

                paths_found.add(new_state, new_moves);

                // Also, save this as the new max number of moves that we should search for

                if (new_moves < max_moves_to_search)
                    max_moves_to_search = new_moves;

                continue;
            }

            // OK - we want to keep searching from
            // this new state

            paths_to_test.add(new_state, new_moves);
        }
    }

    return paths_found.get_length(State::get_complete_state());
}

int main(int argc, char *argv[])
{
    uint64_t answer1 = solve(1);

    std::cout << "Answer #1: " << answer1 << std::endl;

    uint64_t answer2 = solve(2);

    std::cout << "Answer #2: " << answer2 << std::endl;
}