from seis import seis


class TestSeisPkg:
    def test_imports(self):
        new_gather = seis.Gather(1, 2, 3, 0.01, seis.Precision.Double)
        print(new_gather.precision)
        assert type(new_gather) is seis.Gather
