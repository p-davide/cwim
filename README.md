# Cwim - Calc What I Mean
adjacent literals supported

no space: highest precedence
```julia
x 2
# Error (use 2x or x(2))
2x + 7 # 2 * x + 7, as expected
2^ x+y # 2 ^ (x+y)
2^x +y # (2^x) + y
2^x +y | x = 5 | y = 3 # 32
ans # 32

w = 2
cos wx + 7 # cos(w*x) + 7 : f(x)
2 cos x + 8) ^ 3
# -> 2 cos (x + 8) ^ 3 (unmatched right bracket takes first named unary function)
# -> 2
```
Wishes:
- live expression preview
    ```julia
    > 2 x^2
    ------
    2 * (x ^ 2)
    ```
- live commands?
    ```julia
    > frac!
    > # Empty numerator!
      ---
      # Empty denominator!
    ```
    from here, navigate using arrows; splitting fraction with an operator should also be possible by putting it on the line
    ```julia
    > cos x) 7
      ------+-
      2x + 7
      (Enter)
    > cos x    7
      ------ + ------
      2x + 7   2x + 7
    ```
    ```julia
    > cos x) 7
      ------*-
      2x + 7
      (Enter)
      cos x  * 7
      ------
      2x + 7
    ```
- vector and matrix math
    ```julia
    [2 3 4] * [5; 6; 7] # 56
    [2 3 4] + [5 6 7] # [10 18 28]
    [2 3 4] + [5 x 7] # [10 18 28]
    a = [1 2; 3 4]
    b = [5 6 7]
    a b
    # Error: Can't multiply 2x2 int `a` by 3x1 int `b`
    ```
