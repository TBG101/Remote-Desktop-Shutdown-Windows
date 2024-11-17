# Remote Desktop Shutdown (Windows)

Welcome to the **Remote Desktop Shutdown (Windows)** repository! This is a Rust-based companion application that enables **remote shutdown of your PC or execution of Windows Command Prompt (CMD) commands** when requested by an Android app over the local network.

> ⚠️ Note: This repository is the Windows-side software of the system. The corresponding Android application is hosted in a [separate repository](https://github.com/TBG101/Remote-Desktop-Shutdown).

## Features
1. **PC Shutdown:** Accepts shutdown commands from the Android app.
1. **CMD Command Execution:** Runs commands sent from the Android app directly in the Windows Command Prompt.
2. **Local Network Communication:** Operates over the local network for fast interaction.
3. **Lightweight and Efficient:** Built with Rust for high performance and minimal resource usage.

## Prerequisites
1. **Android App:** Install the companion Android application from the [Android Repository](https://github.com/TBG101/Remote-Desktop-Shutdown).
   * This Android app serves as the control interface to send commands to your PC.
2. **Rust Environment (Optional):** If you want to build the application from source, ensure Rust is installed on your system.
   * Install Rust
3. Your PC and Android device must be on the same local network.

## Installation
### Prebuilt Release
1. Download the latest [release](https://github.com/TBG101/Remote-Desktop-Shutdown-Windows/releases) from the Releases page.
2. Extract the downloaded archive and run the executable:
    ```bash
    RemoteDesktopShutdown.exe
    ```
### Building from Source
1. Clone this repository:
    ```bash
    git clone https://github.com/TBG101/Remote-Desktop-Shutdown-Windows
    cd remote-desktop-shutdown-windows
    Build the project:
    ```
2. Build the project
   ```bash
    cargo build --release
   ```
3. The executable will be available in the ``target/release`` directory.

## Usage
1. **Run the Windows App:**
   * Start the application by double-clicking the executable or running it from the terminal.
   * By default, it listens on a specific port 8888 for requests.
   * The program will always auto-start
2. **Perform Actions:**
   * Send shutdown or CMD commands from the Android app to control your PC remotely.

## Built With
   * **Rust** - Programming language for building a secure and high-performance application.


## Contributing
Contributions are welcome!
If you find bugs or have feature suggestions, feel free to open an issue or submit a pull request.

## Fork the repo.
1. **Create a feature branch:** ```git checkout -b feature/your-feature```.
2. **Commit your changes:** ```git commit -m "Add your feature"```.
3. **Push to the branch:** ```git push origin feature/your-feature```.
4. Open a pull request.

## License
This project is licensed under the MIT License.

## Links
* [Android App Repository](https://github.com/TBG101/Remote-Desktop-Shutdown)
  
## Contact
For any questions or feedback, reach out at **ziedhrz@gmail.com**






