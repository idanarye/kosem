import time


class KosemProcedure(object):
    def __init__(self, connection, name):
        self._con = connection
        self.name = name
        self._login()

    def _login(self):
        self.uid = 'call result', self._con.call('LoginAsProcedure', name=self.name)
