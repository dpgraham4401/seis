from seis.lib import my_first_function, create_gather


class TestSeisLib:
    def test_this_setup(self):
        assert 2 == 2

    def test_lib_function(self):
        blah_four = my_first_function()
        assert int(blah_four) == 4

    def test_lib_object(self):
        blah_four = create_gather()
        print(blah_four)
