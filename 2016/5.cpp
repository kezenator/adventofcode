#include <iostream>
#include <iomanip>
#include <string>
#include <sstream>
#include <chrono>
#include <openssl/md5.h>

const std::string input = R"(reyedfim)";

std::string md5_hash(const std::string &str)
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


int main(int argc, char *argv[])
{
    std::string answer1;
    std::string answer2("--------");

    std::cout << answer2 << std::flush;
    auto last_print = std::chrono::steady_clock::now();
    uint64_t last_check = 0;

    for (uint64_t i = 0; (answer1.size() < 8) || (answer2.find('-') != std::string::npos); ++i)
    {
        std::stringstream stream;
        stream << input << i;

        std::string hash = md5_hash(stream.str());

        if ((hash[0] == '0')
            && (hash[1] == '0')
            && (hash[2] == '0')
            && (hash[3] == '0')
            && (hash[4] == '0'))
        {
            if (answer1.size() < 8)
            {
                answer1.push_back(hash[5]);
            }

            if ((hash[5] >= '0')
                && (hash[5] <= '7')
                && (answer2[hash[5] - '0'] == '-'))
            {
                answer2[hash[5] - '0'] = hash[6];
            }
        }

        // Be extra proud of your solution if it
        // uses a cinematic "decrypting" animation.

        if ((last_check + 1000) > i)
        {
            last_check = i;

            auto now = std::chrono::steady_clock::now();

            if ((now - last_print) > std::chrono::milliseconds(100))
            {
                last_print = now;

                std::string random = answer2;
                for (char &ch : random)
                {
                    if (ch == '-')
                        ch = 'a' + (rand() % 26);
                }

                std::cout << '\r' << random << std::flush;
            }
        }
    }

    std::cout << std::endl;

    std::cout << "Answer #1 : " << answer1 << std::endl;
    std::cout << "Answer #2 : " << answer2 << std::endl;

    return 0;
}