from subprocess import check_output


def sh(args=[]):
    return check_output(['./target/debug/sctn'] + args)


assert sh(['one', 'one']) == b'one\n'
assert sh(['one', 'two']) == b''
assert sh(['one']) == b''
assert sh() == b''
