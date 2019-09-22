import time


class KosemProcedure(object):
    def __init__(self, connection, name):
        self._con = connection
        self.name = name
        self._login()

    def _login(self):
        for msg in self._con.notify_and_stream('LoginAsProcedure', name=self.name):
            if msg['method'] == 'LoginConfirmed':
                self.uid = msg['params']['uid']
                return
