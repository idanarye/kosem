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


class Caption(KosemComponent):
    def __init__(self, text):
        self.params = dict(text=text)


class Button(KosemComponent):
    def __init__(self, text):
        self.params = dict(text=text)
