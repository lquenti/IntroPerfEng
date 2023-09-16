def improve(a, b):
    return ((a - b) / abs(b)) * 100

a = 307.19
#b = 205.58
b = 7.814

print(f"improved: {improve(a,b)}%")
