# Cwim - Calc What I Mean
adjacent literals supported

no space: highest precedence
```julia
2x + 7 # 2 * x + 7, as expected
2^ x+y # 2 ^ (x+y)
2^x +y # (2^x) + y
5 * -6 # -30
ans # 30

cos 2x + 7 # -> cos(2*x) + 7
cos2pi # -> cos(2*pi)
cos 2pi # -> cos(2*pi)
cos2 pi # -> cos(2)*pi
cos 2 pi # -> cos(2*pi)
```
