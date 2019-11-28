#include <iostream>
#include <string>
#include <boost/algorithm/string.hpp>

const std::string input = R"(cpy a b
dec b
cpy a d
cpy 0 a
cpy b c
inc a
dec c
jnz c -2
dec d
jnz d -5
dec b
cpy b c
cpy c d
dec d
inc c
jnz d -2
tgl c
cpy -16 c
jnz 1 c
cpy 77 c
jnz 73 d
inc a
inc d
jnz d -2
inc c
jnz c -5)";

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
    int64_t registers[4];
    bool complete;

    ComputerState()
        : pc(0)
        , complete(false)
    {
        for (int i = 0; i < 4; ++i)
            registers[i] = 0;
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

    void update(std::vector<Instruction> &instructions)
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
            else if (inst.code == "tgl")
            {
                int target_inst_index = pc + get(inst.args[0]);

                if ((target_inst_index >= 0)
                    && (target_inst_index < instructions.size()))
                {
                    Instruction &target_inst = instructions[target_inst_index];

                    if (target_inst.args.size() == 1)
                    {
                        if (target_inst.code == "inc")
                            target_inst.code = "dec";
                        else
                            target_inst.code = "inc";
                    }
                    else // 2-argument instructions
                    {
                        if (target_inst.code == "jnz")
                            target_inst.code = "cpy";
                        else
                            target_inst.code = "jnz";
                    }
                }

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

int run_with_arg(int init)
{
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

    while (!state.complete)
    {
        state.update(instructions);

        //state.print();
    }

    return state.registers[0];
}

int main(int argc, char *argv[])
{
    int answer1 = run_with_arg(7);

    std::cout << "Answer #1: " << answer1 << std::endl;

    std::cout << "Answer #2: Calculate f(12) based on these values:" << std::endl;
    for (int i = 7; i <= 10; ++i)
    {
        std::cout << "   f(" << i << ") = " << run_with_arg(i) << std::endl;
    }
    std::cout << std::endl;
    std::cout << "For my input, it looks like f(x) = 5621 + factorial(x)" << std::endl;
    std::cout << "  so f(12) = 5621 + 12! = 5621 = 479007221" << std::endl;
}
