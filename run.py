from pyo3_subclass import make_sub, make_base, SubClass, BaseClass


base = make_base()
assert isinstance(base, BaseClass)
base2 = base.clone_instance()
assert isinstance(base2, BaseClass)

sub = make_sub()
assert isinstance(sub, SubClass)
sub2 = sub.clone_instance()
assert isinstance(sub2, SubClass)

print("done!")
