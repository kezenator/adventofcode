#include <iostream>
#include <string>
#include <limits>
#include <vector>
#include <boost/algorithm/string.hpp>

const std::string PUZZLE_INPUT = R"(278	1689	250	1512	1792	1974	175	1639	235	1635	1690	1947	810	224	928	859
160	50	55	81	68	130	145	21	211	136	119	78	174	155	149	72
4284	185	4499	273	4750	4620	4779	4669	2333	231	416	1603	197	922	5149	2993
120	124	104	1015	1467	110	299	320	1516	137	1473	132	1229	1329	1430	392
257	234	3409	2914	2993	3291	368	284	259	3445	245	1400	3276	339	2207	233
1259	78	811	99	2295	1628	3264	2616	116	3069	2622	1696	1457	1532	268	82
868	619	139	522	168	872	176	160	1010	200	974	1008	1139	552	510	1083
1982	224	3003	234	212	1293	1453	3359	326	3627	3276	3347	1438	2910	248	2512
4964	527	5108	4742	4282	4561	4070	3540	196	228	3639	4848	152	1174	5005	202
1381	1480	116	435	980	1022	155	1452	1372	121	128	869	1043	826	1398	137
2067	2153	622	1479	2405	1134	2160	1057	819	99	106	1628	1538	108	112	1732
4535	2729	4960	241	4372	3960	248	267	230	5083	827	1843	3488	4762	2294	3932
3245	190	2249	2812	2620	2743	2209	465	139	2757	203	2832	2454	177	2799	2278
1308	797	498	791	1312	99	1402	1332	521	1354	1339	101	367	1333	111	92
149	4140	112	3748	148	815	4261	138	1422	2670	32	334	2029	4750	4472	2010
114	605	94	136	96	167	553	395	164	159	284	104	530	551	544	18)";

std::vector<std::vector<int>> parse()
{
    std::vector<std::vector<int>> result;

    std::vector<std::string> lines;
    boost::split(lines, PUZZLE_INPUT, boost::is_any_of("\n"));

    for (const std::string &line : lines)
    {
        result.emplace_back();

        std::vector<std::string> terms;
        boost::split(terms, line, boost::is_any_of("\t"));

        for (const std::string &term : terms)
        {
            result.back().push_back(std::atoi(term.c_str()));
        }
    }

    return result;
}

int part1()
{
    int result = 0;

    for (const auto &row : parse())
    {
        int min = std::numeric_limits<int>::max();
        int max = std::numeric_limits<int>::min();

        for (int val : row)
        {
            if (val < min)
                min = val;
            if (val > max)
                max = val;
        }

        result += max - min;
    }

    return result;
}

int part2()
{
    int result = 0;

    for (const auto &row : parse())
    {
        for (size_t x = 0; x < row.size(); ++x)
        {
            for (size_t y = 0; y < row.size(); ++y)
            {
                if ((x != y)
                    && (row[x] > row[y])
                    && ((row[x] % row[y]) == 0))
                {
                    result += row[x] / row[y];
                }
            }
        }
    }

    return result;
}

int main(int /*argc*/, const char */*argv*/[])
{
    std::cout << "Answer #1: " << part1() << std::endl;
    std::cout << "Answer #2: " << part2() << std::endl;
    return 0;
}