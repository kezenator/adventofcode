#include <iostream>
#include <string>
#include <cassert>

std::string dragon(const std::string str)
{
    std::string result;
    result.reserve(1 + (2 * str.size()));

    result.append(str);
    result.push_back('0');

    for (size_t i = 0; i < str.size(); ++i)
    {
        result.push_back(str[str.size() - 1 - i] ^ 1);
    }

    return result;
}

std::string checksum(const std::string &data)
{
    std::string result = data;

    do 
    {
        assert((result.size() & 1) == 0);

        std::string next;
        next.resize(result.size() / 2);

        for (size_t i = 0; i < next.size(); ++i)
        {
            if (result[2 * i] != result[2 * i + 1])
                next[i] = '0';
            else
                next[i] = '1';
        }

        result = std::move(next);

    } while ((result.size() & 1) == 0);

    return result;
}

std::string checksum_for_disk(const std::string &input, size_t disk_size)
{
    std::string data = input;
    while (data.size() < disk_size)
    {
        data = dragon(data);
    }

    data.resize(disk_size);

    return checksum(data);
}

int main(int argc, char *argv[])
{
    std::cout << "Answer #1: " << checksum_for_disk("11011110011011101", 272) << std::endl;
    std::cout << "Answer #1: " << checksum_for_disk("11011110011011101", 35651584) << std::endl;
}