# Download Sorter

**Download Sorter** is a Rust program designed to help you keep your downloads folder organized. It automatically sorts files into specific directories based on their file extensions, making it easier to find and manage your files.

## Installation

### Installation from Source

To install and run **Download Sorter** from source, follow these steps:

1. Ensure that you have Rust and Cargo installed on your system. You can install Rust by following the instructions on the official [Rust website](https://www.rust-lang.org/).

2. Clone this repository and navigate to the project directory:

   ```
   git clone https://github.com/sivaprasad-r/download_sorter.git
   cd download_sorter
   ```

3. Build the program using Cargo:

   ```
   cargo build --release
   ```

4. Run the program:

   ```
   cargo run
   ```

### Installation from Executable

You can also download the compiled executable for your operating system from the [Releases](https://github.com/sivaprasad-r/download_sorter/releases) section of this repository. Choose the version that matches your operating system.

## Usage

1. Upon running **Download Sorter**, it will create subdirectories within your downloads folder for each file category if they don't already exist.

2. The program will continuously monitor your downloads folder and organize any newly added files into the appropriate subdirectories based on their file extensions.

3. If any errors occur during the file organization process, the program will display an error dialog with relevant information.

4. You can customize the file extensions and target directories by modifying the constants in the code.

## Customization

You can customize the file extensions and target directories by modifying the following constants in the code:

- `IMAGE_EXTENSIONS`
- `VIDEO_EXTENSIONS`
- `AUDIO_EXTENSIONS`
- `DOCUMENT_EXTENSIONS`
- `PROGRAM_EXTENSIONS`
- `ARCHIVE_EXTENSIONS`
- `DIRECTORIES_TO_CHECK`
- `DIRECTORIES_MAP`
