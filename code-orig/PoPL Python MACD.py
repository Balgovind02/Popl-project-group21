import pandas as pd
import numpy as np

def calculate_macd(stock_data, short_term_window, long_term_window, signal_window):
    # Calculate short-term and long-term exponential moving averages
    short_ema = stock_data['Close'].ewm(span=short_term_window).mean()
    long_ema = stock_data['Close'].ewm(span=long_term_window).mean()

    # Calculate MACD line: short_ema - long_ema
    macd = short_ema - long_ema

    # Calculate signal line: 9-day EMA of MACD line
    signal = macd.ewm(span=signal_window).mean()

    # Calculate histogram: MACD line - signal line
    histogram = macd - signal

    # Append the new columns to the original dataframe
    stock_data['MACD'] = macd
    stock_data['Signal'] = signal
    stock_data['Histogram'] = histogram

    return stock_data

def main():
    # Replace 'stock_data.csv' with your actual CSV file
    stock_data = pd.read_csv(r'C:\Users\91932\Downloads\stock_data.csv')  # Use 'r' prefix to treat it as a raw string

    # Strip leading and trailing spaces from column names
    stock_data.columns = stock_data.columns.str.strip()

    short_term_window = 12
    long_term_window = 26
    signal_window = 9

    stock_data = calculate_macd(stock_data, short_term_window, long_term_window, signal_window)

    print(stock_data)

if __name__ == "__main__":
    main()
