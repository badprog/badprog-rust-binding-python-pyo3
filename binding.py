#
import rust_classic_operations

# Comp intantiation
comp = rust_classic_operations.PyComputation()

# Add
comp.add(10, 50)
comp.add(5, 100)
comp.add(8, 40)

total_add = comp.get_val_add()
print(f"Total add: {total_add}")

# Sub
comp.sub(10, 50)
comp.sub(5, 100)
comp.sub(8, 40)

total_sub = comp.get_val_sub()
print(f"Total sub: {total_sub}")

# Div
comp.div(10, 50)
comp.div(5, 100)
comp.div(8, 40)

total_div = comp.get_val_div()
print(f"Total div: {total_div}")

# Mul
comp.mul(10, 50)
comp.mul(5, 100)
comp.mul(8, 40)

total_mul = comp.get_val_mul()
print(f"Total mul: {total_mul}")
