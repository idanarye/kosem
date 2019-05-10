import itertools
import json

import websocket

class KosemConnection(object):
    def __init__(self, host, port):
        self.__con = websocket.create_connection('ws://{host}:{port}/ws-jrpc'.format_map(locals()))
        self.__message_ids = itertools.count(1)

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
