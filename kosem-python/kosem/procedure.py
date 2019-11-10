import time
from contextlib import contextmanager


class KosemProcedure(object):
    from .components import Caption, Button

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

    def push_phase(self, *components):
        is_message_relevant = [
            component.is_message_relevant
            for component in components
            if component.is_message_relevant is not None]
        if is_message_relevant:
            stream = self._con.stream_messages() 
        else:
            stream = None
        uid = self._con.call('PushPhase', components=[c.to_json() for c in components])
        if stream:
            stream = (
                msg for msg in stream
                if msg['params'].get('phase_uid') == uid
                and any(pred(msg) for pred in is_message_relevant))
        return KosemPhase(self, uid, stream)

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
    def __init__(self, procedure, uid, stream):
        self.procedure = procedure
        self.uid = uid
        self.next_component_ordinal = 0
        self.stream = stream

    def __gen_ordinal(self):
        ordinal = self.next_component_ordinal
        self.next_component_ordinal += 1
        return ordinal

    def add_caption(self, text):
        ordinal = self.__gen_ordinal()
        self.procedure._con.call('AddComponent',
                                 phase_uid=self.uid,
                                 ordinal=ordinal,
                                 type='Caption',
                                 params=dict(
                                     text=text,
                                 ))

    def relevant_messages(self):
        if self.stream:
            for msg in self.stream:
                yield msg

    def wait_for_button(self):
        for msg in self.relevant_messages():
            print('Got msg', msg)
            if msg['method'] == 'ButtonClicked':
                return msg['params'].get('button_name', None)
