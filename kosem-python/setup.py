import setuptools

setuptools.setup(
    name='kosem',
    version='0.0.0',
    description='easypy is a collection of python modules that makes developers happy',
    author='Idan Arye',
    author_email='idanarye@gmail.com',
    url='https://github.com/idanarye/kosem',
    license = 'MIT/Apache-2.0',
    packages=setuptools.find_packages(),
    classifiers=["Framework :: Pytest"],
    entry_points={
        'pytest11': ['kosem = kosem']
    },
    install_requires = ['websocket-client'],
)
