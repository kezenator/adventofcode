#include <cassert>
#include <iostream>
#include <iomanip>
#include <vector>
#include <sstream>
#include <boost/algorithm/string.hpp>

const std::string PUZZLE_INPUT = "187,254,0,81,169,219,1,190,19,102,255,56,46,32,2,216";

std::vector<size_t> to_vector(const std::string &str)
{
    std::vector<std::string> parts;
    boost::split(parts, str, boost::is_any_of(","));

    std::vector<size_t> result;
    result.reserve(parts.size());

    for (const auto &part : parts)
        result.push_back(std::atoi(part.c_str()));

    return result;
}

void twist(std::vector<size_t> &loop, size_t cur_pos, size_t twist_size)
{
    for (size_t i = 0; i < (twist_size / 2); ++i)
    {
        size_t ia = (cur_pos + i) % loop.size();
        size_t ib = (cur_pos + twist_size - 1 - i) % loop.size();

        std::swap(loop[ia], loop[ib]);
    }
}

std::vector<size_t> hash_rounds(size_t size, size_t num_rounds, const std::vector<size_t> &lengths)
{
    std::vector<size_t> loop;

    loop.reserve(size);
    for (size_t i = 0; i < size; ++i)
        loop.push_back(i);

    size_t cur_pos = 0;
    size_t skip_size = 0;

    for (size_t round = 0; round < num_rounds; ++round)
    {
        for (size_t twist_size : lengths)
        {
            twist(loop, cur_pos, twist_size);
            cur_pos = (cur_pos + twist_size + skip_size) % size;
            skip_size += 1;
        }
    }

    return loop;
}

int hash1(size_t size, const std::vector<size_t> &lengths)
{
    auto loop = hash_rounds(size, 1, lengths);

    return loop[0] * loop[1];
}

std::string hash2(const std::string &str)
{
    std::vector<size_t> lengths;
    lengths.reserve(str.size() + 5);

    for (char ch : str)
        lengths.push_back(ch);

    lengths.push_back(17);
    lengths.push_back(31);
    lengths.push_back(73);
    lengths.push_back(47);
    lengths.push_back(23);

    std::vector<size_t> loop = hash_rounds(256, 64, lengths);

    assert(loop.size() == 256);

    std::stringstream stream;

    for (size_t i = 0; i < 16; ++i)
    {
        size_t dense_entry = 0;

        for (size_t j = 0; j < 16; ++j)
        {
            dense_entry ^= loop[16*i + j];
        }

        stream << std::hex << std::setfill('0') << std::setw(2) << dense_entry;
    }

    return stream.str();
}

int main(int /*argc*/, const char */*argv*/[])
{
    assert(hash1(5, {3, 4, 1, 5}) == 12);

    assert(hash2("") == "a2582a3a0e66e6e86e3812dcb672a272");
    assert(hash2("AoC 2017") == "33efeb34ea91902bb2f59c9920caa6cd");
    assert(hash2("1,2,3") == "3efbe78a8d82f29979031a4aa0b16a9d");
    assert(hash2("1,2,4") == "63960835bcdc130f0b66d7ff4f6a5a8e");

    std::cout << "Answer #1: " << hash1(256, to_vector(PUZZLE_INPUT)) << std::endl;
    std::cout << "Answer #2: " << hash2(PUZZLE_INPUT) << std::endl;

    return 0;
}