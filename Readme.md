# Word Search and Matching System

This project implements a word search and matching system that identifies occurrences of given words in one or more files. It uses the Aho-Corasick algorithm to efficiently perform multi-word searches.

## Table of Contents

- [Getting Started](#getting-started)
- [Usage](#usage)
- [Code Explanation](#code-explanation)
- [Requirements](#requirements)
- [Contributors](#contributors)
- [License](#license)

## Getting Started

This project is implemented in Rust and is designed for fast word searching and matching. To run this project on your local machine, follow these steps:

### Requirements

- Rust programming language (can be installed using Rustup)
- Cargo (Rust package manager)


## Usage

To run the program, use the following command in your terminal:

1. Start the executable:
    ```sh
    cargo run
    ```

2. The program will prompt you to enter the path to a CSV file and a directory:
    - **CSV file:** A text file containing the list of words to search for, with each word on a new line.
    - **Directory:** The directory containing the files to be searched.

The program will search for the words in each file and output the locations where each word occurs.

## Code Explanation

### Main Functions

- **`get_file_size(path: &str) -> io::Result<u64>`**: Returns the size of the given file.
- **`read_all_bytes(path: &str) -> io::Result<Vec<u8>>`**: Reads the entire file as bytes.
- **`build_matching_machine(arr: &[String], k: usize)`**: Constructs the word matching machine.
- **`find_next_state(current_state: usize, next_input: char) -> usize`**: Determines the next state based on the input character.
- **`search_words(arr: &[String], k: usize, file_path: &str, pb: &ProgressBar) -> io::Result<()>`**: Searches through the file and lists the positions of the words.
- **`sub_dir_list_files(path: &str) -> io::Result<Vec<String>>`**: Lists all file paths in the given directory.

### Data Structures

- **`CHAR_TO_INDEX`**: A `HashMap` that maps characters to their indices.
- **`OUT`**: A `Mutex<HashMap<usize, Vec<usize>>>` that stores the output information for each state.
- **`F`**: A `Mutex<Vec<isize>>` that stores the failure transitions for each state.
- **`G`**: A `Mutex<Vec<Vec<isize>>>` that stores the state transitions.

## License

This project is licensed under the [MIT License](LICENSE). See the `LICENSE` file for more details.
