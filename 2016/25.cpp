#include <iostream>
#include <string>
#include <set>
#include <boost/algorithm/string.hpp>

const std::string input = R"(cpy a d
cpy 14 c
cpy 182 b
inc d
dec b
jnz b -2
dec c
jnz c -5
cpy d a
jnz 0 0
cpy a b
cpy 0 a
cpy 2 c
jnz b 2
jnz 1 6
dec b
dec c
jnz c -4
inc a
jnz 1 -7
cpy 2 b
jnz c 2
jnz 1 4
dec b
dec c
jnz 1 -4
jnz 0 0
out b
jnz a -19
jnz 1 -21)";

struct Argument
{
    int index;
    bool is_register;

    Argument(int _index, bool _is_register)
        : index(_index)
        , is_register(_is_register)
    {
    }
};

struct Instruction
{
    std::string code;
    std::vector<Argument> args;

    void parse(const std::string &str)
    {
        std::vector<std::string> parts;
        boost::split(parts, str, boost::is_any_of(" "));

        code = parts[0];
        args.clear();
        for (int i = 1; i < parts.size(); ++i)
        {
            int index = 0;
            bool is_register = false;

            if ((parts[i].size() == 1)
                && (parts[i][0] >= 'a')
                && (parts[i][0] <= 'd'))
            {
                index = parts[i][0] - 'a';
                is_register = true;
            }
            else
            {
                index = atoi(parts[i].c_str());
                is_register = false;
            }

            args.push_back(Argument(index, is_register));
        }
    }
};

struct ComputerState
{
    int pc;
    int registers[4];
    bool complete;

    ComputerState()
        : pc(0)
        , complete(false)
    {
        for (int i = 0; i < 4; ++i)
            registers[i] = 0;
    }

    bool operator <(const ComputerState &other) const
    {
        int result = 0;

        auto compare = [](int &result, int a, int b)
        {
            if (result == 0)
            {
                if (a < b)
                    result = -1;
                else if (a > b)
                    result = 1;
            }
        };

        compare(result, pc, other.pc);
        compare(result, registers[0], other.registers[0]);
        compare(result, registers[1], other.registers[1]);
        compare(result, registers[2], other.registers[2]);
        compare(result, registers[3], other.registers[3]);

        return (result < 0);
    }

    void set(const Argument &arg, int value)
    {
        assert(arg.is_register);
        registers[arg.index] = value;
    }

    int get(const Argument &arg) const
    {
        if (arg.is_register)
            return registers[arg.index];
        else
            return arg.index;
    }

    void update(std::vector<Instruction> &instructions, std::vector<int> &output)
    {
        if (!complete
            && (pc < instructions.size())
            && (pc >= 0))
        {
            const Instruction &inst = instructions[pc];

            if (inst.code == "cpy")
            {
                assert(inst.args.size() == 2);

                if (inst.args[1].is_register)
                {
                    set(inst.args[1], get(inst.args[0]));
                }

                pc += 1;
            }
            else if (inst.code == "inc")
            {
                assert(inst.args.size() == 1);

                if (inst.args[0].is_register)
                {
                    set(inst.args[0], get(inst.args[0]) + 1);
                }

                pc += 1;
            }
            else if (inst.code == "dec")
            {
                assert(inst.args.size() == 1);

                if (inst.args[0].is_register)
                {
                    set(inst.args[0], get(inst.args[0]) - 1);
                }

                pc += 1;
            }
            else if (inst.code == "jnz")
            {
                assert(inst.args.size() == 2);

                int test = get(inst.args[0]);
                int distance = get(inst.args[1]);

                if (test != 0)
                    pc = pc + distance;
                else
                    pc += 1;
            }
            else if (inst.code == "out")
            {
                assert(inst.args.size() == 1);

                output.push_back(get(inst.args[0]));

                pc += 1;
            }
            else
            {
                assert(false);
            }
        }

        if ((pc >= instructions.size())
            || (pc < 0))
        {
            complete = true;
        }
    }

    void print() const
    {
        std::cout << "pc=" << pc << ", reg="
            << registers[0] << ","
            << registers[1] << ","
            << registers[2] << ","
            << registers[3] << std::endl;
    }
};

bool generates_clock(int init)
{
    std::cout << "Running with " << init << ":";

    std::vector<std::string> lines;
    boost::split(lines, input, boost::is_any_of("\n"));

    std::vector<Instruction> instructions;

    for (const std::string &line : lines)
    {
        instructions.emplace_back();
        instructions.back().parse(line);
    }

    ComputerState state;

    state.registers[0] = init;

    std::vector<int> output;
    std::set<ComputerState> visited_states;

    bool result = false;

    while (!state.complete)
    {
        size_t output_before = output.size();

        state.update(instructions, output);

        if (output.size() != output_before)
        {
            bool expected_next = (output.size() & 1) ? 0 : 1;

            if (output.back() != expected_next)
            {
                result = false;
                break;
            }

            if ((output.size() & 1) == 0)
            {
                auto insert_result = visited_states.insert(state);

                if (!insert_result.second)
                {
                    // We've visited the same state again -
                    // the computer is in a loop
                    result = true;
                    break;
                }
            }
        }

        //state.print();
    }

    for (int i : output)
        std::cout << " " << i;
    std::cout << std::endl;

    if (result)
    {
        for (const ComputerState &visited_state : visited_states)
        {
            visited_state.print();
        }
    }

    return result;
}

int main(int argc, char *argv[])
{
    int answer1 = 1;
    while (true)
    {
        if (generates_clock(answer1))
            break;
        answer1 += 1;
    }

    std::cout << "Answer #1: " << answer1;
}
