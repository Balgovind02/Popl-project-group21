# Popl-project-group21

## PROBLEM STATEMENT
The aim of the project is to develop a robust algorithmic trading system by addressing Python's memory management challenges, including latency due to garbage collection, potential memory leaks, GIL limitations, and inefficiencies in data structures. Seeking to enhance real-time signal generation, minimize memory bloat, and optimize performance for quantitative finance using Rust's efficient memory management, safety assurances, improved parallelism, and optimized data structures.

## POPL ANGLE:
The task involves considerations related to memory management and efficiency in the context of algorithmic trading. The principles of programming languages being considered include:

- Memory Management Efficiency: Addressing issues such as garbage collection overhead and memory leaks to ensure optimal use of memory resources.

- Safety Guarantees: Emphasizing the importance of preventing memory leaks and ensuring secure memory management, critical for maintaining system integrity in algorithmic trading.

- Parallelism: Dealing with the Global Interpreter Lock (GIL) limitation in Python by seeking a programming language that allows for improved parallelism, enabling efficient handling of data and real-time signal generation.

- Optimized Data Structures: Recognizing the significance of using efficient data structures for quantitative finance, aiming to minimize memory usage, reduce execution times, and lower hardware requirements.

- Predictable Performance: Considering the need for a language with predictable performance characteristics, essential for the timely execution of trading signals in dynamic market conditions.

### What problem do we solve?

The task aims to address various challenges prevalent in algorithmic trading systems built using Python, particularly related to memory management. The key problems to solve include:

- Memory Management Overhead: Python's automatic memory management, including garbage collection, introduces latency that can impact the timely execution of trading signals. This delay is undesirable in high-frequency trading environments.

- Memory Leaks: Python's dynamic typing and reference counting can lead to memory leaks if not managed properly. Over time, this could result in memory bloat and degrade the performance of the trading system.

- Global Interpreter Lock (GIL): Python's GIL limits the efficiency of multi-threading and parallelism, making it challenging to fully utilize modern processors for memory-intensive computations. This restricts the system's ability to handle real-time data effectively.

- Inefficient Data Structures: Python's built-in data structures might not be as memory-efficient as those in lower-level languages like Rust. In quantitative finance, where optimization and speed are crucial, inefficient data structures can impact execution times and hardware requirements.

### Examples of POPL aspects implemented in our code:

#### MEMORY MANAGEMENT

Rust:

let mut ema_short = Vec::new();
let mut ema_long = Vec::new();
// ... other vectors initialized similarly …

Controlled Memory Allocation: Rust allows explicit memory allocation and precise control over data structures. Vectors are preallocated, ensuring efficient memory utilization and minimizing reallocation overhead during computations.

Python:

short_ema = stock_data['Close'].ewm(span=short_term_window).mean()
long_ema = stock_data['Close'].ewm(span=long_term_window).mean()
 ... Pandas Series created dynamically …
Dynamic Memory Allocation: Python's Pandas library dynamically allocates memory for Series and DataFrames, potentially leading to increased memory overhead and less predictable memory management compared to Rust's explicit memory control.

#### PERFORMANCE OPTIMIZAZTION

Rust:

let short_ema = (2.0 / (short_period + 1) as f64) * price + (1.0 - 2.0 / (short_period + 1) as f64) * ema_short[i - 1];
let long_ema = (2.0 / (long_period + 1) as f64) * price + (1.0 - 2.0 / (long_period + 1) as f64) * ema_long[i - 1];
// ... arithmetic operations optimized for performance …

Control over Arithmetic Operations:Rust allows fine-tuning arithmetic operations for performance, potentially resulting in optimized execution compared to Python, especially for handling large datasets or complex calculations.

Python:

short_ema = stock_data['Close'].ewm(span=short_term_window).mean()
long_ema = stock_data['Close'].ewm(span=long_term_window).mean()
# ... computations performed using higher-level abstractions …

Higher-Level Abstractions: Python operates at a higher level of abstraction, which might introduce more overhead compared to Rust's lower-level control over computations.

#### TYPE SAFETY

Rust:

let short_ema: f64 = ...;
let long_ema: f64 = ...;
// ... explicit typing ensures type safety …

Static Typing and Compile-Time Checks: Rust enforces static typing, catching type-related issues at compile time, reducing the chances of runtime errors due to type mismatches or unexpected data types.
Python:

 No explicit typing in the provided code snippet
Dynamic Typing: Python is dynamically typed, which might lead to runtime errors if unexpected data types are encountered during computations.

## 4. RESULTS
The outputs as shown in the results folder show that Rust takes 0.503s (Fig 2) for total execution while python takes 0.783s (Fig 5). While this difference seems insignificant, it is occurring when the time period for which the data is being processed is that of only 1 year. When we take historic data of over 20 years, this difference will multiply significantly.

The CPU utilization of Rust is also better with it having only 18% utilization while python has a utilization of 323%.(When the CPU usage is reported as a value greater than 100%, it typically indicates that the program or process is using more than one CPU core.) Thus, Rust is safer and uses less memory in the CPU at the same time.

