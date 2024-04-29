class KosemComponent(object):
    name = None

    def named(self, name):
        self.name = name
        return self

    def to_json(self):
        return dict(
            type=type(self).__name__,
            params=self.params,
            name=self.name)

    is_message_relevant = None


class Caption(KosemComponent):
    def __init__(self, text: str):
        self.params = dict(text=text)


class Button(KosemComponent):
    def __init__(self, text: str):
        self.params = dict(text=text)

    def is_message_relevant(self, msg):
        return msg['method'] == 'ButtonClicked'

class Textbox(KosemComponent):
    def __init__(self, text: str):
        self.params = dict(text=text)
