import socket


class Client:
    def __init__(self, ip="192.168.178.21", port=1001):
        self.remote_addr = (ip, port)
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    def connect(self, username):
        self.username = username
        self.socket.connect(self.remote_addr)
        self.send(self.username)

    def send(self, message):
        self.socket.send(bytes(message + "\n", "UTF-8"))


if __name__ == "__main__":
    client = Client()
    client.connect("testuser")
