from subprocess import check_output


def sh(args=[]):
    return check_output(['./target/debug/sctn'] + args)


assert sh(['one\ntwo\nthree', 'one\n', 'one\n']) == b'one\n'
assert sh(['one\ntwo\nthree', 'one\nfour']) == b'one\n'
assert sh(['one', 'one']) == b'one\n'
assert sh(['one', 'two']) == b''
assert sh(['one']) == b''
assert sh() == b''
