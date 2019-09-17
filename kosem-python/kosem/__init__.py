from .wsjrpc_connection import KosemWsJrpcConnection
from .procedure import KosemProcedure


def connect_procedure(host, port, name):
    connection = KosemWsJrpcConnection('localhost', 8206)
    return KosemProcedure(connection, 'doctor test')
