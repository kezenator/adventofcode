#include <iostream>
#include <iomanip>
#include <string>
#include <sstream>
#include <map>
#include <openssl/md5.h>

const std::string input = R"(yjdafjpo)";

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

std::string md5_hash_of_index(uint64_t index)
{
    std::stringstream stream;
    stream << input << index;

    return md5_hash_of_string(stream.str());
}

std::string streched_hash(uint64_t index)
{
    std::string hash = md5_hash_of_index(index);

    for (int i = 0; i < 2016; ++i)
    {
        hash = md5_hash_of_string(hash);
    }

    return hash;
}

class HashMemorizer
{
public:
    explicit HashMemorizer(std::string(*_hash_func)(uint64_t))
        : m_hash_func(_hash_func)
        , m_mem()
    {
    }

    const std::string &hash(uint64_t index)
    {
        auto it = m_mem.find(index);

        if (it == m_mem.end())
        {
            m_mem[index] = m_hash_func(index);
            return m_mem[index];
        }

        return it->second;
    }

private:
    std::string(*m_hash_func)(uint64_t);
    std::map<uint64_t, std::string> m_mem;
};

bool is_key(uint64_t num, HashMemorizer &hash_mem)
{
    std::string hash = hash_mem.hash(num);

    for (size_t i = 0; i <= (hash.size() - 3); ++i)
    {
        if ((hash[i] == hash[i + 1])
            && (hash[i] == hash[i + 2]))
        {
            std::string run(5, hash[i]);
            for (size_t j = 1; j <= 1000; ++j)
            {
                std::string future = hash_mem.hash(num + j);

                if (future.find(run, 0) != std::string::npos)
                {
                    //std::cout << "** " << num << " is a key" << std::endl;
                    return true;
                }
            }

            // Only consider the first such triplet
            return false;
        }
    }

    // No triplets
    return false;
}

uint64_t index_of_64th_key(HashMemorizer &hash_mem)
{
    uint64_t result = 0;
    int key_count = 0;
    while (true)
    {
        if (is_key(result, hash_mem))
        {
            key_count += 1;
            if (key_count == 64)
                return result;
        }
        result += 1;
    }
}

int main(int argc, char *argv[])
{
    HashMemorizer mem1(&md5_hash_of_index);
    HashMemorizer mem2(&streched_hash);

    uint64_t answer1 = index_of_64th_key(mem1);
    uint64_t answer2 = index_of_64th_key(mem2);

    std::cout << "Answer #1: " << answer1 << std::endl;
    std::cout << "Answer #2: " << answer2 << std::endl;
}