from os.path import exists
from subprocess import check_output

PATH = None

for item in ['debug', 'release']:
    path = './target/{0}/sctn'.format(item)
    if exists(path):
        PATH = path

if not PATH:
    raise RuntimeError("No sctn executeable found.")


def sh(args=[]):
    return check_output([PATH] + args)


assert sh(['one\ntwo\nthree', 'one\ntwo\n', 'one']) == b'one\n'
assert sh(['one\ntwo\nthree', 'one\n', 'two\n']) == b''
assert sh(['one\ntwo\nthree', 'one\n', 'one\n']) == b'one\n'
assert sh(['one\ntwo\nthree', 'one\nfour']) == b'one\n'
assert sh(['one', 'one']) == b'one\n'
assert sh(['one', 'two']) == b''
assert sh(['one']) == b''
assert sh() == b''
