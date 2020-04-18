import socket
import time


BUFSIZE = 1024


class TichuError(Exception):
    pass


class Client:
    def __init__(self, ip="127.0.0.1", port=1001):
        self.remote_addr = (ip, port)
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.cards = []

    def connect(self, username):
        self.username = username
        self.socket.connect(self.remote_addr)
        # it is important to use _send_and_recv because recv blocks the thread until
        # it gets the message, this way it is guaranteed that the connection is
        # established before going on
        status, message = self._send_and_recv(self.username)
        if status == "err":
            raise TichuError(message)

    def _send(self, message):
        self.socket.send(bytes(message + "\n", "UTF-8"))

    def _send_and_recv(self, message):
        self._send(message)
        answer = self.socket.recv(BUFSIZE)
        # answers from the server are formatted like "status:message\n"
        status, message = answer.decode("UTF-8").strip().split(":", 1)
        return (status, message)

    def deal(self):
        status, message = self._send_and_recv("deal")
        if status == "err":
            raise TichuError(message)

    def request_cards(self):
        status, message = self._send_and_recv("takecards")
        if status == "ok":
            # the message contains the cards seperated by comma (last one is empty)
            self.cards = message.lower().split(",")[:-1]
        elif status == "err":
            raise TichuError(message)


if __name__ == "__main__":
    client = Client()
    username = input("username: ")
    client.connect(username)
    client.deal()
    client.request_cards()
    print(client.cards)
