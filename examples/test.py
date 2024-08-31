compiled_code = compile(open("raytracer.py").read(), '<string>', 'exec')
print(compiled_code.co_code)