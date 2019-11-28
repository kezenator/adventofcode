#include <iostream>
#include <string>
#include <vector>
#include <set>
#include <boost/algorithm/string.hpp>

const std::string input = R"(rotate right 3 steps
swap position 7 with position 0
rotate left 3 steps
reverse positions 2 through 5
move position 6 to position 3
reverse positions 0 through 4
swap position 4 with position 2
rotate based on position of letter d
rotate right 0 steps
move position 7 to position 5
swap position 4 with position 5
swap position 3 with position 5
move position 5 to position 3
swap letter e with letter f
swap position 6 with position 3
swap letter a with letter e
reverse positions 0 through 1
reverse positions 0 through 4
swap letter c with letter e
reverse positions 1 through 7
rotate right 1 step
reverse positions 6 through 7
move position 7 to position 1
move position 4 to position 0
move position 4 to position 6
move position 6 to position 3
swap position 1 with position 6
swap position 5 with position 7
swap position 2 with position 5
swap position 6 with position 5
swap position 2 with position 4
reverse positions 2 through 6
reverse positions 3 through 5
move position 3 to position 5
reverse positions 1 through 5
rotate left 1 step
move position 4 to position 5
swap letter c with letter b
swap position 2 with position 1
reverse positions 3 through 4
swap position 3 with position 4
reverse positions 5 through 7
swap letter b with letter d
reverse positions 3 through 4
swap letter c with letter h
rotate based on position of letter b
rotate based on position of letter e
rotate right 3 steps
rotate right 7 steps
rotate left 2 steps
move position 6 to position 1
reverse positions 1 through 3
rotate based on position of letter b
reverse positions 0 through 4
swap letter g with letter c
move position 1 to position 5
rotate right 4 steps
rotate left 2 steps
move position 7 to position 2
rotate based on position of letter c
move position 6 to position 1
swap letter f with letter g
rotate right 6 steps
swap position 6 with position 2
reverse positions 2 through 6
swap position 3 with position 1
rotate based on position of letter h
reverse positions 2 through 5
move position 1 to position 3
rotate right 1 step
rotate right 7 steps
move position 6 to position 3
rotate based on position of letter h
swap letter d with letter h
rotate left 0 steps
move position 1 to position 2
swap letter a with letter g
swap letter a with letter g
swap position 4 with position 2
rotate right 1 step
rotate based on position of letter b
swap position 7 with position 1
rotate based on position of letter e
move position 1 to position 4
move position 6 to position 3
rotate left 3 steps
swap letter f with letter g
swap position 3 with position 1
swap position 4 with position 3
swap letter f with letter c
rotate left 3 steps
rotate left 0 steps
rotate right 3 steps
swap letter d with letter e
swap position 2 with position 7
move position 3 to position 6
swap position 7 with position 1
swap position 3 with position 6
rotate left 5 steps
swap position 2 with position 6)";

std::string scramble_step(std::string password, const std::string &line)
{
    if (line.substr(0, 14) == "swap position ")
    {
        size_t next_space = line.find(' ', 14);
        size_t last_space = line.rfind(' ');

        int a = atoi(line.substr(14, next_space - 14).c_str());
        int b = atoi(line.substr(last_space + 1).c_str());

        //std::cout << "swap " << a << " and " << b << std::endl;

        std::swap(password[a], password[b]);
    }
    if (line.substr(0, 12) == "swap letter ")
    {
        char a = line[12];
        char b = line[26];

        //std::cout << "swap " << a << " and " << b << std::endl;

        for (char &ch : password)
        {
            if (ch == a)
                ch = b;
            else if (ch == b)
                ch = a;
        }
    }
    else if ((line.substr(0, 7) == "rotate ")
        && ((line[7] == 'l') || (line[7] == 'r')))
    {
        bool left = (line[7] == 'l');
        size_t start_pos = left ? 12 : 13;
        size_t space_pos = line.find(' ', start_pos);

        int distance = atoi(line.substr(start_pos, space_pos - start_pos).c_str());

        //std::cout << "rotating " << (left ? "left " : "right ") << distance << std::endl;

        if (left)
        {
            for (int i = 0; i < distance; ++i)
            {
                char ch = password[0];
                password = password.substr(1);
                password.push_back(ch);
            }
        }
        else
        {
            for (int i = 0; i < distance; ++i)
            {
                char ch = password.back();
                password.pop_back();
                password.insert(0, 1, ch);
            }
        }
    }
    else if (line.substr(0, 14) == "move position ")
    {
        size_t next_space = line.find(' ', 14);
        size_t last_space = line.rfind(' ');

        int a = atoi(line.substr(14, next_space - 14).c_str());
        int b = atoi(line.substr(last_space + 1).c_str());

        //std::cout << "move " << a << " to " << b << std::endl;

        char ch = password[a];
        password.erase(a, 1);
        password.insert(b, 1, ch);
    }
    else if (line.substr(0, 18) == "reverse positions ")
    {
        size_t next_space = line.find(' ', 18);
        size_t last_space = line.rfind(' ');

        int a = atoi(line.substr(18, next_space - 18).c_str());
        int b = atoi(line.substr(last_space + 1).c_str());

        //std::cout << "reversing " << a << " to " << b << std::endl;

        while (a < b)
        {
            std::swap(password[a], password[b]);
            ++a;
            --b;
        }
    }
    else if (line.substr(0, 12) == "rotate based")
    {
        char ch = line.back();
        size_t index = password.find(ch);
        size_t rotate = 1 + index + ((index >= 4) ? 1 : 0);

        //std::cout << "rotate based on " << ch << " index " << index << " rotate " << rotate << std::endl;

        for (int i = 0; i < rotate; ++i)
        {
            char ch = password.back();
            password.pop_back();
            password.insert(0, 1, ch);
        }
    }

    return password;
}

std::string scramble(const std::string &str)
{
    std::vector<std::string> lines;
    boost::split(lines, input, boost::is_any_of("\n"));

    std::string result = str;

    for (size_t i = 0; i < lines.size(); ++i)
    {
        result = scramble_step(result, lines[i]);
    }

    return result;
}

std::string unscramble(const std::string &str)
{
    // Generate all permutations

    std::set<std::string> permutations;

    std::set<std::string> to_test;
    to_test.insert("");
    while (!to_test.empty())
    {
        std::string cur = *to_test.begin();
        to_test.erase(cur);

        if (cur.size() == str.size())
        {
            permutations.insert(cur);
        }
        else
        {
            // Increase it by all characters not already in the list
            for (char ch : str)
            {
                if (cur.find(ch) == std::string::npos)
                {
                    to_test.insert(cur + ch);
                }
            }
        }
    }

    // See which one scrambles to the original string

    for (const std::string &trial : permutations)
    {
        if (scramble(trial) == str)
            return trial;
    }
    assert(false);
}

int main(int argc, char *argv[])
{
    std::cout << "Answer #1: " << scramble("abcdefgh") << std::endl;
    std::cout << "Answer #2: " << unscramble("fbgdceah") << std::endl;
}
