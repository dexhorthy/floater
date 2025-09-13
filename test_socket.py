#!/usr/bin/env python3

import socket
import json
import sys

def send_command(command):
    socket_path = "/tmp/floater.sock"

    try:
        # Create a Unix domain socket
        sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)

        # Connect to the socket
        sock.connect(socket_path)

        # Send command
        command_json = json.dumps(command) + "\n"
        sock.sendall(command_json.encode())

        # Receive response
        response = sock.recv(1024).decode().strip()
        print(f"Response: {response}")

        sock.close()

    except FileNotFoundError:
        print(f"Socket file {socket_path} not found. Make sure the Tauri app is running.")
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage:")
        print("  python3 test_socket.py show \"Hello World\"")
        print("  python3 test_socket.py hide")
        sys.exit(1)

    action = sys.argv[1]

    if action == "show" and len(sys.argv) >= 3:
        content = sys.argv[2]
        command = {"action": "show", "content": content}
        send_command(command)
    elif action == "hide":
        command = {"action": "hide"}
        send_command(command)
    else:
        print("Invalid command. Use 'show <content>' or 'hide'")