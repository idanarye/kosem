import itertools
import json
import threading

import websocket

class KosemConnection(object):
    def __init__(self, host, port):
        self.__con = websocket.create_connection('ws://{host}:{port}/ws-jrpc'.format_map(locals()))
        self.__message_ids = itertools.count(1)
        self.__thread = threading.Thread(target=self.__recieve, daemon=True)
        self.__thread.start()
        self.__queue = []

    def close(self):
        self.__con.close()

    def notify(self, method, **kwargs):
        message = dict(
            jsonrpc="2.0",
            method=method,
            params=kwargs)

        self.__con.send(json.dumps(message))

    def call(self, method, *args, **kwargs):
        assert not (args and kwargs), 'both args and kwargs specified'

        message_id = next(self.__message_ids)

        message = dict(
            jsonrpc="2.0",
            id=message_id,
            method=method,
            params=args or kwargs)

        self.__con.send(json.dumps(message))

    def __recieve(self):
        while True:
            try:
                raw_message = self.__con.recv()
            except websocket.WebSocketConnectionClosedException:
                return
            if raw_message:
                message = json.loads(raw_message)
                if message['method'] == 'LoginConfirmed':
                    self.uid = message['params']['uid']
                else:
                    self.__queue.append(message)

    def receive(self):
        return self.__queue.pop(0)

    def receive_all(self):
        while self.__queue:
            yield self.__queue.pop(0)
