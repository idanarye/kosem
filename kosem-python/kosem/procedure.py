import time
from contextlib import contextmanager


class KosemProcedure(object):
    def __init__(self, connection, name):
        self._con = connection
        self.name = name
        self._login()

    def _login(self):
        self.uid = 'call result', self._con.call('LoginAsProcedure', name=self.name)

    def push_phase(self):
        uid = self._con.call('PushPhase')
        return KosemPhase(self, uid)

    @contextmanager
    def phase(self):
        phase = self.push_phase()
        yield phase


class KosemPhase(object):
    def __init__(self, procedure, uid):
        self.procedure = procedure
        self.uid = uid
