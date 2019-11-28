#include <iostream>
#include <string>
#include <boost/algorithm/string.hpp>

const std::string input = R"(cpy 1 a
cpy 1 b
cpy 26 d
jnz c 2
jnz 1 5
cpy 7 c
inc d
dec c
jnz c -2
cpy a c
inc a
dec b
jnz b -2
cpy c b
dec d
jnz d -6
cpy 14 c
cpy 14 d
inc a
dec d
jnz d -2
dec c
jnz c -5)";

int64_t run(const std::initializer_list<int64_t> &init_vals)
{
    std::vector<std::string> lines;
    boost::split(lines, input, boost::is_any_of("\n"));

    std::vector<int64_t> registers(init_vals);

    size_t pc = 0;

    while (pc < lines.size())
    {
        //std::cout << pc << ": " << registers[0] << ", " << registers[1] << ", "
        //    << registers[2] << ", " << registers[3] << ": " << lines[pc] << std::endl;

        const std::string &line = lines[pc];

        if (line.substr(0, 4) == "cpy ")
        {
            int64_t copy_val = 0;
            size_t space_pos = line.find(' ', 4);

            if ((line[4] >= 'a') && (line[4] <= 'd'))
                copy_val = registers[line[4] - 'a'];
            else
                copy_val = atoi(line.substr(4, space_pos - 4).c_str());

            registers[line.back() - 'a'] = copy_val;
            pc += 1;
        }
        else if (line.substr(0, 4) == "inc ")
        {
            registers[line.back() - 'a'] += 1;
            pc += 1;
        }
        else if (line.substr(0, 4) == "dec ")
        {
            registers[line.back() - 'a'] -= 1;
            pc += 1;
        }
        else if (line.substr(0, 4) == "jnz ")
        {
            int64_t test_val = 0;
            size_t space_pos = line.find(' ', 4);

            if ((line[4] >= 'a') && (line[4] <= 'd'))
                test_val = registers[line[4] - 'a'];
            else
                test_val = atoi(line.substr(4, space_pos - 4).c_str());

            if (test_val != 0)
            {
                pc += atoi(line.substr(space_pos + 1).c_str());
            }
            else
            {
                pc += 1;
            }
        }
    }

    return registers[0];
}

int main(int argc, char *argv[])
{
    int64_t answer1 = run({ 0, 0, 0, 0 });
    int64_t answer2 = run({ 0, 0, 1, 0 });

    std::cout << "Answer #1: " << answer1 << std::endl;
    std::cout << "Answer #2: " << answer2 << std::endl;
}
