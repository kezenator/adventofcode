#include <iostream>
#include <string>
#include <vector>

int play_game(size_t num_elfs, bool part_1)
{
    std::vector<int> circle(num_elfs);
    for (size_t i = 0; i < num_elfs; ++i)
        circle[i] = i + 1;

    size_t elf_index = 0;

    while (circle.size() > 1)
    {
        size_t next_index = part_1
            ? ((elf_index + 1) % circle.size())
            : ((elf_index + (circle.size() / 2)) % circle.size());

        //std::cout << "Elf " << circle[elf_index] << " steals from elf " << circle[next_index] << std::endl;

        circle.erase(circle.begin() + next_index);

        if (next_index > elf_index)
        {
            // Erased one later in the vector - to move
            // forward we need to increment by one

            elf_index = (elf_index + 1) % circle.size();
        }
        else
        {
            // Erased one before the current position -
            // to move forward one we don't need to do anything
            // except check for wrap-around as the current elf
            // has moved back one index

            elf_index = elf_index % circle.size();
        }

        //if ((circle.size() % 10000) == 0)
        //    std::cout << "Down to " << circle.size() << " elves" << std::endl;
    }

    return circle[0];
}

int main(int argc, char *argv[])
{
    size_t answer1 = play_game(3012210, true);

    std::cout << "Answer #1: " << answer1 << std::endl;

    size_t answer2 = play_game(3012210, false);

    std::cout << "Answer #2: " << answer2 << std::endl;
}
