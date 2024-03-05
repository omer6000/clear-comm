# CLEARCOMM Project

## Overview
The CLEARCOMM project, developed as part of the Hands-on Dependability course, is designed to tackle the challenges of dependable communication in an error-prone transmission environment. It specifically focuses on establishing a robust communication link between a vessel and a base station, capable of transmitting live video footage with minimal errors.

## Project Aim
This project's primary goal is to create an encoder-decoder system that ensures the reliability of transmitted data over a channel that is susceptible to bit errors and bursts, using the Gilbert-Elliot model to simulate these conditions.

## Implementation Details
I implemented the Hamming code with interleaving to enhance the error detection and correction capabilities of the communication system. This approach significantly improves the system's ability to correct errors caused by common channel disturbances, ensuring the integrity of the transmitted data.

## Technologies Used
- Rust Programming Language
- Asynchronous programming with `async_std`

## Features
- **Hamming Code with Interleaving**: Enhances error correction through a layered approach, combining the reliability of Hamming codes with the robustness of interleaving.
- **Gilbert-Elliot Model**: Simulates an error-prone transmission channel, providing a realistic environment for testing the communication system's dependability.
