import itertools
import json
from threading import Lock

import websocket

from .multiqueue import MultiQueue


class KosemWsJrpcConnection(object):
    def __init__(self, host, port):
        self.__con = websocket.create_connection(
            'ws://{host}:{port}/ws-jrpc'.format_map(locals()),
            timeout=1)
        self.__message_ids = itertools.count(1)
        self.__messages_received = 0
        self.__lock = Lock()
        self.__queue = MultiQueue()

    def close(self):
        self.__con.close()

    def notify(self, method, **kwargs):
        msg = dict(
            jsonrpc="2.0",
            method=method,
            params=kwargs)

        self.__con.send(json.dumps(msg))

    def notify_and_stream(self, method, **kwargs):
        stream = self.stream_messages()
        self.notify(method, **kwargs)
        return stream

    def call(self, method, *args, **kwargs):
        assert not (args and kwargs), 'both args and kwargs specified'

        message_id = next(self.__message_ids)

        msg = dict(
            jsonrpc="2.0",
            id=message_id,
            method=method,
            params=args or kwargs)

        stream = self.stream_messages(also_responses=True)
        self.__con.send(json.dumps(msg))
        for msg in stream:
            if 'method' not in msg:
                if msg['id'] == message_id:
                    try:
                        return msg['result']
                    except KeyError:
                        raise KosemWsJrpcError(**msg['error'])

    def stream_messages(self, also_responses=False):
        receiver = self.__queue.receiver()
        while True:
            current_messages_received = self.__messages_received
            try:
                msg = next(receiver)
            except StopIteration:
                with self.__lock:
                    if current_messages_received < self.__messages_received:
                        # We got new messages from another thread
                        continue
                    try:
                        raw_message = self.__con.recv()
                    except websocket.WebSocketTimeoutException:
                        continue
                    except websocket.WebSocketConnectionClosedException:
                        return
                    msg = json.loads(raw_message)
                    self.__queue.enqueue(msg)
                    self.__messages_received += 1
            else:
                print(also_responses, msg)
                if also_responses or 'method' in msg:
                    yield msg


class KosemWsJrpcError(Exception):
    def __init__(self, code, message, data=None):
        if data:
            super().__init__('[%s]%s: %s' % (code, message, data))
        else:
            super().__init__('[%s]%s' % (code, message))
        self.code = code
        self.message = message
        self.data = data
