import time
from contextlib import contextmanager


class KosemProcedure(object):
    def __init__(self, connection, name):
        self._con = connection
        self.name = name
        self.humans = []
        self._login()

    def _login(self):
        self.uid = self._con.call('LoginAsProcedure', name=self.name)

    @contextmanager
    def request_humans(self):
        stream = self._con.stream_messages()
        human_requests = {}

        def request_dlg(name):
            uid = self._con.call('RequestHuman', name=name)
            human = KosemHuman(self, uid)
            human_requests[uid] = human
            return human
        yield request_dlg

        for msg in stream:
            if msg['method'] == 'HumanJoined':
                human = human_requests.pop(msg['params']['request_uid'], None)
                if human is None:
                    continue
                human._join_confirmation(msg['params'])
                self.humans.append(human)
                if not human_requests:
                    break

    def push_phase(self):
        uid = self._con.call('PushPhase')
        return KosemPhase(self, uid)

    @contextmanager
    def phase(self):
        phase = self.push_phase()
        yield phase


class KosemHuman(object):
    def __init__(self, procedure, uid):
        self.procedure = procedure
        self.request_uid = uid

    def _join_confirmation(self, msg_params):
        self.uid = msg_params['human_uid']
        self.name = msg_params['human_name']


class KosemPhase(object):
    def __init__(self, procedure, uid):
        self.procedure = procedure
        self.uid = uid
        self.next_component_ordinal = 0

    def __gen_ordinal(self):
        ordinal = self.next_component_ordinal
        self.next_component_ordinal += 1
        return ordinal
