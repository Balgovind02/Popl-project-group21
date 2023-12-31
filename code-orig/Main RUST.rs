fn macd_oscillator(
    closing_prices: &[f64],
    short_period: usize,
    long_period: usize,
    signal_period: usize,
) -> Vec<f64> {
    let mut ema_short = Vec::new();
    let mut ema_long = Vec::new();
    let mut macd_line = Vec::new();
    let mut signal_line = Vec::new();
    let mut macd_oscillator = Vec::new();

    for (i, price) in closing_prices.iter().enumerate() {
        if i == 0 {
            ema_short.push(*price);
            ema_long.push(*price);
            macd_line.push(0.0);
            signal_line.push(0.0);
            macd_oscillator.push(0.0);
        } else {
            let short_ema = (2.0 / (short_period + 1) as f64) * price
                + (1.0 - 2.0 / (short_period + 1) as f64) * ema_short[i - 1];
            let long_ema = (2.0 / (long_period + 1) as f64) * price
                + (1.0 - 2.0 / (long_period + 1) as f64) * ema_long[i - 1];
            ema_short.push(short_ema);
            ema_long.push(long_ema);

            let macd = short_ema - long_ema;
            macd_line.push(macd);

            if i < long_period {
                signal_line.push(0.0);
            } else {
                let signal = (2.0 / (signal_period + 1) as f64) * macd
                    + (1.0 - 2.0 / (signal_period + 1) as f64) * signal_line[i - long_period];
                signal_line.push(signal);
            }

            macd_oscillator.push(macd - signal_line[i]);
        }
    }

    macd_oscillator
}

fn main() {
    let closing_prices = vec![
        2062.8, 2080.25, 2086.72, 2070.6, 2078.78, 2096.91, 2104.25, 2124.21, 2134.38, 2133.28,
        2142.27, 2113.7, 2095.0, 2077.47, 2054.88, 1993.3, 2009.69, 2009.69, 1983.25, 1987.74,
        2016.04, 2043.78, 2063.39, 2038.21, 2032.43, 2005.11, 2008.53, 2029.95, 2043.18, 2012.48,
        1951.22, 2008.64, 2004.78, 2007.08, 1997.8, 1984.58, 1973.96, 1974.77, 1878.39, 1930.21,
        1952.02, 1952.02, 1939.7, 1930.08, 1894.81, 1851.11, 1871.21, 1906.1, 1935.71, 1939.11,
        1957.06, 1934.63, 1972.24, 2007.69, 2007.69, 1985.07, 2005.44, 2000.79, 2005.13, 1999.86,
        2005.28, 2017.41, 2035.84, 2034.09, 2056.73, 2087.65, 2087.81, 2079.49, 2062.74, 2083.3,
        2081.54, 2059.94, 2057.89, 2057.89, 2057.89, 2031.77, 2005.42, 2026.84, 2055.75, 2033.06,
        2000.97, 2036.38, 2012.82, 2038.62, 2016.94, 2009.79, 2009.79, 1962.98, 1966.01, 1930.65,
        1911.35, 1892.94, 1883.35, 1842.12, 1838.6, 1849.88, 1900.71, 1899.97, 1844.95, 1892.32,
        1885.76, 1871.14, 1852.52, 1869.97, 1889.49, 1932.13, 1924.74, 1912.54, 1926.85, 1917.12,
        1914.87, 1897.82, 1890.54, 1905.74, 1875.67, 1827.26, 1825.37, 1822.37, 1784.99, 1769.04,
        1768.31, 1808.88, 1779.89, 1798.2, 1815.57, 1831.14, 1836.1, 1829.16, 1824.29, 1823.22,
        1833.21, 1832.61, 1855.64, 1869.1, 1879.03, 1883.62, 1868.72, 1860.82, 1860.4, 1873.2,
        1901.47, 1908.95, 1926.11, 1942.25, 1948.92, 1939.07, 1917.77, 1936.74, 1969.38, 1998.88,
        2022.86, 2026.37, 2029.75, 2032.9, 2033.39, 2045.31, 2045.31, 2044.54, 2059.21, 2065.9,
        2065.9, 2084.82, 2100.54, 2102.09, 2079.85, 2046.95, 2056.84, 2060.31, 2052.15, 2057.33,
        2033.04, 2082.93, 2082.93, 2060.28, 2055.68, 2065.34, 2069.78, 2068.92, 2083.61, 2086.01,
        2103.34, 2116.56, 2104.04, 2095.95, 2048.79, 2058.52, 2081.72, 2065.25, 2063.61, 2028.21,
        1986.56, 1987.72, 1975.37, 1969.89, 1998.1, 1968.28, 2012.95, 2012.95, 2023.46, 2020.72,
        2011.86, 1979.56, 1994.44, 1981.64, 1991.58, 2004.98, 2025.8, 2026.04, 2037.58, 2034.71,
        2048.85, 2042.45, 2042.45, 2055.78, 2058.69, 2083.71, 2100.65, 2091.76, 2088.49, 2097.36,
        2107.3, 2107.3, 2098.85, 2080.31, 2111.18, 2111.16, 2118.79, 2112.09, 2102.38, 2096.54,
        2079.52, 2088.32, 2090.62, 2112.61, 2117.51, 2124.87, 2132.95, 2154.17, 2159.71, 2149.48,
        2147.4, 2142.24, 2133.21, 2136.79, 2120.86, 2120.48, 2130.65, 2135.87, 2111.31, 2090.73,
        2109.67, 2106.61, 2083.26,
    ];

    let short_period = 12;
    let long_period = 26;
    let signal_period = 9;

    let macd_oscillator_values =
        macd_oscillator(&closing_prices, short_period, long_period, signal_period);

    println!("MACD Oscillator Values: {:?}", macd_oscillator_values);
}
