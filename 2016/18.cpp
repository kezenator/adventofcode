#include <iostream>
#include <string>

static const char SAFE = '.';
static const char TRAP = '^';

char is_tile(const char left, const char center, const char right)
{
    if ((left == TRAP) && (center == TRAP) && (right == SAFE))
        return TRAP;
    else if ((center == TRAP) && (right == TRAP) && (left == SAFE))
        return TRAP;
    else if ((left == TRAP) && (center == SAFE) && (right == SAFE))
        return TRAP;
    else if ((left == SAFE) && (center == SAFE) && (right == TRAP))
        return TRAP;
    else
        return SAFE;
}

std::string next_row(const std::string &prev_row)
{
    std::string result = prev_row;

    for (size_t index = 0; index < result.size(); ++index)
    {
        char left = '.';
        char center = prev_row[index];
        char right = '.';

        if (index > 0)
            left = prev_row[index - 1];
        if (index < (result.size() - 1))
            right = prev_row[index + 1];

        result[index] = is_tile(left, center, right);
    }

    return result;
}

size_t count_safe(const std::string &row)
{
    size_t result = 0;
    for (char ch : row)
    {
        if (ch == SAFE)
            result += 1;
    }
    return result;
}

size_t count_safe_in_rows(const std::string &start_row, size_t num_rows)
{
    size_t result = 0;
    std::string cur_row = start_row;

    for (size_t i = 0; i < num_rows; ++i)
    {
        //std::cout << cur_row << std::endl;

        result += count_safe(cur_row);
        cur_row = next_row(cur_row);
    }

    return result;
}

int main(int argc, char *argv[])
{
    const std::string input = "^^^^......^...^..^....^^^.^^^.^.^^^^^^..^...^^...^^^.^^....^..^^^.^.^^...^.^...^^.^^^.^^^^.^^.^..^.^";

    size_t answer1 = count_safe_in_rows(input, 40);
    size_t answer2 = count_safe_in_rows(input, 400000);

    std::cout << "Answer #1: " << answer1 << std::endl;
    std::cout << "Answer #2: " << answer2 << std::endl;
}
