#include <iostream>
#include <string>
#include <boost/algorithm/string.hpp>

const std::string input = R"(rect 1x1
rotate row y=0 by 20
rect 1x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 3
rect 2x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 3
rect 2x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 4
rect 2x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 3
rect 2x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 5
rect 1x1
rotate row y=0 by 2
rect 1x1
rotate row y=0 by 6
rect 5x1
rotate row y=0 by 2
rect 1x3
rotate row y=2 by 8
rotate row y=0 by 8
rotate column x=0 by 1
rect 7x1
rotate row y=2 by 24
rotate row y=0 by 20
rotate column x=5 by 1
rotate column x=4 by 2
rotate column x=2 by 2
rotate column x=0 by 1
rect 7x1
rotate column x=34 by 2
rotate column x=22 by 1
rotate column x=15 by 1
rotate row y=2 by 18
rotate row y=0 by 12
rotate column x=8 by 2
rotate column x=7 by 1
rotate column x=5 by 2
rotate column x=2 by 1
rotate column x=0 by 1
rect 9x1
rotate row y=3 by 28
rotate row y=1 by 28
rotate row y=0 by 20
rotate column x=18 by 1
rotate column x=15 by 1
rotate column x=14 by 1
rotate column x=13 by 1
rotate column x=12 by 2
rotate column x=10 by 3
rotate column x=8 by 1
rotate column x=7 by 2
rotate column x=6 by 1
rotate column x=5 by 1
rotate column x=3 by 1
rotate column x=2 by 2
rotate column x=0 by 1
rect 19x1
rotate column x=34 by 2
rotate column x=24 by 1
rotate column x=23 by 1
rotate column x=14 by 1
rotate column x=9 by 2
rotate column x=4 by 2
rotate row y=3 by 5
rotate row y=2 by 3
rotate row y=1 by 7
rotate row y=0 by 5
rotate column x=0 by 2
rect 3x2
rotate column x=16 by 2
rotate row y=3 by 27
rotate row y=2 by 5
rotate row y=0 by 20
rotate column x=8 by 2
rotate column x=7 by 1
rotate column x=5 by 1
rotate column x=3 by 3
rotate column x=2 by 1
rotate column x=1 by 2
rotate column x=0 by 1
rect 9x1
rotate row y=4 by 42
rotate row y=3 by 40
rotate row y=1 by 30
rotate row y=0 by 40
rotate column x=37 by 2
rotate column x=36 by 3
rotate column x=35 by 1
rotate column x=33 by 1
rotate column x=32 by 1
rotate column x=31 by 3
rotate column x=30 by 1
rotate column x=28 by 1
rotate column x=27 by 1
rotate column x=25 by 1
rotate column x=23 by 3
rotate column x=22 by 1
rotate column x=21 by 1
rotate column x=20 by 1
rotate column x=18 by 1
rotate column x=17 by 1
rotate column x=16 by 3
rotate column x=15 by 1
rotate column x=13 by 1
rotate column x=12 by 1
rotate column x=11 by 2
rotate column x=10 by 1
rotate column x=8 by 1
rotate column x=7 by 2
rotate column x=5 by 1
rotate column x=3 by 3
rotate column x=2 by 1
rotate column x=1 by 1
rotate column x=0 by 1
rect 39x1
rotate column x=44 by 2
rotate column x=42 by 2
rotate column x=35 by 5
rotate column x=34 by 2
rotate column x=32 by 2
rotate column x=29 by 2
rotate column x=25 by 5
rotate column x=24 by 2
rotate column x=19 by 2
rotate column x=15 by 4
rotate column x=14 by 2
rotate column x=12 by 3
rotate column x=9 by 2
rotate column x=5 by 5
rotate column x=4 by 2
rotate row y=5 by 5
rotate row y=4 by 38
rotate row y=3 by 10
rotate row y=2 by 46
rotate row y=1 by 10
rotate column x=48 by 4
rotate column x=47 by 3
rotate column x=46 by 3
rotate column x=45 by 1
rotate column x=43 by 1
rotate column x=37 by 5
rotate column x=36 by 5
rotate column x=35 by 4
rotate column x=33 by 1
rotate column x=32 by 5
rotate column x=31 by 5
rotate column x=28 by 5
rotate column x=27 by 5
rotate column x=26 by 3
rotate column x=25 by 4
rotate column x=23 by 1
rotate column x=17 by 5
rotate column x=16 by 5
rotate column x=13 by 1
rotate column x=12 by 5
rotate column x=11 by 5
rotate column x=3 by 1
rotate column x=0 by 1)";

