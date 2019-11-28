#include <iostream>
#include <iomanip>
#include <sstream>
#include <string>
#include <set>
#include <vector>
#include <cassert>
#include <boost/optional.hpp>
#include <openssl/md5.h>

using Location = std::pair<uint64_t, uint64_t>;
using LocationAndPath = std::pair<Location, std::string>;

struct CompareByPathLength
{
    bool operator()(const LocationAndPath &a, const LocationAndPath &b) const
    {
        if (a.second.size() < b.second.size())
            return true;
        if (a.second.size() > b.second.size())
            return false;

        if (a.second < b.second)
            return true;
        if (a.second > b.second)
            return true;

        return a.first < b.first;
    }
};

std::string md5_hash_of_string(const std::string &str)
{
    MD5_CTX  ctx;
    MD5_Init(&ctx);
    MD5_Update(&ctx, str.c_str(), str.size());

    unsigned char digest[MD5_DIGEST_LENGTH];
    MD5_Final(digest, &ctx);

    std::stringstream stream;

    for (int i = 0; i < MD5_DIGEST_LENGTH; ++i)
        stream << std::hex << std::setfill('0') << std::setw(2) << (int)digest[i];

    return stream.str();
}

std::vector<std::pair<Location, char>> next_locations(const Location &from, const std::string &input, const std::string &path)
{
    std::string hash = md5_hash_of_string(input + path);

    std::vector<std::pair<Location, char>> result;

    auto is_open = [](char ch)
    {
        return (ch >= 'b') && (ch <= 'f');
    };

    if ((from.second > 0) && is_open(hash[0]))
    {
        result.emplace_back(Location(from.first, from.second - 1), 'U');
    }

    if ((from.second < 3) && is_open(hash[1]))
    {
        result.emplace_back(Location(from.first, from.second + 1), 'D');
    }

    if ((from.first > 0) && is_open(hash[2]))
    {
        result.emplace_back(Location(from.first - 1, from.second), 'L');
    }

    if ((from.first < 3) && is_open(hash[3]))
    {
        result.emplace_back(Location(from.first + 1, from.second), 'R');
    }

    return result;
}

std::string path_with_extreme_length(const std::string &input, bool shortest)
{
    const Location from(0, 0);
    const Location to(3, 3);

    std::set<LocationAndPath, CompareByPathLength> to_test;

    to_test.insert(std::make_pair(from, ""));

    boost::optional<std::string> opt_result;

    while (!to_test.empty())
    {
        Location test_location = to_test.begin()->first;
        std::string test_path = to_test.begin()->second;

        to_test.erase(to_test.begin());

        //std::cout << "Testing (" << test_location.first << ", " << test_location.second << "), path = " << test_path << std::endl;

        for (const std::pair<Location, char> new_move : next_locations(test_location, input, test_path))
        {
            const Location new_location = new_move.first;
            const std::string new_path = test_path + new_move.second;

            //std::cout << "   Got move to (" << new_location.first << ", " << new_location.second << "), direction = " << new_move.second << std::endl;

            if (new_location == to)
            {
                if (!opt_result)
                {
                    opt_result = new_path;
                }
                else if (shortest && (opt_result->size() > new_path.size()))
                {
                    opt_result = new_path;
                }
                else if (!shortest && (opt_result->size() < new_path.size()))
                {
                    opt_result = new_path;
                }

                // Don't search past here
                continue;
            }

            to_test.insert(std::make_pair(new_location, new_path));
        }
    }

    assert(opt_result);
    return *opt_result;
}

int main(int argc, char *argv[])
{
    std::string answer1 = path_with_extreme_length("udskfozm", true);
    size_t answer2 = path_with_extreme_length("udskfozm", false).size();

    std::cout << "Answer #1: " << answer1 << std::endl;
    std::cout << "Answer #2: " << answer2 << std::endl;
}
