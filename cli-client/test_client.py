import socket
import time


def build_request(message):
    return f"CASP/{message}/\n"


def send_message(s, message, host, port):
    print("Sending:", message, end="")
    s.sendall(message.encode())
    response = s.recv(1024).decode()
    print("Response:", response)

if __name__ == "__main__":
    # Set the host and port to the server you want to send the message to
    host = '127.0.0.1'
    port = 8080

    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect((host, port))

    # Message to send
    message = "Hello, server! This is a test message."

    send_message(s, build_request("AUTH Password123#"), host, port)
    send_message(s, build_request("SET key \"whut\"whut\""), host, port)
    send_message(s, build_request("GET key"), host, port)

    s.close()