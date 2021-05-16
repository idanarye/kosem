import pytest

from kosem import connect_procedure

pytest_plugins = []


@pytest.fixture(scope='session')
def kosem_procedure():
    with connect_procedure('localhost', 8206, 'Local Kosem') as procedure:
        yield procedure


@pytest.fixture(scope='session')
def kosem_human(kosem_procedure):
    with kosem_procedure.request_humans() as request_human:
        human = request_human(kosem_procedure.name)
    return human


@pytest.fixture
def kosem(request, kosem_procedure, kosem_human):
    with kosem_procedure.phase(kosem_procedure.Caption(request.node.name)):
        yield kosem_procedure