class Screen
{
    Screen(const Screen &) = delete;
    Screen &operator =(const Screen &) = delete;

public:
    static constexpr size_t WIDTH = 50;
    static constexpr size_t HEIGHT = 6;

    Screen()
    {
        memset(m_pixels, 0, WIDTH * HEIGHT * sizeof(bool));
    }

    ~Screen() = default;

    void rect(int a, int b)
    {
        for (int i = 0; i < a; ++i)
        {
            for (int j = 0; j < b; ++j)
            {
                m_pixels[i][j] = true;
            }
        }
    }

    void rotate_row(int row, int by)
    {
        // Limit to one rotation

        by = by % WIDTH;

        // Create rotated row

        bool new_row[WIDTH];

        for (int i = 0; i < WIDTH; ++i)
        {
            new_row[i] = m_pixels[(i + WIDTH - by) % WIDTH][row];
        }

        // Copy onto screen

        for (int i = 0; i < WIDTH; ++i)
        {
            m_pixels[i][row] = new_row[i];
        }
    }

    void rotate_column(int column, int by)
    {
        // Limit to one rotation

        by = by % HEIGHT;

        // Create rotated column

        bool new_coumn[HEIGHT];

        for (int i = 0; i < HEIGHT; ++i)
        {
            new_coumn[i] = m_pixels[column][(i + HEIGHT - by) % HEIGHT];
        }

        // Copy onto screen

        for (int i = 0; i < HEIGHT; ++i)
        {
            m_pixels[column][i] = new_coumn[i];
        }
    }

    int count_on_pixels()
    {
        int count = 0;

        for (const auto &column : m_pixels)
        {
            for (bool pixel : column)
            {
                if (pixel)
                    ++count;
            }
        }

        return count;
    }

    void display()
    {
        for (int row = 0; row < HEIGHT; ++row)
        {
            for (int column = 0; column < WIDTH; ++column)
            {
                if (m_pixels[column][row])
                    std::cout << '#';
                else
                    std::cout << '.';
            }
            std::cout << std::endl;
        }
    }

private:
    bool m_pixels[WIDTH][HEIGHT];
};

void split(const std::string &str, std::string &&split, int &first, int &second)
{
    size_t split_offset = str.find(split[0]);

    std::string first_str = str.substr(0, split_offset);
    std::string second_str = str.substr(split_offset + split.size());

    first = atoi(first_str.c_str());
    second = atoi(second_str.c_str());
}

int main(int argc, char *argv[])
{
    std::vector<std::string> lines;
    boost::split(lines, input, boost::is_any_of("\n"));

    Screen screen;

    for (const std::string &line : lines)
    {
        if (line.substr(0, 5) == "rect ")
        {
            int a, b;
            split(line.substr(5), "x", a, b);
            screen.rect(a, b);
        }
        else if (line.substr(0, 16) == "rotate column x=")
        {
            int column, by;
            split(line.substr(16), " by ", column, by);
            screen.rotate_column(column, by);
        }
        else if (line.substr(0, 13) == "rotate row y=")
        {
            int row, by;
            split(line.substr(13), " by ", row, by);
            screen.rotate_row(row, by);
        }
    }

    screen.display();

    std::cout << "Answer #1: " << screen.count_on_pixels() << std::endl;
    std::cout << "Answer #2: Read it above" << std::endl;
}