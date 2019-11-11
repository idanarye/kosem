from .wsjrpc_connection import KosemWsJrpcConnection
from .procedure import KosemProcedure


def connect_procedure(host, port, name):
    connection = KosemWsJrpcConnection(host, port)
    return KosemProcedure(connection, name)
