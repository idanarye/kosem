from threading import Lock


class MultiQueue(object):
    def __init__(self):
        self.__head = MultiQueueNode(None)
        self.__lock = Lock()

    def receiver(self):
        return MultiQueueReceiver(self.__head)

    def enqueue(self, value):
        node = MultiQueueNode(value)
        with self.__lock:
            self.__head._next = node
            self.__head = node


class MultiQueueNode(object):
    def __init__(self, value):
        self._value = value
        self._next = None


class MultiQueueReceiver(object):
    def __init__(self, node):
        self._node = node

    def __iter__(self):
        return self

    def __next__(self):
        next_node = self._node._next
        if next_node is None:
            raise StopIteration
        else:
            self._node = next_node
            return next_node._value
