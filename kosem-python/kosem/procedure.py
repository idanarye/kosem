import time


class KosemProcedure(object):
    def __init__(self, connection, name):
        self._con = connection
        self.name = name
        self._login()

    def _login(self):
        self._con.call('LoginAsProcedure', name='doctor test')
        while not hasattr(self._con, 'uid'):
            time.sleep(0.001)
