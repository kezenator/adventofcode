#include <cassert>
#include <iostream>
#include <vector>
#include <set>

const std::vector<int> PUZZLE_INPUT = { 2, 8, 8, 5, 4, 2, 3, 1, 5, 5, 1, 2, 15, 13, 5, 14 };

struct State
{
    explicit State(const std::vector<int> &_initial)
        : state(_initial)
    {
    }

    explicit State(std::vector<int> &&_initial)
        : state(std::move(_initial))
    {
    }

    std::vector<int> state;

    State redistribute()
    {
        std::vector<int> new_state = state;

        int biggest = 0;
        for (size_t index = 1; index < new_state.size(); ++index)
        {
            if (new_state[index] > new_state[biggest])
                biggest = index;
        }

        int count = new_state[biggest];
        new_state[biggest] = 0;

        for (int i = 0; i < count; ++i)
        {
            new_state[(biggest + 1 + i) % new_state.size()] += 1;
        }

        return State(std::move(new_state));
    }

    bool operator <(const State &other) const
    {
        if (state.size() < other.state.size())
            return true;
        if (state.size() > other.state.size())
            return false;

        for (size_t index = 0; index < state.size(); ++index)
        {
            if (state[index] < other.state[index])
                return true;
            if (state[index] > other.state[index])
                return false;
        }
        return false;
    }

    bool operator ==(const State &other) const
    {
        return state == other.state;
    }
};

std::pair<int, int> steps_to_loop_and_loop_length(const std::vector<int> &initial)
{
    State state(initial);

    std::vector<State> order_seen;
    std::set<State> already_seen;

    order_seen.push_back(state);
    already_seen.insert(state);

    int steps = 0;
    while (true)
    {
        steps += 1;

        state = state.redistribute();

        if (!already_seen.insert(state).second)
        {
            for (int loop = 1; true; ++loop)
            {
                if (order_seen[order_seen.size() - loop] == state)
                    return std::make_pair(steps, loop);
            }
        }
        order_seen.push_back(state);
    }
}

int main(int /*argc*/, const char */*argv*/[])
{
    assert(std::make_pair(5, 4) == steps_to_loop_and_loop_length({0, 2, 7, 0}));

    std::cout << "Answer #1: " << steps_to_loop_and_loop_length(PUZZLE_INPUT).first << std::endl;
    std::cout << "Answer #2: " << steps_to_loop_and_loop_length(PUZZLE_INPUT).second << std::endl;
    return 0;
}