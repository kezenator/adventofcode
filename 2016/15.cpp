#include <iostream>
#include <string>
#include <vector>

using Disk = std::pair<int, int>;

std::vector<Disk> input = { {17, 1}, {7, 0}, {19, 2}, {5, 0}, {3, 0}, {13, 5} };

int time_for_ball()
{
    for (int time = 0; true; ++time)
    {
        bool passes = true;

        for (size_t index = 0; index < input.size(); ++index)
        {
            int size = input[index].first;
            int time_0_pos = input[index].second;

            if (0 != ((time + time_0_pos + 1 + index) % size))
            {
                passes = false;
                break;
            }
        }

        if (passes)
            return time;
    }
}

int main(int argc, char *argv[])
{
    uint64_t answer1 = time_for_ball();

    std::cout << "Answer #1: " << answer1 << std::endl;

    input.push_back(std::make_pair(11, 0));

    uint64_t answer2 = time_for_ball();

    std::cout << "Answer #2: " << answer2 << std::endl;
}