def binpow(a, b):
    if b == 0:
        return 1
    res = binpow(a, b//2)
    if b % 2 == 1:
        return res * res * a
    else:
        return res * res

# print("43434 raises to 4634777:", 2 << 352534466)

def mulmod(a, b):
    if a == 0:
        return 0
    res = mulmod(a//2, b)
    if a > 0 and a % 2 == 0:
        return 2 * res
    elif a > 0 and a % 2 != 0:
        return 2 * res + b

print("5*8:", mulmod(52,89), 52*89)