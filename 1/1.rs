fn main() {
    let mut count = 0;

    // total 4 increase sample
    //let inputs = [2, 3, 4, 2, 1, 5, 10];

    // GROSS but i can't be bothered to learn to ingest files just yet
    let inputs = [141, 152, 164, 163, 164, 179, 210, 209, 208, 236, 227, 228, 227, 223, 226, 225, 228, 234, 218, 215, 206, 215, 218, 215, 209, 226, 233, 236, 234, 232, 230, 225, 226, 224, 216, 213, 221, 224, 225, 228, 227, 198, 196, 190, 167, 177, 191, 186, 189, 184, 174, 163, 152, 164, 155, 158, 172, 170, 174, 181, 180, 189, 190, 185, 182, 183, 201, 209, 216, 217, 218, 219, 220, 213, 212, 215, 211, 216, 234, 233, 215, 208, 215, 210, 209, 215, 217, 213, 236, 237, 238, 237, 261, 263, 257, 259, 262, 277, 278, 275, 279, 271, 272, 282, 277, 241, 233, 225, 230, 235, 226, 225, 229, 231, 225, 223, 226, 217, 219, 223, 226, 217, 194, 203, 205, 209, 192, 184, 182, 189, 190, 189, 188, 186, 185, 212, 214, 211, 213, 234, 235, 234, 235, 223, 201, 207, 227, 222, 221, 239, 240, 229, 242, 245, 242, 261, 260, 253, 236, 230, 243, 263, 261, 257, 267, 261, 235, 244, 219, 220, 229, 239, 218, 229, 228, 235, 236, 237, 242, 225, 222, 230, 239, 240, 258, 259, 299, 328, 330, 331, 339, 338, 339, 340, 339, 340, 323, 320, 321, 322, 323, 342, 369, 388, 379, 363, 362, 369, 383, 395, 392, 365, 380, 411, 407, 408, 406, 405, 403, 405, 403, 399, 400, 385, 388, 390, 399, 401, 404, 408, 406, 389, 388, 391, 399, 410, 411, 412, 415, 408, 416, 417, 441, 442, 440, 442, 446, 443, 439, 440, 443, 479, 490, 495, 509, 486, 487, 519, 504, 510, 514, 504, 505, 507, 513, 511, 521, 522, 529, 534, 531, 533, 512, 522, 523, 540, 542, 562, 564, 567, 592, 599, 605, 625, 635, 639, 642, 643, 637, 631, 629, 628, 629, 628, 629, 621, 619, 618, 617, 618, 613, 612, 609, 603, 602, 592, 591, 590, 602, 600, 599, 602, 604, 602, 617, 615, 594, 578, 582, 585, 584, 595, 615, 614, 617, 614, 631, 660, 664, 653, 657, 667, 668, 667, 669, 659, 662, 659, 654, 648, 636, 637, 635, 647, 635, 644, 642, 639, 636, 641, 638, 637, 647, 646, 644, 643, 642, 639, 645, 649, 650, 656, 661, 644, 645, 646, 632, 640, 645, 646, 647, 648, 651, 645, 630, 635, 636, 640, 641, 643, 640, 636, 641, 648, 650, 659, 682, 675, 693, 686, 682, 677, 678, 679, 678, 682, 657, 641, 632, 652, 645, 659, 638, 634, 633, 635, 636, 622, 626, 625, 624, 646, 639, 640, 641, 638, 637, 638, 635, 636, 637, 638, 637, 640, 659, 651, 653, 647, 652, 643, 645, 652, 675, 668, 663, 666, 650, 651, 652, 661, 645, 640, 631, 632, 634, 637, 641, 642, 647, 660, 662, 664, 669, 668, 666, 683, 685, 701, 702, 683, 681, 676, 677, 683, 668, 647, 659, 661, 665, 666, 665, 659, 639, 644, 624, 622, 620, 617, 620, 636, 620, 621, 646, 644, 643, 650, 646, 639, 640, 647, 636, 633, 634, 633, 642, 643, 652, 648, 629, 642, 649, 633, 632, 633, 631, 626, 627, 641, 647, 646, 643, 644, 648, 656, 654, 672, 673, 666, 670, 676, 699, 704, 705, 672, 671, 670, 684, 685, 689, 688, 695, 700, 702, 733, 734, 706, 702, 713, 709, 714, 718, 716, 693, 699, 720, 701, 694, 700, 707, 708, 702, 703, 705, 719, 717, 719, 738, 726, 721, 719, 706, 712, 729, 730, 715, 717, 730, 733, 730, 749, 756, 759, 760, 758, 759, 768, 769, 777, 778, 780, 784, 803, 804, 820, 813, 816, 815, 820, 818, 804, 795, 794, 791, 792, 791, 802, 793, 794, 789, 793, 778, 776, 752, 756, 741, 745, 747, 748, 758, 762, 777, 779, 784, 785, 789, 790, 787, 791, 789, 772, 813, 816, 818, 811, 818, 823, 824, 815, 816, 819, 818, 819, 820, 821, 825, 827, 837, 821, 823, 812, 814, 816, 840, 855, 857, 855, 871, 881, 889, 893, 892, 897, 901, 902, 896, 898, 912, 913, 904, 905, 906, 902, 859, 863, 862, 865, 868, 896, 906, 907, 896, 897, 902, 903, 900, 901, 895, 897, 903, 907, 914, 916, 919, 920, 929, 932, 931, 932, 915, 920, 922, 909, 908, 907, 906, 908, 910, 908, 877, 878, 876, 873, 872, 883, 868, 872, 868, 862, 875, 881, 883, 885, 879, 877, 880, 882, 881, 882, 881, 866, 869, 867, 879, 865, 856, 855, 861, 871, 876, 864, 865, 849, 848, 842, 807, 805, 810, 836, 816, 815, 857, 859, 857, 864, 865, 868, 871, 875, 874, 868, 857, 840, 841, 840, 838, 845, 833, 844, 884, 883, 884, 880, 884, 876, 870, 890, 889, 882, 887, 890, 883, 881, 880, 882, 888, 884, 900, 889, 887, 890, 891, 892, 897, 908, 909, 906, 907, 884, 882, 883, 869, 883, 872, 870, 867, 870, 857, 858, 854, 853, 848, 857, 856, 857, 864, 860, 857, 873, 835, 836, 835, 837, 839, 840, 844, 846, 838, 839, 824, 827, 829, 820, 821, 820, 822, 794, 817, 845, 846, 848, 849, 847, 850, 828, 820, 821, 826, 835, 841, 842, 846, 855, 837, 839, 848, 864, 867, 869, 874, 894, 876, 877, 876, 873, 874, 868, 866, 870, 842, 844, 819, 811, 818, 813, 811, 812, 813, 820, 823, 818, 815, 819, 816, 817, 807, 806, 811, 813, 819, 825, 811, 829, 832, 825, 826, 831, 830, 831, 833, 825, 827, 815, 823, 812, 814, 815, 812, 834, 839, 845, 844, 849, 856, 855, 862, 866, 873, 859, 851, 843, 819, 804, 790, 824, 827, 823, 833, 832, 833, 849, 882, 892, 920, 911, 910, 909, 913, 905, 908, 909, 913, 914, 915, 932, 941, 942, 976, 977, 984, 983, 984, 989, 982, 986, 985, 992, 1001, 1022, 1024, 1010, 1007, 996, 978, 979, 969, 957, 951, 948, 946, 943, 930, 929, 922, 924, 923, 924, 953, 949, 950, 922, 890, 889, 882, 902, 901, 896, 871, 903, 909, 931, 937, 945, 953, 958, 963, 964, 994, 971, 970, 959, 968, 974, 973, 974, 973, 972, 973, 972, 973, 977, 1012, 1033, 1032, 1029, 1027, 1028, 1055, 1056, 1070, 1065, 1091, 1074, 1066, 1074, 1075, 1084, 1083, 1081, 1075, 1074, 1071, 1098, 1087, 1076, 1075, 1079, 1094, 1104, 1105, 1111, 1110, 1108, 1102, 1064, 1060, 1076, 1048, 1059, 1065, 1077, 1054, 1061, 1066, 1087, 1093, 1108, 1107, 1105, 1102, 1101, 1105, 1106, 1100, 1104, 1105, 1107, 1106, 1096, 1077, 1075, 1076, 1083, 1091, 1080, 1058, 1050, 1051, 1064, 1074, 1077, 1049, 1051, 1052, 1063, 1077, 1079, 1085, 1074, 1078, 1073, 1070, 1065, 1080, 1097, 1071, 1072, 1074, 1102, 1104, 1113, 1114, 1123, 1125, 1129, 1131, 1149, 1152, 1147, 1154, 1161, 1162, 1163, 1169, 1157, 1151, 1158, 1159, 1155, 1154, 1121, 1112, 1113, 1080, 1082, 1075, 1076, 1071, 1062, 1074, 1079, 1076, 1097, 1100, 1091, 1084, 1098, 1083, 1103, 1132, 1134, 1125, 1127, 1128, 1129, 1131, 1130, 1131, 1135, 1131, 1129, 1142, 1154, 1142, 1141, 1145, 1152, 1158, 1172, 1162, 1179, 1180, 1179, 1190, 1187, 1191, 1190, 1186, 1198, 1182, 1178, 1174, 1158, 1154, 1159, 1160, 1172, 1164, 1145, 1161, 1170, 1181, 1189, 1192, 1186, 1182, 1173, 1190, 1178, 1177, 1184, 1186, 1193, 1198, 1197, 1194, 1203, 1177, 1195, 1196, 1201, 1208, 1205, 1204, 1205, 1195, 1196, 1219, 1220, 1217, 1219, 1220, 1213, 1214, 1219, 1230, 1237, 1241, 1243, 1242, 1243, 1267, 1268, 1264, 1275, 1278, 1273, 1274, 1281, 1286, 1284, 1288, 1284, 1294, 1259, 1280, 1285, 1284, 1300, 1303, 1313, 1303, 1317, 1316, 1315, 1307, 1320, 1333, 1342, 1356, 1359, 1364, 1376, 1382, 1385, 1389, 1390, 1391, 1387, 1388, 1389, 1387, 1388, 1409, 1402, 1400, 1394, 1390, 1356, 1355, 1356, 1359, 1371, 1375, 1376, 1358, 1361, 1344, 1371, 1379, 1373, 1367, 1364, 1365, 1364, 1363, 1362, 1361, 1364, 1361, 1360, 1362, 1367, 1366, 1383, 1384, 1389, 1390, 1416, 1415, 1416, 1440, 1431, 1440, 1441, 1444, 1448, 1453, 1446, 1447, 1428, 1426, 1452, 1453, 1431, 1423, 1421, 1420, 1410, 1417, 1418, 1417, 1412, 1402, 1401, 1406, 1402, 1416, 1419, 1405, 1408, 1413, 1415, 1406, 1407, 1408, 1394, 1405, 1397, 1378, 1382, 1381, 1380, 1384, 1385, 1384, 1392, 1399, 1405, 1409, 1407, 1409, 1410, 1413, 1406, 1422, 1421, 1419, 1420, 1421, 1429, 1433, 1417, 1419, 1416, 1411, 1410, 1415, 1427, 1443, 1439, 1438, 1435, 1436, 1438, 1442, 1458, 1463, 1455, 1481, 1480, 1481, 1491, 1498, 1496, 1504, 1502, 1499, 1494, 1493, 1494, 1496, 1500, 1503, 1494, 1492, 1485, 1487, 1488, 1508, 1510, 1534, 1530, 1540, 1502, 1501, 1494, 1495, 1492, 1490, 1505, 1500, 1501, 1499, 1501, 1495, 1493, 1498, 1504, 1510, 1519, 1535, 1531, 1537, 1546, 1557, 1559, 1558, 1576, 1585, 1590, 1588, 1591, 1603, 1620, 1619, 1620, 1651, 1650, 1628, 1630, 1631, 1630, 1636, 1635, 1636, 1625, 1624, 1622, 1611, 1610, 1608, 1613, 1612, 1609, 1616, 1615, 1637, 1638, 1643, 1645, 1648, 1649, 1650, 1649, 1647, 1648, 1653, 1651, 1654, 1660, 1656, 1654, 1653, 1657, 1646, 1652, 1653, 1670, 1695, 1689, 1690, 1670, 1679, 1664, 1660, 1641, 1625, 1629, 1643, 1647, 1655, 1664, 1667, 1668, 1672, 1671, 1690, 1689, 1691, 1690, 1704, 1701, 1704, 1715, 1710, 1728, 1719, 1742, 1737, 1738, 1727, 1732, 1718, 1720, 1727, 1737, 1741, 1744, 1745, 1747, 1758, 1753, 1745, 1752, 1770, 1778, 1782, 1812, 1813, 1809, 1806, 1804, 1799, 1821, 1845, 1841, 1845, 1850, 1857, 1873, 1879, 1909, 1907, 1906, 1897, 1900, 1897, 1898, 1896, 1900, 1909, 1939, 1940, 1929, 1948, 1959, 1929, 1939, 1963, 1961, 1983, 1984, 1985, 1980, 1981, 1982, 1972, 1973, 1974, 1988, 1996, 1999, 1995, 1982, 1985, 1995, 2008, 2007, 2023, 2025, 2024, 2026, 2025, 2020, 2013, 2014, 2025, 2048, 2055, 2060, 2063, 2066, 2067, 2070, 2071, 2072, 2062, 2057, 2025, 2028, 2029, 2028, 2020, 2015, 2016, 2014, 2012, 2016, 2015, 2014, 2005, 2008, 2009, 2005, 1999, 2003, 1995, 1994, 1990, 1992, 1996, 1995, 1990, 1991, 1999, 2000, 1989, 1991, 1992, 1994, 2024, 2030, 2029, 2024, 2023, 2044, 2045, 2044, 2042, 2040, 2049, 2050, 2046, 2044, 2049, 2064, 2059, 2063, 2064, 2061, 2059, 2062, 2064, 2075, 2074, 2085, 2088, 2075, 2102, 2101, 2082, 2083, 2085, 2084, 2097, 2091, 2088, 2084, 2074, 2082, 2083, 2084, 2089, 2088, 2091, 2095, 2105, 2124, 2118, 2121, 2116, 2117, 2118, 2119, 2127, 2131, 2120, 2154, 2152, 2160, 2170, 2166, 2169, 2173, 2174, 2156, 2149, 2147, 2138, 2140, 2141, 2136, 2137, 2138, 2139, 2129, 2130, 2128, 2120, 2112, 2119, 2126, 2129, 2128, 2137, 2142, 2161, 2147, 2139, 2142, 2148, 2152, 2154, 2136, 2119, 2121, 2098, 2115, 2099, 2100, 2104, 2099, 2098, 2097, 2112, 2108, 2147, 2146, 2148, 2147, 2125, 2099, 2103, 2107, 2111, 2123, 2126, 2128, 2130, 2129, 2131, 2126, 2127, 2130, 2112, 2103, 2100, 2113, 2121, 2118, 2119, 2129, 2123, 2112, 2102, 2069, 2071, 2075, 2094, 2102, 2106, 2096, 2097, 2100, 2109, 2104, 2103, 2108, 2100, 2103, 2104, 2108, 2115, 2117, 2112, 2120, 2127, 2128, 2126, 2106, 2104, 2106, 2100, 2105, 2102, 2112, 2119, 2127, 2129, 2136, 2150, 2166, 2153, 2181, 2159, 2160, 2159, 2183, 2184, 2186, 2174, 2173, 2183, 2178, 2163, 2156, 2161, 2162, 2163, 2156, 2145, 2149, 2145, 2147, 2151, 2147, 2150, 2138, 2150, 2151, 2149, 2150, 2151, 2154, 2136, 2173, 2184, 2183, 2185, 2186, 2187, 2185, 2184, 2188, 2186, 2190, 2197, 2166, 2167, 2166, 2167, 2170, 2178, 2196, 2181, 2182, 2184, 2196, 2207, 2206, 2211, 2218, 2217, 2216, 2212, 2216, 2232, 2235, 2241, 2242, 2238, 2239, 2236, 2250, 2272, 2281, 2277, 2278, 2298, 2294, 2296, 2292, 2285, 2303, 2296, 2290, 2292, 2272, 2285, 2287, 2311, 2315, 2336, 2344, 2345, 2326, 2331, 2333, 2344, 2359, 2366, 2364, 2366, 2370, 2368, 2360, 2373, 2397, 2394, 2398, 2397, 2399, 2407, 2412, 2426, 2424, 2461, 2469, 2467, 2466, 2465, 2464, 2473, 2475, 2474, 2471, 2473, 2469, 2470, 2460, 2461, 2478, 2465, 2471, 2475, 2482, 2485, 2494, 2492, 2482, 2493, 2509, 2501, 2513, 2486, 2489, 2492, 2505, 2507, 2505, 2506, 2505, 2507, 2509, 2510, 2511, 2518, 2523, 2529, 2540, 2526, 2530, 2535, 2550, 2552, 2565, 2558, 2569, 2563, 2573, 2577, 2579, 2575, 2573, 2577, 2575, 2591, 2607, 2609, 2614, 2619, 2648, 2649, 2650, 2646, 2648, 2651, 2660, 2653, 2645, 2651, 2652, 2658, 2657, 2627, 2628, 2642, 2631, 2632, 2634, 2639, 2648, 2647, 2648, 2649, 2654, 2655, 2660, 2671, 2682];


    for i in 0..inputs.len() {

	    if i == 0 { 
		    println!("nope."); // find a noop or next/skip/continue
	    } 
	    else if inputs[i] > inputs[i-1] {
	    	count += 1;
	    }
    }

    println!("{}", count); // because main() can't return an int
                           // at least not like this
}